use axum::{Router, Json, routing::post, extract::State};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use rand::Rng;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth;

// ============================================================
// Router — /api/onboarding/*
// Public endpoints (no auth required)
// ============================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/stores/register", post(register_store))
        .route("/api/devices/register", post(register_device))
}

// ============================================================
// Helpers
// ============================================================

/// Generate 6-char alphanumeric activation key (uppercase, no ambiguous chars)
fn generate_activation_key() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    (0..6).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

/// Generate license key: NODI-XXXX-XXXX-XXXX
fn generate_license_key() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let segment = |rng: &mut rand::rngs::ThreadRng| -> String {
        (0..4).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
    };
    format!("NODI-{}-{}-{}", segment(&mut rng), segment(&mut rng), segment(&mut rng))
}

// ============================================================
// POST /api/stores/register — Public store registration
// ============================================================

#[derive(Deserialize)]
struct RegisterStoreRequest {
    name: String,
    #[serde(default)]
    owner_name: Option<String>,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    address: Option<String>,
    #[serde(default)]
    province: Option<String>,
    #[serde(default)]
    district: Option<String>,
}

async fn register_store(
    State(state): State<AppState>,
    Json(req): Json<RegisterStoreRequest>,
) -> Result<Json<Value>, AppError> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::BadRequest("Tên cửa hàng không được để trống".into()));
    }

    // Generate unique license_key
    let mut license_key = generate_license_key();
    for _ in 0..10 {
        let exists: Option<(i32,)> = sqlx::query_as(
            "SELECT id FROM stores WHERE license_key = $1"
        ).bind(&license_key).fetch_optional(&state.pool).await?;
        if exists.is_none() { break; }
        license_key = generate_license_key();
    }

    // Generate unique activation_key
    let mut activation_key = generate_activation_key();
    for _ in 0..10 {
        let exists: Option<(i32,)> = sqlx::query_as(
            "SELECT id FROM stores WHERE activation_key = $1"
        ).bind(&activation_key).fetch_optional(&state.pool).await?;
        if exists.is_none() { break; }
        activation_key = generate_activation_key();
    }

    // Insert store
    let store_id: (i32,) = sqlx::query_as(
        "INSERT INTO stores (name, license_key, owner_name, phone, address, province, district, \
         license_type, activation_key, is_active) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'basic', $8, true) \
         RETURNING id"
    )
    .bind(&name)
    .bind(&license_key)
    .bind(&req.owner_name)
    .bind(&req.phone)
    .bind(&req.address)
    .bind(&req.province)
    .bind(&req.district)
    .bind(&activation_key)
    .fetch_one(&state.pool)
    .await?;

    // Seed default agriculture categories for the new store
    seed_store_defaults(&state.pool, store_id.0).await?;

    tracing::info!(
        "🏪 Store registered: id={}, name={}, key={}, activation={}",
        store_id.0, name, license_key, activation_key
    );

    Ok(Json(json!({
        "success": true,
        "store_id": store_id.0,
        "activation_key": activation_key,
        "license_key": license_key,
        "license_type": "basic",
        "message": "Đăng ký cửa hàng thành công. Dùng activation_key để kích hoạt thiết bị."
    })))
}

