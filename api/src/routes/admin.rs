use axum::{Router, Json, extract::{State, Path, Query}, routing::{get, post, put}};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use rand::Rng;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{verify_token, Claims};

fn extract_admin(headers: &HeaderMap, secret: &str) -> Result<Claims, AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    let claims = verify_token(token, secret)?;
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    Ok(claims)
}

fn generate_license_key() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let segment = |r: &mut rand::rngs::ThreadRng| -> String {
        (0..4).map(|_| chars[r.gen_range(0..chars.len())]).collect()
    };
    format!("NODI-{}-{}-{}", segment(&mut rng), segment(&mut rng), segment(&mut rng))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/overview", get(overview))
        .route("/api/admin/licenses", get(licenses_list))
        .route("/api/admin/licenses", post(create_license))
        .route("/api/admin/licenses/expiring", get(licenses_expiring))
        .route("/api/admin/licenses/{id}", put(update_license))
        .route("/api/admin/licenses/{id}/payments", get(get_payments).post(create_payment))
        .route("/api/admin/billing/summary", get(billing_summary))
        .route("/api/admin/stores", get(stores_list))
        .route("/api/admin/stores/{id}", get(store_detail))
        .route("/api/admin/alerts", get(alerts))
        .route("/api/admin/intelligence/top-products", get(intel_top_products))
        .route("/api/admin/intelligence/revenue-trend", get(intel_revenue_trend))
        .route("/api/admin/intelligence/manufacturers", get(intel_manufacturers))
        .route("/api/admin/backups", get(all_backups))
        .route("/api/admin/system", get(system_health))
}

// ===== API 1: Admin Overview =====
async fn overview(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let total_stores: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stores")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    let active_stores: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stores WHERE is_active = true")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    let total_rev: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(total_amount::bigint), 0) FROM synced_invoices")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    let total_orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_invoices")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    let total_products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_products")
        .fetch_one(&state.pool).await.unwrap_or((0,));
    let total_customers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_customers")
        .fetch_one(&state.pool).await.unwrap_or((0,));

    let last_sync = sqlx::query_as::<_, (Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT s.owner_name, i.synced_at FROM synced_invoices i JOIN stores s ON s.id = i.store_id ORDER BY i.synced_at DESC NULLS LAST LIMIT 1"
    ).fetch_optional(&state.pool).await?.unwrap_or((None, None));

    Ok(Json(json!({
        "total_stores": total_stores.0,
        "active_stores": active_stores.0,
        "total_licenses": total_stores.0,
        "active_licenses": active_stores.0,
        "total_revenue_all_stores": total_rev.0,
        "total_orders_all_stores": total_orders.0,
        "total_products_all_stores": total_products.0,
        "total_customers_all_stores": total_customers.0,
        "last_sync": {
            "store_name": last_sync.0,
            "synced_at": last_sync.1.map(|d| d.to_string())
        }
    })))
}

// ===== API 2: Licenses List =====
#[derive(Deserialize)]
struct LicenseQuery { page: Option<i64>, limit: Option<i64> }

async fn licenses_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<LicenseQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = (page - 1) * limit;

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<bool>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<i32>)>(
        "SELECT id, license_key, license_type, is_active, owner_name, hwid, activated_at, license_expires_at, created_at, revoked_at, duration_days FROM stores ORDER BY id DESC LIMIT $1 OFFSET $2"
    ).bind(limit).bind(offset).fetch_all(&state.pool).await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stores").fetch_one(&state.pool).await.unwrap_or((0,));

    let now = chrono::Utc::now().naive_utc();
    let mut licenses = Vec::new();
    for r in &rows {
        // 5-state status logic
        let status = if r.9.is_some() { "REVOKED" }
            else if r.5.is_none() { "PENDING" }
            else if let Some(exp) = r.7 { if exp < now { "EXPIRED" } else if exp < now + chrono::Duration::days(7) { "EXPIRING" } else { "ACTIVE" } }
            else { "ACTIVE" };
        let remaining_days = r.7.map(|exp| (exp - now).num_days());
        // Check last payment
        let last_pay: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as(
            "SELECT MAX(created_at) FROM license_payments WHERE store_id = $1"
        ).bind(r.0).fetch_optional(&state.pool).await.unwrap_or(None);
        let has_recent_payment = last_pay.and_then(|p| p.0).map(|d| (now - d).num_days() < 30).unwrap_or(false);

        licenses.push(json!({
            "id": r.0, "license_key": r.1, "license_type": r.2,
            "status": status, "store_name": r.4, "hwid": r.5,
            "activated_at": r.6.map(|d| d.to_string()),
            "expires_at": r.7.map(|d| d.to_string()),
            "created_at": r.8.map(|d| d.to_string()),
            "revoked_at": r.9.map(|d| d.to_string()),
            "duration_days": r.10,
            "remaining_days": remaining_days,
            "has_recent_payment": has_recent_payment
        }));
    }

    let counts = sqlx::query_as::<_, (i64, i64, i64, i64)>(
        "SELECT COUNT(*) FILTER (WHERE revoked_at IS NULL AND hwid IS NOT NULL AND (license_expires_at IS NULL OR license_expires_at > NOW())),\
         COUNT(*) FILTER (WHERE revoked_at IS NOT NULL),\
         COUNT(*) FILTER (WHERE revoked_at IS NULL AND license_expires_at IS NOT NULL AND license_expires_at < NOW()),\
         COUNT(*) FILTER (WHERE hwid IS NULL AND revoked_at IS NULL)\
         FROM stores"
    ).fetch_one(&state.pool).await.unwrap_or((0,0,0,0));

    Ok(Json(json!({
        "licenses": licenses, "total": total.0,
        "active": counts.0, "revoked": counts.1, "expired": counts.2, "pending": counts.3
    })))
}

