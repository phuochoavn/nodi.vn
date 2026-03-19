use axum::{Router, Json, extract::{State, Path, Query, Multipart}, routing::{get, post, put, delete}};
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
        // Update management
        .route("/api/admin/update", get(get_update_config).put(edit_update_config))
        .route("/api/admin/update/upload", post(upload_installer))
        .route("/api/admin/update/files/{filename}", delete(delete_installer_file))
        // Phase 1: Account management
        .route("/api/admin/accounts", get(accounts_list))
        .route("/api/admin/accounts/{id}/toggle", put(account_toggle))
        // Phase 2: Audit log
        .route("/api/admin/audit-log", get(audit_log_list))
        // Phase 3: Notifications
        .route("/api/admin/notifications", get(notifications_list).post(create_notification))
        .route("/api/admin/notifications/{id}", delete(delete_notification))
        // Phase 4: Analytics
        .route("/api/admin/license-revenue", get(license_revenue))
        .route("/api/admin/geo", get(geo_analysis))
        .route("/api/admin/stores-compare", get(stores_compare))
        // Phase 5: Export
        .route("/api/admin/export/{type}", get(export_csv))
        // Phase 6: Multi-device license
        .route("/api/admin/license/{id}/devices", get(admin_license_devices))
        // Phase 7: Payment orders
        .route("/api/admin/orders", get(admin_orders_list))
        .route("/api/admin/orders/{id}/confirm", post(admin_confirm_order))
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
            // Also deactivate all devices for this store
            sqlx::query("UPDATE devices SET is_active = false WHERE store_id = $1")
                .bind(id).execute(&state.pool).await?;
            Ok(Json(json!({ "success": true, "message": "HWID reset & all devices deactivated" })))
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

// ===== Update Management =====

const DOWNLOADS_DIR: &str = "/opt/nodi/downloads";

// GET /api/admin/update — return current config + file list
async fn get_update_config(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let config = crate::routes::health::read_update_config();

    // List files in downloads directory
    let mut files: Vec<Value> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(DOWNLOADS_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            // Skip the config json file
            if name == "update_config.json" || name == "test.txt" { continue; }
            let meta = std::fs::metadata(&path).ok();
            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let modified = meta.and_then(|m| m.modified().ok())
                .map(|t| {
                    let dt: chrono::DateTime<chrono::Utc> = t.into();
                    dt.to_rfc3339()
                })
                .unwrap_or_default();
            files.push(json!({
                "name": name,
                "size_bytes": size,
                "size_mb": format!("{:.1}", size as f64 / 1024.0 / 1024.0),
                "modified_at": modified,
                "url": format!("https://nodi.vn/download/{}", name)
            }));
        }
    }
    // Sort by modified desc
    files.sort_by(|a, b| {
        let ma = a["modified_at"].as_str().unwrap_or("");
        let mb = b["modified_at"].as_str().unwrap_or("");
        mb.cmp(ma)
    });

    Ok(Json(json!({
        "config": {
            "latest_version": config.latest_version,
            "download_url": config.download_url,
            "release_notes": config.release_notes,
            "file_size": config.file_size,
            "updated_at": config.updated_at
        },
        "files": files
    })))
}

// PUT /api/admin/update — edit config without uploading
#[derive(Deserialize)]
struct EditUpdateConfig {
    latest_version: Option<String>,
    download_url: Option<String>,
    release_notes: Option<String>,
    file_size: Option<String>,
}

async fn edit_update_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<EditUpdateConfig>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let mut config = crate::routes::health::read_update_config();
    if let Some(v) = body.latest_version { config.latest_version = v; }
    if let Some(u) = body.download_url { config.download_url = u; }
    if let Some(n) = body.release_notes { config.release_notes = n; }
    if let Some(s) = body.file_size { config.file_size = s; }
    config.updated_at = chrono::Utc::now().to_rfc3339();

    crate::routes::health::write_update_config(&config)
        .map_err(|e| AppError::Internal(e))?;

    tracing::info!("✅ Update config edited: version={}", config.latest_version);

    Ok(Json(json!({
        "success": true,
        "config": {
            "latest_version": config.latest_version,
            "download_url": config.download_url,
            "release_notes": config.release_notes,
            "file_size": config.file_size,
            "updated_at": config.updated_at
        }
    })))
}

