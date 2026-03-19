use axum::{Router, Json, routing::{post, get}, extract::{State, Query}};
use serde::Deserialize;
use serde_json::{json, Value};
use rand::Rng;

use crate::AppState;
use crate::error::AppError;

const MAX_DEVICES: i64 = 10;
const MAX_ACTIVE_INVITES: i64 = 5;
const INVITE_EXPIRY_MINUTES: i64 = 15;

// Exclude: O, 0, I, 1, L (dễ nhầm)
const CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";

fn generate_invite_code() -> String {
    let mut rng = rand::thread_rng();
    let code: String = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("NODI-{}", code)
}

#[derive(Deserialize)]
pub struct CreateInviteRequest {
    pub license_key: String,
    pub staff_id: i32,
    pub staff_name: String,
}

#[derive(Deserialize)]
pub struct RedeemInviteRequest {
    pub invite_code: String,
    pub hwid: String,
    pub device_type: Option<String>,
}

#[derive(Deserialize)]
pub struct ListInvitesQuery {
    pub license_key: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/staff-invite/create", post(create_invite))
        .route("/api/staff-invite/redeem", post(redeem_invite))
        .route("/api/staff-invite/list", get(list_invites))
}

// ============================================================
// POST /api/staff-invite/create
// ============================================================
async fn create_invite(
    State(state): State<AppState>,
    Json(req): Json<CreateInviteRequest>,
) -> Result<Json<Value>, AppError> {
    // Validate required fields
    if req.license_key.is_empty() || req.staff_name.is_empty() {
        return Ok(Json(json!({
            "success": false,
            "message": "Thiếu thông tin bắt buộc"
        })));
    }

    // 1. Find store by license_key
    let store = sqlx::query_as::<_, (i32, Option<bool>, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>)>(
        "SELECT id, is_active, license_expires_at, revoked_at FROM stores WHERE license_key = $1"
    )
    .bind(&req.license_key)
    .fetch_optional(&state.pool)
    .await?;

    let (store_id, is_active, expires_at, revoked_at) = match store {
        Some(r) => r,
        None => return Ok(Json(json!({
            "success": false,
            "message": "License key không hợp lệ"
        }))),
    };

    // 2. Validate license
    if revoked_at.is_some() || !is_active.unwrap_or(false) {
        return Ok(Json(json!({
            "success": false,
            "message": "License không hợp lệ hoặc đã bị thu hồi"
        })));
    }

    if let Some(exp) = expires_at {
        if exp < chrono::Utc::now().naive_utc() {
            return Ok(Json(json!({
                "success": false,
                "message": "License đã hết hạn"
            })));
        }
    }

    // 3. Check rate limit: max 5 active invites per store
    let active_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM staff_invites WHERE store_id = $1 AND redeemed_at IS NULL AND expires_at > NOW()"
    )
    .bind(store_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or((0,));

    if active_count.0 >= MAX_ACTIVE_INVITES {
        return Ok(Json(json!({
            "success": false,
            "message": format!("Tối đa {} mã mời đang hoạt động cùng lúc", MAX_ACTIVE_INVITES)
        })));
    }

    // 4. Generate unique invite code (retry up to 5 times for uniqueness)
    let mut invite_code = String::new();
    for _ in 0..5 {
        let candidate = generate_invite_code();
        let exists: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM staff_invites WHERE invite_code = $1)"
        )
        .bind(&candidate)
        .fetch_one(&state.pool)
        .await
        .unwrap_or((false,));

        if !exists.0 {
            invite_code = candidate;
            break;
        }
    }

    if invite_code.is_empty() {
        return Ok(Json(json!({
            "success": false,
            "message": "Không thể tạo mã mời, vui lòng thử lại"
        })));
    }

    // 5. Insert invite
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(INVITE_EXPIRY_MINUTES);

    sqlx::query(
        "INSERT INTO staff_invites (store_id, staff_id, staff_name, invite_code, expires_at) \
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(store_id)
    .bind(req.staff_id)
    .bind(&req.staff_name)
    .bind(&invite_code)
    .bind(expires_at)
    .execute(&state.pool)
    .await?;

    // 6. Build QR data
    let qr_data = json!({
        "type": "nodi_staff_invite",
        "code": &invite_code
    }).to_string();

    tracing::info!("✅ Staff invite created: store_id={}, code={}, staff={}", store_id, invite_code, req.staff_name);

    Ok(Json(json!({
        "success": true,
        "invite_code": invite_code,
        "qr_data": qr_data,
        "expires_at": expires_at.to_rfc3339(),
        "staff_id": req.staff_id,
        "staff_name": req.staff_name
    })))
}

