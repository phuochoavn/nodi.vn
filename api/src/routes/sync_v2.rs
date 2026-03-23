use axum::{Router, Json, routing::{post, get}, extract::{State, Query, Extension}};
use axum::body::Bytes;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use prost::Message;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::tenant::TenantContext;
use crate::models::sync_v2::SyncV2PushRequest;
use crate::models::proto::*;
use crate::services::merge_engine;

// ============================================================
// Router — prefix /api/v2/sync/
// Applied with tenant_middleware in main.rs
// ============================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v2/sync/push", post(handle_push))
        .route("/api/v2/sync/pull", get(handle_pull))
        .route("/api/v2/sync/snapshot", get(handle_snapshot))
}

// ============================================================
// Helpers
// ============================================================

fn get_header(headers: &HeaderMap, key: &str) -> Option<String> {
    headers.get(key)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// Check if request wants protobuf
fn is_protobuf(headers: &HeaderMap) -> bool {
    headers.get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains("protobuf"))
        .unwrap_or(false)
}

/// Check if client accepts protobuf response
fn accepts_protobuf(headers: &HeaderMap) -> bool {
    headers.get("accept")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.contains("protobuf"))
        .unwrap_or(false)
        || is_protobuf(headers)  // mirror request content-type
}

/// Build a protobuf response
fn proto_response(msg: &impl Message) -> Response {
    let bytes = msg.encode_to_vec();
    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/x-protobuf"))],
        bytes,
    ).into_response()
}

// ============================================================
// POST /api/v2/sync/push — accepts JSON or Protobuf
// ============================================================

async fn handle_push(
    State(state): State<AppState>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    let store_id = ctx.store_id;
    let use_proto = is_protobuf(&headers);
    let respond_proto = accepts_protobuf(&headers);

    // Parse request — JSON or Protobuf
    let (device_id, batch_id, changes, max_journal_id) = if use_proto {
        let proto_req = ProtoPushRequest::decode(body)
            .map_err(|e| AppError::BadRequest(format!("Invalid protobuf: {}", e)))?;
        proto_req.into_changes()
    } else {
        let json_req: SyncV2PushRequest = serde_json::from_slice(&body)
            .map_err(|e| AppError::BadRequest(format!("Invalid JSON: {}", e)))?;
        (
            json_req.device_id.clone(),
            json_req.batch_id.clone(),
            json_req.changes,
            json_req.max_journal_id,
        )
    };

    // Get device_id from TenantContext, header, or body
    let device_id = ctx.device_id.unwrap_or_else(|| {
        get_header(&headers, "x-device-id").unwrap_or(device_id)
    });

    if device_id.is_empty() {
        return Err(AppError::BadRequest("Missing device_id".into()));
    }

    let batch_id = get_header(&headers, "x-batch-id").unwrap_or(batch_id);
    if batch_id.is_empty() {
        return Err(AppError::BadRequest("Missing batch_id".into()));
    }

    let total_changes: usize = changes.values().map(|v| v.len()).sum();

    // Process via merge engine
    let (new_cursor, processed, conflicts, computed_updates) =
        merge_engine::process_push(
            &state.pool,
            store_id,
            &device_id,
            &batch_id,
            &changes,
        ).await?;

    let conflict_count = conflicts.len();

    // Sprint 164: Broadcast sync_update to WS subscribers (DashMap, lock-free)
    if processed > 0 {
        let changed_tables: Vec<&str> = changes.keys().map(|s| s.as_str()).collect();
        crate::routes::ws_sync::broadcast_sync_event(
            &state.sync_rooms, store_id, &changed_tables
        );
    }

    tracing::info!(
        "✅ V2 Push: store_id={}, device={}, batch={}, total={}, processed={}, conflicts={}, proto={}",
        store_id, device_id, batch_id, total_changes, processed, conflict_count, use_proto
    );

    // Response — Protobuf or JSON
    if respond_proto {
        let proto_resp = ProtoPushResponse {
            success: true,
            message: format!("Đã nhận {} thay đổi, xử lý {}", total_changes, processed),
            new_cursor,
            processed: processed as i32,
            conflicts: conflicts.iter().map(ProtoConflictInfo::from).collect(),
            computed_updates: serde_json::to_vec(&computed_updates).unwrap_or_default(),
            last_processed_client_tx_id: max_journal_id,
        };
        Ok(proto_response(&proto_resp))
    } else {
        Ok(Json(json!({
            "success": true,
            "message": format!("Đã nhận {} thay đổi, xử lý {}, xung đột {}", total_changes, processed, conflict_count),
            "data": {
                "new_cursor": new_cursor,
                "processed": processed,
                "conflicts": conflicts,
                "computed_updates": computed_updates,
                "last_processed_client_tx_id": max_journal_id
            }
        })).into_response())
    }
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
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
    Query(params): Query<PullQuery>,
) -> Result<Response, AppError> {
    let store_id = ctx.store_id;
    let respond_proto = accepts_protobuf(&headers);

    let device_id = ctx.device_id
        .or_else(|| get_header(&headers, "x-device-id"))
        .ok_or_else(|| AppError::BadRequest("Missing X-Device-Id header".into()))?;

    if device_id.is_empty() {
        return Err(AppError::BadRequest("X-Device-Id cannot be empty".into()));
    }

    let limit = params.limit.min(1000).max(1);

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
        "📥 V2 Pull: store_id={}, device={}, cursor={}→{}, changes={}, has_more={}, proto={}",
        store_id, device_id, params.cursor, new_cursor, total_changes, has_more, respond_proto
    );

    // Sprint 173: Re-enable protobuf encoding for pull response.
    // Proto structs verified to match client definitions (push already works).
    if respond_proto {
        let proto_changes: Vec<ProtoPullTableChanges> = changes.into_iter().map(|(table, records)| {
            ProtoPullTableChanges {
                table_name: table,
                records: records.into_iter().map(|r| ProtoPullChangeRecord {
                    uuid: r.uuid,
                    operation: r.operation,
                    data: serde_json::to_vec(&r.data).unwrap_or_default(),
                }).collect(),
            }
        }).collect();
        let proto_resp = ProtoPullResponse {
            success: true,
            cursor: new_cursor,
            has_more,
            changes: proto_changes,
            computed_updates: serde_json::to_vec(&computed_updates).unwrap_or_default(),
        };
        tracing::info!("📥 Pull: responding with protobuf ({} bytes)", proto_resp.encoded_len());
        Ok(proto_response(&proto_resp))
    } else {
        Ok(Json(json!({
            "success": true,
            "data": {
                "cursor": new_cursor,
                "has_more": has_more,
                "changes": changes,
                "computed_updates": computed_updates
            }
        })).into_response())
    }
}