// POST /api/admin/update/upload — upload installer file + update config
async fn upload_installer(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut version: Option<String> = None;
    let mut release_notes: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                file_name = field.file_name().map(|n| n.to_string());
                file_data = Some(
                    field.bytes().await
                        .map_err(|e| AppError::Internal(format!("File read error: {}", e)))?
                        .to_vec()
                );
            }
            "version" => {
                version = Some(field.text().await.unwrap_or_default());
            }
            "release_notes" => {
                release_notes = Some(field.text().await.unwrap_or_default());
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or(AppError::BadRequest("No file uploaded".into()))?;
    let orig_name = file_name.unwrap_or_else(|| "installer.exe".into());

    // Save file
    tokio::fs::create_dir_all(DOWNLOADS_DIR).await
        .map_err(|e| AppError::Internal(format!("Dir create error: {}", e)))?;

    let filepath = format!("{}/{}", DOWNLOADS_DIR, orig_name);
    let file_size = file_data.len();

    tokio::fs::write(&filepath, &file_data).await
        .map_err(|e| AppError::Internal(format!("File write error: {}", e)))?;

    tracing::info!("✅ Installer uploaded: file={}, size={}", orig_name, file_size);

    // Update config if version provided
    let ver = version.unwrap_or_else(|| {
        // Try to extract version from filename like NodiPOS_1.2.0_x64-setup.exe
        orig_name.split('_').nth(1).unwrap_or("1.0.0").to_string()
    });
    let download_url = format!("https://nodi.vn/download/{}", orig_name);
    let size_str = format!("{:.1} MB", file_size as f64 / 1024.0 / 1024.0);

    let config = crate::routes::health::UpdateConfig {
        latest_version: ver.clone(),
        download_url: download_url.clone(),
        release_notes: release_notes.unwrap_or_default(),
        file_size: size_str.clone(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    crate::routes::health::write_update_config(&config)
        .map_err(|e| AppError::Internal(e))?;

    Ok(Json(json!({
        "success": true,
        "filename": orig_name,
        "size_bytes": file_size,
        "size_mb": size_str,
        "download_url": download_url,
        "version": ver,
        "config": {
            "latest_version": config.latest_version,
            "download_url": config.download_url,
            "release_notes": config.release_notes,
            "file_size": config.file_size,
            "updated_at": config.updated_at
        }
    })))
}

// DELETE /api/admin/update/files/:filename — delete an installer file
async fn delete_installer_file(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(filename): Path<String>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Security: prevent path traversal
    if filename.contains('/') || filename.contains("..") {
        return Err(AppError::BadRequest("Invalid filename".into()));
    }

    let filepath = format!("{}/{}", DOWNLOADS_DIR, filename);
    if !std::path::Path::new(&filepath).exists() {
        return Err(AppError::NotFound("File not found".into()));
    }

    tokio::fs::remove_file(&filepath).await
        .map_err(|e| AppError::Internal(format!("Delete error: {}", e)))?;

    tracing::info!("🗑️ Installer deleted: {}", filename);

    Ok(Json(json!({
        "success": true,
        "message": format!("Deleted {}", filename)
    })))
}

// ============================================================
// Phase 1: Account Management
// ============================================================

#[derive(Deserialize)]
struct AccountsQuery { status: Option<String> }

async fn accounts_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AccountsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, String, bool, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<String>)>(
        "SELECT a.id, a.username, a.display_name, a.phone, a.store_id, a.is_active, a.created_at, a.updated_at, a.hwid FROM accounts a ORDER BY a.created_at DESC"
    ).fetch_all(&state.pool).await?;

    let mut accounts = Vec::new();
    for r in &rows {
        let active = r.5;
        if let Some(ref s) = q.status {
            if s == "active" && !active { continue; }
            if s == "inactive" && active { continue; }
        }
        let stores_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM account_stores WHERE account_id = $1"
        ).bind(r.0).fetch_one(&state.pool).await.unwrap_or((0,));

        let store_names: Vec<(Option<String>,)> = sqlx::query_as(
            "SELECT store_name FROM account_stores WHERE account_id = $1 ORDER BY is_default DESC"
        ).bind(r.0).fetch_all(&state.pool).await.unwrap_or_default();

        let names: Vec<&str> = store_names.iter()
            .filter_map(|s| s.0.as_deref())
            .collect();

        accounts.push(json!({
            "id": r.0, "username": r.1, "display_name": r.2,
            "phone": r.3, "store_id": r.4, "is_active": active,
            "created_at": r.6.map(|d| d.to_string()),
            "updated_at": r.7.map(|d| d.to_string()),
            "hwid": r.8,
            "stores_count": stores_count.0,
            "store_names": names,
        }));
    }

    let total = accounts.len();
    let active = accounts.iter().filter(|a| a["is_active"].as_bool().unwrap_or(false)).count();

    Ok(Json(json!({
        "accounts": accounts,
        "total": total,
        "active": active,
        "inactive": total - active,
    })))
}

