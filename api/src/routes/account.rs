use axum::{Router, Json, routing::post, extract::State};
use serde::Deserialize;
use serde_json::{json, Value};
use rand::Rng;
use sqlx::PgPool;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth;
use chrono;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/unbind-device", post(unbind_device))
        .route("/api/update-phone", post(update_phone))
}

// ============================================================
// Helpers
// ============================================================

/// Generate STORE-XXXXXXXX (8 uppercase alphanumeric chars)
fn generate_store_id() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let code: String = (0..8).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("STORE-{}", code)
}

/// Normalize phone: strip spaces, accept 0xxx or +84xxx
fn normalize_phone(phone: &str) -> Option<String> {
    let trimmed = phone.trim().replace(" ", "").replace("-", "");
    if trimmed.is_empty() {
        return None;
    }
    // Accept +84... or 0...
    if trimmed.starts_with("+84") && trimmed.len() >= 11 {
        Some(trimmed)
    } else if trimmed.starts_with('0') && trimmed.len() >= 10 && trimmed.len() <= 11 {
        Some(trimmed)
    } else {
        Some(trimmed) // Store as-is, don't block registration
    }
}

/// Migrate anonymous HWID data to account.
/// When user registers/logs in with HWID, find any synced data under that HWID's
/// old store entry and migrate it to the account's store_id (account_id + 1M).
async fn migrate_hwid_data(pool: &PgPool, hwid: &str, data_store_id: i32) {
    if hwid.is_empty() { return; }
    let new_store_id = data_store_id;

    // Find old store_id(s) linked to this HWID in the stores table
    let old_ids: Vec<(i32,)> = sqlx::query_as(
        "SELECT id FROM stores WHERE hwid = $1"
    ).bind(hwid).fetch_all(pool).await.unwrap_or_default();

    if old_ids.is_empty() { return; }

    for (old_store_id,) in &old_ids {
        if *old_store_id == new_store_id { continue; }

        // Check if there's actually data to migrate
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1"
        ).bind(old_store_id).fetch_one(pool).await.unwrap_or((0,));

        let prod_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM synced_products WHERE store_id = $1"
        ).bind(old_store_id).fetch_one(pool).await.unwrap_or((0,));

        if count.0 == 0 && prod_count.0 == 0 { continue; }

        tracing::info!("🔄 Migrating HWID data: store_id={} → {} (invoices={}, products={})",
            old_store_id, new_store_id, count.0, prod_count.0);

        // Delete any conflicting data at new_store_id first, then migrate
        // Use a list of all synced tables
        let tables = [
            "synced_invoice_payments", "synced_invoice_items", "synced_invoices",
            "synced_product_units", "synced_product_batches", "synced_product_transactions", "synced_products",
            "synced_customer_transactions", "synced_customers",
            "synced_supplier_transactions", "synced_suppliers",
            "synced_purchase_items", "synced_purchase_orders",
            "synced_cash_transactions", "synced_payment_vouchers",
            "synced_store_funds", "synced_store_settings",
        ];

        for table in &tables {
            // Delete conflicts at target first
            let del_sql = format!("DELETE FROM {} WHERE store_id = $1", table);
            let _ = sqlx::query(&del_sql).bind(new_store_id).execute(pool).await;
            // Migrate old data
            let upd_sql = format!("UPDATE {} SET store_id = $1 WHERE store_id = $2", table);
            let _ = sqlx::query(&upd_sql).bind(new_store_id).bind(old_store_id).execute(pool).await;
        }

        tracing::info!("✅ HWID data migration complete: {} → {}", old_store_id, new_store_id);
    }
}

