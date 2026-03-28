//! Sprint 180: Firebase Cloud Messaging (FCM) — Data Messages
//!
//! Sends FCM data messages to background devices when push succeeds.
//! Uses FCM HTTP v1 API with service account authentication.
//! Gracefully skips if no Firebase config is available.

use sqlx::PgPool;
use serde_json::json;

/// FCM HTTP v1 endpoint template
const FCM_SEND_URL: &str = "https://fcm.googleapis.com/v1/projects/{PROJECT_ID}/messages:send";

/// Send FCM data messages to all devices in the same store EXCEPT the source device.
/// This is fire-and-forget — errors are logged but don't affect the push response.
pub async fn send_data_message(
    pool: &PgPool,
    store_id: i32,
    source_device: &str,
    tables_changed: Vec<String>,
) {
    // 1. Check if FCM is configured
    let project_id = match std::env::var("FCM_PROJECT_ID") {
        Ok(id) if !id.is_empty() => id,
        _ => {
            tracing::debug!("FCM not configured (FCM_PROJECT_ID not set), skipping");
            return;
        }
    };

    let server_key = match std::env::var("FCM_SERVER_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => {
            tracing::debug!("FCM not configured (FCM_SERVER_KEY not set), skipping");
            return;
        }
    };

    // 2. Get FCM tokens for other devices in this store
    let tokens: Vec<(String, String)> = match sqlx::query_as::<_, (String, String)>(
        "SELECT device_id, fcm_token FROM sync_devices \
         WHERE store_id = $1 AND device_id != $2 AND fcm_token IS NOT NULL AND fcm_token != ''"
    )
    .bind(store_id)
    .bind(source_device)
    .fetch_all(pool)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            tracing::warn!("FCM: Failed to query device tokens: {}", e);
            return;
        }
    };

    if tokens.is_empty() {
        tracing::debug!("FCM: No other devices with FCM tokens for store_id={}", store_id);
        return;
    }

    // 3. Build FCM URL
    let url = FCM_SEND_URL.replace("{PROJECT_ID}", &project_id);
    let tables_str = tables_changed.join(",");

    // 4. Send to each device
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("FCM: Failed to build HTTP client: {}", e);
            return;
        }
    };

    let mut sent = 0usize;
    let mut failed = 0usize;

    for (device_id, fcm_token) in &tokens {
        let body = json!({
            "message": {
                "token": fcm_token,
                "data": {
                    "type": "DATA_CHANGED",
                    "tables": &tables_str,
                    "source_device": source_device,
                    "store_id": store_id.to_string()
                },
                // Android high priority — wake device from Doze
                "android": {
                    "priority": "high"
                }
            }
        });

        let result = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", server_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    sent += 1;
                } else {
                    let error_body = resp.text().await.unwrap_or_default();
                    tracing::warn!(
                        "FCM: Failed to send to device={}, status={}, body={}",
                        device_id, status, error_body
                    );
                    // If token is invalid (404/400), remove it
                    if status.as_u16() == 404 || error_body.contains("NOT_FOUND") || error_body.contains("UNREGISTERED") {
                        let _ = sqlx::query(
                            "UPDATE sync_devices SET fcm_token = NULL WHERE store_id = $1 AND device_id = $2"
                        )
                        .bind(store_id)
                        .bind(device_id)
                        .execute(pool)
                        .await;
                        tracing::info!("FCM: Removed stale token for device={}", device_id);
                    }
                    failed += 1;
                }
            }
            Err(e) => {
                tracing::warn!("FCM: HTTP error sending to device={}: {}", device_id, e);
                failed += 1;
            }
        }
    }

    if sent > 0 || failed > 0 {
        tracing::info!(
            "FCM: store_id={}, sent={}, failed={}, tables=[{}]",
            store_id, sent, failed, tables_str
        );
    }
}