async fn account_toggle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let current: Option<(bool,)> = sqlx::query_as(
        "SELECT is_active FROM accounts WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await?;

    let is_active = match current {
        Some((a,)) => a,
        None => return Err(AppError::NotFound("Account not found".into())),
    };

    let new_status = !is_active;
    sqlx::query("UPDATE accounts SET is_active = $1, updated_at = NOW() WHERE id = $2")
        .bind(new_status).bind(id).execute(&state.pool).await?;

    // Log audit
    log_audit(&state.pool, "ACCOUNT_TOGGLE", "admin",
        Some("account"), Some(&id.to_string()),
        Some(&format!("is_active: {} → {}", is_active, new_status))
    ).await;

    Ok(Json(json!({
        "success": true,
        "is_active": new_status,
        "message": if new_status { "Đã kích hoạt tài khoản" } else { "Đã vô hiệu hóa tài khoản" }
    })))
}

// ============================================================
// Phase 2: Audit Log
// ============================================================

pub async fn log_audit(pool: &sqlx::PgPool, action: &str, actor: &str, target_type: Option<&str>, target_id: Option<&str>, details: Option<&str>) {
    let _ = sqlx::query(
        "INSERT INTO audit_log (action, actor, target_type, target_id, details) VALUES ($1, $2, $3, $4, $5)"
    ).bind(action).bind(actor).bind(target_type).bind(target_id).bind(details)
    .execute(pool).await;
}

#[derive(Deserialize)]
struct AuditQuery { action: Option<String>, limit: Option<i64>, offset: Option<i64> }

async fn audit_log_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AuditQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    let limit = q.limit.unwrap_or(100).min(500);
    let offset = q.offset.unwrap_or(0);

    let (rows, total) = if let Some(ref action) = q.action {
        let r = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
            "SELECT id, action, actor, target_type, target_id, details, created_at FROM audit_log WHERE action = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(action).bind(limit).bind(offset).fetch_all(&state.pool).await?;
        let t: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_log WHERE action = $1").bind(action).fetch_one(&state.pool).await.unwrap_or((0,));
        (r, t.0)
    } else {
        let r = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
            "SELECT id, action, actor, target_type, target_id, details, created_at FROM audit_log ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        ).bind(limit).bind(offset).fetch_all(&state.pool).await?;
        let t: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM audit_log").fetch_one(&state.pool).await.unwrap_or((0,));
        (r, t.0)
    };

    let logs: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "action": r.1, "actor": r.2,
        "target_type": r.3, "target_id": r.4,
        "details": r.5, "created_at": r.6.map(|d| d.to_string())
    })).collect();

    Ok(Json(json!({ "logs": logs, "total": total })))
}

