use axum::{Router, Json, routing::{get, post}, extract::{State, Query}};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::AppState;

const CONFIG_PATH: &str = "/opt/nodi/downloads/update_config.json";
const DEFAULT_VERSION: &str = "1.0.0";
const DEFAULT_URL: &str = "https://nodi.vn/download/NodiPOS_1.0.0_x64-setup.exe";

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateConfig {
    pub latest_version: String,
    pub download_url: String,
    pub release_notes: String,
    #[serde(default)]
    pub file_size: String,
    #[serde(default)]
    pub updated_at: String,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            latest_version: DEFAULT_VERSION.to_string(),
            download_url: DEFAULT_URL.to_string(),
            release_notes: String::new(),
            file_size: String::new(),
            updated_at: String::new(),
        }
    }
}

/// Read update config from JSON file, fallback to defaults  
pub fn read_update_config() -> UpdateConfig {
    match std::fs::read_to_string(CONFIG_PATH) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => UpdateConfig::default(),
    }
}

/// Write update config to JSON file
pub fn write_update_config(config: &UpdateConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("JSON serialize error: {}", e))?;
    std::fs::write(CONFIG_PATH, json)
        .map_err(|e| format!("File write error: {}", e))?;
    Ok(())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/check-update", get(check_update))
        .route("/api/update/check", get(update_check).post(update_check_post))
        .route("/api/downloads/info", get(downloads_info))
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
    let config = read_update_config();
    Json(json!({
        "version": config.latest_version,
        "download_url": config.download_url,
        "release_notes": config.release_notes,
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
    let config = read_update_config();
    let has_update = version_cmp(&current, &config.latest_version);

    if has_update {
        Json(json!({
            "has_update": true,
            "latest_version": config.latest_version,
            "download_url": config.download_url,
            "release_notes": config.release_notes,
            "file_size": config.file_size
        }))
    } else {
        Json(json!({
            "has_update": false,
            "latest_version": config.latest_version,
            "download_url": null,
            "release_notes": null,
            "file_size": null
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

// ============================================================
// GET /api/downloads/info — All platforms download info
// ============================================================
async fn downloads_info() -> Json<Value> {
    // Read raw JSON to include android/ios sections
    match std::fs::read_to_string(CONFIG_PATH) {
        Ok(content) => {
            match serde_json::from_str::<Value>(&content) {
                Ok(val) => Json(val),
                Err(_) => {
                    let config = read_update_config();
                    Json(json!({
                        "latest_version": config.latest_version,
                        "download_url": config.download_url,
                        "release_notes": config.release_notes,
                        "file_size": config.file_size
                    }))
                }
            }
        },
        Err(_) => Json(json!({
            "latest_version": DEFAULT_VERSION,
            "download_url": DEFAULT_URL,
            "release_notes": "",
            "file_size": ""
        }))
    }
}
