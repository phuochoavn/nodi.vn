use axum::{Router, Json, extract::{State, Query}, routing::get};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::verify_token;

fn extract_admin(headers: &HeaderMap, secret: &str) -> Result<(), AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    let claims = verify_token(token, secret)?;
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    Ok(())
}

#[derive(Deserialize)]
struct MonthsQuery { months: Option<i32> }

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/market/overview", get(overview))
        .route("/api/admin/market/products", get(products))
        .route("/api/admin/market/supply-chain", get(supply_chain))
        .route("/api/admin/market/credit", get(credit))
        .route("/api/admin/market/cashflow", get(cashflow))
}

// ===== TAB 1: Overview =====
async fn overview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<MonthsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let months = q.months.unwrap_or(12);

    // Stats
    let total_rev: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(final_amount::bigint), 0) FROM synced_invoices"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    let total_debt_customer: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt::bigint), 0) FROM synced_customers"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    let total_debt_supplier: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt::bigint), 0) FROM synced_suppliers"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    let total_products: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT (store_id, local_id)) FROM synced_products"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    let active_stores: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM stores WHERE is_active = true AND hwid IS NOT NULL"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    // Revenue by month (TIMESTAMP column)
    let rev_trend = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT TO_CHAR(created_at, 'YYYY-MM'), COALESCE(SUM(final_amount::bigint), 0), COUNT(*) \
         FROM synced_invoices \
         WHERE created_at >= NOW() - ($1 || ' months')::interval \
         GROUP BY 1 ORDER BY 1"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    let trend: Vec<Value> = rev_trend.iter().map(|r| json!({
        "month": r.0, "revenue": r.1, "orders": r.2
    })).collect();

    Ok(Json(json!({
        "total_revenue": total_rev.0,
        "total_debt_customer": total_debt_customer.0,
        "total_debt_supplier": total_debt_supplier.0,
        "total_products": total_products.0,
        "active_stores": active_stores.0,
        "revenue_by_month": trend
    })))
}

// ===== TAB 2: Products =====
async fn products(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<MonthsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let months = q.months.unwrap_or(12);

    // Top 10 products by revenue
    let top = sqlx::query_as::<_, (Option<String>, i64, i64, i64)>(
        "SELECT ii.product_name, COALESCE(SUM(ii.quantity::bigint), 0), COALESCE(SUM(ii.total::bigint), 0), COUNT(DISTINCT ii.store_id) \
         FROM synced_invoice_items ii \
         JOIN synced_invoices inv ON inv.store_id = ii.store_id AND inv.local_id = ii.invoice_local_id \
         WHERE inv.created_at >= NOW() - ($1 || ' months')::interval \
         GROUP BY ii.product_name ORDER BY 3 DESC LIMIT 10"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    let top_products: Vec<Value> = top.iter().map(|r| json!({
        "name": r.0, "quantity": r.1, "revenue": r.2, "stores": r.3
    })).collect();

    // Revenue by category
    let cats = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT p.category, COALESCE(SUM(ii.total::bigint), 0) \
         FROM synced_invoice_items ii \
         JOIN synced_products p ON p.store_id = ii.store_id AND p.local_id = ii.product_local_id \
         JOIN synced_invoices inv ON inv.store_id = ii.store_id AND inv.local_id = ii.invoice_local_id \
         WHERE inv.created_at >= NOW() - ($1 || ' months')::interval \
         GROUP BY p.category ORDER BY 2 DESC"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    let categories: Vec<Value> = cats.iter().map(|r| json!({
        "category": r.0.as_deref().unwrap_or("Khác"), "revenue": r.1
    })).collect();

    // Top manufacturers
    let mfrs = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT p.manufacturer, COALESCE(SUM(ii.total::bigint), 0), COUNT(DISTINCT p.local_id) \
         FROM synced_invoice_items ii \
         JOIN synced_products p ON p.store_id = ii.store_id AND p.local_id = ii.product_local_id \
         JOIN synced_invoices inv ON inv.store_id = ii.store_id AND inv.local_id = ii.invoice_local_id \
         WHERE inv.created_at >= NOW() - ($1 || ' months')::interval \
           AND p.manufacturer IS NOT NULL AND p.manufacturer != '' \
         GROUP BY p.manufacturer ORDER BY 2 DESC LIMIT 10"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    let manufacturers: Vec<Value> = mfrs.iter().map(|r| json!({
        "name": r.0, "revenue": r.1, "products_count": r.2
    })).collect();

    // Average margin
    let margin: Option<(Option<f64>,)> = sqlx::query_as(
        "SELECT AVG((ii.unit_price::float - p.cost_price) / NULLIF(ii.unit_price::float, 0) * 100) \
         FROM synced_invoice_items ii \
         JOIN synced_products p ON p.store_id = ii.store_id AND p.local_id = ii.product_local_id \
         WHERE p.cost_price > 0 AND ii.unit_price > 0"
    ).fetch_optional(&state.pool).await.unwrap_or(None);

    let avg_margin = margin.and_then(|r| r.0).unwrap_or(0.0);

    Ok(Json(json!({
        "top_products": top_products,
        "revenue_by_category": categories,
        "top_manufacturers": manufacturers,
        "avg_margin": (avg_margin * 100.0).round() / 100.0
    })))
}

