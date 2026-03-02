use axum::{Router, Json, extract::{State, Query}, routing::{get, put, post}};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{verify_token, Claims};

fn extract_claims(headers: &HeaderMap, secret: &str) -> Result<Claims, AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    verify_token(token, secret)
}

fn get_store_id(claims: &Claims, query_store_id: Option<i32>) -> i32 {
    if claims.role == "admin" {
        query_store_id.unwrap_or(claims.store_id)
    } else {
        claims.store_id
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/dashboard/einvoice/config", get(get_config))
        .route("/api/dashboard/einvoice/config", put(update_config))
        .route("/api/dashboard/einvoice/issue", post(issue_invoice))
}

#[derive(Deserialize)]
struct StoreQuery {
    store_id: Option<i32>,
}

// ===== GET /api/dashboard/einvoice/config =====
async fn get_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<StoreQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);

    let row: Option<(String, Option<String>, Option<String>, Option<String>, Option<String>, bool)> = sqlx::query_as(
        "SELECT provider, api_key, api_secret, tax_code, series_symbol, is_active FROM einvoice_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;

    let tax_row: Option<(String, f64, f64, String)> = sqlx::query_as(
        "SELECT business_type, vat_rate::float8, pit_rate::float8, tax_period FROM tax_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;

    match row {
        Some((provider, api_key, _api_secret, tax_code, series_symbol, is_active)) => {
            let tax_config = tax_row.map(|(bt, vr, pr, tp)| json!({
                "business_type": bt,
                "vat_rate": vr,
                "pit_rate": pr,
                "tax_period": tp,
            })).unwrap_or(json!({
                "business_type": "retail",
                "vat_rate": 1.0,
                "pit_rate": 0.5,
                "tax_period": "quarterly",
            }));

            Ok(Json(json!({
                "einvoice": {
                    "provider": provider,
                    "has_api_key": api_key.is_some(),
                    "tax_code": tax_code,
                    "series_symbol": series_symbol,
                    "is_active": is_active,
                },
                "tax": tax_config,
                "status": if is_active { "active" } else { "inactive" },
            })))
        },
        None => {
            Ok(Json(json!({
                "einvoice": {
                    "provider": "none",
                    "has_api_key": false,
                    "tax_code": null,
                    "series_symbol": null,
                    "is_active": false,
                },
                "tax": {
                    "business_type": "retail",
                    "vat_rate": 1.0,
                    "pit_rate": 0.5,
                    "tax_period": "quarterly",
                },
                "status": "not_configured",
            })))
        }
    }
}

// ===== PUT /api/dashboard/einvoice/config =====
#[derive(Deserialize)]
struct UpdateConfigRequest {
    // E-Invoice config
    provider: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
    tax_code: Option<String>,
    series_symbol: Option<String>,
    is_active: Option<bool>,
    // Tax config
    business_type: Option<String>,
    vat_rate: Option<f64>,
    pit_rate: Option<f64>,
    tax_period: Option<String>,
}

async fn update_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UpdateConfigRequest>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id; // Only store owner can update their own config

    // Upsert einvoice_config
    sqlx::query(
        "INSERT INTO einvoice_config (store_id, provider, api_key, api_secret, tax_code, series_symbol, is_active, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW()) \
         ON CONFLICT (store_id) DO UPDATE SET \
         provider = COALESCE($2, einvoice_config.provider), \
         api_key = COALESCE($3, einvoice_config.api_key), \
         api_secret = COALESCE($4, einvoice_config.api_secret), \
         tax_code = COALESCE($5, einvoice_config.tax_code), \
         series_symbol = COALESCE($6, einvoice_config.series_symbol), \
         is_active = COALESCE($7, einvoice_config.is_active), \
         updated_at = NOW()"
    )
    .bind(sid)
    .bind(&req.provider)
    .bind(&req.api_key)
    .bind(&req.api_secret)
    .bind(&req.tax_code)
    .bind(&req.series_symbol)
    .bind(req.is_active)
    .execute(&state.pool).await?;

    // Upsert tax_config
    sqlx::query(
        "INSERT INTO tax_config (store_id, business_type, vat_rate, pit_rate, tax_period, updated_at) \
         VALUES ($1, $2, $3, $4, $5, NOW()) \
         ON CONFLICT (store_id) DO UPDATE SET \
         business_type = COALESCE($2, tax_config.business_type), \
         vat_rate = COALESCE($3, tax_config.vat_rate), \
         pit_rate = COALESCE($4, tax_config.pit_rate), \
         tax_period = COALESCE($5, tax_config.tax_period), \
         updated_at = NOW()"
    )
    .bind(sid)
    .bind(&req.business_type)
    .bind(req.vat_rate)
    .bind(req.pit_rate)
    .bind(&req.tax_period)
    .execute(&state.pool).await?;

    tracing::info!("📋 E-Invoice config updated: store_id={}", sid);

    Ok(Json(json!({
        "success": true,
        "message": "Cập nhật cấu hình thành công"
    })))
}

// ===== POST /api/dashboard/einvoice/issue =====
#[derive(Deserialize)]
struct IssueInvoiceRequest {
    invoice_id: Option<i64>,
    invoice_number: Option<String>,
}

async fn issue_invoice(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<IssueInvoiceRequest>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = claims.store_id;

    // Check if E-Invoice is configured
    let config: Option<(String, bool)> = sqlx::query_as(
        "SELECT provider, is_active FROM einvoice_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;

    match config {
        None => {
            return Err(AppError::BadRequest("Chưa cấu hình hóa đơn điện tử. Vui lòng cấu hình trước.".into()));
        }
        Some((provider, is_active)) => {
            if !is_active {
                return Err(AppError::BadRequest("Hóa đơn điện tử chưa được kích hoạt.".into()));
            }
            if provider == "none" {
                return Err(AppError::BadRequest("Chưa chọn nhà cung cấp hóa đơn điện tử.".into()));
            }

            // Placeholder: log the request, do not actually call provider API
            tracing::info!(
                "🧾 E-Invoice issue request: store_id={}, provider={}, invoice_id={:?}, invoice_number={:?}",
                sid, provider, req.invoice_id, req.invoice_number
            );

            // In the future, this is where we'd call provider API:
            // match provider.as_str() {
            //     "vnpt" => call_vnpt_api(...)
            //     "viettel" => call_viettel_api(...)
            //     "misa" => call_misa_api(...)
            // }

            Ok(Json(json!({
                "success": true,
                "message": format!("Yêu cầu phát hành HĐĐT đã được ghi nhận (provider: {}). Chức năng sẽ hoạt động khi kết nối API nhà cung cấp.", provider),
                "provider": provider,
                "invoice_id": req.invoice_id,
                "invoice_number": req.invoice_number,
                "status": "pending_integration",
            })))
        }
    }
}