// ============================================================
// Phase 3: Notifications
// ============================================================

#[derive(Deserialize)]
struct CreateNotification { title: String, message: String, #[serde(default)] target_type: String, #[serde(default)] target_id: Option<String> }

async fn notifications_list(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, title, message, target_type, target_id, created_at FROM notifications ORDER BY created_at DESC LIMIT 200"
    ).fetch_all(&state.pool).await?;

    let notifications: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "title": r.1, "message": r.2,
        "target_type": r.3, "target_id": r.4,
        "created_at": r.5.map(|d| d.to_string())
    })).collect();

    Ok(Json(json!({ "notifications": notifications, "total": rows.len() })))
}

async fn create_notification(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateNotification>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let target = if body.target_type.is_empty() { "all" } else { &body.target_type };

    let id: (i32,) = sqlx::query_as(
        "INSERT INTO notifications (title, message, target_type, target_id) VALUES ($1, $2, $3, $4) RETURNING id"
    ).bind(&body.title).bind(&body.message).bind(target).bind(&body.target_id)
    .fetch_one(&state.pool).await?;

    log_audit(&state.pool, "NOTIFICATION_CREATED", "admin",
        Some("notification"), Some(&id.0.to_string()),
        Some(&format!("title: {}, target: {}", body.title, target))
    ).await;

    Ok(Json(json!({ "success": true, "id": id.0 })))
}

async fn delete_notification(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;
    sqlx::query("DELETE FROM notifications WHERE id = $1").bind(id).execute(&state.pool).await?;
    Ok(Json(json!({ "success": true })))
}

// ============================================================
// Phase 4: Enhanced Analytics
// ============================================================

async fn license_revenue(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Total collected
    let total: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount), 0) FROM license_payments"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    // Monthly revenue trend
    let monthly = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT TO_CHAR(created_at, 'YYYY-MM'), COALESCE(SUM(amount), 0), COUNT(*) \
         FROM license_payments WHERE created_at >= NOW() - INTERVAL '12 months' \
         GROUP BY 1 ORDER BY 1"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let trend: Vec<Value> = monthly.iter().map(|r| json!({
        "month": r.0, "revenue": r.1, "payments": r.2
    })).collect();

    // MRR (last 30 days)
    let mrr: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount), 0) FROM license_payments WHERE created_at >= NOW() - INTERVAL '30 days'"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    // License type breakdown
    let types = sqlx::query_as::<_, (Option<String>, i64)>(
        "SELECT license_type, COUNT(*) FROM stores WHERE license_type IS NOT NULL GROUP BY license_type ORDER BY 2 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let license_types: Vec<Value> = types.iter().map(|r| json!({
        "type": r.0.as_deref().unwrap_or("unknown"), "count": r.1
    })).collect();

    // New stores per month
    let new_stores = sqlx::query_as::<_, (String, i64)>(
        "SELECT TO_CHAR(created_at, 'YYYY-MM'), COUNT(*) FROM stores \
         WHERE created_at >= NOW() - INTERVAL '12 months' \
         GROUP BY 1 ORDER BY 1"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let growth: Vec<Value> = new_stores.iter().map(|r| json!({
        "month": r.0, "new_stores": r.1
    })).collect();

    Ok(Json(json!({
        "total_collected": total.0,
        "mrr": mrr.0,
        "revenue_by_month": trend,
        "license_types": license_types,
        "store_growth": growth,
    })))
}

