use axum::{Router, Json, routing::{post, get}, extract::{State, Multipart, Query}};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::{json, Value};
use chrono::Utc;

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/backup/upload", post(upload_backup))
        .route("/api/backup/list", get(list_backups))
        .route("/api/backup/download", get(download_backup))
}

// ============================================================
// POST /api/backup/upload (multipart)
// ============================================================

async fn upload_backup(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut identifier: Option<String> = None;
    let mut backup_type = "backup".to_string();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                file_data = Some(
                    field.bytes().await
                        .map_err(|e| AppError::Internal(format!("File read error: {}", e)))?
                        .to_vec()
                );
            }
            "license_key" => {
                identifier = Some(
                    field.text().await
                        .map_err(|e| AppError::Internal(format!("Field read error: {}", e)))?
                );
            }
            "type" => {
                backup_type = field.text().await.unwrap_or_else(|_| "backup".into());
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or(AppError::Internal("No file uploaded".into()))?;
    let identifier = identifier.ok_or(AppError::Unauthorized("No license_key provided".into()))?;

    // Identify store (accepts license_key OR HWID)
    let store = sqlx::query_as::<_, (i32, Option<String>)>(
        "SELECT id, license_type FROM stores WHERE (license_key = $1 OR hwid = $1) AND is_active = true"
    )
    .bind(&identifier)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("Invalid license key or HWID".into()))?;

    let store_id = store.0;
    let license_type = store.1.unwrap_or_else(|| "free".to_string());

    // Quota check based on license type
    let (max_backups, max_file_size): (i64, usize) = match license_type.as_str() {
        "free" => (3, 50 * 1024 * 1024),      // 3 backups, 50MB each
        _ => (10, 200 * 1024 * 1024),           // 10 backups, 200MB each
    };

    let file_data_len = file_data.len();
    if file_data_len > max_file_size {
        return Err(AppError::BadRequest(format!(
            "File quá lớn. Giới hạn: {}MB cho gói {}",
            max_file_size / (1024 * 1024), license_type
        )));
    }

    // Check backup count quota
    let current_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM backup_files WHERE store_id = $1"
    )
    .bind(store_id)
    .fetch_one(&state.pool)
    .await?;

    if current_count >= max_backups {
        return Err(AppError::BadRequest(format!(
            "Đã đạt giới hạn {} bản backup cho gói {}. Vui lòng xóa backup cũ hoặc nâng cấp gói.",
            max_backups, license_type
        )));
    }

    // Create directory
    let dir = format!("/opt/nodi/backups/{}", store_id);
    tokio::fs::create_dir_all(&dir).await
        .map_err(|e| AppError::Internal(format!("Dir create error: {}", e)))?;

    // Save file
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("backup_{}.db", timestamp);
    let filepath = format!("{}/{}", dir, filename);
    let file_size = file_data.len() as i64;

    tokio::fs::write(&filepath, &file_data).await
        .map_err(|e| AppError::Internal(format!("File write error: {}", e)))?;

    // Insert into backup_files
    sqlx::query(
        "INSERT INTO backup_files (store_id, filename, file_size, file_path, backup_type) VALUES ($1,$2,$3,$4,$5)"
    )
    .bind(store_id).bind(&filename).bind(file_size).bind(&filepath).bind(&backup_type)
    .execute(&state.pool)
    .await?;

    // Rotation: keep max N per license type
    cleanup_old_backups(store_id, max_backups, &state.pool).await;

    let url = format!("/backups/{}/{}", identifier, filename);
    tracing::info!("✅ Backup uploaded: store_id={}, file={}, size={}", store_id, filename, file_size);

    Ok(Json(json!({
        "success": true,
        "url": url
    })))
}

// ============================================================
// GET /api/backup/list (JWT required)
// ============================================================

#[derive(Deserialize)]
pub struct BackupQuery {
    pub store_id: Option<i32>,
}

async fn list_backups(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<BackupQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_jwt(&headers, &state.config.jwt_secret)?;

    // Get store_id: admin can specify, others use their own
    let store_id = if claims.role == "admin" && query.store_id.is_some() {
        query.store_id.unwrap()
    } else {
        sqlx::query_scalar::<_, Option<i32>>("SELECT store_id FROM users WHERE id = $1")
            .bind(claims.sub)
            .fetch_one(&state.pool)
            .await?
            .ok_or(AppError::Unauthorized("User has no store".into()))?
    };

    let rows = sqlx::query_as::<_, (i32, String, i64, chrono::NaiveDateTime)>(
        "SELECT id, filename, file_size, created_at FROM backup_files WHERE store_id = $1 ORDER BY created_at DESC"
    )
    .bind(store_id)
    .fetch_all(&state.pool)
    .await?;

    let backups: Vec<Value> = rows.iter().map(|(id, filename, size, created_at)| {
        json!({
            "id": id,
            "filename": filename,
            "size_bytes": size,
            "created_at": created_at.and_utc().to_rfc3339()
        })
    }).collect();

    Ok(Json(json!({ "backups": backups })))
}

// ============================================================
// GET /api/backup/download (JWT required)
// ============================================================

async fn download_backup(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<BackupQuery>,
) -> Result<impl IntoResponse, AppError> {
    let claims = extract_jwt(&headers, &state.config.jwt_secret)?;

    let store_id = if claims.role == "admin" && query.store_id.is_some() {
        query.store_id.unwrap()
    } else {
        sqlx::query_scalar::<_, Option<i32>>("SELECT store_id FROM users WHERE id = $1")
            .bind(claims.sub)
            .fetch_one(&state.pool)
            .await?
            .ok_or(AppError::Unauthorized("User has no store".into()))?
    };

    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT filename, file_path FROM backup_files WHERE store_id = $1 ORDER BY created_at DESC LIMIT 1"
    )
    .bind(store_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound("No backup found".into()))?;

    let (filename, file_path) = row;
    let data = tokio::fs::read(&file_path).await
        .map_err(|e| AppError::Internal(format!("File read error: {}", e)))?;

    Ok((
        [
            ("Content-Type", "application/octet-stream".to_string()),
            ("Content-Disposition", format!("attachment; filename=\"{}\"", filename)),
        ],
        data,
    ))
}

// ============================================================
// Helpers
// ============================================================

fn extract_jwt(headers: &HeaderMap, secret: &str) -> Result<auth::Claims, AppError> {
    let header = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = header.strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized("Invalid Authorization format".into()))?;
    auth::verify_token(token, secret)
}

async fn cleanup_old_backups(store_id: i32, max_backups: i64, pool: &sqlx::PgPool) {
    // Keep max N backups per store (N depends on license type)
    let old_files = sqlx::query_as::<_, (i32, String)>(
        &format!("SELECT id, file_path FROM backup_files WHERE store_id = $1 ORDER BY created_at DESC OFFSET {}", max_backups)
    )
    .bind(store_id)
    .fetch_all(pool)
    .await;

    if let Ok(files) = old_files {
        for (id, path) in files {
            let _ = tokio::fs::remove_file(&path).await;
            let _ = sqlx::query("DELETE FROM backup_files WHERE id = $1")
                .bind(id).execute(pool).await;
        }
    }
}
