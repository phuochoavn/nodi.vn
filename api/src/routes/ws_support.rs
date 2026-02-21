use axum::{
    Router,
    extract::{State, Query, ws::{WebSocket, WebSocketUpgrade, Message}},
    routing::get,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::{json, Value};
use futures::{SinkExt, StreamExt};
use chrono::{NaiveDateTime, Duration};

use crate::AppState;
use crate::middleware::auth::verify_token;

/// Format NaiveDateTime (stored as UTC) to Vietnam time string (UTC+7)
fn fmt_vn(dt: &NaiveDateTime) -> String {
    let vn = *dt + Duration::hours(7);
    vn.format("%Y-%m-%dT%H:%M:%S+07:00").to_string()
}

#[derive(Deserialize)]
pub struct WsQuery {
    ticket_id: i32,
    license_key: Option<String>,
    token: Option<String>,
}

/// Returns (sender_type, sender_name)
async fn authenticate(pool: &sqlx::PgPool, jwt_secret: &str, q: &WsQuery) -> Option<(String, String)> {
    // Admin auth via JWT token
    if let Some(ref token) = q.token {
        if let Ok(claims) = verify_token(token, jwt_secret) {
            if claims.role == "admin" {
                return Some(("admin".into(), "Admin".into()));
            }
        }
    }

    // Customer auth via license_key
    if let Some(ref lk) = q.license_key {
        let row: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT owner_name FROM stores WHERE license_key = $1 AND is_active = true"
        ).bind(lk).fetch_optional(pool).await.ok()?;

        if let Some((name,)) = row {
            return Some(("customer".into(), name.unwrap_or_else(|| "Khách hàng".into())));
        }
    }

    None
}

/// Verify the ticket exists (and belongs to customer if customer auth)
async fn verify_ticket(pool: &sqlx::PgPool, ticket_id: i32, sender_type: &str, license_key: Option<&str>) -> bool {
    if sender_type == "admin" {
        // Admin can access any ticket
        let r: Option<(i32,)> = sqlx::query_as("SELECT id FROM support_tickets WHERE id = $1")
            .bind(ticket_id).fetch_optional(pool).await.unwrap_or(None);
        return r.is_some();
    }

    // Customer must own the ticket
    if let Some(lk) = license_key {
        let r: Option<(i32,)> = sqlx::query_as(
            "SELECT id FROM support_tickets WHERE id = $1 AND license_key = $2"
        ).bind(ticket_id).bind(lk).fetch_optional(pool).await.unwrap_or(None);
        return r.is_some();
    }

    false
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/support/ws", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(q): Query<WsQuery>,
) -> impl IntoResponse {
    // Authenticate before upgrade
    let auth = authenticate(&state.pool, &state.config.jwt_secret, &q).await;

    ws.on_upgrade(move |socket| async move {
        let (sender_type, sender_name) = match auth {
            Some(a) => a,
            None => {
                // Send auth error and close
                let (mut sink, _) = socket.split();
                let _ = sink.send(Message::Text(
                    json!({"type": "error", "message": "Authentication failed"}).to_string().into()
                )).await;
                let _ = sink.close().await;
                return;
            }
        };

        // Verify ticket access
        if !verify_ticket(&state.pool, q.ticket_id, &sender_type, q.license_key.as_deref()).await {
            let (mut sink, _) = socket.split();
            let _ = sink.send(Message::Text(
                json!({"type": "error", "message": "Ticket not found or access denied"}).to_string().into()
            )).await;
            let _ = sink.close().await;
            return;
        }

        handle_socket(socket, state, q.ticket_id, sender_type, sender_name).await;
    })
}