async fn geo_analysis(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Stores by province
    let provinces = sqlx::query_as::<_, (Option<String>, i64, i64)>(
        "SELECT s.province, COUNT(*), COALESCE(SUM(sub.rev), 0) FROM stores s \
         LEFT JOIN (SELECT store_id, SUM(final_amount::bigint) as rev FROM synced_invoices GROUP BY store_id) sub ON sub.store_id = s.id \
         WHERE s.province IS NOT NULL AND s.province != '' \
         GROUP BY s.province ORDER BY 2 DESC"
    ).fetch_all(&state.pool).await.unwrap_or_default();

    let by_province: Vec<Value> = provinces.iter().map(|r| json!({
        "province": r.0, "stores": r.1, "revenue": r.2
    })).collect();

    // Stores without province
    let no_province: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM stores WHERE province IS NULL OR province = ''"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    Ok(Json(json!({
        "by_province": by_province,
        "no_province_count": no_province.0,
    })))
}

async fn stores_compare(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<bool>)>(
        "SELECT id, owner_name, license_type, is_active FROM stores WHERE is_active = true ORDER BY id"
    ).fetch_all(&state.pool).await?;

    let mut stores = Vec::new();
    for r in &rows {
        let sid = r.0;
        let rev: (i64,) = sqlx::query_as("SELECT COALESCE(SUM(final_amount::bigint), 0) FROM synced_invoices WHERE store_id = $1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_products WHERE store_id = $1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let customers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM synced_customers WHERE store_id = $1").bind(sid).fetch_one(&state.pool).await.unwrap_or((0,));
        let last_sync: Option<(Option<chrono::NaiveDateTime>,)> = sqlx::query_as(
            "SELECT MAX(created_at) FROM synced_invoices WHERE store_id = $1"
        ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);
        let avg_order = if orders.0 > 0 { rev.0 / orders.0 } else { 0 };

        stores.push(json!({
            "id": sid, "name": r.1, "license_type": r.2,
            "revenue": rev.0, "orders": orders.0,
            "products": products.0, "customers": customers.0,
            "avg_order_value": avg_order,
            "last_activity": last_sync.and_then(|s| s.0.map(|d| d.to_string())),
        }));
    }

    // Sort by revenue descending
    stores.sort_by(|a, b| b["revenue"].as_i64().cmp(&a["revenue"].as_i64()));

    Ok(Json(json!({ "stores": stores })))
}

// ============================================================
// Phase 5: CSV Export
// ============================================================

async fn export_csv(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(export_type): Path<String>,
) -> Result<axum::response::Response, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let csv_content = match export_type.as_str() {
        "stores" => {
            let rows = sqlx::query_as::<_, (i32, Option<String>, Option<String>, Option<String>, Option<bool>, Option<String>, Option<chrono::NaiveDateTime>)>(
                "SELECT id, owner_name, phone, license_type, is_active, province, created_at FROM stores ORDER BY id"
            ).fetch_all(&state.pool).await?;
            let mut csv = "ID,Tên,SĐT,Gói,Hoạt động,Tỉnh/TP,Ngày tạo\n".to_string();
            for r in &rows {
                csv += &format!("{},{},{},{},{},{},{}\n",
                    r.0, r.1.as_deref().unwrap_or(""), r.2.as_deref().unwrap_or(""),
                    r.3.as_deref().unwrap_or("free"), r.4.unwrap_or(false),
                    r.5.as_deref().unwrap_or(""), r.6.map(|d| d.to_string()).unwrap_or_default()
                );
            }
            csv
        },
        "accounts" => {
            let rows = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, bool, Option<chrono::NaiveDateTime>)>(
                "SELECT id, username, display_name, phone, is_active, created_at FROM accounts ORDER BY id"
            ).fetch_all(&state.pool).await?;
            let mut csv = "ID,Username,Tên,SĐT,Hoạt động,Ngày tạo\n".to_string();
            for r in &rows {
                csv += &format!("{},{},{},{},{},{}\n",
                    r.0, r.1, r.2.as_deref().unwrap_or(""), r.3.as_deref().unwrap_or(""),
                    r.4, r.5.map(|d| d.to_string()).unwrap_or_default()
                );
            }
            csv
        },
        "products" => {
            let rows = sqlx::query_as::<_, (i32, i32, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>)>(
                "SELECT store_id, local_id, name, sku, sell_price, cost_price, stock FROM synced_products ORDER BY store_id, local_id LIMIT 10000"
            ).fetch_all(&state.pool).await?;
            let mut csv = "Store ID,Product ID,Tên SP,SKU,Giá bán,Giá vốn,Tồn kho\n".to_string();
            for r in &rows {
                csv += &format!("{},{},{},{},{},{},{}\n",
                    r.0, r.1, r.2.as_deref().unwrap_or(""), r.3.as_deref().unwrap_or(""),
                    r.4.unwrap_or(0.0), r.5.unwrap_or(0.0), r.6.unwrap_or(0.0)
                );
            }
            csv
        },
        "orders" => {
            let rows = sqlx::query_as::<_, (i32, i32, Option<f64>, Option<f64>, Option<String>, Option<chrono::NaiveDateTime>)>(
                "SELECT store_id, local_id, total_amount, final_amount, customer_name, created_at FROM synced_invoices ORDER BY created_at DESC LIMIT 10000"
            ).fetch_all(&state.pool).await?;
            let mut csv = "Store ID,Order ID,Tổng tiền,Thành tiền,Khách hàng,Ngày\n".to_string();
            for r in &rows {
                csv += &format!("{},{},{},{},{},{}\n",
                    r.0, r.1, r.2.unwrap_or(0.0), r.3.unwrap_or(0.0),
                    r.4.as_deref().unwrap_or(""),
                    r.5.map(|d| d.to_string()).unwrap_or_default()
                );
            }
            csv
        },
        _ => return Err(AppError::BadRequest("Invalid export type. Use: stores, accounts, products, orders".into())),
    };

    log_audit(&state.pool, "EXPORT_CSV", "admin", Some("export"), Some(&export_type), None).await;

    let filename = format!("nodi_{}.csv", export_type);
    Ok(axum::response::Response::builder()
        .header("Content-Type", "text/csv; charset=utf-8")
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(axum::body::Body::from(csv_content))
        .unwrap())
}

