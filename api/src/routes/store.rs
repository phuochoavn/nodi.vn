use axum::{Router, Json, routing::{get, post}, extract::State};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};

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

    let stores = sqlx::query_as::<_, (String, Option<String>, String, bool)>(
        "SELECT store_id, store_name, role, is_default FROM account_stores WHERE account_id = $1 ORDER BY is_default DESC, created_at"
    ).bind(user_id).fetch_all(&state.pool).await?;

    let stores_json: Vec<Value> = stores.iter().map(|s| json!({
        "store_id": s.0,
        "store_name": s.1,
        "role": s.2,
        "is_default": s.3
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

    // Verify user owns this store
    let store = sqlx::query_as::<_, (String, Option<String>, String)>(
        "SELECT store_id, store_name, role FROM account_stores WHERE account_id = $1 AND store_id = $2"
    ).bind(user_id).bind(&req.store_id).fetch_optional(&state.pool).await?;

    let (store_id, store_name, role) = match store {
        Some(s) => s,
        None => {
            return Ok(Json(json!({ "success": false, "message": "Không tìm thấy cửa hàng" })));
        }
    };

    // Generate new JWT with this store's offset
    let jwt_store_id = user_id + 1_000_000;
    let token = crate::middleware::auth::create_token(user_id, jwt_store_id, &role, &state.config.jwt_secret)?;
    let refresh_token = crate::middleware::auth::create_refresh_token(user_id, jwt_store_id, &role, &state.config.jwt_secret)?;

    tracing::info!("🔄 Store switched: user_id={}, store_id={}", user_id, store_id);

    Ok(Json(json!({
        "success": true,
        "token": token,
        "refresh_token": refresh_token,
        "store_id": store_id,
        "store_name": store_name
    })))
}
