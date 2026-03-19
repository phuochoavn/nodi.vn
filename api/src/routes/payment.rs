use axum::{Router, Json, routing::{get, post}, extract::{State, Path}};
use serde::Deserialize;
use serde_json::{json, Value};
use rand::Rng;

use crate::AppState;
use crate::error::AppError;

fn simple_url_encode(s: &str) -> String {
    s.replace(' ', "%20")
}

const BANK_CODE: &str = "VCB";
const ACCOUNT_NO: &str = "3374222326";
const ACCOUNT_NAME: &str = "NGUYEN PHUOC HOA";

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/payment/create-order", post(create_order))
        .route("/api/payment/check/{order_code}", get(check_order))
        .route("/api/payment/webhook", post(webhook))
}

/// Create orders table on startup
pub async fn init_table(pool: &sqlx::PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (
            id SERIAL PRIMARY KEY,
            order_code VARCHAR(20) UNIQUE NOT NULL,
            plan VARCHAR(10) NOT NULL,
            amount INTEGER NOT NULL,
            customer_name VARCHAR(255),
            customer_phone VARCHAR(20) NOT NULL,
            customer_email VARCHAR(255),
            status VARCHAR(20) DEFAULT 'PENDING',
            license_key VARCHAR(50),
            paid_at TIMESTAMPTZ,
            expired_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )"
    ).execute(pool).await.ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_code ON orders(order_code)")
        .execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)")
        .execute(pool).await.ok();

    tracing::info!("✅ orders table ready");
}

fn generate_order_code() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789".chars().collect();
    let code: String = (0..8).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("NODI{}", code)
}

fn generate_license_key() -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let segment = |r: &mut rand::rngs::ThreadRng| -> String {
        (0..4).map(|_| chars[r.gen_range(0..chars.len())]).collect()
    };
    format!("NODI-{}-{}-{}", segment(&mut rng), segment(&mut rng), segment(&mut rng))
}

// ============================================================
// POST /api/payment/create-order
// ============================================================
#[derive(Deserialize)]
struct CreateOrderBody {
    plan: String,              // "MONTHLY" | "YEARLY"
    customer_name: Option<String>,
    customer_phone: String,
    customer_email: Option<String>,
}

async fn create_order(
    State(state): State<AppState>,
    Json(body): Json<CreateOrderBody>,
) -> Result<Json<Value>, AppError> {
    let plan = body.plan.to_uppercase();
    if plan != "MONTHLY" && plan != "YEARLY" {
        return Err(AppError::BadRequest("Plan must be MONTHLY or YEARLY".into()));
    }
    if body.customer_phone.is_empty() {
        return Err(AppError::BadRequest("Phone number is required".into()));
    }

    let amount = match plan.as_str() {
        "YEARLY" => 1_990_000,
        _ => 299_000,
    };

    // Generate unique order code
    let mut order_code = generate_order_code();
    for _ in 0..10 {
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM orders WHERE order_code = $1)")
            .bind(&order_code).fetch_one(&state.pool).await.unwrap_or((false,));
        if !exists.0 { break; }
        order_code = generate_order_code();
    }

    // Expire after 30 minutes
    let expired_at = chrono::Utc::now() + chrono::Duration::minutes(30);

    sqlx::query(
        "INSERT INTO orders (order_code, plan, amount, customer_name, customer_phone, customer_email, status, expired_at)
         VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', $7)"
    )
    .bind(&order_code)
    .bind(&plan)
    .bind(amount)
    .bind(&body.customer_name)
    .bind(&body.customer_phone)
    .bind(&body.customer_email)
    .bind(expired_at)
    .execute(&state.pool).await?;

    // Build VietQR URL
    let qr_url = format!(
        "https://img.vietqr.io/image/{}-{}-compact2.png?amount={}&addInfo={}&accountName={}",
        BANK_CODE, ACCOUNT_NO, amount,
        simple_url_encode(&order_code),
        simple_url_encode(ACCOUNT_NAME)
    );

    tracing::info!("📦 Order created: {} — {} — {}đ — {}", order_code, plan, amount, body.customer_phone);

    Ok(Json(json!({
        "success": true,
        "order_code": order_code,
        "plan": plan,
        "amount": amount,
        "amount_formatted": if plan == "YEARLY" { "1.990.000đ" } else { "299.000đ" },
        "qr_url": qr_url,
        "bank": {
            "bank_name": "Vietcombank",
            "account_no": ACCOUNT_NO,
            "account_name": ACCOUNT_NAME,
            "transfer_content": order_code
        },
        "expires_in_minutes": 30,
        "expired_at": expired_at.to_rfc3339()
    })))
}

