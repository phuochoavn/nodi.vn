use axum::{Router, Json, extract::{State, Path, Query}, routing::{get, put, post}};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::http::header;
use serde::Deserialize;
use serde_json::{json, Value};
use rust_xlsxwriter::{Workbook, Format, FormatAlign, Color};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{verify_token, Claims};

fn extract_claims(headers: &HeaderMap, secret: &str) -> Result<Claims, AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    verify_token(token, secret)
}

fn get_store_id(claims: &Claims, query_store_id: Option<i32>) -> i32 {
    if claims.role == "admin" {
        query_store_id.unwrap_or(claims.store_id)
    } else {
        claims.store_id
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/dashboard/overview", get(overview))
        .route("/api/dashboard/orders", get(orders_list))
        .route("/api/dashboard/orders/{id}", get(order_detail))
        .route("/api/dashboard/inventory", get(inventory))
        .route("/api/dashboard/debts", get(debts))
        .route("/api/dashboard/reports/revenue", get(revenue_report))
        .route("/api/dashboard/reports/top-products", get(top_products))
        .route("/api/dashboard/settings", get(settings))
        .route("/api/dashboard/settings/password", put(change_password))
        .route("/api/dashboard/purchase-orders", get(purchase_orders_list))
        .route("/api/dashboard/staff", get(staff_list))
        .route("/api/dashboard/staff/{id}/permissions", put(staff_update_permissions))
        .route("/api/dashboard/staff/{id}/toggle-active", put(staff_toggle_active))
        .route("/api/dashboard/staff/{id}/pin", put(staff_update_pin))
        .route("/api/dashboard/inventory/export", get(inventory_export))
        .route("/api/dashboard/notifications", get(notifications))
        .route("/api/dashboard/force-resync", post(force_resync))
}

// ===== API 1: Overview =====
#[derive(Deserialize)]
struct StoreQuery { store_id: Option<i32> }

async fn overview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<StoreQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);

    let revenue_today: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount)::bigint, 0) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) = CURRENT_DATE"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let revenue_month: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount)::bigint, 0) FROM synced_invoices WHERE store_id = $1 AND DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE)"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let orders_today: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) = CURRENT_DATE"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let orders_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1 AND DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE)"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let total_products: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_products WHERE store_id = $1"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let low_stock: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_products WHERE store_id = $1 AND stock_quantity::float8 <= COALESCE(min_stock, 5)"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let expiring: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_product_batches WHERE store_id = $1 AND remaining_quantity > 0 AND expiry_date <= (CURRENT_DATE + INTERVAL '30 days')::text"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let total_customers: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_customers WHERE store_id = $1"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let debt_customers: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt)::bigint, 0) FROM synced_customers WHERE store_id = $1 AND total_debt > 0"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let debt_suppliers: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_debt)::bigint, 0) FROM synced_suppliers WHERE store_id = $1 AND total_debt > 0"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let store_balance: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT COALESCE(current_balance)::bigint FROM synced_store_funds WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;
    let balance_val = store_balance.and_then(|r| r.0).unwrap_or(0);

    let last_sync: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as(
        "SELECT MAX(synced_at) FROM synced_invoices WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;

    Ok(Json(json!({
        "revenue_today": revenue_today.0,
        "revenue_this_month": revenue_month.0,
        "orders_today": orders_today.0,
        "orders_this_month": orders_month.0,
        "total_products": total_products.0,
        "low_stock_count": low_stock.0,
        "expiring_soon_count": expiring.0,
        "total_customers": total_customers.0,
        "total_debt_customers": debt_customers.0,
        "total_debt_suppliers": debt_suppliers.0,
        "store_balance": balance_val,
        "last_sync_at": last_sync.and_then(|s| s.0).map(|d| d.to_string())
    })))
}

// ===== API 2: Orders List =====
#[derive(Deserialize)]
struct OrdersQuery {
    page: Option<i64>,
    limit: Option<i64>,
    from: Option<String>,
    to: Option<String>,
    store_id: Option<i32>,
}

