use axum::{Router, Json, routing::post, extract::State};
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

use crate::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct VerifyLicenseRequest {
    pub license_key: String,
    pub hwid: String,
}

#[derive(Deserialize)]
pub struct CheckActivationRequest {
    pub hwid: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/verify-license", post(verify_license))
        .route("/api/check-activation", post(check_activation))
}

async fn verify_license(
    State(state): State<AppState>,
    Json(req): Json<VerifyLicenseRequest>,
) -> Result<Json<Value>, AppError> {
    // 1. Find store by license_key
    let row = sqlx::query_as::<_, (i32, Option<bool>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, is_active, license_expires_at, hwid, license_type, revoked_at FROM stores WHERE license_key = $1"
    )
    .bind(&req.license_key)
    .fetch_optional(&state.pool)
    .await?;

    let (store_id, is_active, expires_at, existing_hwid, license_type, revoked_at) = match row {
        Some(r) => r,
        None => return Ok(Json(json!({
            "success": false,
            "message": "License key không tồn tại"
        }))),
    };

    // 2. Check revoked
    if revoked_at.is_some() {
        return Ok(Json(json!({
            "success": false,
            "message": "License đã bị thu hồi"
        })));
    }

    // 3. Check is_active
    if !is_active.unwrap_or(false) {
        return Ok(Json(json!({
            "success": false,
            "message": "License đã bị tạm ngưng"
        })));
    }

    // 4. Check expiration
    if let Some(exp) = expires_at {
        if exp < Utc::now().naive_utc() {
            return Ok(Json(json!({
                "success": false,
                "message": "License đã hết hạn"
            })));
        }
    }

    // 5. HWID binding logic
    match existing_hwid {
        None => {
            // First activation — save HWID
            sqlx::query("UPDATE stores SET hwid = $1, activated_at = NOW() WHERE id = $2")
                .bind(&req.hwid)
                .bind(store_id)
                .execute(&state.pool)
                .await?;
            Ok(Json(json!({
                "success": true,
                "message": "Kích hoạt thành công",
                "store_id": store_id,
                "license_type": license_type
            })))
        }
        Some(ref h) if h != &req.hwid => {
            Ok(Json(json!({
                "success": false,
                "message": "Thiết bị không khớp — HWID đã được đăng ký trên máy khác"
            })))
        }
        _ => {
            // HWID matches, all good
            Ok(Json(json!({
                "success": true,
                "message": "License hợp lệ",
                "store_id": store_id,
                "license_type": license_type
            })))
        }
    }
}

async fn check_activation(
    State(state): State<AppState>,
    Json(req): Json<CheckActivationRequest>,
) -> Result<Json<Value>, AppError> {
    let row = sqlx::query_as::<_, (i32, String, Option<chrono::NaiveDateTime>)>(
        "SELECT id, license_key, activated_at FROM stores WHERE hwid = $1 AND is_active = true"
    )
    .bind(&req.hwid)
    .fetch_optional(&state.pool)
    .await?;

    match row {
        Some((store_id, license_key, activated_at)) => {
            let user_id: Option<i32> = sqlx::query_scalar(
                "SELECT id FROM users WHERE store_id = $1 LIMIT 1"
            )
            .bind(store_id)
            .fetch_optional(&state.pool)
            .await?;

            Ok(Json(json!({
                "found": true,
                "license_key": license_key,
                "user_id": user_id,
                "activated_at": activated_at.map(|t| t.and_utc().to_rfc3339())
            })))
        }
        None => {
            Ok(Json(json!({ "found": false })))
        }
    }
}