/// Seed default data for a new store (agriculture categories, store settings)
async fn seed_store_defaults(pool: &sqlx::PgPool, store_id: i32) -> Result<(), AppError> {
    // Default store settings
    sqlx::query(
        "INSERT INTO synced_store_settings (store_id, name, uuid) \
         VALUES ($1, 'Cửa hàng mới', $2) \
         ON CONFLICT (store_id) DO NOTHING"
    )
    .bind(store_id)
    .bind(uuid::Uuid::new_v4().to_string())
    .execute(pool).await.ok();

    // Default store fund
    sqlx::query(
        "INSERT INTO synced_store_funds (store_id, current_balance, uuid) \
         VALUES ($1, 0, $2) \
         ON CONFLICT DO NOTHING"
    )
    .bind(store_id)
    .bind(uuid::Uuid::new_v4().to_string())
    .execute(pool).await.ok();

    // Default product categories (agriculture)
    let categories = [
        ("Phân bón", "PHAN-BON"),
        ("Thuốc BVTV", "THUOC-BVTV"),
        ("Giống cây trồng", "GIONG-CAY"),
        ("Thức ăn chăn nuôi", "TACN"),
        ("Thuốc thú y", "THUOC-THU-Y"),
        ("Dụng cụ nông nghiệp", "DUNG-CU"),
        ("Vật tư tưới tiêu", "VAT-TU-TUOI"),
        ("Khác", "KHAC"),
    ];

    for (name, sku) in &categories {
        sqlx::query(
            "INSERT INTO synced_products (store_id, local_id, name, category, sku, uuid, stock_quantity) \
             VALUES ($1, 0, $2, 'Danh mục mặc định', $3, $4, 0) \
             ON CONFLICT DO NOTHING"
        )
        .bind(store_id)
        .bind(name)
        .bind(sku)
        .bind(uuid::Uuid::new_v4().to_string())
        .execute(pool).await.ok();
    }

    tracing::info!("🌱 Seeded defaults for store_id={}", store_id);
    Ok(())
}

// ============================================================
// POST /api/devices/register — Activate device with key
// ============================================================

#[derive(Deserialize)]
struct RegisterDeviceRequest {
    activation_key: String,
    #[serde(default)]
    device_name: Option<String>,
}

async fn register_device(
    State(state): State<AppState>,
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<Json<Value>, AppError> {
    let key = req.activation_key.trim().to_uppercase();
    if key.is_empty() {
        return Err(AppError::BadRequest("activation_key không được để trống".into()));
    }

    // Lookup store by activation_key
    let store: Option<(i32, String)> = sqlx::query_as(
        "SELECT id, name FROM stores WHERE activation_key = $1 AND is_active = true"
    )
    .bind(&key)
    .fetch_optional(&state.pool)
    .await?;

    let (store_id, store_name) = match store {
        Some(s) => s,
        None => return Err(AppError::NotFound("Mã kích hoạt không hợp lệ hoặc cửa hàng không hoạt động".into())),
    };

    // Generate device_id
    let device_id = uuid::Uuid::new_v4().to_string();
    let device_name = req.device_name.unwrap_or_else(|| format!("Device-{}", &device_id[..8]));

    // Insert sync_devices record
    sqlx::query(
        "INSERT INTO sync_devices (store_id, device_id, device_name, pull_cursor) \
         VALUES ($1, $2, $3, 0) \
         ON CONFLICT (store_id, device_id) DO UPDATE SET device_name = $3, last_pull_at = NOW()"
    )
    .bind(store_id)
    .bind(&device_id)
    .bind(&device_name)
    .execute(&state.pool)
    .await?;

    // Update store activated_at if first activation
    sqlx::query(
        "UPDATE stores SET activated_at = COALESCE(activated_at, NOW()) WHERE id = $1"
    )
    .bind(store_id)
    .execute(&state.pool)
    .await?;

    // Issue JWT (sub = store_id as user proxy, store_id = store_id)
    let access_token = auth::create_token(
        store_id, store_id, "device", "free", &state.config.jwt_secret
    )?;
    let refresh_token = auth::create_refresh_token(
        store_id, store_id, "device", "free", &state.config.jwt_secret
    )?;

    tracing::info!(
        "📱 Device registered: store_id={}, device_id={}, name={}",
        store_id, device_id, device_name
    );

    Ok(Json(json!({
        "success": true,
        "store_id": store_id,
        "store_name": store_name,
        "device_id": device_id,
        "device_name": device_name,
        "jwt_token": access_token,
        "refresh_token": refresh_token,
        "message": "Kích hoạt thiết bị thành công. Gọi /api/v2/sync/snapshot để tải dữ liệu ban đầu."
    })))
}
