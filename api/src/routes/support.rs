use axum::{Router, Json, extract::{State, Path, Query}, routing::{get, post, put, patch}};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::{NaiveDateTime, Duration, FixedOffset, DateTime};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{verify_token, Claims};

/// Format NaiveDateTime (stored as UTC) to Vietnam time string (UTC+7)
fn fmt_vn(dt: &NaiveDateTime) -> String {
    let vn = *dt + Duration::hours(7);
    vn.format("%Y-%m-%dT%H:%M:%S+07:00").to_string()
}

/// Parse a since timestamp (may include +07:00 offset) to UTC NaiveDateTime
fn parse_since_to_utc(since: &str) -> NaiveDateTime {
    // Try parsing as timezone-aware first (e.g. 2026-02-19T17:22:47+07:00)
    if let Ok(dt) = DateTime::<FixedOffset>::parse_from_rfc3339(since) {
        return dt.naive_utc();
    }
    // Try common format with offset (e.g. 2026-02-19T17:22:47+07:00 without fractional seconds)
    if let Ok(dt) = DateTime::parse_from_str(since, "%Y-%m-%dT%H:%M:%S%z") {
        return dt.naive_utc();
    }
    // Fallback: parse as naive (assume UTC)
    if let Ok(dt) = NaiveDateTime::parse_from_str(since, "%Y-%m-%dT%H:%M:%S") {
        return dt;
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(since, "%Y-%m-%d %H:%M:%S") {
        return dt;
    }
    // Last resort: epoch
    NaiveDateTime::parse_from_str("1970-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
}

fn extract_admin(headers: &HeaderMap, secret: &str) -> Result<Claims, AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    let claims = verify_token(token, secret)?;
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    Ok(claims)
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Customer-facing
        .route("/api/support/ticket", post(create_ticket))
        .route("/api/support/my-tickets", get(my_tickets))
        .route("/api/support/ticket/{id}/messages", get(ticket_messages))
        .route("/api/support/ticket/{id}/reply", post(customer_reply))
        .route("/api/support/ticket/{id}/close", patch(customer_close_ticket))
        .route("/api/support/ticket/{id}/status", get(ticket_status))
        // Admin-facing
        .route("/api/admin/support/tickets", get(admin_list_tickets))
        .route("/api/admin/support/tickets/{id}", get(admin_ticket_detail))
        .route("/api/admin/support/tickets/{id}/reply", post(admin_reply))
        .route("/api/admin/support/tickets/{id}/status", put(admin_update_status))
        .route("/api/admin/support/unread", get(admin_unread))
}

// ========================
// Request structs
// ========================

#[derive(Deserialize)]
struct CreateTicketReq {
    store_name: Option<String>,
    phone: Option<String>,
    subject: String,
    message: String,
}

#[derive(Deserialize)]
struct CustomerReplyReq {
    message: String,
}

#[derive(Deserialize)]
struct MessagesQuery {
    since: Option<String>,
}

#[derive(Deserialize)]
struct AdminReplyReq {
    message: String,
}

#[derive(Deserialize)]
struct AdminStatusReq {
    status: String,
}

#[derive(Deserialize)]
struct AdminTicketsQuery {
    status: Option<String>,
}

// ========================
// Helper: extract license_key from header
// ========================
fn extract_license_key(headers: &HeaderMap) -> Result<String, AppError> {
    let key = headers.get("license_key")
        .or_else(|| headers.get("license-key"))
        .or_else(|| headers.get("License-Key"))
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing license_key header".into()))?;
    Ok(key.to_string())
}

// ========================
// Helper: verify license_key exists
// ========================
async fn verify_license(pool: &sqlx::PgPool, license_key: &str) -> Result<(i32, Option<String>), AppError> {
    let row: Option<(i32, Option<String>)> = sqlx::query_as(
        "SELECT id, owner_name FROM stores WHERE license_key = $1"
    ).bind(license_key).fetch_optional(pool).await?;
    match row {
        Some(r) => Ok(r),
        None => Err(AppError::Unauthorized("Invalid license key".into())),
    }
}

// ========================
// Customer Endpoints
// ========================

// POST /api/support/ticket — license_key in header
async fn create_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<CreateTicketReq>,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    let (store_id, owner_name) = verify_license(&state.pool, &license_key).await?;

    let store_name = body.store_name.or(owner_name).unwrap_or_default();

    let ticket: (i32,) = sqlx::query_as(
        "INSERT INTO support_tickets (store_id, license_key, store_name, phone, subject) \
         VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(store_id).bind(&license_key).bind(&store_name)
    .bind(&body.phone).bind(&body.subject)
    .fetch_one(&state.pool).await?;

    // Insert first message
    sqlx::query(
        "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
         VALUES ($1, 'customer', $2, $3)"
    )
    .bind(ticket.0).bind(&store_name).bind(&body.message)
    .execute(&state.pool).await?;

    Ok(Json(json!({ "ticket_id": ticket.0, "status": "open" })))
}

// GET /api/support/my-tickets — license_key in header
async fn my_tickets(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    verify_license(&state.pool, &license_key).await?;

    let rows = sqlx::query_as::<_, (i32, String, String, NaiveDateTime, NaiveDateTime)>(
        "SELECT id, subject, status, created_at, updated_at FROM support_tickets \
         WHERE license_key = $1 ORDER BY updated_at DESC"
    ).bind(&license_key).fetch_all(&state.pool).await?;

    let mut tickets = Vec::new();
    for r in &rows {
        let last_msg: Option<(String,)> = sqlx::query_as(
            "SELECT message FROM support_messages WHERE ticket_id = $1 ORDER BY created_at DESC LIMIT 1"
        ).bind(r.0).fetch_optional(&state.pool).await.unwrap_or(None);

        let unread: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM support_messages WHERE ticket_id = $1 AND sender_type = 'admin' \
             AND created_at > (SELECT COALESCE(MAX(created_at), '1970-01-01') FROM support_messages WHERE ticket_id = $1 AND sender_type = 'customer')"
        ).bind(r.0).fetch_one(&state.pool).await.unwrap_or((0,));

        tickets.push(json!({
            "id": r.0, "subject": r.1, "status": r.2,
            "last_message": last_msg.map(|m| m.0),
            "unread_count": unread.0,
            "created_at": fmt_vn(&r.3), "updated_at": fmt_vn(&r.4)
        }));
    }

    Ok(Json(json!(tickets)))
}