// ===== API 3: Create License =====
#[derive(Deserialize)]
struct CreateLicense {
    license_type: String,
    note: Option<String>,
    duration_days: Option<i32>,
}

async fn create_license(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateLicense>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let mut key = generate_license_key();
    // Ensure unique
    for _ in 0..10 {
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM stores WHERE license_key = $1)")
            .bind(&key).fetch_one(&state.pool).await.unwrap_or((false,));
        if !exists.0 { break; }
        key = generate_license_key();
    }

    // Calculate duration based on license_type
    let days = body.duration_days.unwrap_or(match body.license_type.to_uppercase().as_str() {
        "YEARLY" => 365,
        "MONTHLY" => 30,
        "TRIAL" => 30,
        _ => 30,
    });

    let store_name = body.note.clone().unwrap_or_else(|| format!("Store-{}", &key[5..9]));

    sqlx::query(
        "INSERT INTO stores (name, license_key, license_type, is_active, owner_name, license_expires_at, created_at) \
         VALUES ($1, $2, $3, true, $1, NOW() + ($4 || ' days')::interval, NOW())"
    )
    .bind(&store_name)
    .bind(&key)
    .bind(&body.license_type)
    .bind(days.to_string())
    .execute(&state.pool).await?;

    Ok(Json(json!({
        "license_key": key, "license_type": body.license_type,
        "duration_days": days,
        "status": "PENDING", "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

// ===== API 4: Update License =====
#[derive(Deserialize)]
struct UpdateLicense { action: String, extend_days: Option<i32> }

async fn update_license(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<UpdateLicense>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    match body.action.as_str() {
        "revoke" => {
            sqlx::query("UPDATE stores SET is_active = false, revoked_at = NOW() WHERE id = $1")
                .bind(id).execute(&state.pool).await?;
            Ok(Json(json!({ "success": true, "message": "License revoked" })))
        }
        "activate" => {
            sqlx::query("UPDATE stores SET is_active = true, revoked_at = NULL WHERE id = $1")
                .bind(id).execute(&state.pool).await?;
            Ok(Json(json!({ "success": true, "message": "License activated" })))
        }
        "extend" => {
            let days = body.extend_days.unwrap_or(30);
            sqlx::query("UPDATE stores SET license_expires_at = GREATEST(COALESCE(license_expires_at, NOW()), NOW()) + ($1 || ' days')::interval, revoked_at = NULL, is_active = true WHERE id = $2")
                .bind(days.to_string()).bind(id).execute(&state.pool).await?;
            // Fetch new expires_at
            let new_exp: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as(
                "SELECT license_expires_at FROM stores WHERE id = $1"
            ).bind(id).fetch_optional(&state.pool).await?;
            Ok(Json(json!({ "success": true, "message": format!("Extended {} days", days), "new_expires_at": new_exp.and_then(|e| e.0.map(|d| d.to_string())) })))
        }
        "reset_hwid" => {
            sqlx::query("UPDATE stores SET hwid = NULL, activated_at = NULL WHERE id = $1")
                .bind(id).execute(&state.pool).await?;
            Ok(Json(json!({ "success": true, "message": "HWID reset" })))
        }
        _ => Err(AppError::BadRequest("Invalid action".into())),
    }
}

// ===== API 5: Stores List =====
async fn stores_list(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<String>, Option<bool>, Option<String>)>(
        "SELECT id, owner_name, license_key, license_type, is_active, hwid FROM stores ORDER BY id"
    ).fetch_all(&state.pool).await?;

    let mut stores = Vec::new();
    for r in &rows {
        let sid = r.0;
        let pc: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_products WHERE store_id=$1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let cc: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_customers WHERE store_id=$1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let oc: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_invoices WHERE store_id=$1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let rev: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(total_amount),0)::bigint FROM synced_invoices WHERE store_id=$1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let ls: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as("SELECT MAX(synced_at) FROM synced_invoices WHERE store_id=$1").bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);
        let bc: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM backup_files WHERE store_id=$1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));

        let status = if !r.4.unwrap_or(false) { "SUSPENDED" }
            else if r.5.is_some() { "ACTIVE" }
            else { "PENDING" };

        stores.push(json!({
            "id": sid, "name": r.1, "license_key": r.2,
            "license_type": r.3, "status": status,
            "products_count": pc.0, "customers_count": cc.0,
            "orders_count": oc.0, "revenue_total": rev.0,
            "last_sync_at": ls.and_then(|s| s.0.map(|d| d.to_string())),
            "backup_count": bc.0
        }));
    }

    Ok(Json(json!({ "stores": stores, "total": rows.len() })))
}