// ============================================================
// GET /api/admin/license/:id/devices — Admin: view devices per store
// ============================================================
async fn admin_license_devices(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = sqlx::query_as::<_, (i32, String, String, Option<String>, bool, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, device_id, device_type, device_name, is_active, first_activated_at, last_active_at \
         FROM devices WHERE store_id = $1 ORDER BY is_active DESC, last_active_at DESC"
    )
    .bind(id)
    .fetch_all(&state.pool)
    .await?;

    let devices: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0,
        "device_id": r.1,
        "device_type": r.2,
        "device_name": r.3,
        "is_active": r.4,
        "first_activated_at": r.5.map(|t| t.and_utc().to_rfc3339()),
        "last_active_at": r.6.map(|t| t.and_utc().to_rfc3339())
    })).collect();

    let active_count = rows.iter().filter(|r| r.4).count();

    Ok(Json(json!({
        "store_id": id,
        "devices": devices,
        "active_count": active_count,
        "total_count": devices.len(),
        "max_devices": 10
    })))
}

// ===== Phase 7: Payment Orders Management =====

#[derive(Deserialize)]
struct OrdersQuery { status: Option<String>, page: Option<i64>, limit: Option<i64> }

async fn admin_orders_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<OrdersQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = (page - 1) * limit;

    let (rows, total) = if let Some(ref status) = q.status {
        let r = sqlx::query_as::<_, (i32, String, String, i32, Option<String>, String, Option<String>, String, Option<String>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, order_code, plan, amount, customer_name, customer_phone, customer_email, status, license_key, paid_at, expired_at, created_at \
             FROM orders WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        ).bind(status).bind(limit).bind(offset).fetch_all(&state.pool).await?;
        let t: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders WHERE status = $1")
            .bind(status).fetch_one(&state.pool).await.unwrap_or((0,));
        (r, t.0)
    } else {
        let r = sqlx::query_as::<_, (i32, String, String, i32, Option<String>, String, Option<String>, String, Option<String>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>, chrono::DateTime<chrono::Utc>)>(
            "SELECT id, order_code, plan, amount, customer_name, customer_phone, customer_email, status, license_key, paid_at, expired_at, created_at \
             FROM orders ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        ).bind(limit).bind(offset).fetch_all(&state.pool).await?;
        let t: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM orders")
            .fetch_one(&state.pool).await.unwrap_or((0,));
        (r, t.0)
    };

    let orders: Vec<Value> = rows.iter().map(|r| {
        let now = chrono::Utc::now();
        let display_status = if r.7 == "PENDING" {
            if let Some(exp) = r.10 { if now > exp { "EXPIRED" } else { "PENDING" } } else { "PENDING" }
        } else { &r.7 };

        json!({
            "id": r.0, "order_code": r.1, "plan": r.2, "amount": r.3,
            "customer_name": r.4, "customer_phone": r.5, "customer_email": r.6,
            "status": display_status, "license_key": r.8,
            "paid_at": r.9.map(|d| d.to_rfc3339()),
            "expired_at": r.10.map(|d| d.to_rfc3339()),
            "created_at": r.11.to_rfc3339()
        })
    }).collect();

    // Summary counts
    let counts = sqlx::query_as::<_, (i64, i64, i64)>(
        "SELECT COUNT(*) FILTER (WHERE status = 'PENDING'), \
         COUNT(*) FILTER (WHERE status = 'PAID'), \
         COUNT(*) FILTER (WHERE status = 'EXPIRED' OR (status = 'PENDING' AND expired_at < NOW())) \
         FROM orders"
    ).fetch_one(&state.pool).await.unwrap_or((0,0,0));

    Ok(Json(json!({
        "orders": orders,
        "total": total,
        "pending": counts.0,
        "paid": counts.1,
        "expired": counts.2
    })))
}