// GET /api/support/ticket/:id/messages — license_key in header
// Returns { messages: [...], ticket_status: "open" }
async fn ticket_messages(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Query(q): Query<MessagesQuery>,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    verify_license(&state.pool, &license_key).await?;

    // Verify ticket belongs to this license + get status
    let ticket = sqlx::query_as::<_, (i32, String)>(
        "SELECT id, status FROM support_tickets WHERE id = $1 AND license_key = $2"
    ).bind(id).bind(&license_key).fetch_optional(&state.pool).await?;

    let (_, ticket_st) = match ticket {
        Some(t) => t,
        None => return Err(AppError::NotFound("Ticket not found".into())),
    };

    let since_utc = match &q.since {
        Some(s) => parse_since_to_utc(s),
        None => NaiveDateTime::parse_from_str("1970-01-01T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap(),
    };

    let rows = sqlx::query_as::<_, (i32, String, Option<String>, String, NaiveDateTime)>(
        "SELECT id, sender_type, sender_name, message, created_at FROM support_messages \
         WHERE ticket_id = $1 AND created_at > $2 ORDER BY created_at ASC"
    ).bind(id).bind(since_utc).fetch_all(&state.pool).await?;

    let messages: Vec<Value> = rows.iter().map(|r| json!({
        "id": r.0, "sender_type": r.1, "sender_name": r.2,
        "message": r.3, "created_at": fmt_vn(&r.4)
    })).collect();

    Ok(Json(json!({
        "messages": messages,
        "ticket_status": ticket_st
    })))
}

// POST /api/support/ticket/:id/reply — license_key in header
async fn customer_reply(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<CustomerReplyReq>,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    let (_store_id, owner_name) = verify_license(&state.pool, &license_key).await?;

    // Verify ticket belongs to this license
    let ticket: Option<(i32,)> = sqlx::query_as(
        "SELECT id FROM support_tickets WHERE id = $1 AND license_key = $2"
    ).bind(id).bind(&license_key).fetch_optional(&state.pool).await?;
    if ticket.is_none() {
        return Err(AppError::NotFound("Ticket not found".into()));
    }

    let name = owner_name.unwrap_or_else(|| "Khách hàng".into());
    let msg: (i32,) = sqlx::query_as(
        "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
         VALUES ($1, 'customer', $2, $3) RETURNING id"
    ).bind(id).bind(&name).bind(&body.message).fetch_one(&state.pool).await?;

    // Update ticket timestamp
    sqlx::query("UPDATE support_tickets SET updated_at = NOW() WHERE id = $1")
        .bind(id).execute(&state.pool).await.ok();

    Ok(Json(json!({ "message_id": msg.0 })))
}

// PATCH /api/support/ticket/:id/close — license_key in header
async fn customer_close_ticket(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    verify_license(&state.pool, &license_key).await?;

    // Verify ticket belongs to this license
    let ticket = sqlx::query_as::<_, (i32, String)>(
        "SELECT id, status FROM support_tickets WHERE id = $1 AND license_key = $2"
    ).bind(id).bind(&license_key).fetch_optional(&state.pool).await?;

    let (ticket_id, current_status) = match ticket {
        Some(t) => t,
        None => return Err(AppError::NotFound("Ticket not found".into())),
    };

    if current_status == "closed" {
        return Ok(Json(json!({ "status": "closed", "ticket_id": ticket_id })));
    }

    sqlx::query(
        "UPDATE support_tickets SET status = 'closed', closed_by = 'customer', \
         closed_at = NOW(), updated_at = NOW() WHERE id = $1"
    ).bind(ticket_id).execute(&state.pool).await?;

    // System message
    sqlx::query(
        "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
         VALUES ($1, 'system', 'Hệ thống', 'Khách hàng đã kết thúc phiên hỗ trợ.')"
    ).bind(ticket_id).execute(&state.pool).await.ok();

    Ok(Json(json!({ "status": "closed", "ticket_id": ticket_id })))
}

// GET /api/support/ticket/:id/status — license_key in header
async fn ticket_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    let license_key = extract_license_key(&headers)?;
    verify_license(&state.pool, &license_key).await?;

    let ticket = sqlx::query_as::<_, (i32, String, Option<String>, Option<NaiveDateTime>)>(
        "SELECT id, status, closed_by, closed_at FROM support_tickets \
         WHERE id = $1 AND license_key = $2"
    ).bind(id).bind(&license_key).fetch_optional(&state.pool).await?;

    let (ticket_id, status, closed_by, closed_at) = match ticket {
        Some(t) => t,
        None => return Err(AppError::NotFound("Ticket not found".into())),
    };

    Ok(Json(json!({
        "ticket_id": ticket_id,
        "status": status,
        "closed_by": closed_by,
        "closed_at": closed_at.map(|dt| fmt_vn(&dt))
    })))
}

