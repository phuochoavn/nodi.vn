use axum::{Router, Json, routing::post};
use serde_json::{json, Value};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/scanner/connect", post(scanner_connect))
        .route("/api/scanner/scan", post(scanner_scan))
        .route("/api/scanner/status", post(scanner_status))
}

// ============================================================
// Stub endpoints for remote barcode scanner
// Real implementation deferred to future sprint
// ============================================================

async fn scanner_connect() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Scanner service available"
    }))
}

async fn scanner_scan() -> Json<Value> {
    Json(json!({
        "success": true,
        "barcode": null,
        "message": "No scanner connected"
    }))
}

async fn scanner_status() -> Json<Value> {
    Json(json!({
        "success": true,
        "connected": false,
        "message": "Scanner service stub — full implementation in future sprint"
    }))
}