// ============================================================
// GET /api/payment/check/:order_code
// ============================================================
async fn check_order(
    State(state): State<AppState>,
    Path(order_code): Path<String>,
) -> Result<Json<Value>, AppError> {
    let row = sqlx::query_as::<_, (String, i32, String, Option<String>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT plan, amount, status, license_key, paid_at, expired_at FROM orders WHERE order_code = $1"
    )
    .bind(&order_code)
    .fetch_optional(&state.pool).await?;

    match row {
        Some((plan, amount, status, license_key, paid_at, expired_at)) => {
            // Check if expired
            let now = chrono::Utc::now();
            let actual_status = if status == "PENDING" {
                if let Some(exp) = expired_at {
                    if now > exp { "EXPIRED" } else { "PENDING" }
                } else { "PENDING" }
            } else {
                &status
            };

            Ok(Json(json!({
                "found": true,
                "order_code": order_code,
                "plan": plan,
                "amount": amount,
                "status": actual_status,
                "license_key": license_key,
                "paid_at": paid_at.map(|d| d.to_rfc3339()),
            })))
        }
        None => Ok(Json(json!({
            "found": false,
            "order_code": order_code,
            "status": "NOT_FOUND"
        })))
    }
}

// ============================================================
// POST /api/payment/webhook — Casso/SePay webhook
// ============================================================
// Casso sends: { "data": [{ "description": "...", "amount": 299000, ... }] }
// SePay sends: { "transferType": "in", "content": "NODI...", "transferAmount": 299000 }
#[derive(Deserialize)]
struct WebhookPayload {
    // Casso format
    data: Option<Vec<CassoTransaction>>,
    // SePay format
    content: Option<String>,
    #[serde(alias = "transferAmount")]
    transfer_amount: Option<i64>,
    #[serde(alias = "transferType")]
    transfer_type: Option<String>,
}

#[derive(Deserialize)]
struct CassoTransaction {
    description: Option<String>,
    amount: Option<i64>,
}

async fn webhook(
    State(state): State<AppState>,
    Json(body): Json<WebhookPayload>,
) -> Result<Json<Value>, AppError> {
    let mut matched = 0;

    // Try Casso format
    if let Some(transactions) = &body.data {
        for txn in transactions {
            if let (Some(desc), Some(amt)) = (&txn.description, txn.amount) {
                if let Some(code) = extract_order_code(desc) {
                    if try_confirm_order(&state.pool, &code, amt).await {
                        matched += 1;
                    }
                }
            }
        }
    }

    // Try SePay format
    if let (Some(content), Some(amt)) = (&body.content, body.transfer_amount) {
        if body.transfer_type.as_deref() == Some("in") {
            if let Some(code) = extract_order_code(content) {
                if try_confirm_order(&state.pool, &code, amt).await {
                    matched += 1;
                }
            }
        }
    }

    tracing::info!("💰 Webhook received — matched {} orders", matched);

    Ok(Json(json!({
        "success": true,
        "matched_orders": matched
    })))
}

/// Extract NODI order code from bank transfer description
fn extract_order_code(text: &str) -> Option<String> {
    // Look for NODI followed by 8 alphanumeric chars
    let upper = text.to_uppercase().replace(' ', "");
    if let Some(pos) = upper.find("NODI") {
        let candidate = &upper[pos..];
        if candidate.len() >= 12 {
            let code = &candidate[..12]; // NODI + 8 chars
            // Validate it's alphanumeric
            if code.chars().all(|c| c.is_alphanumeric()) {
                return Some(code.to_string());
            }
        }
    }
    None
}

/// Try to confirm an order and auto-create a license
async fn try_confirm_order(pool: &sqlx::PgPool, order_code: &str, amount: i64) -> bool {
    // Find pending order matching code + amount
    let row = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, plan, customer_phone FROM orders WHERE order_code = $1 AND status = 'PENDING' AND amount = $2"
    )
    .bind(order_code)
    .bind(amount as i32)
    .fetch_optional(pool).await;

    let (order_id, plan, phone) = match row {
        Ok(Some(r)) => r,
        _ => return false,
    };

    // Generate license key
    let mut key = generate_license_key();
    for _ in 0..10 {
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM stores WHERE license_key = $1)")
            .bind(&key).fetch_one(pool).await.unwrap_or((false,));
        if !exists.0 { break; }
        key = generate_license_key();
    }

    // Calculate duration: +30 days trial included
    let days = match plan.as_str() {
        "YEARLY" => 365 + 30,  // 1 năm + 1 tháng trial
        _ => 30 + 30,          // 1 tháng + 1 tháng trial
    };

    let store_name = format!("Store-{}", &phone[phone.len().saturating_sub(4)..]);

    // Create store/license
    let store_result = sqlx::query(
        "INSERT INTO stores (name, license_key, license_type, is_active, owner_name, license_expires_at, created_at, duration_days)
         VALUES ($1, $2, $3, true, $1, NOW() + ($4 || ' days')::interval, NOW(), $4)"
    )
    .bind(&store_name)
    .bind(&key)
    .bind(if plan == "YEARLY" { "YEARLY" } else { "MONTHLY" })
    .bind(days.to_string())
    .execute(pool).await;

    if store_result.is_err() {
        tracing::error!("❌ Failed to create license for order {}: {:?}", order_code, store_result);
        return false;
    }

    // Update order: PAID + license_key
    sqlx::query("UPDATE orders SET status = 'PAID', license_key = $1, paid_at = NOW() WHERE id = $2")
        .bind(&key)
        .bind(order_id)
        .execute(pool).await.ok();

    tracing::info!("✅ Order {} confirmed — License {} created for {}", order_code, key, phone);
    true
}
