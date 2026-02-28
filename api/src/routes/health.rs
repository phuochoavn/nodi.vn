use axum::{Router, Json, routing::{get, post}, extract::{State, Query}};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;

const LATEST_VERSION: &str = "1.0.0";
const DOWNLOAD_URL: &str = "https://nodi.vn/download/NodiPOS_1.0.0_x64-setup.exe";
const RELEASE_NOTES: &str = "";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/check-update", get(check_update))
        .route("/api/update/check", get(update_check).post(update_check_post))
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    let uptime = state.start_time.elapsed().as_secs();
    Json(json!({
        "status": "ok",
        "version": "0.1.0",
        "uptime": uptime
    }))
}

// Legacy endpoint — kept for backward compatibility
async fn check_update() -> Json<Value> {
    Json(json!({
        "version": LATEST_VERSION,
        "download_url": DOWNLOAD_URL,
        "release_notes": RELEASE_NOTES,
        "is_mandatory": false
    }))
}

// GET /api/update/check?current_version=x.y.z
#[derive(Deserialize)]
struct UpdateCheckQuery {
    current_version: Option<String>,
}

async fn update_check(Query(q): Query<UpdateCheckQuery>) -> Json<Value> {
    build_update_response(q.current_version.unwrap_or_default())
}

// POST /api/update/check — { "current_version": "x.y.z" }
#[derive(Deserialize)]
struct UpdateCheckBody {
    current_version: Option<String>,
}

async fn update_check_post(Json(body): Json<UpdateCheckBody>) -> Json<Value> {
    build_update_response(body.current_version.unwrap_or_default())
}

fn build_update_response(current: String) -> Json<Value> {
    let has_update = version_cmp(&current, LATEST_VERSION);

    if has_update {
        Json(json!({
            "has_update": true,
            "latest_version": LATEST_VERSION,
            "download_url": DOWNLOAD_URL,
            "release_notes": RELEASE_NOTES
        }))
    } else {
        Json(json!({
            "has_update": false,
            "latest_version": LATEST_VERSION,
            "download_url": null,
            "release_notes": null
        }))
    }
}

/// Returns true if `current` < `latest` using semver-style comparison
fn version_cmp(current: &str, latest: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.').filter_map(|s| s.parse().ok()).collect()
    };
    let c = parse(current);
    let l = parse(latest);
    for i in 0..l.len().max(c.len()) {
        let cv = c.get(i).copied().unwrap_or(0);
        let lv = l.get(i).copied().unwrap_or(0);
        if cv < lv { return true; }
        if cv > lv { return false; }
    }
    false
}

