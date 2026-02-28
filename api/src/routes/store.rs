use axum::{Router, Json, routing::{get, post}, extract::State};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use rand::Rng;

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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/stores", get(list_stores))
        .route("/api/stores/switch", post(switch_store))
        .route("/api/stores/create", post(create_store))
}

/// Generate STORE-XXXXXXXX (8 uppercase alphanumeric chars)
fn generate_store_id() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let code: String = (0..8).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("STORE-{}", code)
}

// ============================================================
// GET /api/stores — list all shops for authenticated user
// ============================================================

async fn list_stores(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let user_id = claims.sub;

    let stores = sqlx::query_as::<_, (String, Option<String>, String, bool, i32)>(
        "SELECT store_id, store_name, role, is_default, data_store_id FROM account_stores WHERE account_id = $1 ORDER BY is_default DESC, created_at"
    ).bind(user_id).fetch_all(&state.pool).await?;

    let stores_json: Vec<Value> = stores.iter().map(|s| json!({
        "store_id": s.0,
        "store_name": s.1,
        "role": s.2,
        "is_default": s.3,
        "data_store_id": s.4
    })).collect();

    Ok(Json(json!({ "stores": stores_json })))
}

// ============================================================
// POST /api/stores/switch — switch active store, return new JWT
// ============================================================

#[derive(Deserialize)]
struct SwitchRequest {
    store_id: String,
}

async fn switch_store(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SwitchRequest>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let user_id = claims.sub;

    // Verify user owns this store, fetch data_store_id
    let store = sqlx::query_as::<_, (String, Option<String>, String, i32)>(
        "SELECT store_id, store_name, role, data_store_id FROM account_stores WHERE account_id = $1 AND store_id = $2"
    ).bind(user_id).bind(&req.store_id).fetch_optional(&state.pool).await?;

    let (store_id, store_name, role, data_store_id) = match store {
        Some(s) => s,
        None => {
            return Ok(Json(json!({ "success": false, "message": "Không tìm thấy cửa hàng" })));
        }
    };

    // Generate new JWT with this store's data_store_id
    let token = crate::middleware::auth::create_token(user_id, data_store_id, &role, &state.config.jwt_secret)?;
    let refresh_token = crate::middleware::auth::create_refresh_token(user_id, data_store_id, &role, &state.config.jwt_secret)?;

    tracing::info!("🔄 Store switched: user_id={}, store_id={}, data_store_id={}", user_id, store_id, data_store_id);

    Ok(Json(json!({
        "success": true,
        "token": token,
        "refresh_token": refresh_token,
        "store_id": store_id,
        "store_name": store_name,
        "data_store_id": data_store_id
    })))
}

// ============================================================
// POST /api/stores/create — create a new store for account
// ============================================================

#[derive(Deserialize)]
struct CreateStoreRequest {
    store_name: String,
}

async fn create_store(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CreateStoreRequest>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let user_id = claims.sub;
    let store_name = req.store_name.trim().to_string();

    if store_name.is_empty() {
        return Ok(Json(json!({ "success": false, "message": "Tên cửa hàng không được để trống" })));
    }

    // Limit: max 10 stores per account
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM account_stores WHERE account_id = $1"
    ).bind(user_id).fetch_one(&state.pool).await?;
    if count.0 >= 10 {
        return Ok(Json(json!({ "success": false, "message": "Tối đa 10 cửa hàng mỗi tài khoản" })));
    }

    // Generate unique store_id
    let mut store_id = generate_store_id();
    for _ in 0..10 {
        let exists: Option<(i32,)> = sqlx::query_as(
            "SELECT id FROM account_stores WHERE store_id = $1"
        ).bind(&store_id).fetch_optional(&state.pool).await?;
        if exists.is_none() { break; }
        store_id = generate_store_id();
    }

    // Generate unique data_store_id (next available integer >= 1_000_001)
    let max_id: (Option<i64>,) = sqlx::query_as(
        "SELECT MAX(data_store_id::bigint) FROM account_stores"
    ).fetch_one(&state.pool).await.unwrap_or((None,));
    let data_store_id = (max_id.0.unwrap_or(1_000_000) + 1) as i32;

    // Insert
    sqlx::query(
        "INSERT INTO account_stores (account_id, store_id, store_name, role, is_default, data_store_id) \
         VALUES ($1, $2, $3, 'owner', FALSE, $4)"
    ).bind(user_id).bind(&store_id).bind(&store_name).bind(data_store_id)
    .execute(&state.pool).await?;

    tracing::info!("🏪 New store created: user_id={}, store_id={}, data_store_id={}, name={}",
        user_id, store_id, data_store_id, store_name);

    Ok(Json(json!({
        "success": true,
        "store_id": store_id,
        "store_name": store_name,
        "data_store_id": data_store_id,
        "message": "Tạo cửa hàng thành công"
    })))
}