// ===== API 6: Store Detail =====
async fn store_detail(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let store = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<bool>, Option<String>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>)>(
        "SELECT owner_name, license_key, license_type, phone, address, is_active, hwid, activated_at, license_expires_at, revoked_at FROM stores WHERE id=$1"
    ).bind(id).fetch_optional(&state.pool).await?;
    let s = match store {
        Some(s) => s,
        None => return Err(AppError::NotFound("Store not found".into())),
    };

    let products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_products WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let customers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_customers WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_invoices WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let revenue: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(final_amount),0)::bigint FROM synced_invoices WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let cust_debt: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(total_debt),0)::bigint FROM synced_customers WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let sup_debt: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(total_debt),0)::bigint FROM synced_suppliers WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    let last_sync: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as("SELECT MAX(synced_at) FROM synced_invoices WHERE store_id=$1").bind(id).fetch_optional(&state.pool).await.unwrap_or(None);
    let total_syncs: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT DATE(synced_at)) FROM synced_invoices WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));

    let now = chrono::Utc::now().naive_utc();
    let status = if s.9.is_some() { "REVOKED" }
        else if s.6.is_none() { "PENDING" }
        else if let Some(exp) = s.8 { if exp < now { "EXPIRED" } else if exp < now + chrono::Duration::days(7) { "EXPIRING" } else { "ACTIVE" } }
        else { "ACTIVE" };

    Ok(Json(json!({
        "store": { "name": s.0, "phone": s.3, "address": s.4 },
        "license": { "key": s.1, "type": s.2, "status": status, "expires_at": s.8.map(|d| d.to_string()), "activated_at": s.7.map(|d| d.to_string()), "is_active": s.5 },
        "sync": { "last_synced_at": last_sync.and_then(|s| s.0.map(|d| d.to_string())), "total_syncs": total_syncs.0 },
        "stats": {
            "total_products": products.0,
            "total_customers": customers.0,
            "total_orders": orders.0,
            "total_revenue": revenue.0,
            "total_customer_debt": cust_debt.0,
            "total_supplier_debt": sup_debt.0
        }
    })))
}

// ===== Licenses Expiring =====
async fn licenses_expiring(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, license_key, license_type, owner_name, license_expires_at FROM stores \
         WHERE revoked_at IS NULL AND license_expires_at IS NOT NULL \
         AND license_expires_at < NOW() + INTERVAL '7 days' AND license_expires_at >= NOW() \
         ORDER BY license_expires_at ASC"
    ).fetch_all(&state.pool).await?;
    let now = chrono::Utc::now().naive_utc();
    let licenses: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "license_key": r.1, "license_type": r.2, "store_name": r.3,
        "expires_at": r.4.map(|d| d.to_string()),
        "remaining_days": r.4.map(|d| (d - now).num_days())
    })).collect();
    Ok(Json(json!({ "licenses": licenses })))
}

// ===== Billing: Create Payment =====
#[derive(Deserialize)]
struct CreatePayment {
    amount: i32,
    payment_method: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    note: Option<String>,
}