// ========================
// Admin Endpoints
// ========================

// GET /api/admin/support/tickets?status=
async fn admin_list_tickets(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AdminTicketsQuery>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let rows = if let Some(ref status) = q.status {
        sqlx::query_as::<_, (i32, Option<String>, Option<String>, String, String, NaiveDateTime, NaiveDateTime, Option<String>)>(
            "SELECT id, store_name, phone, subject, status, created_at, updated_at, license_key \
             FROM support_tickets WHERE status = $1 ORDER BY updated_at DESC"
        ).bind(status).fetch_all(&state.pool).await?
    } else {
        sqlx::query_as::<_, (i32, Option<String>, Option<String>, String, String, NaiveDateTime, NaiveDateTime, Option<String>)>(
            "SELECT id, store_name, phone, subject, status, created_at, updated_at, license_key \
             FROM support_tickets ORDER BY \
             CASE WHEN status='open' THEN 0 WHEN status='in_progress' THEN 1 ELSE 2 END, \
             updated_at DESC"
        ).fetch_all(&state.pool).await?
    };

    let mut tickets = Vec::new();
    for r in &rows {
        let last_msg: Option<(String,)> = sqlx::query_as(
            "SELECT message FROM support_messages WHERE ticket_id = $1 ORDER BY created_at DESC LIMIT 1"
        ).bind(r.0).fetch_optional(&state.pool).await.unwrap_or(None);

        // Unread = customer messages after last admin message
        let unread: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM support_messages WHERE ticket_id = $1 AND sender_type = 'customer' \
             AND created_at > (SELECT COALESCE(MAX(created_at), '1970-01-01') FROM support_messages WHERE ticket_id = $1 AND sender_type = 'admin')"
        ).bind(r.0).fetch_one(&state.pool).await.unwrap_or((0,));

        tickets.push(json!({
            "id": r.0, "store_name": r.1, "phone": r.2,
            "subject": r.3, "status": r.4,
            "unread_count": unread.0,
            "last_message": last_msg.map(|m| m.0),
            "created_at": fmt_vn(&r.5), "updated_at": fmt_vn(&r.6),
            "license_key": r.7
        }));
    }

    Ok(Json(json!(tickets)))
}