async fn orders_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<OrdersQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT i.local_id, i.invoice_number, c.name, i.total_amount::float8, i.payment_method, i.status, i.created_at \
         FROM synced_invoices i LEFT JOIN synced_customers c ON i.customer_id = c.local_id AND c.store_id = i.store_id \
         WHERE i.store_id = $1 ORDER BY i.created_at DESC LIMIT $2 OFFSET $3"
    ).bind(sid).bind(limit).bind(offset).fetch_all(&state.pool).await?;

    let orders: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0,
        "invoice_number": r.1,
        "customer_name": r.2.as_deref().unwrap_or("Khách lẻ"),
        "total_amount": r.3.unwrap_or(0.0),
        "payment_method": r.4.as_deref().unwrap_or("CASH"),
        "status": r.5.as_deref().unwrap_or("COMPLETED"),
        "created_at": r.6.map(|d| d.to_string()),
    })).collect();

    Ok(Json(json!({ "orders": orders, "total": total.0, "page": page, "limit": limit })))
}

// ===== API 3: Order Detail =====
async fn order_detail(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    let order = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT i.local_id, i.invoice_number, c.name, c.phone, i.total_amount::float8, i.discount_amount, i.final_amount, i.payment_method, i.status, i.created_at \
         FROM synced_invoices i LEFT JOIN synced_customers c ON i.customer_id = c.local_id AND c.store_id = i.store_id \
         WHERE i.local_id = $1 AND i.store_id = $2"
    ).bind(id).bind(sid).fetch_optional(&state.pool).await?;

    let o = match order {
        Some(o) => o,
        None => return Err(AppError::NotFound("Order not found".into())),
    };

    let items = sqlx::query_as::<_, (Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>)>(
        "SELECT p.name, ii.unit_name, ii.quantity, ii.unit_price, ii.total \
         FROM synced_invoice_items ii LEFT JOIN synced_products p ON ii.product_local_id = p.local_id AND p.store_id = ii.store_id \
         WHERE ii.invoice_local_id = $1 AND ii.store_id = $2"
    ).bind(id).bind(sid).fetch_all(&state.pool).await?;

    let items_json: Vec<Value> = items.iter().map(|i| json!({
        "product_name": i.0.as_deref().unwrap_or("N/A"),
        "unit_name": i.1.as_deref().unwrap_or(""),
        "quantity": i.2.unwrap_or(0.0),
        "unit_price": i.3.unwrap_or(0.0),
        "subtotal": i.4.unwrap_or(0.0),
    })).collect();

    Ok(Json(json!({
        "order": {
            "id": o.0, "invoice_number": o.1, "customer_name": o.2.as_deref().unwrap_or("Khách lẻ"),
            "customer_phone": o.3, "total_amount": o.4, "discount_amount": o.5, "final_amount": o.6,
            "payment_method": o.7, "status": o.8, "created_at": o.9.map(|d| d.to_string()),
            "items": items_json
        }
    })))
}

// ===== API 4: Inventory =====
#[derive(Deserialize)]
struct InventoryQuery {
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
    filter: Option<String>,
    store_id: Option<i32>,
}

async fn inventory(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<InventoryQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = (page - 1) * limit;

    let mut where_clause = "WHERE store_id = $1".to_string();
    let search = q.search.unwrap_or_default();
    if !search.is_empty() {
        where_clause.push_str(" AND (name ILIKE '%' || $4 || '%' OR barcode ILIKE '%' || $4 || '%')");
    }

    match q.filter.as_deref() {
        Some("low_stock") => where_clause.push_str(" AND stock_quantity <= COALESCE(min_stock, 5)"),
        Some("expiring") => { /* handled separately via batches */ },
        _ => {},
    }

    let count_sql = format!("SELECT COUNT(*) FROM synced_products {}", where_clause);
    let data_sql = format!(
        "SELECT local_id, name, category, stock_quantity::float8, base_unit, cost_price, sell_price, min_stock, expiry_date \
         FROM synced_products {} ORDER BY name LIMIT $2 OFFSET $3", where_clause
    );

    let total: (i64,) = if search.is_empty() {
        sqlx::query_as(&count_sql).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,))
    } else {
        sqlx::query_as(&count_sql).bind(sid).bind(0i64).bind(0i64).bind(&search).fetch_one(&state.pool).await.unwrap_or((0,))
    };

    let rows = if search.is_empty() {
        sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<String>, Option<f64>, Option<f64>, Option<i32>, Option<String>)>(
            &data_sql
        ).bind(sid).bind(limit).bind(offset).fetch_all(&state.pool).await?
    } else {
        sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<String>, Option<f64>, Option<f64>, Option<i32>, Option<String>)>(
            &data_sql
        ).bind(sid).bind(limit).bind(offset).bind(&search).fetch_all(&state.pool).await?
    };

    let low_stock_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_products WHERE store_id = $1 AND stock_quantity::float8 <= COALESCE(min_stock, 5)"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    let products: Vec<Value> = rows.iter().map(|r| {
        let min_s = r.7.unwrap_or(5);
        let stock = r.3.unwrap_or(0.0);
        json!({
            "id": r.0, "name": r.1, "category": r.2,
            "stock_quantity": stock, "base_unit": r.4,
            "cost_price": r.5, "sell_price": r.6,
            "min_stock": min_s, "is_low_stock": stock <= min_s as f64,
            "expiry_date": r.8
        })
    }).collect();

    Ok(Json(json!({
        "products": products, "total": total.0,
        "low_stock_count": low_stock_count.0, "expiring_count": 0
    })))
}

