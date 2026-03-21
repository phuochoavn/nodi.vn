use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::sync::Arc;

use crate::AppState;

// ============================================================
// TenantContext — Injected into request extensions by middleware
// ============================================================

/// Extracted tenant information, available in request extensions
/// after the tenant middleware runs.
#[derive(Clone, Debug)]
pub struct TenantContext {
    pub store_id: i32,
    pub device_id: Option<String>,
}

// ============================================================
// SET LOCAL helper — call inside an existing transaction
// ============================================================

/// Set `app.current_store_id` on a transaction so RLS policies filter by store.
/// Uses `SET LOCAL` which scopes the setting to the current transaction only.
pub async fn set_tenant_on_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
) -> Result<(), crate::error::AppError> {
    sqlx::query(&format!(
        "SET LOCAL app.current_store_id = '{}'",
        store_id
    ))
    .execute(&mut **tx)
    .await?;
    Ok(())
}

// ============================================================
// Tenant Middleware — extracts store_id and injects TenantContext
// ============================================================

/// Axum middleware that extracts tenant identity from the request
/// (JWT or X-HWID) and injects `TenantContext` into extensions.
///
/// Should be applied only to V2 sync routes.
pub async fn tenant_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let headers = req.headers();

    // Reuse existing identify_store logic
    let store_id = match crate::routes::sync::identify_store(
        headers,
        &state.pool,
        &state.config.jwt_secret,
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            // Return early with error response
            return e.into_response();
        }
    };

    // Extract device_id from X-Device-Id header if present
    let device_id = req
        .headers()
        .get("x-device-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Inject TenantContext into request extensions
    req.extensions_mut().insert(TenantContext {
        store_id,
        device_id,
    });

    next.run(req).await
}