async fn create_payment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<CreatePayment>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let method = body.payment_method.unwrap_or_else(|| "BANK_TRANSFER".into());
    let pid: (i32,) = sqlx::query_as(
        "INSERT INTO license_payments (store_id, amount, payment_method, period_start, period_end, note) \
         VALUES ($1, $2, $3, $4::date, $5::date, $6) RETURNING id"
    )
    .bind(id).bind(body.amount).bind(&method)
    .bind(&body.period_start).bind(&body.period_end).bind(&body.note)
    .fetch_one(&state.pool).await?;
    Ok(Json(json!({ "success": true, "payment_id": pid.0 })))
}

// ===== Billing: Get Payments =====
async fn get_payments(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let rows = sqlx::query_as::<_, (i32, i32, Option<String>, Option<chrono::NaiveDate>, Option<chrono::NaiveDate>, Option<String>, chrono::NaiveDateTime)>(
        "SELECT id, amount, payment_method, period_start, period_end, note, created_at \
         FROM license_payments WHERE store_id = $1 ORDER BY created_at DESC"
    ).bind(id).fetch_all(&state.pool).await?;
    let payments: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "amount": r.1, "payment_method": r.2,
        "period_start": r.3.map(|d| d.to_string()), "period_end": r.4.map(|d| d.to_string()),
        "note": r.5, "created_at": r.6.to_string()
    })).collect();
    let total: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(amount),0)::bigint FROM license_payments WHERE store_id=$1").bind(id).fetch_one(&state.pool).await.unwrap_or((0,));
    Ok(Json(json!({ "payments": payments, "total_paid": total.0 })))
}

// ===== Billing Summary =====
async fn billing_summary(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let total: (i64, i64) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount),0)::bigint, COUNT(*)::bigint FROM license_payments"
    ).fetch_one(&state.pool).await.unwrap_or((0,0));
    let this_month: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount),0)::bigint FROM license_payments WHERE created_at >= DATE_TRUNC('month', NOW())"
    ).fetch_one(&state.pool).await.unwrap_or((0,));
    let by_method = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT payment_method, COALESCE(SUM(amount),0)::bigint, COUNT(*)::bigint FROM license_payments GROUP BY payment_method"
    ).fetch_all(&state.pool).await.unwrap_or_default();
    let methods: Vec<Value> = by_method.iter().map(|r| json!({ "method": r.0, "amount": r.1, "count": r.2 })).collect();
    Ok(Json(json!({ "total_revenue": total.0, "total_payments": total.1, "this_month": this_month.0, "by_method": methods })))
}

// ===== Alerts =====
async fn alerts(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let mut alert_list: Vec<Value> = Vec::new();

    // Expiring licenses (< 7 days)
    let expiring = sqlx::query_as::<_, (Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT license_key, owner_name, license_expires_at FROM stores \
         WHERE revoked_at IS NULL AND license_expires_at IS NOT NULL \
         AND license_expires_at BETWEEN NOW() AND NOW() + INTERVAL '7 days'"
    ).fetch_all(&state.pool).await.unwrap_or_default();
    let now = chrono::Utc::now().naive_utc();
    for r in &expiring {
        let days = r.2.map(|d| (d - now).num_days()).unwrap_or(0);
        alert_list.push(json!({ "type": "LICENSE_EXPIRING", "severity": "warning",
            "message": format!("{} hết hạn trong {} ngày", r.0.as_deref().unwrap_or("?"), days),
            "license_key": r.0, "store_name": r.1, "store_id": Value::Null }));
    }

    // Expired licenses
    let expired = sqlx::query_as::<_, (Option<String>, Option<String>)>(
        "SELECT license_key, owner_name FROM stores \
         WHERE revoked_at IS NULL AND license_expires_at IS NOT NULL AND license_expires_at < NOW()"
    ).fetch_all(&state.pool).await.unwrap_or_default();
    for r in &expired {
        alert_list.push(json!({ "type": "LICENSE_EXPIRED", "severity": "error",
            "message": format!("{} đã hết hạn", r.0.as_deref().unwrap_or("?")),
            "license_key": r.0, "store_name": r.1 }));
    }

    // Inactive stores (no sync in 7 days)
    let inactive = sqlx::query_as::<_, (i32, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT s.id, s.owner_name, MAX(i.synced_at) as last_sync FROM stores s \
         LEFT JOIN synced_invoices i ON i.store_id = s.id \
         WHERE s.is_active = true AND s.hwid IS NOT NULL AND s.license_key != 'ADMIN-MASTER-KEY' \
         GROUP BY s.id HAVING MAX(i.synced_at) < NOW() - INTERVAL '7 days' OR MAX(i.synced_at) IS NULL"
    ).fetch_all(&state.pool).await.unwrap_or_default();
    for r in &inactive {
        alert_list.push(json!({ "type": "STORE_INACTIVE", "severity": "info",
            "message": format!("{} không sync trong 7 ngày", r.1.as_deref().unwrap_or("?")),
            "store_id": r.0, "store_name": r.1 }));
    }

    Ok(Json(json!({ "alerts": alert_list, "count": alert_list.len() })))
}

