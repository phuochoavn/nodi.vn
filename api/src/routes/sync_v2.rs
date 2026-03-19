use axum::{Router, Json, routing::{post, get}, extract::{State, Query}};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;
use crate::error::AppError;
use crate::models::sync_v2::SyncV2PushRequest;
use crate::services::merge_engine;

// ============================================================
// Router — prefix /api/v2/sync/
// ============================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v2/sync/push", post(handle_push))
        .route("/api/v2/sync/pull", get(handle_pull))
}

// ============================================================
// Helpers
// ============================================================

fn get_header(headers: &HeaderMap, key: &str) -> Option<String> {
    headers.get(key)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// ============================================================
// POST /api/v2/sync/push
// ============================================================

async fn handle_push(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SyncV2PushRequest>,
) -> Result<Json<Value>, AppError> {
    // 1. Authenticate — reuse v1 identify_store
    let store_id = crate::routes::sync::identify_store(&headers, &state.pool, &state.config.jwt_secret).await?;

    // 2. Get device_id from header or body
    let device_id = get_header(&headers, "x-device-id")
        .unwrap_or_else(|| payload.device_id.clone());

    if device_id.is_empty() {
        return Err(AppError::BadRequest("Missing device_id (X-Device-Id header or body)".into()));
    }

    // 3. Get batch_id from header or body
    let batch_id = get_header(&headers, "x-batch-id")
        .unwrap_or_else(|| payload.batch_id.clone());

    if batch_id.is_empty() {
        return Err(AppError::BadRequest("Missing batch_id (X-Batch-Id header or body)".into()));
    }

    // 4. Count total changes
    let total_changes: usize = payload.changes.values().map(|v| v.len()).sum();

    // 5. Process via merge engine
    let (new_cursor, processed, conflicts, computed_updates) =
        merge_engine::process_push(
            &state.pool,
            store_id,
            &device_id,
            &batch_id,
            &payload.changes,
        ).await?;

    let conflict_count = conflicts.len();

    tracing::info!(
        "✅ V2 Push: store_id={}, device={}, batch={}, total={}, processed={}, conflicts={}",
        store_id, device_id, batch_id, total_changes, processed, conflict_count
    );

    Ok(Json(json!({
        "success": true,
        "message": format!("Đã nhận {} thay đổi, xử lý {}, xung đột {}", total_changes, processed, conflict_count),
        "data": {
            "new_cursor": new_cursor,
            "processed": processed,
            "conflicts": conflicts,
            "computed_updates": computed_updates
        }
    })))
}

// ============================================================
// GET /api/v2/sync/pull
// ============================================================

#[derive(Deserialize)]
struct PullQuery {
    #[serde(default)]
    cursor: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 { 500 }

async fn handle_pull(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<PullQuery>,
) -> Result<Json<Value>, AppError> {
    // 1. Authenticate
    let store_id = crate::routes::sync::identify_store(&headers, &state.pool, &state.config.jwt_secret).await?;

    // 2. Get device_id
    let device_id = get_header(&headers, "x-device-id")
        .ok_or_else(|| AppError::BadRequest("Missing X-Device-Id header".into()))?;

    if device_id.is_empty() {
        return Err(AppError::BadRequest("X-Device-Id cannot be empty".into()));
    }

    // 3. Clamp limit
    let limit = params.limit.min(1000).max(1);

    // 4. Build changes from journal
    let (new_cursor, has_more, changes, computed_updates) =
        merge_engine::build_pull_changes(
            &state.pool,
            store_id,
            &device_id,
            params.cursor,
            limit,
        ).await?;

    let total_changes: usize = changes.values().map(|v| v.len()).sum();

    tracing::info!(
        "📥 V2 Pull: store_id={}, device={}, cursor={}→{}, changes={}, has_more={}",
        store_id, device_id, params.cursor, new_cursor, total_changes, has_more
    );

    Ok(Json(json!({
        "success": true,
        "data": {
            "cursor": new_cursor,
            "has_more": has_more,
            "changes": changes,
            "computed_updates": computed_updates
        }
    })))
}