async fn admin_confirm_order(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Find the order
    let order = sqlx::query_as::<_, (String, String, i32, String, String)>(
        "SELECT order_code, plan, amount, status, customer_phone FROM orders WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await?;

    let (order_code, plan, _amount, status, phone) = match order {
        Some(o) => o,
        None => return Err(AppError::NotFound("Order not found".into())),
    };

    if status != "PENDING" {
        return Err(AppError::BadRequest(format!("Order is already {}", status)));
    }

    // Generate license key
    let mut key = generate_license_key();
    for _ in 0..10 {
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM stores WHERE license_key = $1)")
            .bind(&key).fetch_one(&state.pool).await.unwrap_or((false,));
        if !exists.0 { break; }
        key = generate_license_key();
    }

    // Duration: plan + 30 days trial
    let days = match plan.as_str() {
        "YEARLY" => 365 + 30,
        _ => 30 + 30,
    };

    let store_name = format!("Store-{}", &phone[phone.len().saturating_sub(4)..]);
    let license_type = if plan == "YEARLY" { "YEARLY" } else { "MONTHLY" };

    // Create store/license
    sqlx::query(
        "INSERT INTO stores (name, license_key, license_type, is_active, owner_name, license_expires_at, created_at, duration_days) \
         VALUES ($1, $2, $3, true, $1, NOW() + ($4 || ' days')::interval, NOW(), $4)"
    )
    .bind(&store_name)
    .bind(&key)
    .bind(license_type)
    .bind(days.to_string())
    .execute(&state.pool).await?;

    // Update order
    sqlx::query("UPDATE orders SET status = 'PAID', license_key = $1, paid_at = NOW() WHERE id = $2")
        .bind(&key)
        .bind(id)
        .execute(&state.pool).await?;

    // Log audit
    log_audit(&state.pool, "ORDER_CONFIRMED", "admin", Some("order"), Some(&order_code), Some(&format!("license_key={}", key))).await;

    tracing::info!("✅ Admin confirmed order {} — License {} created", order_code, key);

    Ok(Json(json!({
        "success": true,
        "order_code": order_code,
        "license_key": key,
        "license_type": license_type,
        "duration_days": days,
        "customer_phone": phone
    })))
}