// ===== API 7: Intelligence - Top Products =====
#[derive(Deserialize)]
struct IntelQuery { from: Option<String>, to: Option<String>, limit: Option<i64> }

async fn intel_top_products(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<IntelQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let lim = q.limit.unwrap_or(20);

    let rows = sqlx::query_as::<_, (Option<String>, Option<String>, i64, i64, i64)>(
        "SELECT p.name, p.category, COALESCE(SUM(ii.quantity::bigint),0), COALESCE(SUM(ii.subtotal::bigint),0), COUNT(DISTINCT ii.store_id) \
         FROM synced_invoice_items ii \
         JOIN synced_products p ON p.store_id = ii.store_id AND p.local_id = ii.product_id \
         GROUP BY p.name, p.category ORDER BY 4 DESC LIMIT $1"
    ).bind(lim).fetch_all(&state.pool).await?;

    let products: Vec<Value> = rows.iter().map(|r| json!({
        "name": r.0, "category": r.1,
        "total_quantity_sold": r.2, "total_revenue": r.3, "stores_selling": r.4
    })).collect();

    Ok(Json(json!({ "products": products })))
}

// ===== API 8: Intelligence - Revenue Trend =====
#[derive(Deserialize)]
struct TrendQuery { months: Option<i32> }

async fn intel_revenue_trend(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TrendQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let months = q.months.unwrap_or(6);

    let rows = sqlx::query_as::<_, (String, i64, i64, i64)>(
        "SELECT TO_CHAR(DATE_TRUNC('month', created_at), 'YYYY-MM'), \
         COALESCE(SUM(total_amount::bigint),0), COUNT(*), COUNT(DISTINCT store_id) \
         FROM synced_invoices \
         WHERE created_at >= NOW() - ($1 || ' months')::interval \
         GROUP BY 1 ORDER BY 1"
    ).bind(months.to_string()).fetch_all(&state.pool).await?;

    let trend: Vec<Value> = rows.iter().map(|r| json!({
        "month": r.0, "revenue": r.1, "orders": r.2, "stores_active": r.3
    })).collect();

    Ok(Json(json!({ "trend": trend })))
}

// ===== API 9: Intelligence - Manufacturers =====
async fn intel_manufacturers(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT p.manufacturer, COALESCE(SUM(ii.subtotal::bigint),0), COUNT(DISTINCT p.local_id) \
         FROM synced_invoice_items ii \
         JOIN synced_products p ON p.store_id = ii.store_id AND p.local_id = ii.product_id \
         WHERE p.manufacturer IS NOT NULL AND p.manufacturer != '' \
         GROUP BY p.manufacturer ORDER BY 2 DESC LIMIT 20"
    ).fetch_all(&state.pool).await?;

    let manufacturers: Vec<Value> = rows.iter().map(|r| json!({
        "name": r.0, "total_revenue": r.1, "products_count": r.2
    })).collect();

    Ok(Json(json!({ "manufacturers": manufacturers })))
}

// ===== API 10: All Backups =====
async fn all_backups(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, i32, Option<String>, String, i64, chrono::NaiveDateTime)>(
        "SELECT b.id, b.store_id, s.owner_name, b.filename, b.size_bytes, b.created_at \
         FROM backup_files b JOIN stores s ON s.id = b.store_id \
         ORDER BY b.created_at DESC"
    ).fetch_all(&state.pool).await?;

    let total_size: i64 = rows.iter().map(|r| r.4).sum();

    let backups: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "store_id": r.1, "store_name": r.2,
        "filename": r.3, "size_bytes": r.4, "created_at": r.5.to_string()
    })).collect();

    Ok(Json(json!({
        "backups": backups,
        "total_size_mb": total_size as f64 / 1024.0 / 1024.0,
        "total_files": backups.len()
    })))
}

// ===== API 11: System Health =====
async fn system_health(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let db_size: Option<(i64,)> = sqlx::query_as(
        "SELECT pg_database_size(current_database())"
    ).fetch_optional(&state.pool).await?;

    let uptime = state.start_time.elapsed().as_secs();

    Ok(Json(json!({
        "uptime_seconds": uptime,
        "db_size_mb": db_size.map(|s| s.0 as f64 / 1024.0 / 1024.0).unwrap_or(0.0),
        "containers": 4,
        "api_version": "0.1.0"
    })))
}