// ============================================================
// POST /api/register
// ============================================================

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
    #[serde(default)]
    store_name: String,
    #[serde(default)]
    phone: String,
    #[serde(default)]
    hwid: String,
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<Value>, AppError> {
    let username = req.username.trim().to_lowercase();
    let store_name = if req.store_name.trim().is_empty() {
        "Cửa hàng mới".to_string()
    } else {
        req.store_name.trim().to_string()
    };
    let phone = normalize_phone(&req.phone);

    // Validate username
    if username.is_empty() || req.password.is_empty() {
        return Ok(Json(json!({ "success": false, "message": "Vui lòng điền đầy đủ thông tin" })));
    }
    if username.len() < 3 || username.len() > 50 {
        return Ok(Json(json!({ "success": false, "message": "Tên đăng nhập phải từ 3-50 ký tự" })));
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Ok(Json(json!({ "success": false, "message": "Tên đăng nhập chỉ gồm chữ, số và _" })));
    }

    // Validate password
    if req.password.len() < 6 || req.password.len() > 100 {
        return Ok(Json(json!({ "success": false, "message": "Mật khẩu phải từ 6-100 ký tự" })));
    }

    // Validate phone (REQUIRED)
    let phone_val = match &phone {
        Some(p) if p.len() >= 9 => p.clone(),
        _ => {
            return Ok(Json(json!({ "success": false, "message": "Số điện thoại bắt buộc (tối thiểu 9 số)" })));
        }
    };

    // Check username uniqueness
    let existing = sqlx::query_scalar::<_, i32>("SELECT id FROM accounts WHERE username = $1 LIMIT 1")
        .bind(&username).fetch_optional(&state.pool).await?;
    if existing.is_some() {
        return Ok(Json(json!({ "success": false, "message": "Tên đăng nhập đã tồn tại" })));
    }

    // Check phone uniqueness
    let phone_exists = sqlx::query_scalar::<_, i32>("SELECT id FROM accounts WHERE phone = $1 LIMIT 1")
        .bind(&phone_val).fetch_optional(&state.pool).await?;
    if phone_exists.is_some() {
        return Ok(Json(json!({ "success": false, "message": "Số điện thoại đã được sử dụng" })));
    }

    // Hash password
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))?;

    // Generate unique store_id
    let mut store_id = generate_store_id();
    for _ in 0..10 {
        let collision = sqlx::query_scalar::<_, i32>("SELECT id FROM accounts WHERE store_id = $1 LIMIT 1")
            .bind(&store_id).fetch_optional(&state.pool).await?;
        if collision.is_none() { break; }
        store_id = generate_store_id();
    }

    // Insert account
    let user_id: i32 = sqlx::query_scalar(
        "INSERT INTO accounts (username, password_hash, display_name, store_name, store_id, phone, hwid, role) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'owner') RETURNING id"
    )
    .bind(&username).bind(&password_hash).bind(&store_name).bind(&store_name)
    .bind(&store_id).bind(&phone_val)
    .bind(if req.hwid.is_empty() { None } else { Some(&req.hwid) })
    .fetch_one(&state.pool).await?;

    // Insert into account_stores with data_store_id
    let data_store_id = user_id + 1_000_000;
    let _ = sqlx::query(
        "INSERT INTO account_stores (account_id, store_id, store_name, role, is_default, data_store_id) VALUES ($1, $2, $3, 'owner', TRUE, $4)"
    ).bind(user_id).bind(&store_id).bind(&store_name).bind(data_store_id).execute(&state.pool).await?;

    // Auto-migrate anonymous HWID data to this new account
    if !req.hwid.is_empty() {
        migrate_hwid_data(&state.pool, &req.hwid, data_store_id).await;
    }

    // Generate JWT token using data_store_id
    let token = auth::create_token(user_id, data_store_id, "owner", &state.config.jwt_secret)?;
    let refresh_token = auth::create_refresh_token(user_id, data_store_id, "owner", &state.config.jwt_secret)?;

    tracing::info!("✅ New account registered: username={}, phone={}, store_id={}, user_id={}", username, phone_val, store_id, user_id);

    Ok(Json(json!({
        "success": true,
        "user_id": user_id,
        "store_id": store_id,
        "store_name": store_name,
        "token": token,
        "refresh_token": refresh_token,
        "user": { "id": user_id, "username": username, "display_name": store_name, "role": "owner" },
        "stores": [{ "store_id": store_id, "store_name": store_name, "role": "owner", "is_default": true, "data_store_id": data_store_id }],
        "message": "Đăng ký thành công"
    })))
}