// ============================================================
// POST /api/staff-invite/redeem
// ============================================================
async fn redeem_invite(
    State(state): State<AppState>,
    Json(req): Json<RedeemInviteRequest>,
) -> Result<Json<Value>, AppError> {
    let device_type = req.device_type.as_deref().unwrap_or("android");

    // 1. Find invite
    let invite = sqlx::query_as::<_, (i32, i32, i32, String, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT id, store_id, staff_id, staff_name, expires_at, redeemed_at \
         FROM staff_invites WHERE invite_code = $1"
    )
    .bind(&req.invite_code)
    .fetch_optional(&state.pool)
    .await?;

    let (invite_id, store_id, staff_id, staff_name, expires_at, redeemed_at) = match invite {
        Some(r) => r,
        None => return Ok(Json(json!({
            "success": false,
            "message": "Mã mời không hợp lệ"
        }))),
    };

    // 2. Check already redeemed
    if redeemed_at.is_some() {
        return Ok(Json(json!({
            "success": false,
            "message": "Mã mời đã được sử dụng"
        })));
    }

    // 3. Check expired
    if expires_at < chrono::Utc::now() {
        return Ok(Json(json!({
            "success": false,
            "message": "Mã mời đã hết hạn"
        })));
    }

    // 4. Check device limit
    let device_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM devices WHERE store_id = $1 AND is_active = true"
    )
    .bind(store_id)
    .fetch_one(&state.pool)
    .await
    .unwrap_or((0,));

    if device_count.0 >= MAX_DEVICES {
        return Ok(Json(json!({
            "success": false,
            "message": "Đã đạt giới hạn 10 thiết bị. Vui lòng xóa thiết bị cũ."
        })));
    }

    // 5. Register device (reuse same pattern as verify_license)
    let default_name = format!("{} - {}", staff_name, device_type);

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

    // 6. Mark invite as redeemed
    sqlx::query(
        "UPDATE staff_invites SET redeemed_at = NOW(), redeemed_by_hwid = $1, redeemed_device_type = $2 WHERE id = $3"
    )
    .bind(&req.hwid)
    .bind(device_type)
    .bind(invite_id)
    .execute(&state.pool)
    .await?;

    // 7. Get store info for response
    let store_info = sqlx::query_as::<_, (String, String)>(
        "SELECT COALESCE(owner_name, ''), license_key FROM stores WHERE id = $1"
    )
    .bind(store_id)
    .fetch_one(&state.pool)
    .await?;

    tracing::info!("✅ Staff invite redeemed: code={}, hwid={}, staff={}", req.invite_code, req.hwid, staff_name);

    Ok(Json(json!({
        "success": true,
        "store_name": store_info.0,
        "staff_name": staff_name,
        "staff_id": staff_id,
        "license_key": store_info.1,
        "device_registered": true,
        "device_count": device_count.0 + 1,
        "max_devices": MAX_DEVICES
    })))
}

// ============================================================
// GET /api/staff-invite/list?license_key=xxx
// ============================================================
async fn list_invites(
    State(state): State<AppState>,
    Query(q): Query<ListInvitesQuery>,
) -> Result<Json<Value>, AppError> {
    // 1. Validate license_key → find store
    let store = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM stores WHERE license_key = $1"
    )
    .bind(&q.license_key)
    .fetch_optional(&state.pool)
    .await?;

    let (store_id,) = match store {
        Some(r) => r,
        None => return Ok(Json(json!({
            "success": false,
            "message": "License key không hợp lệ"
        }))),
    };

    // 2. Fetch invites (newest first)
    let rows = sqlx::query_as::<_, (
        i32,                                    // id
        i32,                                    // staff_id
        String,                                 // staff_name
        String,                                 // invite_code
        chrono::DateTime<chrono::Utc>,          // expires_at
        chrono::DateTime<chrono::Utc>,          // created_at
        Option<chrono::DateTime<chrono::Utc>>,  // redeemed_at
        Option<String>,                         // redeemed_device_type
    )>(
        "SELECT id, staff_id, staff_name, invite_code, expires_at, created_at, redeemed_at, redeemed_device_type \
         FROM staff_invites WHERE store_id = $1 ORDER BY created_at DESC LIMIT 50"
    )
    .bind(store_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let now = chrono::Utc::now();
    let invites: Vec<Value> = rows.iter().map(|r| {
        let status = if r.6.is_some() {
            "redeemed"
        } else if r.4 < now {
            "expired"
        } else {
            "active"
        };

        json!({
            "id": r.0,
            "staff_id": r.1,
            "staff_name": r.2,
            "invite_code": r.3,
            "status": status,
            "expires_at": r.4.to_rfc3339(),
            "created_at": r.5.to_rfc3339(),
            "redeemed_at": r.6.map(|t| t.to_rfc3339()),
            "redeemed_device_type": r.7
        })
    }).collect();

    Ok(Json(json!({
        "success": true,
        "invites": invites,
        "count": invites.len()
    })))
}
