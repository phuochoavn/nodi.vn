use axum::{Router, Json, routing::post, extract::State};
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

use crate::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub license_key: String,
    #[serde(default)]
    pub hwid: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/login-with-license", post(login_with_license))
}

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
    let token = crate::middleware::auth::create_token(
        user_id, store_id, &role_str, &state.config.jwt_secret,
    )?;

    Ok(Json(json!({
        "success": true,
        "token": token,
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
