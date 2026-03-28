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
        .route("/api/v2/sync/register-device", post(handle_register_device))
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

    // ── Sprint 197: Quota enforcement for invoices ──────────────────────────
    let mut changes = changes; // make mutable
    let mut rejected_tables: Vec<String> = Vec::new();
    let mut rejection_info: Option<(i64, i64)> = None; // (orders_today, limit)

    let invoice_tables = ["invoices", "invoice_items", "invoice_payments"];
    let has_invoices = changes.contains_key("invoices") && !changes.get("invoices").map_or(true, |v| v.is_empty());

    if has_invoices {
        // Query account plan info for this store
        let plan_info = sqlx::query_as::<_, (String, Option<chrono::DateTime<chrono::Utc>>, Option<i32>)>(
            "SELECT COALESCE(a.plan_type, 'free'), a.trial_ends_at, a.orders_limit \
             FROM accounts a \
             JOIN account_stores ast ON a.id = ast.account_id \
             WHERE ast.data_store_id = $1 \
             LIMIT 1"
        )
        .bind(store_id)
        .fetch_optional(&state.pool)
        .await
        .ok()
        .flatten();

        let should_enforce = match &plan_info {
            Some((plan_type, trial_ends_at, _)) => match plan_type.as_str() {
                "pro" | "lifetime" => false,
                "trial" => match trial_ends_at {
                    Some(ends_at) if *ends_at > chrono::Utc::now() => false,
                    _ => true,
                },
                _ => true, // "free" → enforce
            },
            None => false, // No account info (legacy/device auth) → no enforcement
        };

        if should_enforce {
            let current_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) = CURRENT_DATE"
            )
            .bind(store_id)
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);

            let limit = plan_info.as_ref().and_then(|(_, _, ol)| *ol).unwrap_or(20) as i64;
            let batch_invoice_count = changes.get("invoices").map_or(0, |v| v.len()) as i64;

            if current_count + batch_invoice_count > limit {
                // Remove invoice-related tables from changes
                for table in &invoice_tables {
                    if changes.remove(*table).is_some() {
                        rejected_tables.push(table.to_string());
                    }
                }
                rejection_info = Some((current_count, limit));
                tracing::warn!(
                    "⚠️ Quota exceeded: store_id={}, plan=free, today={}, batch={}, limit={}. Rejected: {:?}",
                    store_id, current_count, batch_invoice_count, limit, rejected_tables
                );
            }
        }
    }
    // ── End quota enforcement ───────────────────────────────────────────────

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
            &state.sync_rooms, store_id, &changed_tables, &device_id
        );

        // Sprint 180: FCM push for background/killed devices (async, non-blocking)
        let fcm_pool = state.pool.clone();
        let fcm_device = device_id.clone();
        let fcm_tables: Vec<String> = changes.keys().cloned().collect();
        tokio::spawn(async move {
            crate::services::fcm::send_data_message(&fcm_pool, store_id, &fcm_device, fcm_tables).await;
        });
    }

    tracing::info!(
        "✅ V2 Push: store_id={}, device={}, batch={}, total={}, processed={}, conflicts={}, proto={}, rejected={:?}",
        store_id, device_id, batch_id, total_changes, processed, conflict_count, use_proto, rejected_tables
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
        let mut resp = json!({
            "success": true,
            "message": format!("Đã nhận {} thay đổi, xử lý {}, xung đột {}", total_changes, processed, conflict_count),
            "data": {
                "new_cursor": new_cursor,
                "processed": processed,
                "conflicts": conflicts,
                "computed_updates": computed_updates,
                "last_processed_client_tx_id": max_journal_id
            }
        });
        // Append rejection info if any tables were rejected
        if !rejected_tables.is_empty() {
            if let Some((orders_today, limit)) = rejection_info {
                resp["rejected_tables"] = json!(rejected_tables);
                resp["reason"] = json!("FREE_LIMIT_REACHED");
                resp["orders_today"] = json!(orders_today);
                resp["orders_limit"] = json!(limit);
            }
        }
        Ok(Json(resp).into_response())
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

    // Sprint 210: Check if device has been revoked — trigger wipe on client
    let revoked = sqlx::query_as::<_, (bool,)>(
        "SELECT COALESCE(wipe_flag, false) FROM devices \
         WHERE store_id = $1 AND device_id = $2 AND (wipe_flag = true OR revoked_at IS NOT NULL)"
    )
    .bind(store_id)
    .bind(&device_id)
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten();

    if revoked.is_some() {
        tracing::warn!("🔒 Sprint 210: Revoked device attempted pull: store_id={}, device={}", store_id, device_id);
        return Ok((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "WIPE_AUTHORIZED",
                "message": "Device has been revoked"
            })),
        ).into_response());
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

// ============================================================
// POST /api/v2/sync/register-device — Sprint 180: FCM token registration
// ============================================================

#[derive(Deserialize)]
struct RegisterDeviceRequest {
    #[serde(default)]
    fcm_token: Option<String>,
    #[serde(default)]
    platform: Option<String>,
}

async fn handle_register_device(
    State(state): State<AppState>,
    Extension(ctx): Extension<TenantContext>,
    headers: HeaderMap,
    Json(body): Json<RegisterDeviceRequest>,
) -> Result<Json<Value>, AppError> {
    let store_id = ctx.store_id;
    let device_id = ctx.device_id
        .or_else(|| get_header(&headers, "x-device-id"))
        .ok_or_else(|| AppError::BadRequest("Missing device_id".into()))?;

    if device_id.is_empty() {
        return Err(AppError::BadRequest("device_id cannot be empty".into()));
    }

    let fcm_token = body.fcm_token.unwrap_or_default();
    let platform = body.platform.unwrap_or_else(|| "unknown".to_string());

    // Upsert device with FCM token
    sqlx::query(
        "INSERT INTO sync_devices (store_id, device_id, fcm_token, platform)          VALUES ($1, $2, $3, $4)          ON CONFLICT (store_id, device_id) DO UPDATE SET          fcm_token = EXCLUDED.fcm_token, platform = EXCLUDED.platform"
    )
    .bind(store_id)
    .bind(&device_id)
    .bind(&fcm_token)
    .bind(&platform)
    .execute(&state.pool)
    .await?;

    tracing::info!(
        "Sprint 180: Device registered: store_id={}, device={}, platform={}, has_token={}",
        store_id, device_id, platform, !fcm_token.is_empty()
    );

    Ok(Json(json!({
        "success": true,
        "message": "Device registered",
        "device_id": device_id,
        "platform": platform
    })))
}