// ===== TAB 3: Supply Chain =====
async fn supply_chain(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<MonthsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let months = q.months.unwrap_or(12);

    // Channel distribution: company vs independent
    let channels = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT CASE WHEN s.company IS NOT NULL AND s.company != '' THEN 'Công ty / Đại lý' ELSE 'Cá nhân / Khác' END, \
         COALESCE(SUM(po.total_amount::bigint), 0), COUNT(*) \
         FROM synced_purchase_orders po \
         JOIN synced_suppliers s ON s.store_id = po.store_id AND s.local_id = po.supplier_id \
         GROUP BY 1 ORDER BY 2 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let channel_distribution: Vec<Value> = channels.iter().map(|r| json!({
        "channel": r.0, "amount": r.1, "orders": r.2
    })).collect();

    // Top suppliers by purchase volume
    let top_sup = sqlx::query_as::<_, (Option<String>, Option<String>, i64, i64, i64)>(
        "SELECT s.name, s.company, COALESCE(SUM(po.total_amount::bigint), 0), COUNT(*), COALESCE(s.total_debt::bigint, 0) \
         FROM synced_suppliers s \
         LEFT JOIN synced_purchase_orders po ON s.store_id = po.store_id AND s.local_id = po.supplier_id \
         GROUP BY s.id, s.name, s.company, s.total_debt ORDER BY 3 DESC LIMIT 10"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let top_suppliers: Vec<Value> = top_sup.iter().map(|r| json!({
        "name": r.0, "company": r.1, "total_purchase": r.2, "orders": r.3, "current_debt": r.4
    })).collect();

    // Total supplier debt
    let total_debt: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt::bigint), 0) FROM synced_suppliers"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    // Supplier payments by month (created_at is TEXT)
    let payments = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT LEFT(created_at, 7), COALESCE(SUM(amount), 0) \
         FROM synced_payment_vouchers \
         WHERE created_at >= TO_CHAR(NOW() - ($1 || ' months')::interval, 'YYYY-MM') \
         GROUP BY 1 ORDER BY 1"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    let payments_by_month: Vec<Value> = payments.iter().map(|r| json!({
        "month": r.0, "amount": r.1
    })).collect();

    Ok(Json(json!({
        "channel_distribution": channel_distribution,
        "top_suppliers": top_suppliers,
        "total_supplier_debt": total_debt.0,
        "supplier_payments_by_month": payments_by_month
    })))
}