// ===== API 5: Debts =====
#[derive(Deserialize)]
struct DebtsQuery {
    #[serde(rename = "type")]
    debt_type: Option<String>,
    store_id: Option<i32>,
}

async fn debts(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<DebtsQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let dtype = q.debt_type.unwrap_or_else(|| "customer".to_string());

    if dtype == "supplier" {
        let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<String>)>(
            "SELECT local_id, name, phone, total_debt::FLOAT8, company FROM synced_suppliers WHERE store_id = $1 AND total_debt > 0 ORDER BY total_debt DESC"
        ).bind(sid).fetch_all(&state.pool).await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(total_debt)::bigint, 0) FROM synced_suppliers WHERE store_id = $1 AND total_debt > 0"
        ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

        let debts: Vec<Value> = rows.iter().map(|r| json!({
            "id": r.0, "name": r.1, "phone": r.2,
            "current_debt": r.3, "company": r.4
        })).collect();

        Ok(Json(json!({ "debts": debts, "total_debt": total.0, "count": debts.len() })))
    } else {
        let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<f64>, Option<f64>)>(
            "SELECT local_id, name, phone, total_debt::FLOAT8, credit_limit::FLOAT8 FROM synced_customers WHERE store_id = $1 AND total_debt > 0 ORDER BY total_debt DESC"
        ).bind(sid).fetch_all(&state.pool).await?;

        let total: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(total_debt)::bigint, 0) FROM synced_customers WHERE store_id = $1 AND total_debt > 0"
        ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

        let debts: Vec<Value> = rows.iter().map(|r| json!({
            "id": r.0, "name": r.1, "phone": r.2,
            "current_debt": r.3, "credit_limit": r.4
        })).collect();

        Ok(Json(json!({ "debts": debts, "total_debt": total.0, "count": debts.len() })))
    }
}

// ===== API 6: Revenue Report =====
#[derive(Deserialize)]
struct RevenueQuery {
    period: Option<String>,
    from: Option<String>,
    to: Option<String>,
    store_id: Option<i32>,
}

async fn revenue_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<RevenueQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let from = q.from.unwrap_or_else(|| "2020-01-01".to_string());
    let to = q.to.unwrap_or_else(|| "2099-12-31".to_string());

    let rows = sqlx::query_as::<_, (chrono::NaiveDate, i64, i64)>(
        "SELECT DATE(created_at) as d, COALESCE(SUM(total_amount)::bigint, 0), COUNT(*) \
         FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date \
         GROUP BY d ORDER BY d"
    ).bind(sid).bind(&from).bind(&to).fetch_all(&state.pool).await?;

    let data: Vec<Value> = rows.iter().map(|r| json!({
        "date": r.0.to_string(), "revenue": r.1, "orders": r.2, "profit": r.1 / 4
    })).collect();

    let total_rev: i64 = rows.iter().map(|r| r.1).sum();
    let total_ord: i64 = rows.iter().map(|r| r.2).sum();

    Ok(Json(json!({
        "data": data,
        "total_revenue": total_rev,
        "total_orders": total_ord,
        "total_profit": total_rev / 4
    })))
}

// ===== API 7: Top Products =====
#[derive(Deserialize)]
struct TopQuery {
    from: Option<String>,
    to: Option<String>,
    limit: Option<i64>,
    store_id: Option<i32>,
}

