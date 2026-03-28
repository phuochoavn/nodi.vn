use axum::{Router, Json, routing::post, extract::State};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub license_key: String,
    #[serde(default)]
    pub hwid: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
    pub password: String,
    pub store_name: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/login-with-license", post(login_with_license))
        .route("/api/auth/register", post(register))
        .route("/api/auth/refresh", post(refresh_token))
}

// ============================================================
// POST /api/auth/register
// ============================================================

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<Value>, AppError> {
    // Validate phone: 10-11 digits, starts with 0
    let phone = req.phone.trim();
    if phone.len() < 10 || phone.len() > 11 || !phone.starts_with('0') || !phone.chars().all(|c| c.is_ascii_digit()) {
        return Err(AppError::BadRequest("Số điện thoại không hợp lệ (10-11 số, bắt đầu bằng 0)".into()));
    }

    // Validate password: min 6 chars
    if req.password.len() < 6 {
        return Err(AppError::BadRequest("Mật khẩu phải có ít nhất 6 ký tự".into()));
    }

    // Validate store_name
    let store_name = req.store_name.trim();
    if store_name.is_empty() || store_name.len() > 255 {
        return Err(AppError::BadRequest("Tên cửa hàng không hợp lệ".into()));
    }

    // Check phone not already registered
    let existing = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM users WHERE phone = $1 LIMIT 1"
    )
    .bind(phone)
    .fetch_optional(&state.pool)
    .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Số điện thoại đã được đăng ký".into()));
    }

    // Generate license key: FREE-XXXX-XXXX-XXXX
    let license_key = generate_free_license_key();

    // Hash password
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))?;

    // Create store
    let store_id: i32 = sqlx::query_scalar(
        "INSERT INTO stores (name, license_key, owner_name, phone, license_type, is_active, duration_days) \
         VALUES ($1, $2, $3, $4, 'free', true, 36500) RETURNING id"
    )
    .bind(store_name)
    .bind(&license_key)
    .bind(store_name)
    .bind(phone)
    .fetch_one(&state.pool)
    .await?;

    // Create user
    let user_id: i32 = sqlx::query_scalar(
        "INSERT INTO users (store_id, phone, password_hash, role, display_name) \
         VALUES ($1, $2, $3, 'store_owner', $4) RETURNING id"
    )
    .bind(store_id)
    .bind(phone)
    .bind(&password_hash)
    .bind(store_name)
    .fetch_one(&state.pool)
    .await?;

    // Generate JWT
    let token = auth::create_token(user_id, store_id, "store_owner", "free", &state.config.jwt_secret)?;

    // Generate refresh token
    let refresh_token = auth::create_refresh_token(user_id, store_id, "store_owner", "free", &state.config.jwt_secret)?;

    tracing::info!("✅ New user registered: phone={}, store_id={}, license={}", phone, store_id, license_key);

    Ok(Json(json!({
        "success": true,
        "token": token,
        "refresh_token": refresh_token,
        "user": {
            "id": user_id,
            "store_id": store_id,
            "username": phone,
            "display_name": store_name,
            "role": "store_owner"
        },
        "store": {
            "id": store_id,
            "name": store_name,
            "license_key": license_key,
            "license_type": "free"
        },
        "server_time": Utc::now().to_rfc3339(),
        "message": "Đăng ký thành công!"
    })))
}

fn generate_free_license_key() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let mut segment = || -> String {
        (0..4).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
    };
    format!("FREE-{}-{}-{}", segment(), segment(), segment())
}

// ============================================================
// POST /api/auth/refresh
// ============================================================

async fn refresh_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let header = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = header.strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid Authorization format".into()))?;

    // Verify refresh token
    let claims = auth::verify_refresh_token(token, &state.config.jwt_secret)?;

    // Check user still exists and is active
    let user = sqlx::query_as::<_, (i32, i32, Option<String>)>(
        "SELECT u.id, u.store_id, u.role FROM users u JOIN stores s ON u.store_id = s.id \
         WHERE u.id = $1 AND s.is_active = true"
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await?;

    let (user_id, store_id, role) = match user {
        Some(u) => u,
        None => return Err(AppError::Unauthorized("User not found or inactive".into())),
    };

    let role_str = role.unwrap_or_else(|| "store_owner".to_string());

    // Query plan_type from accounts for JWT (fallback "free" for legacy users)
    let plan_type: String = sqlx::query_scalar(
        "SELECT COALESCE(plan_type, 'free') FROM accounts WHERE id = $1"
    ).bind(user_id).fetch_optional(&state.pool).await?.unwrap_or_else(|| "free".to_string());

    // Generate new access token
    let new_token = auth::create_token(user_id, store_id, &role_str, &plan_type, &state.config.jwt_secret)?;
    // Generate new refresh token
    let new_refresh = auth::create_refresh_token(user_id, store_id, &role_str, &plan_type, &state.config.jwt_secret)?;

    Ok(Json(json!({
        "success": true,
        "token": new_token,
        "refresh_token": new_refresh,
        "expires_in": 86400
    })))
}

