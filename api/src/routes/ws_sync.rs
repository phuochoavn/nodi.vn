use axum::{
    Router,
    extract::{State, Query, ws::{WebSocket, WebSocketUpgrade, Message}},
    routing::get,
    response::IntoResponse,
};
use dashmap::DashMap;
use serde::Deserialize;
use serde_json::json;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::AppState;
use crate::middleware::auth::verify_token;

// ============================================================
// Sprint 164: DashMap + broadcast registry
// Key: store_id → broadcast::Sender<String>
// Each connected device subscribes via sender.subscribe()
// ============================================================

pub type WsRegistry = Arc<DashMap<i32, broadcast::Sender<String>>>;

const BROADCAST_CAPACITY: usize = 100;
const HEARTBEAT_INTERVAL_SECS: u64 = 45;
const HEARTBEAT_TIMEOUT_SECS: u64 = 90; // 2 missed pings

#[derive(Deserialize)]
pub struct WsSyncQuery {
    token: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ws/sync", get(ws_sync_handler))
}

async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(q): Query<WsSyncQuery>,
) -> impl IntoResponse {
    // Authenticate via JWT before upgrade
    let auth = verify_token(&q.token, &state.config.jwt_secret);

    ws.on_upgrade(move |socket| async move {
        let claims = match auth {
            Ok(c) => c,
            Err(_) => {
                let (mut sink, _) = socket.split();
                let _ = sink.send(Message::Text(
                    json!({"type": "error", "message": "Authentication failed"}).to_string().into()
                )).await;
                let _ = sink.close().await;
                return;
            }
        };

        let store_id = claims.store_id;
        handle_sync_socket(socket, state, store_id).await;
    })
}

async fn handle_sync_socket(socket: WebSocket, state: AppState, store_id: i32) {
    let (mut sink, mut stream) = socket.split();

    // Get or create broadcast channel for this store
    let tx = state.sync_rooms
        .entry(store_id)
        .or_insert_with(|| broadcast::channel(BROADCAST_CAPACITY).0)
        .clone();

    // Subscribe to broadcast
    let mut rx = tx.subscribe();

    let receiver_count = tx.receiver_count();
    tracing::info!("🔌 WS sync connected: store_id={}, total_subscribers={}", store_id, receiver_count);

    // Send welcome message
    let _ = sink.send(Message::Text(
        json!({
            "type": "connected",
            "store_id": store_id,
            "message": "Real-time sync active"
        }).to_string().into()
    )).await;

    // Track last pong time for heartbeat timeout detection
    let last_pong = Arc::new(tokio::sync::Mutex::new(tokio::time::Instant::now()));
    let last_pong_write = last_pong.clone();

    // Spawn task: forward broadcast messages → WebSocket sink
    let write_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => {
                    if sink.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!("WS client lagged {} messages for store_id={}", n, store_id);
                    // Continue — skip missed messages
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    // Spawn heartbeat ping task (45s interval)
    let heartbeat_tx = tx.clone();
    let heartbeat_last_pong = last_pong.clone();
    let heartbeat_store_id = store_id;
    let ping_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(HEARTBEAT_INTERVAL_SECS));
        loop {
            interval.tick().await;

            // Check if we've exceeded heartbeat timeout
            let elapsed = {
                let lp = heartbeat_last_pong.lock().await;
                lp.elapsed()
            };

            if elapsed > tokio::time::Duration::from_secs(HEARTBEAT_TIMEOUT_SECS) {
                tracing::warn!("💔 WS heartbeat timeout: store_id={}, elapsed={}s",
                    heartbeat_store_id, elapsed.as_secs());
                break; // Will trigger cleanup
            }

            // Send ping via broadcast (all subscribers for this store get it)
            let ping = json!({"type": "ping"}).to_string();
            if heartbeat_tx.send(ping).is_err() {
                break; // No receivers
            }
        }
    });

    // Read loop: handle pong, close, and other incoming messages
    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Text(text) => {
                let text_str: &str = &text;
                if let Ok(evt) = serde_json::from_str::<serde_json::Value>(text_str) {
                    let evt_type = evt.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    if evt_type == "pong" {
                        // Update last pong time
                        let mut lp = last_pong_write.lock().await;
                        *lp = tokio::time::Instant::now();
                    }
                }
            }
            Message::Pong(_) => {
                // WebSocket-level pong — also counts as heartbeat
                let mut lp = last_pong_write.lock().await;
                *lp = tokio::time::Instant::now();
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup: abort background tasks
    ping_task.abort();
    write_task.abort();

    // Remove broadcast channel if no more receivers
    if tx.receiver_count() == 0 {
        state.sync_rooms.remove(&store_id);
        tracing::info!("🧹 WS registry cleaned: store_id={} (no receivers)", store_id);
    }

    tracing::info!("🔌 WS sync disconnected: store_id={}", store_id);
}

/// Broadcast a sync_update event to all WebSocket subscribers for a store.
/// Called from sync handlers after successful push/commit.
/// DashMap lookup is lock-free — safe to call from any async context.
/// Sprint 177: Added source_device so App can filter out self-originated events.
pub fn broadcast_sync_event(sync_rooms: &WsRegistry, store_id: i32, collections: &[&str], source_device: &str) {
    let msg = json!({
        "type": "sync_update",
        "store_id": store_id,
        "collections": collections,
        "source_device": source_device,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }).to_string();

    if let Some(tx) = sync_rooms.get(&store_id) {
        let count = tx.receiver_count();
        if count > 0 {
            let _ = tx.send(msg);
            tracing::info!("Broadcast sync_update to {} subscribers for store_id={}, collections={:?}, source={}",
                count, store_id, collections, source_device);
        }
    }
}

/// Send a direct message to all WebSocket subscribers for a store.
/// Used for force_resync and other direct notifications.
pub fn broadcast_message(sync_rooms: &WsRegistry, store_id: i32, msg: String) -> usize {
    if let Some(tx) = sync_rooms.get(&store_id) {
        let count = tx.receiver_count();
        if count > 0 {
            let _ = tx.send(msg);
        }
        count
    } else {
        0
    }
}