async fn top_products(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TopQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let lim = q.limit.unwrap_or(10);

    let rows = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT COALESCE(ii.product_name, p.name), COALESCE(SUM(ii.quantity)::bigint, 0), COALESCE(SUM(ii.total)::bigint, 0) \
         FROM synced_invoice_items ii \
         LEFT JOIN synced_products p ON ii.product_local_id = p.local_id AND p.store_id = ii.store_id \
         WHERE ii.store_id = $1 \
         GROUP BY 1 ORDER BY 3 DESC LIMIT $2"
    ).bind(sid).bind(lim).fetch_all(&state.pool).await?;

    let products: Vec<Value> = rows.iter().map(|r| json!({
        "name": r.0, "quantity_sold": r.1, "revenue": r.2
    })).collect();

    Ok(Json(json!({ "products": products })))
}

// ===== API 8: Settings =====
async fn settings(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;

    // For new accounts (store_id >= 1_000_000), query accounts table
    if claims.store_id >= 1_000_000 {
        let account_id = claims.store_id - 1_000_000;
        let acc = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>)>(
            "SELECT display_name, store_name, phone, store_id FROM accounts WHERE id = $1"
        ).bind(account_id).fetch_optional(&state.pool).await?;

        let a = acc.unwrap_or((None, None, None, None));
        return Ok(Json(json!({
            "store": {
                "name": a.0.unwrap_or_default(),
                "address": "",
                "phone": a.2.unwrap_or_default(),
                "license_key": null,
                "license_type": "free",
                "activated_at": null,
                "store_id": a.3
            }
        })));
    }

    let store = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT owner_name, s2.address, s2.phone, s.license_key, s.license_type, s.activated_at \
         FROM stores s LEFT JOIN synced_store_settings s2 ON s2.store_id = s.id WHERE s.id = $1"
    ).bind(claims.store_id).fetch_optional(&state.pool).await?;

    let s = store.unwrap_or((None, None, None, None, None, None));

    Ok(Json(json!({
        "store": {
            "name": s.0.unwrap_or_default(),
            "address": s.1.unwrap_or_default(),
            "phone": s.2.unwrap_or_default(),
            "license_key": s.3.as_deref().map(|k| {
                if k.len() > 8 { format!("{}...{}", &k[..4], &k[k.len()-4..]) } else { k.to_string() }
            }),
            "license_type": s.4,
            "activated_at": s.5.map(|d| d.to_string())
        }
    })))
}

// ===== API 9: Change Password =====
#[derive(Deserialize)]
struct PasswordChange {
    current_password: String,
    new_password: String,
}

async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<PasswordChange>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;

    // For new accounts (store_id >= 1_000_000), use accounts table
    if claims.store_id >= 1_000_000 {
        let account_id = claims.store_id - 1_000_000;
        let hash: Option<(String,)> = sqlx::query_as(
            "SELECT password_hash FROM accounts WHERE id = $1"
        ).bind(account_id).fetch_optional(&state.pool).await?;

        let current_hash = match hash {
            Some(h) => h.0,
            None => return Err(AppError::NotFound("Account not found".into())),
        };

        if !bcrypt::verify(&body.current_password, &current_hash).unwrap_or(false) {
            return Err(AppError::Unauthorized("Mật khẩu hiện tại không đúng".into()));
        }

        let new_hash = bcrypt::hash(&body.new_password, 10)
            .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))?;

        sqlx::query("UPDATE accounts SET password_hash = $1, updated_at = NOW() WHERE id = $2")
            .bind(&new_hash).bind(account_id).execute(&state.pool).await?;

        return Ok(Json(json!({ "success": true })));
    }

    let hash: Option<(String,)> = sqlx::query_as(
        "SELECT password_hash FROM users WHERE id = $1"
    ).bind(claims.sub).fetch_optional(&state.pool).await?;

    let current_hash = match hash {
        Some(h) => h.0,
        None => return Err(AppError::NotFound("User not found".into())),
    };

    if !bcrypt::verify(&body.current_password, &current_hash).unwrap_or(false) {
        return Err(AppError::Unauthorized("Mật khẩu hiện tại không đúng".into()));
    }

    let new_hash = bcrypt::hash(&body.new_password, 10)
        .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))?;

    sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(&new_hash).bind(claims.sub).execute(&state.pool).await?;

    Ok(Json(json!({ "success": true })))
}

// ===== API 10: Purchase Orders =====
#[derive(Deserialize)]
struct PurchaseOrdersQuery {
    store_id: Option<i32>,
    page: Option<i32>,
    limit: Option<i32>,
    search: Option<String>,
}