// ============================================================
// GET /api/v2/sync/snapshot
// ============================================================

#[derive(Deserialize)]
struct SnapshotQuery {
    #[serde(default)]
    store_id: Option<i32>,
}

async fn handle_snapshot(
    State(state): State<AppState>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
    Query(params): Query<SnapshotQuery>,
) -> Result<Response, AppError> {
    let store_id = ctx.store_id;
    let respond_proto = accepts_protobuf(&headers);

    let target_store_id = params.store_id.unwrap_or(store_id);

    let (snapshot, watermark_cursor) =
        merge_engine::build_snapshot(&state.pool, target_store_id).await?;

    let total_records: usize = snapshot.values().map(|v| v.len()).sum();

    // Sprint 171A: Log per-table record counts for diagnostic
    let table_summary: Vec<String> = snapshot.iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{}:{}", k, v.len()))
        .collect();
    tracing::info!(
        "📸 V2 Snapshot: store_id={}, tables={}, total_records={}, watermark={}, detail=[{}]",
        target_store_id, snapshot.len(), total_records, watermark_cursor,
        table_summary.join(", ")
    );

    // Sprint 173: Re-enable protobuf encoding for snapshot response.
    // Proto structs verified to match client definitions.
    if respond_proto {
        let proto_tables: Vec<ProtoSnapshotTable> = snapshot.into_iter().map(|(table, records)| {
            ProtoSnapshotTable {
                table_name: table,
                records: records.into_iter().map(|r| {
                    serde_json::to_vec(&r).unwrap_or_default()
                }).collect(),
            }
        }).collect();
        let proto_resp = ProtoSnapshotResponse {
            tables: proto_tables,
            watermark_cursor,
        };
        tracing::info!("📸 Snapshot: responding with protobuf ({} bytes)", proto_resp.encoded_len());
        Ok(proto_response(&proto_resp))
    } else {
        Ok(Json(json!({
            "snapshot": snapshot,
            "watermark_cursor": watermark_cursor
        })).into_response())
    }
}
