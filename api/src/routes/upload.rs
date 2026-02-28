use axum::{Router, Json, routing::post, extract::{State, Multipart}};
use serde_json::{json, Value};
use chrono::Utc;
use uuid::Uuid;

use crate::AppState;
use crate::error::AppError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/upload", post(handle_upload))
}

// ============================================================
// POST /api/upload (multipart)
// Fields: file, type (backup|product_image), license_key
// ============================================================

async fn handle_upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut license_key: Option<String> = None;
    let mut upload_type = "product_image".to_string();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                file_name = field.file_name().map(|n| n.to_string());
                file_data = Some(
                    field.bytes().await
                        .map_err(|e| AppError::Internal(format!("File read error: {}", e)))?
                        .to_vec()
                );
            }
            "license_key" => {
                license_key = Some(
                    field.text().await
                        .map_err(|e| AppError::Internal(format!("Field read error: {}", e)))?
                );
            }
            "type" => {
                upload_type = field.text().await.unwrap_or_else(|_| "product_image".into());
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or(AppError::BadRequest("No file uploaded".into()))?;
    let license_key = license_key.ok_or(AppError::Unauthorized("No license_key provided".into()))?;

    // Validate license_key
    let store = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM stores WHERE (license_key = $1 OR hwid = $1) AND is_active = true"
    )
    .bind(&license_key)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized("Invalid license key or HWID".into()))?;

    let store_id = store.0;

    match upload_type.as_str() {
        "backup" => {
            // Save backup file to /opt/nodi/backups/{license_key}/
            let dir = format!("/opt/nodi/backups/{}", license_key);
            tokio::fs::create_dir_all(&dir).await
                .map_err(|e| AppError::Internal(format!("Dir create error: {}", e)))?;

            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
            let orig_name = file_name.unwrap_or_else(|| "backup.db.gz".into());
            let ext = orig_name.rsplit('.').next().unwrap_or("gz");
            let filename = format!("backup_{}.{}", timestamp, ext);
            let filepath = format!("{}/{}", dir, filename);

            tokio::fs::write(&filepath, &file_data).await
                .map_err(|e| AppError::Internal(format!("File write error: {}", e)))?;

            let url = format!("https://api.nodi.vn/backups/{}/{}", license_key, filename);
            tracing::info!("✅ Upload (backup): store_id={}, file={}, size={}", store_id, filename, file_data.len());

            Ok(Json(json!({
                "success": true,
                "url": url,
                "filename": filename,
                "size": file_data.len()
            })))
        }
        _ => {
            // product_image or other — save to /opt/nodi/uploads/
            let dir = "/opt/nodi/uploads";
            tokio::fs::create_dir_all(dir).await
                .map_err(|e| AppError::Internal(format!("Dir create error: {}", e)))?;

            let orig_name = file_name.unwrap_or_else(|| "image.jpg".into());
            let ext = orig_name.rsplit('.').next().unwrap_or("jpg");
            let filename = format!("{}_{}.{}", Uuid::new_v4(), Utc::now().format("%Y%m%d%H%M%S"), ext);
            let filepath = format!("{}/{}", dir, filename);

            tokio::fs::write(&filepath, &file_data).await
                .map_err(|e| AppError::Internal(format!("File write error: {}", e)))?;

            let url = format!("https://api.nodi.vn/uploads/{}", filename);
            tracing::info!("✅ Upload (image): store_id={}, file={}, size={}", store_id, filename, file_data.len());

            Ok(Json(json!({
                "success": true,
                "url": url,
                "filename": filename,
                "size": file_data.len()
            })))
        }
    }
}