async fn purchase_orders_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<PurchaseOrdersQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = (q.page.unwrap_or(1) - 1).max(0) * limit;

    let search_filter = q.search.as_deref().unwrap_or("");
    let like_pattern = format!("%{}%", search_filter);

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<f64>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>, Option<String>)>(
        "SELECT local_id, supplier_name, total_amount::float8, status, import_date, created_at, note \
         FROM synced_purchase_orders WHERE store_id = $1 \
         AND ($4 = '' OR LOWER(COALESCE(supplier_name,'')) LIKE LOWER($4)) \
         ORDER BY created_at DESC NULLS LAST LIMIT $2 OFFSET $3"
    ).bind(sid).bind(limit).bind(offset).bind(&like_pattern)
    .fetch_all(&state.pool).await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_purchase_orders WHERE store_id = $1 \
         AND ($2 = '' OR LOWER(COALESCE(supplier_name,'')) LIKE LOWER($2))"
    ).bind(sid).bind(&like_pattern).fetch_one(&state.pool).await.unwrap_or((0,));

    let orders: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0,
        "supplier_name": r.1,
        "total_amount": r.2.unwrap_or(0.0),
        "status": r.3,
        "import_date": r.4,
        "created_at": r.5.map(|d| d.to_string()),
        "note": r.6,
    })).collect();

    Ok(Json(json!({
        "purchase_orders": orders,
        "total": total.0,
        "page": q.page.unwrap_or(1),
        "limit": limit,
    })))
}

// ===== API 11: Staff Members =====
#[derive(Deserialize)]
struct StaffQuery {
    store_id: Option<i32>,
}

async fn staff_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<StaffQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);

    let rows = sqlx::query_as::<_, (i64, String, String, String, Option<String>, Value, bool, Option<chrono::NaiveDateTime>)>(
        "SELECT id, username, display_name, role, pin, permissions, is_active, created_at \
         FROM sync_staff_members WHERE store_id = $1 ORDER BY role DESC, created_at ASC"
    ).bind(sid).fetch_all(&state.pool).await?;

    let staff: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0,
        "username": r.1,
        "display_name": r.2,
        "role": r.3,
        "pin_set": r.4.is_some() && !r.4.as_deref().unwrap_or("").is_empty(),
        "permissions": r.5,
        "is_active": r.6,
        "created_at": r.7.map(|d| d.to_string()),
    })).collect();

    Ok(Json(json!({
        "staff": staff,
        "total": staff.len(),
    })))
}

// ===== API 11b: Update Staff Permissions =====
#[derive(Deserialize)]
struct PermissionsPayload {
    permissions: Value,
}

async fn staff_update_permissions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(body): Json<PermissionsPayload>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    // Verify staff member exists and belongs to this store
    let exists = sqlx::query_as::<_, (String,)>(
        "SELECT role FROM sync_staff_members WHERE store_id = $1 AND id = $2"
    ).bind(sid).bind(id).fetch_optional(&state.pool).await?;

    match exists {
        None => return Err(AppError::NotFound("Staff member not found".into())),
        Some((role,)) if role == "owner" => {
            return Err(AppError::BadRequest("Cannot modify owner permissions".into()));
        }
        _ => {}
    }

    sqlx::query(
        "UPDATE sync_staff_members SET permissions = $1, updated_at = NOW() \
         WHERE store_id = $2 AND id = $3"
    ).bind(&body.permissions).bind(sid).bind(id).execute(&state.pool).await?;

    Ok(Json(json!({
        "success": true,
        "message": "Cập nhật quyền thành công"
    })))
}

// ===== API 11c: Toggle Staff Active =====
async fn staff_toggle_active(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    let row = sqlx::query_as::<_, (String, bool)>(
        "SELECT role, is_active FROM sync_staff_members WHERE store_id = $1 AND id = $2"
    ).bind(sid).bind(id).fetch_optional(&state.pool).await?;

    match row {
        None => return Err(AppError::NotFound("Staff member not found".into())),
        Some((role, _)) if role == "owner" => {
            return Err(AppError::BadRequest("Cannot deactivate owner".into()));
        }
        Some((_, current)) => {
            sqlx::query(
                "UPDATE sync_staff_members SET is_active = $1, updated_at = NOW() \
                 WHERE store_id = $2 AND id = $3"
            ).bind(!current).bind(sid).bind(id).execute(&state.pool).await?;

            Ok(Json(json!({
                "success": true,
                "is_active": !current,
                "message": if !current { "Đã kích hoạt nhân viên" } else { "Đã vô hiệu hóa nhân viên" }
            })))
        }
    }
}