// ============================================================
// POST /api/login
// username field accepts BOTH username and phone number
// ============================================================

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
    #[serde(default)]
    hwid: String,
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    let input = req.username.trim().to_lowercase();

    if input.is_empty() || req.password.is_empty() {
        return Ok(Json(json!({ "success": false, "message": "Vui lòng điền đầy đủ thông tin" })));
    }

    // Find account by username OR phone
    let account = sqlx::query_as::<_, (i32, String, String, String, String, String, bool, i32, Option<chrono::NaiveDateTime>, String)>(
        "SELECT id, username, password_hash, display_name, store_name, store_id, is_active, \
         COALESCE(failed_login_attempts, 0), locked_until, COALESCE(role, 'owner') \
         FROM accounts WHERE username = $1 OR phone = $1 LIMIT 1"
    )
    .bind(&input)
    .fetch_optional(&state.pool)
    .await?;

    let (user_id, actual_username, password_hash, display_name, _store_name, _store_id, is_active, failed_attempts, locked_until, db_role) = match account {
        Some(a) => a,
        None => {
            return Ok(Json(json!({ "success": false, "message": "Tên đăng nhập hoặc mật khẩu không đúng" })));
        }
    };

    // Check active
    if !is_active {
        return Ok(Json(json!({ "success": false, "message": "Tài khoản đã bị khóa" })));
    }

    // Check account lockout
    if let Some(lock_time) = locked_until {
        let now = chrono::Utc::now().naive_utc();
        if now < lock_time {
            let remaining = (lock_time - now).num_minutes() + 1;
            return Ok(Json(json!({
                "success": false,
                "message": format!("Tài khoản tạm khóa, thử lại sau {} phút", remaining)
            })));
        }
    }

    // Verify password
    let valid = bcrypt::verify(&req.password, &password_hash).unwrap_or(false);
    if !valid {
        // Increment failed attempts
        let new_attempts = failed_attempts + 1;
        if new_attempts >= 5 {
            let _ = sqlx::query("UPDATE accounts SET failed_login_attempts = $1, locked_until = NOW() + INTERVAL '15 minutes' WHERE id = $2")
                .bind(new_attempts).bind(user_id).execute(&state.pool).await;
            return Ok(Json(json!({ "success": false, "message": "Sai mật khẩu 5 lần. Tài khoản tạm khóa 15 phút" })));
        } else {
            let _ = sqlx::query("UPDATE accounts SET failed_login_attempts = $1 WHERE id = $2")
                .bind(new_attempts).bind(user_id).execute(&state.pool).await;
        }
        return Ok(Json(json!({ "success": false, "message": "Tên đăng nhập hoặc mật khẩu không đúng" })));
    }

    // Reset failed attempts on success
    let _ = sqlx::query("UPDATE accounts SET failed_login_attempts = 0, locked_until = NULL, updated_at = NOW() WHERE id = $1")
        .bind(user_id).execute(&state.pool).await;

    // Update hwid + auto-migrate anonymous data
    if !req.hwid.is_empty() {
        let _ = sqlx::query("UPDATE accounts SET hwid = $1 WHERE id = $2")
            .bind(&req.hwid).bind(user_id).execute(&state.pool).await;
        // Migrate any anonymous HWID data to this account's default store
        let default_data_store_id = sqlx::query_scalar::<_, i32>(
            "SELECT data_store_id FROM account_stores WHERE account_id = $1 AND is_default = TRUE LIMIT 1"
        ).bind(user_id).fetch_optional(&state.pool).await?.unwrap_or(user_id + 1_000_000);
        migrate_hwid_data(&state.pool, &req.hwid, default_data_store_id).await;
    }

    // Fetch stores for this account
    let stores = sqlx::query_as::<_, (String, Option<String>, String, bool, i32)>(
        "SELECT store_id, store_name, role, is_default, data_store_id FROM account_stores WHERE account_id = $1 ORDER BY is_default DESC, created_at"
    ).bind(user_id).fetch_all(&state.pool).await?;

    // Find default store
    let default_store = stores.iter().find(|s| s.3).or(stores.first());
    let (active_store_id, active_store_name, active_data_store_id) = match default_store {
        Some(s) => (s.0.clone(), s.1.clone().unwrap_or_default(), s.4),
        None => (_store_id.clone(), _store_name.clone(), user_id + 1_000_000),
    };

    let stores_json: Vec<Value> = stores.iter().map(|s| json!({
        "store_id": s.0,
        "store_name": s.1,
        "role": s.2,
        "is_default": s.3,
        "data_store_id": s.4
    })).collect();

    // Generate JWT with default store's data_store_id and actual role
    let jwt_role = db_role.as_str();
    let token = auth::create_token(user_id, active_data_store_id, jwt_role, &state.config.jwt_secret)?;
    let refresh_token = auth::create_refresh_token(user_id, active_data_store_id, jwt_role, &state.config.jwt_secret)?;

    tracing::info!("✅ Account login: username={}, role={}, store_id={}, stores={}", actual_username, jwt_role, active_store_id, stores.len());

    Ok(Json(json!({
        "success": true,
        "token": token,
        "refresh_token": refresh_token,
        "user": { "id": user_id, "username": actual_username, "display_name": display_name, "role": jwt_role },
        "store_id": active_store_id,
        "store_name": active_store_name,
        "stores": stores_json
    })))
}

