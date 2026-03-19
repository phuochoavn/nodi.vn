use axum::{Router, Json, routing::{post, get, delete, patch}, extract::{State, Path}};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth;

const MAX_DEVICES: i64 = 10;

#[derive(Deserialize)]
pub struct VerifyLicenseRequest {
    pub license_key: String,
    pub hwid: String,
    pub device_type: Option<String>,   // "windows" | "android" | "ios" — default "windows"
    pub device_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CheckActivationRequest {
    pub hwid: String,
}

#[derive(Deserialize)]
pub struct RenameDeviceRequest {
    pub device_name: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/verify-license", post(verify_license))
        .route("/api/check-activation", post(check_activation))
        .route("/api/devices", get(list_devices))
        .route("/api/devices/{id}", delete(remove_device))
        .route("/api/devices/{id}", patch(rename_device))
}

// ============================================================
// Helper: extract JWT from Authorization header
// ============================================================
fn extract_jwt(headers: &HeaderMap, secret: &str) -> Result<auth::Claims, AppError> {
    let header = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = header.strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid Authorization format".into()))?;
    auth::verify_token(token, secret)
}

// ============================================================
// POST /api/verify-license — Multi-device support
// ============================================================
async fn verify_license(
    State(state): State<AppState>,
    Json(req): Json<VerifyLicenseRequest>,
) -> Result<Json<Value>, AppError> {
    let device_type = req.device_type.as_deref().unwrap_or("windows");

    // 1. Find store by license_key
    let row = sqlx::query_as::<_, (i32, Option<bool>, Option<chrono::NaiveDateTime>, Option<String>, Option<String>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, is_active, license_expires_at, hwid, license_type, revoked_at FROM stores WHERE license_key = $1"
    )
    .bind(&req.license_key)
    .fetch_optional(&state.pool)
    .await?;

    let (store_id, is_active, expires_at, _existing_hwid, license_type, revoked_at) = match row {
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

    // 5. Multi-device binding logic
    // Check if device already registered
    let existing_device = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM devices WHERE store_id = $1 AND device_id = $2 AND is_active = true"
    )
    .bind(store_id)
    .bind(&req.hwid)
    .fetch_optional(&state.pool)
    .await?;

    let device_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM devices WHERE store_id = $1 AND is_active = true"
    )
    .bind(store_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or((0,));

    if let Some(_dev) = existing_device {
        // Device already registered — update last_active_at
        sqlx::query("UPDATE devices SET last_active_at = NOW() WHERE store_id = $1 AND device_id = $2 AND is_active = true")
            .bind(store_id)
            .bind(&req.hwid)
            .execute(&state.pool)
            .await?;

        // Also update stores.hwid for backward compat
        sqlx::query("UPDATE stores SET hwid = $1 WHERE id = $2")
            .bind(&req.hwid)
            .bind(store_id)
            .execute(&state.pool)
            .await?;

        Ok(Json(json!({
            "success": true,
            "message": "License hợp lệ",
            "store_id": store_id,
            "license_type": license_type,
            "device_count": device_count.0,
            "max_devices": MAX_DEVICES
        })))
    } else {
        // Device not registered — check limit
        if device_count.0 >= MAX_DEVICES {
            // Fetch current devices for the error message
            let devices = sqlx::query_as::<_, (String, Option<String>, Option<chrono::NaiveDateTime>)>(
                "SELECT device_type, device_name, last_active_at FROM devices WHERE store_id = $1 AND is_active = true ORDER BY last_active_at DESC"
            )
            .bind(store_id)
            .fetch_all(&state.pool)
            .await
            .unwrap_or_default();

            let device_list: Vec<Value> = devices.iter().map(|d| json!({
                "device_type": d.0,
                "device_name": d.1,
                "last_active": d.2.map(|t| t.format("%Y-%m-%d").to_string())
            })).collect();

            return Ok(Json(json!({
                "success": false,
                "message": format!("Đã đạt giới hạn {} thiết bị. Vui lòng gỡ thiết bị cũ tại nodi.vn/dashboard/cai-dat", MAX_DEVICES),
                "device_count": device_count.0,
                "max_devices": MAX_DEVICES,
                "devices": device_list
            })));
        }

        // Register new device
        let default_name = req.device_name.clone().unwrap_or_else(|| {
            format!("Thiết bị {} #{}", device_type, device_count.0 + 1)
        });

        sqlx::query(
            "INSERT INTO devices (store_id, device_id, device_type, device_name, is_active, first_activated_at, last_active_at) \
             VALUES ($1, $2, $3, $4, true, NOW(), NOW()) \
             ON CONFLICT (store_id, device_id) DO UPDATE SET is_active = true, last_active_at = NOW(), device_type = $3"
        )
        .bind(store_id)
        .bind(&req.hwid)
        .bind(device_type)
        .bind(&default_name)
        .execute(&state.pool)
        .await?;

        // Also update stores.hwid + activated_at for backward compat
        sqlx::query("UPDATE stores SET hwid = $1, activated_at = COALESCE(activated_at, NOW()) WHERE id = $2")
            .bind(&req.hwid)
            .bind(store_id)
            .execute(&state.pool)
            .await?;

        Ok(Json(json!({
            "success": true,
            "message": "Kích hoạt thành công",
            "store_id": store_id,
            "license_type": license_type,
            "device_count": device_count.0 + 1,
            "max_devices": MAX_DEVICES
        })))
    }
}

// ============================================================
// POST /api/check-activation — Multi-device aware
// ============================================================
async fn check_activation(
    State(state): State<AppState>,
    Json(req): Json<CheckActivationRequest>,
) -> Result<Json<Value>, AppError> {
    // First: check devices table (new system)
    let device_row = sqlx::query_as::<_, (i32, String, Option<chrono::NaiveDateTime>)>(
        "SELECT s.id, s.license_key, d.first_activated_at \
         FROM devices d JOIN stores s ON d.store_id = s.id \
         WHERE d.device_id = $1 AND d.is_active = true AND s.is_active = true \
         LIMIT 1"
    )
    .bind(&req.hwid)
    .fetch_optional(&state.pool)
    .await?;

    if let Some((store_id, license_key, activated_at)) = device_row {
        let user_id: Option<i32> = sqlx::query_scalar(
            "SELECT id FROM users WHERE store_id = $1 LIMIT 1"
        )
        .bind(store_id)
        .fetch_optional(&state.pool)
        .await?;

        // Update last_active_at
        sqlx::query("UPDATE devices SET last_active_at = NOW() WHERE device_id = $1 AND is_active = true")
            .bind(&req.hwid)
            .execute(&state.pool)
            .await?;

        return Ok(Json(json!({
            "found": true,
            "license_key": license_key,
            "user_id": user_id,
            "activated_at": activated_at.map(|t| t.and_utc().to_rfc3339())
        })));
    }

    // Fallback: check stores.hwid (backward compat for old data not yet migrated)
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

// ============================================================
// GET /api/devices — List devices for current user's store (JWT)
// ============================================================
async fn list_devices(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let claims = extract_jwt(&headers, &state.config.jwt_secret)?;
    let store_id = claims.store_id;

    let rows = sqlx::query_as::<_, (i32, String, String, Option<String>, bool, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, device_id, device_type, device_name, is_active, first_activated_at, last_active_at \
         FROM devices WHERE store_id = $1 AND is_active = true ORDER BY last_active_at DESC"
    )
    .bind(store_id)
    .fetch_all(&state.pool)
    .await?;

    let devices: Vec<Value> = rows.iter().map(|r| {
        let did = &r.1;
        let masked = if did.len() > 6 {
            format!("{}...{}", &did[..3], &did[did.len()-3..])
        } else {
            did.clone()
        };
        json!({
            "id": r.0,
            "device_type": r.2,
            "device_name": r.3,
            "device_id_masked": masked,
            "is_active": r.4,
            "first_activated_at": r.5.map(|t| t.and_utc().to_rfc3339()),
            "last_active_at": r.6.map(|t| t.and_utc().to_rfc3339())
        })
    }).collect();

    Ok(Json(json!({
        "devices": devices,
        "count": devices.len(),
        "max_devices": MAX_DEVICES
    })))
}

// ============================================================
// DELETE /api/devices/:id — Soft-delete device (JWT)
// ============================================================
async fn remove_device(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_jwt(&headers, &state.config.jwt_secret)?;
    let store_id = claims.store_id;

    // Verify device belongs to user's store
    let device = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM devices WHERE id = $1 AND store_id = $2 AND is_active = true"
    )
    .bind(id)
    .bind(store_id)
    .fetch_optional(&state.pool)
    .await?;

    if device.is_none() {
        return Err(AppError::NotFound("Thiết bị không tồn tại hoặc không thuộc cửa hàng này".into()));
    }

    // Soft delete
    sqlx::query("UPDATE devices SET is_active = false WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(Json(json!({
        "success": true,
        "message": "Đã gỡ thiết bị thành công"
    })))
}

// ============================================================
// PATCH /api/devices/:id — Rename device (JWT)
// ============================================================
async fn rename_device(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<RenameDeviceRequest>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_jwt(&headers, &state.config.jwt_secret)?;
    let store_id = claims.store_id;

    // Verify device belongs to user's store
    let device = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM devices WHERE id = $1 AND store_id = $2 AND is_active = true"
    )
    .bind(id)
    .bind(store_id)
    .fetch_optional(&state.pool)
    .await?;

    if device.is_none() {
        return Err(AppError::NotFound("Thiết bị không tồn tại".into()));
    }

    sqlx::query("UPDATE devices SET device_name = $1 WHERE id = $2")
        .bind(&body.device_name)
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(Json(json!({ "success": true })))
}