// ===== API 11d: Update Staff PIN =====
#[derive(Deserialize)]
struct PinPayload {
    pin: String,
}

async fn staff_update_pin(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i64>,
    Json(body): Json<PinPayload>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    // Validate PIN format: exactly 4 digits
    if body.pin.len() != 4 || !body.pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::BadRequest("PIN phải là 4 chữ số".into()));
    }

    let exists = sqlx::query_as::<_, (String,)>(
        "SELECT role FROM sync_staff_members WHERE store_id = $1 AND id = $2"
    ).bind(sid).bind(id).fetch_optional(&state.pool).await?;

    match exists {
        None => return Err(AppError::NotFound("Staff member not found".into())),
        Some((role,)) if role == "owner" => {
            return Err(AppError::BadRequest("Cannot change owner PIN from web".into()));
        }
        _ => {}
    }

    sqlx::query(
        "UPDATE sync_staff_members SET pin = $1, updated_at = NOW() \
         WHERE store_id = $2 AND id = $3"
    ).bind(&body.pin).bind(sid).bind(id).execute(&state.pool).await?;

    Ok(Json(json!({
        "success": true,
        "message": "Đã cập nhật mã PIN"
    })))
}

// ===== API 12: Product Inventory Export (Excel) =====
#[derive(Deserialize)]
struct ExportQuery {
    store_id: Option<i32>,
}

async fn inventory_export(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ExportQuery>,
) -> Result<Response, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);

    let rows = sqlx::query_as::<_, (Option<String>, Option<String>, Option<f64>, Option<String>, Option<f64>, Option<f64>, Option<String>)>(
        "SELECT name, category, stock_quantity::float8, base_unit, cost_price::float8, sell_price::float8, expiry_date \
         FROM synced_products WHERE store_id = $1 ORDER BY name"
    ).bind(sid).fetch_all(&state.pool).await?;

    let mut wb = Workbook::new();
    let sheet = wb.add_worksheet();
    sheet.set_name("Tồn kho").ok();

    // Header format
    let hdr_fmt = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x2563EB))
        .set_font_color(Color::White)
        .set_align(FormatAlign::Center);

    let money_fmt = Format::new()
        .set_num_format("#,##0")
        .set_align(FormatAlign::Right);

    let num_fmt = Format::new()
        .set_align(FormatAlign::Right);

    let headers_list = ["Tên sản phẩm", "Danh mục", "Tồn kho", "ĐVT", "Giá vốn", "Giá bán", "HSD"];
    let widths = [30.0, 15.0, 10.0, 8.0, 15.0, 15.0, 12.0];
    for (c, h) in headers_list.iter().enumerate() {
        sheet.write_string_with_format(0, c as u16, *h, &hdr_fmt).ok();
        sheet.set_column_width(c as u16, widths[c]).ok();
    }

    for (i, r) in rows.iter().enumerate() {
        let row = (i + 1) as u32;
        sheet.write_string(row, 0, r.0.as_deref().unwrap_or("")).ok();
        sheet.write_string(row, 1, r.1.as_deref().unwrap_or("")).ok();
        sheet.write_number_with_format(row, 2, r.2.unwrap_or(0.0), &num_fmt).ok();
        sheet.write_string(row, 3, r.3.as_deref().unwrap_or("")).ok();
        sheet.write_number_with_format(row, 4, r.4.unwrap_or(0.0), &money_fmt).ok();
        sheet.write_number_with_format(row, 5, r.5.unwrap_or(0.0), &money_fmt).ok();
        sheet.write_string(row, 6, r.6.as_deref().unwrap_or("")).ok();
    }

    let buf = wb.save_to_buffer()
        .map_err(|e| AppError::Internal(format!("Excel error: {}", e)))?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string()),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"ton-kho.xlsx\"".to_string()),
        ],
        buf,
    ).into_response())
}

// ===== API 13: Computed Notifications =====
#[derive(Deserialize)]
struct NotifQuery {
    store_id: Option<i32>,
}