// ============================================================
// POST /api/unbind-device
// ============================================================

#[derive(Deserialize)]
struct UnbindDeviceRequest {
    store_id: String,
    device_hwid: String,
    owner_username: String,
    owner_password: String,
}

async fn unbind_device(
    State(state): State<AppState>,
    Json(req): Json<UnbindDeviceRequest>,
) -> Result<Json<Value>, AppError> {
    // Find account by username
    let account = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, password_hash, store_id FROM accounts WHERE username = $1"
    )
    .bind(req.owner_username.trim().to_lowercase())
    .fetch_optional(&state.pool)
    .await?;

    let (account_id, password_hash, account_store_id) = match account {
        Some(a) => a,
        None => {
            return Ok(Json(json!({
                "success": false,
                "message": "Thông tin đăng nhập không đúng"
            })));
        }
    };

    // Verify password
    let valid = bcrypt::verify(&req.owner_password, &password_hash).unwrap_or(false);
    if !valid {
        return Ok(Json(json!({
            "success": false,
            "message": "Thông tin đăng nhập không đúng"
        })));
    }

    // Check store ownership
    if account_store_id != req.store_id {
        return Ok(Json(json!({
            "success": false,
            "message": "Bạn không có quyền gỡ liên kết thiết bị của cửa hàng này"
        })));
    }

    // Clear hwid if it matches the device_hwid
    let result = sqlx::query(
        "UPDATE accounts SET hwid = NULL, updated_at = NOW() WHERE id = $1 AND hwid = $2"
    )
    .bind(account_id)
    .bind(&req.device_hwid)
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(Json(json!({
            "success": false,
            "message": "Không tìm thấy thiết bị này trong cửa hàng"
        })));
    }

    tracing::info!("✅ Device unbound: store_id={}, hwid={}", req.store_id, req.device_hwid);

    Ok(Json(json!({
        "success": true,
        "message": "Đã gỡ liên kết thiết bị thành công"
    })))
}

// ============================================================
// POST /api/update-phone
// ============================================================

#[derive(Deserialize)]
struct UpdatePhoneRequest {
    store_id: String,
    username: String,
    password: String,
    phone: String,
}

async fn update_phone(
    State(state): State<AppState>,
    Json(req): Json<UpdatePhoneRequest>,
) -> Result<Json<Value>, AppError> {
    let username = req.username.trim().to_lowercase();
    let phone = normalize_phone(&req.phone);

    let phone = match phone {
        Some(p) => p,
        None => {
            return Ok(Json(json!({
                "success": false,
                "message": "Số điện thoại không hợp lệ"
            })));
        }
    };

    // Find account
    let account = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, password_hash, store_id FROM accounts WHERE username = $1"
    )
    .bind(&username)
    .fetch_optional(&state.pool)
    .await?;

    let (account_id, password_hash, account_store_id) = match account {
        Some(a) => a,
        None => {
            return Ok(Json(json!({
                "success": false,
                "message": "Thông tin đăng nhập không đúng"
            })));
        }
    };

    // Verify password
    let valid = bcrypt::verify(&req.password, &password_hash).unwrap_or(false);
    if !valid {
        return Ok(Json(json!({
            "success": false,
            "message": "Thông tin đăng nhập không đúng"
        })));
    }

    // Verify store ownership
    if account_store_id != req.store_id {
        return Ok(Json(json!({
            "success": false,
            "message": "Thông tin cửa hàng không khớp"
        })));
    }

    // Update phone
    sqlx::query("UPDATE accounts SET phone = $1, updated_at = NOW() WHERE id = $2")
        .bind(&phone)
        .bind(account_id)
        .execute(&state.pool)
        .await?;

    tracing::info!("✅ Phone updated: username={}, phone={}", username, phone);

    Ok(Json(json!({
        "success": true,
        "message": "Đã cập nhật số điện thoại"
    })))
}