// GET /api/admin/support/tickets/:id
async fn admin_ticket_detail(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let ticket = sqlx::query_as::<_, (i32, Option<String>, Option<String>, String, String, NaiveDateTime, Option<String>, Option<String>)>(
        "SELECT id, store_name, phone, subject, status, created_at, license_key, \
         (SELECT license_type FROM stores WHERE license_key = support_tickets.license_key LIMIT 1) \
         FROM support_tickets WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await?;

    let t = match ticket {
        Some(t) => t,
        None => return Err(AppError::NotFound("Ticket not found".into())),
    };

    let messages = sqlx::query_as::<_, (i32, String, Option<String>, String, NaiveDateTime)>(
        "SELECT id, sender_type, sender_name, message, created_at FROM support_messages \
         WHERE ticket_id = $1 ORDER BY created_at ASC"
    ).bind(id).fetch_all(&state.pool).await?;

    let msgs: Vec<Value> = messages.iter().map(|r| json!({
        "id": r.0, "sender_type": r.1, "sender_name": r.2,
        "message": r.3, "created_at": fmt_vn(&r.4)
    })).collect();

    Ok(Json(json!({
        "ticket": {
            "id": t.0, "store_name": t.1, "phone": t.2,
            "subject": t.3, "status": t.4,
            "created_at": fmt_vn(&t.5),
            "license_key": t.6, "license_type": t.7
        },
        "messages": msgs
    })))
}

// POST /api/admin/support/tickets/:id/reply
async fn admin_reply(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<AdminReplyReq>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Verify ticket exists
    let ticket: Option<(i32,)> = sqlx::query_as(
        "SELECT id FROM support_tickets WHERE id = $1"
    ).bind(id).fetch_optional(&state.pool).await?;
    if ticket.is_none() {
        return Err(AppError::NotFound("Ticket not found".into()));
    }

    let msg: (i32,) = sqlx::query_as(
        "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
         VALUES ($1, 'admin', 'Admin', $2) RETURNING id"
    ).bind(id).bind(&body.message).fetch_one(&state.pool).await?;

    // Auto-move to in_progress if open
    sqlx::query(
        "UPDATE support_tickets SET updated_at = NOW(), \
         status = CASE WHEN status = 'open' THEN 'in_progress' ELSE status END \
         WHERE id = $1"
    ).bind(id).execute(&state.pool).await.ok();

    Ok(Json(json!({ "message_id": msg.0 })))
}

// PUT /api/admin/support/tickets/:id/status
async fn admin_update_status(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(body): Json<AdminStatusReq>,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    let valid = ["open", "in_progress", "resolved", "closed"];
    if !valid.contains(&body.status.as_str()) {
        return Err(AppError::BadRequest(format!("Invalid status: {}. Must be one of: {:?}", body.status, valid)));
    }

    let is_closing = body.status == "closed" || body.status == "resolved";

    if is_closing {
        sqlx::query(
            "UPDATE support_tickets SET status = $1, updated_at = NOW(), \
             closed_by = 'admin', closed_at = NOW(), \
             resolved_at = CASE WHEN $1 = 'resolved' THEN NOW() ELSE resolved_at END \
             WHERE id = $2"
        ).bind(&body.status).bind(id).execute(&state.pool).await?;

        // Insert system message
        let sys_result = sqlx::query_as::<_, (i32, chrono::NaiveDateTime)>(
            "INSERT INTO support_messages (ticket_id, sender_type, sender_name, message) \
             VALUES ($1, 'system', 'Hệ thống', 'Phiên hỗ trợ đã được kết thúc bởi kỹ thuật viên.') \
             RETURNING id, created_at"
        ).bind(id).fetch_optional(&state.pool).await.ok().flatten();

        // Broadcast ticket_closed + system message to WS room
        {
            let rooms = state.chat_rooms.read().await;
            if let Some(clients) = rooms.get(&id) {
                // ticket_closed event
                let closed_evt = serde_json::json!({
                    "type": "ticket_closed",
                    "ticket_id": id,
                    "closed_by": "admin",
                    "status": &body.status
                }).to_string();
                for (_, _, tx) in clients {
                    let _ = tx.send(closed_evt.clone());
                }

                // system message event (so WS clients see it immediately)
                if let Some((msg_id, created_at)) = sys_result {
                    let vn = created_at + chrono::Duration::hours(7);
                    let sys_msg = serde_json::json!({
                        "type": "message",
                        "id": msg_id,
                        "text": "Phiên hỗ trợ đã được kết thúc bởi kỹ thuật viên.",
                        "sender_type": "system",
                        "sender_name": "Hệ thống",
                        "created_at": vn.format("%Y-%m-%dT%H:%M:%S+07:00").to_string()
                    }).to_string();
                    for (_, _, tx) in clients {
                        let _ = tx.send(sys_msg.clone());
                    }
                }
            }
        }
    } else {
        // Reopening — clear closed_by/closed_at
        sqlx::query(
            "UPDATE support_tickets SET status = $1, updated_at = NOW(), \
             closed_by = NULL, closed_at = NULL WHERE id = $2"
        ).bind(&body.status).bind(id).execute(&state.pool).await?;
    }

    Ok(Json(json!({ "ok": true })))
}

// GET /api/admin/support/unread
async fn admin_unread(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    extract_admin(&headers, &state.config.jwt_secret)?;

    // Count tickets with unread customer messages (status = open or in_progress)
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM support_tickets t \
         WHERE t.status IN ('open', 'in_progress') \
         AND EXISTS (SELECT 1 FROM support_messages m WHERE m.ticket_id = t.id AND m.sender_type = 'customer' \
         AND m.created_at > (SELECT COALESCE(MAX(m2.created_at), '1970-01-01') FROM support_messages m2 WHERE m2.ticket_id = t.id AND m2.sender_type = 'admin'))"
    ).fetch_one(&state.pool).await.unwrap_or((0,));

    Ok(Json(json!({ "count": count.0 })))
}