async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    ticket_id: i32,
    sender_type: String,
    sender_name: String,
) {
    let (mut sink, mut stream) = socket.split();

    // Create a channel for this client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // Register in room
    {
        let mut rooms = state.chat_rooms.write().await;
        rooms.entry(ticket_id)
            .or_insert_with(Vec::new)
            .push((sender_type.clone(), sender_name.clone(), tx.clone()));
    }

    // Broadcast online status
    broadcast_to_room(&state, ticket_id, &json!({
        "type": "online",
        "sender_type": &sender_type,
        "sender_name": &sender_name,
        "online": true
    }), Some(&tx)).await;

    // Spawn a task to forward channel messages → WebSocket sink
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Read loop: process incoming WebSocket messages
    let pool = state.pool.clone();
    let rooms = state.chat_rooms.clone();
    let st = sender_type.clone();
    let sn = sender_name.clone();
    let tx2 = tx.clone();

    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Text(text) => {
                let text_str: &str = &text;
                if let Ok(evt) = serde_json::from_str::<Value>(text_str) {
                    let evt_type = evt.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    match evt_type {
                        "message" => {
                            let msg_text = evt.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            if msg_text.is_empty() { continue; }

                            // Save to DB
                            let result = sqlx::query_as::<_, (i32, NaiveDateTime)>(
                                "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
                                 VALUES ($1, $2, $3, $4) RETURNING id, created_at"
                            )
                            .bind(ticket_id).bind(&st).bind(&sn).bind(&msg_text)
                            .fetch_one(&pool).await;

                            if let Ok((msg_id, created_at)) = result {
                                // Update ticket timestamp + auto-move status
                                if st == "admin" {
                                    sqlx::query(
                                        "UPDATE support_tickets SET updated_at = NOW(), \
                                         status = CASE WHEN status = 'open' THEN 'in_progress' ELSE status END \
                                         WHERE id = $1"
                                    ).bind(ticket_id).execute(&pool).await.ok();
                                } else {
                                    sqlx::query("UPDATE support_tickets SET updated_at = NOW() WHERE id = $1")
                                        .bind(ticket_id).execute(&pool).await.ok();
                                }

                                // Broadcast to all in room
                                broadcast_to_room(&state, ticket_id, &json!({
                                    "type": "message",
                                    "id": msg_id,
                                    "text": msg_text,
                                    "sender_type": &st,
                                    "sender_name": &sn,
                                    "created_at": fmt_vn(&created_at)
                                }), None).await;
                            }
                        }
                        "typing" => {
                            broadcast_to_room(&state, ticket_id, &json!({
                                "type": "typing",
                                "sender_type": &st,
                                "sender_name": &sn
                            }), Some(&tx2)).await;
                        }
                        "stop_typing" => {
                            broadcast_to_room(&state, ticket_id, &json!({
                                "type": "stop_typing",
                                "sender_type": &st
                            }), Some(&tx2)).await;
                        }
                        "read" => {
                            if let Some(ids) = evt.get("message_ids").and_then(|v| v.as_array()) {
                                let msg_ids: Vec<i32> = ids.iter().filter_map(|v| v.as_i64().map(|n| n as i32)).collect();
                                if !msg_ids.is_empty() {
                                    // Update read_at for these messages
                                    sqlx::query(
                                        "UPDATE support_messages SET read_at = NOW() \
                                         WHERE id = ANY($1) AND ticket_id = $2 AND read_at IS NULL"
                                    )
                                    .bind(&msg_ids).bind(ticket_id)
                                    .execute(&pool).await.ok();

                                    broadcast_to_room(&state, ticket_id, &json!({
                                        "type": "read",
                                        "message_ids": msg_ids,
                                        "reader": &st
                                    }), Some(&tx2)).await;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup: remove from room
    {
        let mut rooms_lock = rooms.write().await;
        if let Some(clients) = rooms_lock.get_mut(&ticket_id) {
            clients.retain(|(_, _, t)| !t.same_channel(&tx));
            if clients.is_empty() {
                rooms_lock.remove(&ticket_id);
            }
        }
    }

    // Broadcast offline
    broadcast_to_room(&state, ticket_id, &json!({
        "type": "online",
        "sender_type": &sender_type,
        "sender_name": &sender_name,
        "online": false
    }), None).await;

    // Abort the write task
    write_task.abort();
}

/// Broadcast a JSON event to all clients in a ticket room.
/// If `exclude` is Some, skip that sender (don't echo back to self for typing events).
async fn broadcast_to_room(state: &AppState, ticket_id: i32, event: &Value, exclude: Option<&tokio::sync::mpsc::UnboundedSender<String>>) {
    let msg = event.to_string();
    let rooms = state.chat_rooms.read().await;
    if let Some(clients) = rooms.get(&ticket_id) {
        for (_, _, tx) in clients {
            if let Some(excl) = exclude {
                if tx.same_channel(excl) {
                    continue;
                }
            }
            let _ = tx.send(msg.clone());
        }
    }
}