// ============================================================
// POST /api/login-with-license (existing)
// ============================================================

async fn login_with_license(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {
    let is_web_login = req.license_key.is_empty() && req.hwid.is_empty();

    let (store_id, is_active, license_type, owner_name, license_expires_at);

    if is_web_login {
        // Web login: find user by phone across all stores, no license required
        let user_row = sqlx::query_as::<_, (i32, i32)>(
            "SELECT u.id, u.store_id FROM users u JOIN stores s ON u.store_id = s.id WHERE u.phone = $1 AND s.is_active = true LIMIT 1"
        )
        .bind(&req.username)
        .fetch_optional(&state.pool)
        .await?;

        let (_, sid) = match user_row {
            Some(r) => r,
            None => return Ok(Json(json!({
                "success": false, "user": null,
                "message": "Tên đăng nhập hoặc mật khẩu không đúng"
            }))),
        };

        let store = sqlx::query_as::<_, (i32, Option<bool>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
            "SELECT id, is_active, license_type, owner_name, license_expires_at FROM stores WHERE id = $1"
        )
        .bind(sid)
        .fetch_one(&state.pool)
        .await?;

        store_id = store.0;
        is_active = store.1;
        license_type = store.2;
        owner_name = store.3;
        license_expires_at = store.4;
    } else {
        // App login: find store by license_key
        let store = sqlx::query_as::<_, (i32, Option<bool>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
            "SELECT id, is_active, license_type, owner_name, license_expires_at FROM stores WHERE license_key = $1"
        )
        .bind(&req.license_key)
        .fetch_optional(&state.pool)
        .await?;

        let s = match store {
            Some(s) => s,
            None => return Err(AppError::NotFound("License key not found".to_string())),
        };
        store_id = s.0;
        is_active = s.1;
        license_type = s.2;
        owner_name = s.3;
        license_expires_at = s.4;
    }

    // Check license active
    if !is_active.unwrap_or(false) {
        return Err(AppError::Forbidden("License has been suspended".to_string()));
    }

    // Find user by phone AND store_id
    let user = sqlx::query_as::<_, (i32, String, String, Option<String>, Option<String>)>(
        "SELECT id, phone, password_hash, display_name, role FROM users WHERE phone = $1 AND store_id = $2"
    )
    .bind(&req.username)
    .bind(store_id)
    .fetch_optional(&state.pool)
    .await?;

    let (user_id, phone, password_hash, display_name, role) = match user {
        Some(u) => u,
        None => {
            return Ok(Json(json!({
                "success": false, "user": null,
                "message": "Tên đăng nhập hoặc mật khẩu không đúng"
            })));
        }
    };

    // Verify password
    let valid = bcrypt::verify(&req.password, &password_hash).unwrap_or(false);
    if !valid {
        return Ok(Json(json!({
            "success": false, "user": null,
            "message": "Tên đăng nhập hoặc mật khẩu không đúng"
        })));
    }

    // Update last_login_at
    sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
        .bind(user_id).execute(&state.pool).await?;

    // Trial status
    let is_trial = license_type.as_deref() == Some("trial");
    let trial_ends_at = if is_trial {
        license_expires_at.map(|t| t.and_utc().to_rfc3339())
    } else { None };

    // Generate JWT (now includes store_id)
    let role_str = role.clone().unwrap_or_else(|| "store_owner".to_string());
    let token = auth::create_token(
        user_id, store_id, &role_str, "free", &state.config.jwt_secret,
    )?;

    // Generate refresh token
    let refresh_token = auth::create_refresh_token(
        user_id, store_id, &role_str, "free", &state.config.jwt_secret,
    )?;

    Ok(Json(json!({
        "success": true,
        "token": token,
        "refresh_token": refresh_token,
        "user": {
            "id": user_id,
            "store_id": store_id,
            "username": phone,
            "display_name": display_name.unwrap_or_else(|| owner_name.unwrap_or_default()),
            "role": role
        },
        "is_trial": is_trial,
        "trial_ends_at": trial_ends_at,
        "server_time": Utc::now().to_rfc3339(),
        "message": null
    })))
}
