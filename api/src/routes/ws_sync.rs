use axum::{
    Router,
    extract::{State, Query, ws::{WebSocket, WebSocketUpgrade, Message}},
    routing::get,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;
use crate::middleware::auth::verify_token;

/// Sync rooms: store_id → list of WebSocket senders
pub type SyncSender = tokio::sync::mpsc::UnboundedSender<String>;
pub type SyncRooms = Arc<RwLock<HashMap<i32, Vec<SyncSender>>>>;

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

    // Create channel for this client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // Register in sync room
    {
        let mut rooms = state.sync_rooms.write().await;
        rooms.entry(store_id).or_insert_with(Vec::new).push(tx.clone());
    }

    let client_count = {
        let rooms = state.sync_rooms.read().await;
        rooms.get(&store_id).map(|v| v.len()).unwrap_or(0)
    };
    tracing::info!("🔌 WS sync connected: store_id={}, total_clients={}", store_id, client_count);

    // Send welcome message
    let _ = sink.send(Message::Text(
        json!({
            "type": "connected",
            "store_id": store_id,
            "message": "Real-time sync active"
        }).to_string().into()
    )).await;

    // Spawn task: forward channel messages → WebSocket
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Read loop: handle ping/pong and close
    let tx_clone = tx.clone();
    let sync_rooms = state.sync_rooms.clone();
    let ping_interval = tokio::spawn({
        let tx = tx_clone.clone();
        async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                let ping = json!({"type": "ping"}).to_string();
                if tx.send(ping).is_err() {
                    break;
                }
            }
        }
    });

    // Listen for incoming messages (pong, close)
    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Text(text) => {
                let text_str: &str = &text;
                // Client can send pong or other messages — just log
                if let Ok(evt) = serde_json::from_str::<serde_json::Value>(text_str) {
                    let evt_type = evt.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    if evt_type == "pong" {
                        // Heartbeat OK
                    }
                }
            }
            Message::Pong(_) => { /* WebSocket-level pong, connection alive */ }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup: remove from room
    {
        let mut rooms = sync_rooms.write().await;
        if let Some(clients) = rooms.get_mut(&store_id) {
            clients.retain(|t| !t.same_channel(&tx));
            if clients.is_empty() {
                rooms.remove(&store_id);
            }
        }
    }

    tracing::info!("🔌 WS sync disconnected: store_id={}", store_id);

    // Abort background tasks
    ping_interval.abort();
    write_task.abort();
}

/// Broadcast a sync_update event to all WebSocket clients in a store room.
/// Called from handle_sync() after successful commit.
pub async fn broadcast_sync_event(sync_rooms: &SyncRooms, store_id: i32, collections: &[&str]) {
    let msg = json!({
        "type": "sync_update",
        "store_id": store_id,
        "collections": collections,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }).to_string();

    let rooms = sync_rooms.read().await;
    if let Some(clients) = rooms.get(&store_id) {
        let count = clients.len();
        for tx in clients {
            let _ = tx.send(msg.clone());
        }
        if count > 0 {
            tracing::info!("📡 Broadcast sync_update to {} clients for store_id={}, collections={:?}",
                count, store_id, collections);
        }
    }
}