// ===== TAB 4: Credit / Debt =====
async fn credit(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(_q): Query<MonthsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Total customer debt
    let total_debt: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt::bigint), 0) FROM synced_customers"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    // Debt by season
    let seasons = sqlx::query_as::<_, (Option<String>, f64, f64)>(
        "SELECT season, \
         COALESCE(SUM(CASE WHEN transaction_type = 'DEBIT' THEN amount ELSE 0 END), 0), \
         COALESCE(SUM(CASE WHEN transaction_type = 'CREDIT' THEN amount ELSE 0 END), 0) \
         FROM synced_customer_transactions \
         WHERE season IS NOT NULL AND season != '' \
         GROUP BY season ORDER BY 2 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let debt_by_season: Vec<Value> = seasons.iter().map(|r| json!({
        "season": r.0.as_deref().unwrap_or("Khác"), "debit": r.1, "credit": r.2
    })).collect();

    // Collection rate
    let collection: Option<(f64, f64)> = sqlx::query_as(
        "SELECT \
         COALESCE(SUM(CASE WHEN transaction_type = 'DEBIT' THEN amount ELSE 0 END), 0), \
         COALESCE(SUM(CASE WHEN transaction_type = 'CREDIT' THEN amount ELSE 0 END), 0) \
         FROM synced_customer_transactions"
    ).fetch_optional(&state.pool).await.unwrap_or(None);

    let (total_debit, total_credit) = collection.unwrap_or((0.0, 0.0));
    let collection_rate = if total_debit > 0.0 { (total_credit / total_debit * 10000.0).round() / 100.0 } else { 0.0 };

    // Top debtors
    let debtors = sqlx::query_as::<_, (Option<String>, Option<String>, i64, Option<f64>)>(
        "SELECT name, phone, COALESCE(total_debt::bigint, 0), credit_limit \
         FROM synced_customers WHERE total_debt > 0 ORDER BY total_debt DESC LIMIT 10"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let top_debtors: Vec<Value> = debtors.iter().map(|r| {
        let usage = if let Some(limit) = r.3 { if limit > 0.0 { (r.2 as f64 / limit * 100.0).round() } else { 0.0 } } else { 0.0 };
        json!({
            "name": r.0, "phone": r.1, "debt": r.2, "credit_limit": r.3, "usage_pct": usage
        })
    }).collect();

    // Payment methods from invoice_payments
    let methods = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT payment_method, COUNT(*), COALESCE(SUM(amount::bigint), 0) \
         FROM synced_invoice_payments \
         GROUP BY payment_method ORDER BY 3 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let payment_methods: Vec<Value> = methods.iter().map(|r| json!({
        "method": r.0.as_deref().unwrap_or("Khác"), "count": r.1, "amount": r.2
    })).collect();

    Ok(Json(json!({
        "total_customer_debt": total_debt.0,
        "debt_by_season": debt_by_season,
        "collection_rate": collection_rate,
        "top_debtors": top_debtors,
        "payment_methods": payment_methods
    })))
}

// ===== TAB 5: Cash Flow =====
async fn cashflow(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<MonthsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let months = q.months.unwrap_or(12);

    // Total in / out
    let totals = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT flow_type, COALESCE(SUM(amount), 0) FROM synced_cash_transactions GROUP BY flow_type"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let mut total_in: i64 = 0;
    let mut total_out: i64 = 0;
    for t in &totals {
        match t.0.as_deref() {
            Some("IN") => total_in = t.1,
            Some("OUT") => total_out = t.1,
            _ => {}
        }
    }

    // Cash flow by month (created_at is TEXT)
    let monthly = sqlx::query_as::<_, (Option<String>, Option<String>, i64)>(
        "SELECT LEFT(created_at, 7), flow_type, COALESCE(SUM(amount), 0) \
         FROM synced_cash_transactions \
         WHERE created_at >= TO_CHAR(NOW() - ($1 || ' months')::interval, 'YYYY-MM') \
         GROUP BY 1, 2 ORDER BY 1"
    ).bind(months.to_string()).fetch_all(&state.pool).await.unwrap_or_default();

    // Pivot monthly data
    let mut month_map: std::collections::BTreeMap<String, (i64, i64)> = std::collections::BTreeMap::new();
    for r in &monthly {
        if let Some(m) = &r.0 {
            let entry = month_map.entry(m.clone()).or_insert((0, 0));
            match r.1.as_deref() {
                Some("IN") => entry.0 = r.2,
                Some("OUT") => entry.1 = r.2,
                _ => {}
            }
        }
    }
    let cashflow_by_month: Vec<Value> = month_map.iter().map(|(m, (i, o))| json!({
        "month": m, "income": i, "expense": o
    })).collect();

    // Expense breakdown by category (flow_type = OUT)
    let expenses = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT category, COALESCE(SUM(amount), 0) \
         FROM synced_cash_transactions WHERE flow_type = 'OUT' \
         GROUP BY category ORDER BY 2 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let expense_breakdown: Vec<Value> = expenses.iter().map(|r| json!({
        "category": r.0.as_deref().unwrap_or("Khác"), "amount": r.1
    })).collect();

    // Store balances
    let balances = sqlx::query_as::<_, (i32, Option<String>, i64, Option<String>)>(
        "SELECT sf.store_id, s.owner_name, COALESCE(sf.current_balance, 0), sf.updated_at \
         FROM synced_store_funds sf \
         JOIN stores s ON s.id = sf.store_id \
         ORDER BY sf.current_balance DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let store_balances: Vec<Value> = balances.iter().map(|r| json!({
        "store_id": r.0, "store_name": r.1, "balance": r.2, "updated_at": r.3
    })).collect();

    Ok(Json(json!({
        "total_in": total_in,
        "total_out": total_out,
        "net_profit": total_in - total_out,
        "cashflow_by_month": cashflow_by_month,
        "expense_breakdown": expense_breakdown,
        "store_balances": store_balances
    })))
}