async fn notifications(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<NotifQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let mut notifs: Vec<Value> = Vec::new();

    // 1. Low stock products
    let low_stock = sqlx::query_as::<_, (Option<String>, Option<f64>, Option<i32>)>(
        "SELECT name, stock_quantity::float8, min_stock FROM synced_products \
         WHERE store_id = $1 AND stock_quantity <= COALESCE(min_stock, 5) \
         ORDER BY stock_quantity ASC LIMIT 10"
    ).bind(sid).fetch_all(&state.pool).await.unwrap_or_default();

    for r in &low_stock {
        notifs.push(json!({
            "type": "LOW_STOCK", "severity": "warning",
            "title": format!("⚠️ {} sắp hết hàng", r.0.as_deref().unwrap_or("?")),
            "message": format!("Còn {} sản phẩm (tối thiểu: {})", r.1.unwrap_or(0.0) as i64, r.2.unwrap_or(5)),
        }));
    }

    // 2. Expiring products (within 30 days)
    let expiring = sqlx::query_as::<_, (Option<String>, Option<String>)>(
        "SELECT name, expiry_date FROM synced_products \
         WHERE store_id = $1 AND expiry_date IS NOT NULL \
         AND expiry_date::date <= CURRENT_DATE + INTERVAL '30 days' \
         AND expiry_date::date >= CURRENT_DATE \
         ORDER BY expiry_date ASC LIMIT 10"
    ).bind(sid).fetch_all(&state.pool).await.unwrap_or_default();

    for r in &expiring {
        notifs.push(json!({
            "type": "EXPIRING", "severity": "error",
            "title": format!("🔴 {} sắp hết hạn", r.0.as_deref().unwrap_or("?")),
            "message": format!("HSD: {}", r.1.as_deref().unwrap_or("?")),
        }));
    }

    // 3. Customer debts > 0
    let cust_debt: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(debt_amount)::bigint, 0) FROM synced_customers \
         WHERE store_id = $1 AND debt_amount > 0"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    if cust_debt.0 > 0 {
        notifs.push(json!({
            "type": "DEBT", "severity": "info",
            "title": "💰 Công nợ khách hàng",
            "message": format!("Tổng: {}đ chưa thu", cust_debt.0),
        }));
    }

    // 4. Supplier debts > 0
    let sup_debt: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(debt_amount)::bigint, 0) FROM synced_suppliers \
         WHERE store_id = $1 AND debt_amount > 0"
    ).bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

    if sup_debt.0 > 0 {
        notifs.push(json!({
            "type": "DEBT", "severity": "info",
            "title": "📦 Công nợ nhà cung cấp",
            "message": format!("Tổng: {}đ chưa trả", sup_debt.0),
        }));
    }

    Ok(Json(json!({
        "notifications": notifs,
        "count": notifs.len(),
    })))
}

// ===== API 14: Force Re-Sync =====
async fn force_resync(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    // Clear server-side sync state so device will re-push everything
    let inbox_deleted: u64 = sqlx::query("DELETE FROM sync_inbox WHERE store_id = $1")
        .bind(sid).execute(&state.pool).await?.rows_affected();

    let journal_deleted: u64 = sqlx::query("DELETE FROM sync_journal WHERE store_id = $1")
        .bind(sid).execute(&state.pool).await?.rows_affected();

    // Reset device pull cursors
    sqlx::query("UPDATE sync_devices SET pull_cursor = 0 WHERE store_id = $1")
        .bind(sid).execute(&state.pool).await?;

    // Broadcast request_full_sync via WebSocket to all connected devices
    let msg = serde_json::json!({
        "type": "request_full_sync",
        "store_id": sid,
        "message": "Server requests full data re-sync"
    }).to_string();

    let ws_count = {
        let rooms = state.sync_rooms.read().await;
        if let Some(clients) = rooms.get(&sid) {
            for tx in clients {
                let _ = tx.send(msg.clone());
            }
            clients.len()
        } else {
            0
        }
    };

    tracing::info!("🔄 Force re-sync: store_id={}, inbox_cleared={}, journal_cleared={}, ws_notified={}",
        sid, inbox_deleted, journal_deleted, ws_count);

    Ok(Json(json!({
        "success": true,
        "message": format!("Đã xóa {} inbox, {} journal records. Đã gửi yêu cầu re-sync đến {} thiết bị.", inbox_deleted, journal_deleted, ws_count),
        "data": {
            "inbox_cleared": inbox_deleted,
            "journal_cleared": journal_deleted,
            "devices_notified": ws_count
        }
    })))
}
