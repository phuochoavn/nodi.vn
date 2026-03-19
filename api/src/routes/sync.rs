use axum::{Router, Json, routing::{post, get}, extract::State};
use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::AppState;
use crate::error::AppError;

// ============================================================
// Sync Payload Structs (14 collections)
// ============================================================

#[derive(Deserialize)]
pub struct SyncPayload {
    #[serde(default)] pub customers: Vec<SyncCustomer>,
    #[serde(default)] pub products: Vec<SyncProduct>,
    #[serde(default)] pub orders: Vec<SyncOrder>,
    #[serde(default)] pub suppliers: Vec<SyncSupplier>,
    #[serde(default)] pub purchase_orders: Vec<SyncPurchaseOrder>,
    #[serde(default)] pub customer_transactions: Vec<SyncCustomerTransaction>,
    #[serde(default)] pub supplier_transactions: Vec<SyncSupplierTransaction>,
    #[serde(default)] pub product_units: Vec<SyncProductUnit>,
    pub store_funds: Option<SyncStoreFunds>,
    #[serde(default)] pub cash_transactions: Vec<SyncCashTransaction>,
    #[serde(default)] pub product_batches: Vec<SyncProductBatch>,
    #[serde(default)] pub payment_vouchers: Vec<SyncPaymentVoucher>,
    pub store_settings: Option<SyncStoreSettings>,
    #[serde(default)] pub product_transactions: Vec<SyncProductTransaction>,
    #[serde(default)] pub staff_members: Vec<SyncStaffMember>,
    #[serde(default)] pub loyalty_transactions: Vec<SyncLoyaltyTransaction>,
    pub loyalty_settings: Option<SyncLoyaltySettings>,
    // Sprint 97: new tables
    #[serde(default)] pub promotions: Vec<SyncPromotion>,
    #[serde(default)] pub vouchers: Vec<SyncVoucher>,
    #[serde(default)] pub daily_closings: Vec<SyncDailyClosing>,
    #[serde(default)] pub returns: Vec<SyncReturn>,
}

#[derive(Deserialize)]
pub struct SyncCustomer {
    pub id: i64, pub name: String,
    pub phone: Option<String>, pub address: Option<String>,
    pub current_debt: Option<f64>, pub credit_limit: Option<f64>,
    pub cccd: Option<String>, pub cccd_front_img: Option<String>,
    pub cccd_back_img: Option<String>, pub created_at: Option<String>,
    pub loyalty_points: Option<i32>, pub total_spent: Option<f64>,
    pub loyalty_tier: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncProduct {
    pub id: i64, pub name: String,
    pub barcode: Option<String>, pub sku: Option<String>,
    pub description: Option<String>, pub category: Option<String>,
    pub manufacturer: Option<String>, pub base_unit: Option<String>,
    pub stock_quantity: Option<f64>, pub cost_price: Option<f64>,
    pub sell_price: Option<f64>, pub expiry_date: Option<String>,
    pub created_at: Option<String>, pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncOrder {
    pub id: i64, pub invoice_number: Option<String>,
    pub customer_id: Option<i64>, pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub total_amount: Option<f64>, pub discount_amount: Option<f64>,
    pub final_amount: Option<f64>, pub customer_pay: Option<f64>,
    pub change_money: Option<f64>, pub payment_method: Option<String>,
    pub status: Option<String>, pub notes: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)] pub items: Vec<SyncOrderItem>,
    #[serde(default)] pub payments: Vec<SyncPayment>,
}

#[derive(Deserialize)]
pub struct SyncOrderItem {
    pub id: i64, pub product_id: Option<i64>,
    pub product_name: Option<String>, pub product_sku: Option<String>,
    pub unit_name: Option<String>, pub exchange_value: Option<f64>,
    pub quantity: Option<f64>, pub unit_price: Option<f64>,
    pub subtotal: Option<f64>,
}

#[derive(Deserialize)]
pub struct SyncPayment {
    pub amount: Option<f64>, pub payment_method: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncSupplier {
    pub id: i64, pub code: Option<String>, pub name: String,
    pub phone: Option<String>, pub address: Option<String>,
    #[serde(rename = "type")] pub supplier_type: Option<String>,
    pub current_debt: Option<f64>, pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncPurchaseOrder {
    pub id: i64, pub supplier_id: Option<i64>,
    pub supplier_name: Option<String>, pub total_amount: Option<f64>,
    pub status: Option<String>, pub import_date: Option<String>,
    pub is_tax_invoice: Option<bool>, pub invoice_type: Option<String>,
    pub note: Option<String>, pub created_at: Option<String>,
    #[serde(default)] pub items: Vec<SyncPurchaseItem>,
}

#[derive(Deserialize)]
pub struct SyncPurchaseItem {
    pub id: i64, pub product_id: Option<i64>,
    pub product_name: Option<String>, pub unit: Option<String>,
    pub exchange_value: Option<f64>, pub quantity: Option<f64>,
    pub import_price: Option<f64>, pub subtotal: Option<f64>,
    pub expiry_date: Option<String>, pub batch_number: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncCustomerTransaction {
    pub id: i64, pub customer_id: Option<i64>,
    pub amount: Option<f64>, pub transaction_type: Option<String>,
    pub note: Option<String>, pub season: Option<String>,
    pub invoice_id: Option<i64>, pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncSupplierTransaction {
    pub id: i64, pub supplier_id: Option<i64>,
    pub amount: Option<f64>, pub ref_type: Option<String>,
    pub ref_id: Option<i64>, pub note: Option<String>,
    pub balance_after: Option<f64>, pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncProductUnit {
    pub id: i64, pub product_id: Option<i64>,
    pub unit_name: Option<String>, pub exchange_value: Option<f64>,
    pub price: Option<f64>, pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncStoreFunds {
    pub current_balance: Option<i64>, pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncCashTransaction {
    pub id: i64, pub amount: Option<i64>,
    pub flow_type: Option<String>, pub category: Option<String>,
    pub ref_id: Option<i64>, pub balance_after: Option<i64>,
    pub note: Option<String>, pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncProductBatch {
    pub id: i64, pub product_id: Option<i64>,
    pub purchase_item_id: Option<i64>,
    pub expiry_date: Option<String>, pub import_date: Option<String>,
    pub quantity: Option<f64>, pub remaining_quantity: Option<f64>,
    pub created_at: Option<String>, pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncPaymentVoucher {
    pub id: i64, pub voucher_code: Option<String>,
    pub supplier_id: Option<i64>, pub amount: Option<i64>,
    pub payment_method: Option<String>, pub note: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncStoreSettings {
    pub name: Option<String>, pub address: Option<String>,
    pub phone: Option<String>, pub security_pin: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncProductTransaction {
    pub id: i64, pub product_id: Option<i64>,
    pub transaction_type: Option<String>, pub quantity: Option<f64>,
    pub reference_type: Option<String>, pub reference_id: Option<i64>,
    pub note: Option<String>, pub balance_after: Option<f64>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncStaffMember {
    pub id: i64,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub role: Option<String>,
    pub pin: Option<String>,
    #[serde(default)] pub pin_set: Option<bool>,  // desktop sends this instead of pin
    pub permissions: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncLoyaltyTransaction {
    pub id: i64,
    pub customer_id: Option<i64>,
    #[serde(alias = "transaction_type")] pub r#type: Option<String>,
    pub points: Option<i32>,
    #[serde(alias = "amount")] pub ref_invoice_id: Option<i64>,
    #[serde(alias = "invoice_id")] _invoice_id_compat: Option<i64>,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncLoyaltySettings {
    #[serde(alias = "enabled")] pub is_enabled: Option<bool>,
    pub points_per_amount: Option<i32>,
    pub point_value: Option<i32>,
    #[serde(alias = "bronze_threshold")] pub tier_silver: Option<i32>,
    #[serde(alias = "silver_threshold")] pub tier_gold: Option<i32>,
    #[serde(alias = "gold_threshold")] pub tier_diamond: Option<i32>,
    #[serde(alias = "diamond_threshold")] _compat: Option<f64>,
    pub updated_at: Option<String>,
}

// Sprint 97 new structs
#[derive(Deserialize)]
pub struct SyncPromotion {
    pub id: i64, pub name: String,
    #[serde(default = "default_percent")] pub r#type: String,
    pub value: f64,
    pub min_order_value: Option<f64>, pub max_discount: Option<f64>,
    pub start_date: String, pub end_date: String,
    pub is_active: Option<bool>, pub applies_to: Option<String>,
    pub created_at: Option<String>,
}
fn default_percent() -> String { "PERCENT".to_string() }

#[derive(Deserialize)]
pub struct SyncVoucher {
    pub id: i64, pub code: String,
    pub promotion_id: Option<i64>,
    pub usage_limit: Option<i32>, pub used_count: Option<i32>,
    pub is_active: Option<bool>, pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncDailyClosing {
    pub id: i64, pub closing_date: String,
    pub total_revenue: Option<f64>, pub total_invoices: Option<i32>,
    pub total_cash_sales: Option<f64>, pub total_debt_sales: Option<f64>,
    pub total_transfer_sales: Option<f64>,
    pub expected_cash: Option<f64>, pub actual_cash: Option<f64>,
    pub difference: Option<f64>, pub total_returns: Option<f64>,
    pub total_debt_collected: Option<f64>,
    pub note: Option<String>, pub created_by: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Deserialize)]
pub struct SyncReturn {
    pub id: i64,
    pub original_invoice_id: Option<i64>,
    pub return_number: String,
    pub customer_id: Option<i64>, pub customer_name: Option<String>,
    pub total_refund: Option<f64>,
    pub refund_method: Option<String>,
    pub reason: Option<String>, pub note: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)] pub items: Vec<SyncReturnItem>,
}

#[derive(Deserialize)]
pub struct SyncReturnItem {
    pub id: i64,
    pub return_id: Option<i64>,
    pub product_id: Option<i64>,
    pub product_name: String,
    pub unit_name: String,
    pub exchange_value: Option<f64>,
    pub quantity: f64,
    pub unit_price: f64,
    pub subtotal: f64,
}

// ============================================================
// Router
// ============================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/sync", post(handle_sync))
        .route("/api/sync/pull", get(handle_pull))
}

// ============================================================
// Store identification from headers
// ============================================================

pub async fn identify_store(headers: &HeaderMap, pool: &PgPool, jwt_secret: &str) -> Result<i32, AppError> {
    // 1. JWT Authorization header (highest priority — user has account)
    if let Some(auth) = headers.get("Authorization").and_then(|v| v.to_str().ok()) {
        let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
        if !token.is_empty() {
            if let Ok(claims) = crate::middleware::auth::verify_token(token, jwt_secret) {
                tracing::debug!("Sync: store_id={} from JWT (sub={})", claims.store_id, claims.sub);
                return Ok(claims.store_id);
            }
        }
    }

    // 2. X-HWID — anonymous sync (user has no account yet)
    if let Some(hwid) = headers.get("X-HWID").and_then(|v| v.to_str().ok()) {
        if !hwid.is_empty() {
            // 2a. Lookup in stores table (existing anonymous entry)
            if let Some(id) = sqlx::query_scalar::<_, i32>(
                "SELECT id FROM stores WHERE hwid = $1 AND is_active = true"
            ).bind(hwid).fetch_optional(pool).await? {
                return Ok(id);
            }
            // 2b. Lookup in accounts table (HWID bound to an account)
            if let Some(id) = sqlx::query_scalar::<_, i32>(
                "SELECT id FROM accounts WHERE hwid = $1 AND is_active = true"
            ).bind(hwid).fetch_optional(pool).await? {
                return Ok(id + 1_000_000);
            }
            // 2c. New HWID — auto-create anonymous store entry
            let new_id = sqlx::query_scalar::<_, i32>(
                "INSERT INTO stores (hwid, name, is_active) VALUES ($1, 'Anonymous Store', true) RETURNING id"
            ).bind(hwid).fetch_one(pool).await?;
            tracing::info!("🆕 Auto-created anonymous store for HWID: store_id={}", new_id);
            return Ok(new_id);
        }
    }

    Err(AppError::Unauthorized("Missing Authorization or X-HWID header".to_string()))
}

// ============================================================
// V1→V2 Journal bridge: generate UUID + write sync_journal entry
// so that V2 Pull can discover V1-pushed data.
// ============================================================

async fn v1_journal_log(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    table_name: &str,
    pg_table: &str,
    local_id: i64,
) -> Result<(), AppError> {
    let uuid = format!("v1-{}-{}-{}", table_name, store_id, local_id);
    // Backfill uuid on the synced row (idempotent)
    let update_sql = format!(
        "UPDATE {} SET uuid = $3 WHERE store_id = $1 AND local_id = $2 AND (uuid IS NULL OR uuid = '')",
        pg_table
    );
    let _ = sqlx::query(&update_sql)
        .bind(store_id).bind(local_id as i32).bind(&uuid)
        .execute(&mut **tx).await;
    // Write journal entry (source_device_id = NULL → visible to ALL devices)
    let _ = sqlx::query(
        "INSERT INTO sync_journal (store_id, table_name, record_uuid, operation, source_device_id) \
         VALUES ($1, $2, $3, 'INSERT', NULL)"
    )
    .bind(store_id).bind(table_name).bind(&uuid)
    .execute(&mut **tx).await;
    Ok(())
}

// ============================================================
// Main sync handler
// ============================================================

async fn handle_sync(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SyncPayload>,
) -> Result<Json<Value>, AppError> {
    let store_id = identify_store(&headers, &state.pool, &state.config.jwt_secret).await?;
    let mut tx = state.pool.begin().await?;

    let mut counts = serde_json::Map::new();

    // 1. Customers (with loyalty fields)
    for c in &payload.customers {
        sqlx::query("INSERT INTO synced_customers (store_id,local_id,name,phone,address,total_debt,credit_limit,cccd,cccd_front_img,cccd_back_img,loyalty_points,total_spent,loyalty_tier,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET name=EXCLUDED.name,phone=EXCLUDED.phone,address=EXCLUDED.address,total_debt=EXCLUDED.total_debt,credit_limit=EXCLUDED.credit_limit,cccd=EXCLUDED.cccd,cccd_front_img=EXCLUDED.cccd_front_img,cccd_back_img=EXCLUDED.cccd_back_img,loyalty_points=EXCLUDED.loyalty_points,total_spent=EXCLUDED.total_spent,loyalty_tier=EXCLUDED.loyalty_tier,synced_at=NOW()")
            .bind(store_id).bind(c.id as i32).bind(&c.name).bind(&c.phone).bind(&c.address)
            .bind(c.current_debt).bind(c.credit_limit).bind(&c.cccd)
            .bind(&c.cccd_front_img).bind(&c.cccd_back_img)
            .bind(c.loyalty_points.unwrap_or(0)).bind(c.total_spent.unwrap_or(0.0))
            .bind(c.loyalty_tier.as_deref().unwrap_or("bronze"))
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "customers", "synced_customers", c.id).await?;
    }
    counts.insert("customers".into(), json!(payload.customers.len()));

    // 2. Products
    for p in &payload.products {
        sqlx::query("INSERT INTO synced_products (store_id,local_id,name,barcode,sku,description,category,manufacturer,base_unit,stock_quantity,cost_price,sell_price,expiry_date,updated_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET name=EXCLUDED.name,barcode=EXCLUDED.barcode,sku=EXCLUDED.sku,description=EXCLUDED.description,category=EXCLUDED.category,manufacturer=EXCLUDED.manufacturer,base_unit=EXCLUDED.base_unit,stock_quantity=EXCLUDED.stock_quantity,cost_price=EXCLUDED.cost_price,sell_price=EXCLUDED.sell_price,expiry_date=EXCLUDED.expiry_date,updated_at=EXCLUDED.updated_at,synced_at=NOW()")
            .bind(store_id).bind(p.id as i32).bind(&p.name).bind(&p.barcode).bind(&p.sku)
            .bind(&p.description).bind(&p.category).bind(&p.manufacturer).bind(&p.base_unit)
            .bind(p.stock_quantity).bind(p.cost_price).bind(p.sell_price)
            .bind(&p.expiry_date).bind(&p.updated_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "products", "synced_products", p.id).await?;
    }
    counts.insert("products".into(), json!(payload.products.len()));

    // 3. Orders → synced_invoices + items + payments
    let mut order_items_count = 0usize;
    for o in &payload.orders {
        sqlx::query("INSERT INTO synced_invoices (store_id,local_id,invoice_number,customer_id,customer_name,customer_phone,total_amount,discount_amount,final_amount,customer_pay,change_money,payment_method,status,notes,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15::timestamp,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET invoice_number=EXCLUDED.invoice_number,customer_id=EXCLUDED.customer_id,customer_name=EXCLUDED.customer_name,customer_phone=EXCLUDED.customer_phone,total_amount=EXCLUDED.total_amount,discount_amount=EXCLUDED.discount_amount,final_amount=EXCLUDED.final_amount,customer_pay=EXCLUDED.customer_pay,change_money=EXCLUDED.change_money,payment_method=EXCLUDED.payment_method,status=EXCLUDED.status,notes=EXCLUDED.notes,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(o.id as i32).bind(&o.invoice_number)
            .bind(o.customer_id.map(|v| v as i32)).bind(&o.customer_name).bind(&o.customer_phone)
            .bind(o.total_amount).bind(o.discount_amount).bind(o.final_amount)
            .bind(o.customer_pay).bind(o.change_money).bind(&o.payment_method)
            .bind(&o.status).bind(&o.notes).bind(&o.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "invoices", "synced_invoices", o.id).await?;
        // Items
        for item in &o.items {
            sqlx::query("INSERT INTO synced_invoice_items (store_id,invoice_local_id,local_id,product_local_id,product_name,product_sku,unit_name,exchange_value,quantity,unit_price,total,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET invoice_local_id=EXCLUDED.invoice_local_id,product_local_id=EXCLUDED.product_local_id,product_name=EXCLUDED.product_name,product_sku=EXCLUDED.product_sku,unit_name=EXCLUDED.unit_name,exchange_value=EXCLUDED.exchange_value,quantity=EXCLUDED.quantity,unit_price=EXCLUDED.unit_price,total=EXCLUDED.total,synced_at=NOW()")
                .bind(store_id).bind(o.id as i32).bind(item.id)
                .bind(item.product_id.map(|v| v as i32)).bind(&item.product_name)
                .bind(&item.product_sku).bind(&item.unit_name).bind(item.exchange_value)
                .bind(item.quantity).bind(item.unit_price).bind(item.subtotal)
                .execute(&mut *tx).await?;
            v1_journal_log(&mut tx, store_id, "invoice_items", "synced_invoice_items", item.id).await?;
            order_items_count += 1;
        }
        // Payments: delete existing then insert
        sqlx::query("DELETE FROM synced_invoice_payments WHERE store_id=$1 AND invoice_local_id=$2")
            .bind(store_id).bind(o.id).execute(&mut *tx).await?;
        for pay in &o.payments {
            sqlx::query("INSERT INTO synced_invoice_payments (store_id,invoice_local_id,amount,payment_method) VALUES ($1,$2,$3,$4)")
                .bind(store_id).bind(o.id).bind(pay.amount).bind(&pay.payment_method)
                .execute(&mut *tx).await?;
        }
    }
    counts.insert("orders".into(), json!(payload.orders.len()));
    counts.insert("order_items".into(), json!(order_items_count));

    // 4. Suppliers
    for s in &payload.suppliers {
        sqlx::query("INSERT INTO synced_suppliers (store_id,local_id,name,phone,address,company,total_debt,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET name=EXCLUDED.name,phone=EXCLUDED.phone,address=EXCLUDED.address,company=EXCLUDED.company,total_debt=EXCLUDED.total_debt,synced_at=NOW()")
            .bind(store_id).bind(s.id as i32).bind(&s.name).bind(&s.phone)
            .bind(&s.address).bind(&s.code).bind(s.current_debt)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "suppliers", "synced_suppliers", s.id).await?;
    }
    counts.insert("suppliers".into(), json!(payload.suppliers.len()));

    // 5. Purchase Orders + Items
    for po in &payload.purchase_orders {
        sqlx::query("INSERT INTO synced_purchase_orders (store_id,local_id,supplier_id,supplier_name,total_amount,status,import_date,is_tax_invoice,invoice_type,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET supplier_id=EXCLUDED.supplier_id,supplier_name=EXCLUDED.supplier_name,total_amount=EXCLUDED.total_amount,status=EXCLUDED.status,import_date=EXCLUDED.import_date,is_tax_invoice=EXCLUDED.is_tax_invoice,invoice_type=EXCLUDED.invoice_type,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(po.id).bind(po.supplier_id).bind(&po.supplier_name)
            .bind(po.total_amount).bind(&po.status).bind(&po.import_date)
            .bind(po.is_tax_invoice).bind(&po.invoice_type).bind(&po.note).bind(&po.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "purchase_orders", "synced_purchase_orders", po.id).await?;
        for item in &po.items {
            sqlx::query("INSERT INTO synced_purchase_items (store_id,purchase_order_id,local_id,product_id,product_name,unit,exchange_value,quantity,import_price,subtotal,expiry_date,batch_number) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) ON CONFLICT (store_id,local_id) DO UPDATE SET purchase_order_id=EXCLUDED.purchase_order_id,product_id=EXCLUDED.product_id,product_name=EXCLUDED.product_name,unit=EXCLUDED.unit,exchange_value=EXCLUDED.exchange_value,quantity=EXCLUDED.quantity,import_price=EXCLUDED.import_price,subtotal=EXCLUDED.subtotal,expiry_date=EXCLUDED.expiry_date,batch_number=EXCLUDED.batch_number")
                .bind(store_id).bind(po.id).bind(item.id).bind(item.product_id)
                .bind(&item.product_name).bind(&item.unit).bind(item.exchange_value)
                .bind(item.quantity).bind(item.import_price).bind(item.subtotal)
                .bind(&item.expiry_date).bind(&item.batch_number)
                .execute(&mut *tx).await?;
            v1_journal_log(&mut tx, store_id, "purchase_items", "synced_purchase_items", item.id).await?;
        }
    }
    counts.insert("purchase_orders".into(), json!(payload.purchase_orders.len()));

    // 6. Customer Transactions
    for ct in &payload.customer_transactions {
        sqlx::query("INSERT INTO synced_customer_transactions (store_id,local_id,customer_id,amount,transaction_type,note,season,invoice_id,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET customer_id=EXCLUDED.customer_id,amount=EXCLUDED.amount,transaction_type=EXCLUDED.transaction_type,note=EXCLUDED.note,season=EXCLUDED.season,invoice_id=EXCLUDED.invoice_id,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(ct.id).bind(ct.customer_id).bind(ct.amount)
            .bind(&ct.transaction_type).bind(&ct.note).bind(&ct.season)
            .bind(ct.invoice_id).bind(&ct.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "customer_transactions", "synced_customer_transactions", ct.id).await?;
    }
    counts.insert("customer_transactions".into(), json!(payload.customer_transactions.len()));

    // 7. Supplier Transactions
    for st in &payload.supplier_transactions {
        sqlx::query("INSERT INTO synced_supplier_transactions (store_id,local_id,supplier_id,amount,ref_type,ref_id,note,balance_after,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET supplier_id=EXCLUDED.supplier_id,amount=EXCLUDED.amount,ref_type=EXCLUDED.ref_type,ref_id=EXCLUDED.ref_id,note=EXCLUDED.note,balance_after=EXCLUDED.balance_after,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(st.id).bind(st.supplier_id).bind(st.amount)
            .bind(&st.ref_type).bind(st.ref_id).bind(&st.note)
            .bind(st.balance_after).bind(&st.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "supplier_transactions", "synced_supplier_transactions", st.id).await?;
    }
    counts.insert("supplier_transactions".into(), json!(payload.supplier_transactions.len()));

    // 8. Product Units
    for pu in &payload.product_units {
        sqlx::query("INSERT INTO synced_product_units (store_id,local_id,product_id,unit_name,exchange_value,price,is_active,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (store_id,local_id) DO UPDATE SET product_id=EXCLUDED.product_id,unit_name=EXCLUDED.unit_name,exchange_value=EXCLUDED.exchange_value,price=EXCLUDED.price,is_active=EXCLUDED.is_active,created_at=EXCLUDED.created_at")
            .bind(store_id).bind(pu.id).bind(pu.product_id).bind(&pu.unit_name)
            .bind(pu.exchange_value).bind(pu.price).bind(pu.is_active).bind(&pu.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "product_units", "synced_product_units", pu.id).await?;
    }
    counts.insert("product_units".into(), json!(payload.product_units.len()));

    // 9. Store Funds (single record per store)
    if let Some(sf) = &payload.store_funds {
        sqlx::query("INSERT INTO synced_store_funds (store_id,current_balance,updated_at) VALUES ($1,$2,$3) ON CONFLICT (store_id) DO UPDATE SET current_balance=EXCLUDED.current_balance,updated_at=EXCLUDED.updated_at")
            .bind(store_id).bind(sf.current_balance).bind(&sf.updated_at)
            .execute(&mut *tx).await?;
    }

    // 10. Cash Transactions
    for ct in &payload.cash_transactions {
        sqlx::query("INSERT INTO synced_cash_transactions (store_id,local_id,amount,flow_type,category,ref_id,balance_after,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET amount=EXCLUDED.amount,flow_type=EXCLUDED.flow_type,category=EXCLUDED.category,ref_id=EXCLUDED.ref_id,balance_after=EXCLUDED.balance_after,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(ct.id).bind(ct.amount).bind(&ct.flow_type)
            .bind(&ct.category).bind(ct.ref_id).bind(ct.balance_after)
            .bind(&ct.note).bind(&ct.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "cash_transactions", "synced_cash_transactions", ct.id).await?;
    }
    counts.insert("cash_transactions".into(), json!(payload.cash_transactions.len()));

    // 11. Product Batches
    for pb in &payload.product_batches {
        sqlx::query("INSERT INTO synced_product_batches (store_id,local_id,product_id,purchase_item_id,expiry_date,import_date,quantity,remaining_quantity,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT (store_id,local_id) DO UPDATE SET product_id=EXCLUDED.product_id,purchase_item_id=EXCLUDED.purchase_item_id,expiry_date=EXCLUDED.expiry_date,import_date=EXCLUDED.import_date,quantity=EXCLUDED.quantity,remaining_quantity=EXCLUDED.remaining_quantity,created_at=EXCLUDED.created_at,updated_at=EXCLUDED.updated_at")
            .bind(store_id).bind(pb.id).bind(pb.product_id).bind(pb.purchase_item_id)
            .bind(&pb.expiry_date).bind(&pb.import_date).bind(pb.quantity)
            .bind(pb.remaining_quantity).bind(&pb.created_at).bind(&pb.updated_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "product_batches", "synced_product_batches", pb.id).await?;
    }
    counts.insert("product_batches".into(), json!(payload.product_batches.len()));

    // 12. Payment Vouchers
    for pv in &payload.payment_vouchers {
        sqlx::query("INSERT INTO synced_payment_vouchers (store_id,local_id,voucher_code,supplier_id,amount,payment_method,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET voucher_code=EXCLUDED.voucher_code,supplier_id=EXCLUDED.supplier_id,amount=EXCLUDED.amount,payment_method=EXCLUDED.payment_method,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(pv.id).bind(&pv.voucher_code).bind(pv.supplier_id)
            .bind(pv.amount).bind(&pv.payment_method).bind(&pv.note).bind(&pv.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "payment_vouchers", "synced_payment_vouchers", pv.id).await?;
    }
    counts.insert("payment_vouchers".into(), json!(payload.payment_vouchers.len()));

    // 13. Store Settings (single record per store)
    if let Some(ss) = &payload.store_settings {
        sqlx::query("INSERT INTO synced_store_settings (store_id,name,address,phone,security_pin,updated_at) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (store_id) DO UPDATE SET name=EXCLUDED.name,address=EXCLUDED.address,phone=EXCLUDED.phone,security_pin=EXCLUDED.security_pin,updated_at=EXCLUDED.updated_at")
            .bind(store_id).bind(&ss.name).bind(&ss.address).bind(&ss.phone)
            .bind(&ss.security_pin).bind(&ss.updated_at)
            .execute(&mut *tx).await?;
    }

    // 14. Product Transactions
    for pt in &payload.product_transactions {
        sqlx::query("INSERT INTO synced_product_transactions (store_id,local_id,product_id,transaction_type,quantity,reference_type,reference_id,note,balance_after,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET product_id=EXCLUDED.product_id,transaction_type=EXCLUDED.transaction_type,quantity=EXCLUDED.quantity,reference_type=EXCLUDED.reference_type,reference_id=EXCLUDED.reference_id,note=EXCLUDED.note,balance_after=EXCLUDED.balance_after,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(pt.id).bind(pt.product_id).bind(&pt.transaction_type)
            .bind(pt.quantity).bind(&pt.reference_type).bind(pt.reference_id)
            .bind(&pt.note).bind(pt.balance_after).bind(&pt.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "product_transactions", "synced_product_transactions", pt.id).await?;
    }
    counts.insert("product_transactions".into(), json!(payload.product_transactions.len()));

    // 15. Staff Members (desktop users table)
    for sm in &payload.staff_members {
        let perms_json: Value = sm.permissions.as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or(json!({}));
        sqlx::query(
            "INSERT INTO sync_staff_members (store_id,id,username,display_name,role,pin,permissions,is_active,created_at,updated_at,synced_at) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9::timestamptz,NOW(),NOW()) \
             ON CONFLICT (store_id,id) DO UPDATE SET \
             username=EXCLUDED.username,display_name=EXCLUDED.display_name,role=EXCLUDED.role,\
             pin=EXCLUDED.pin,permissions=EXCLUDED.permissions,is_active=EXCLUDED.is_active,\
             updated_at=NOW(),synced_at=NOW()"
        )
            .bind(store_id).bind(sm.id).bind(sm.username.as_deref().unwrap_or(""))
            .bind(sm.display_name.as_deref().unwrap_or(""))
            .bind(sm.role.as_deref().unwrap_or("staff"))
            .bind(&sm.pin)
            .bind(perms_json)
            .bind(sm.is_active.unwrap_or(true))
            .bind(&sm.created_at)
            .execute(&mut *tx).await?;
    }
    counts.insert("staff_members".into(), json!(payload.staff_members.len()));

    // 16. Loyalty Transactions (v2)
    for lt in &payload.loyalty_transactions {
        sqlx::query("INSERT INTO synced_loyalty_transactions_v2 (store_id,client_id,customer_client_id,type,points,ref_invoice_id,description,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET customer_client_id=EXCLUDED.customer_client_id,type=EXCLUDED.type,points=EXCLUDED.points,ref_invoice_id=EXCLUDED.ref_invoice_id,description=EXCLUDED.description,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(lt.id as i32).bind(lt.customer_id.map(|v| v as i32))
            .bind(&lt.r#type).bind(lt.points.unwrap_or(0))
            .bind(lt.ref_invoice_id.map(|v| v as i32))
            .bind(&lt.description).bind(&lt.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "loyalty_transactions", "synced_loyalty_transactions_v2", lt.id).await?;
    }
    counts.insert("loyalty_transactions".into(), json!(payload.loyalty_transactions.len()));

    // 17. Loyalty Settings (v2, singleton per store)
    if let Some(ls) = &payload.loyalty_settings {
        sqlx::query("INSERT INTO synced_loyalty_settings_v2 (store_id,is_enabled,points_per_amount,point_value,tier_silver,tier_gold,tier_diamond,updated_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW()) ON CONFLICT (store_id) DO UPDATE SET is_enabled=EXCLUDED.is_enabled,points_per_amount=EXCLUDED.points_per_amount,point_value=EXCLUDED.point_value,tier_silver=EXCLUDED.tier_silver,tier_gold=EXCLUDED.tier_gold,tier_diamond=EXCLUDED.tier_diamond,updated_at=EXCLUDED.updated_at,synced_at=NOW()")
            .bind(store_id).bind(ls.is_enabled.unwrap_or(true))
            .bind(ls.points_per_amount.unwrap_or(10000)).bind(ls.point_value.unwrap_or(1000))
            .bind(ls.tier_silver.unwrap_or(500)).bind(ls.tier_gold.unwrap_or(2000))
            .bind(ls.tier_diamond.unwrap_or(5000))
            .bind(&ls.updated_at)
            .execute(&mut *tx).await?;
    }

    // 18. Promotions
    for p in &payload.promotions {
        sqlx::query("INSERT INTO synced_promotions (store_id,client_id,name,type,value,min_order_value,max_discount,start_date,end_date,is_active,applies_to,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET name=EXCLUDED.name,type=EXCLUDED.type,value=EXCLUDED.value,min_order_value=EXCLUDED.min_order_value,max_discount=EXCLUDED.max_discount,start_date=EXCLUDED.start_date,end_date=EXCLUDED.end_date,is_active=EXCLUDED.is_active,applies_to=EXCLUDED.applies_to,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(p.id as i32).bind(&p.name).bind(&p.r#type)
            .bind(p.value).bind(p.min_order_value).bind(p.max_discount)
            .bind(&p.start_date).bind(&p.end_date)
            .bind(p.is_active.unwrap_or(true)).bind(&p.applies_to)
            .bind(&p.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "promotions", "synced_promotions", p.id).await?;
    }
    counts.insert("promotions".into(), json!(payload.promotions.len()));

    // 19. Vouchers
    for v in &payload.vouchers {
        sqlx::query("INSERT INTO synced_vouchers (store_id,client_id,code,promotion_client_id,usage_limit,used_count,is_active,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET code=EXCLUDED.code,promotion_client_id=EXCLUDED.promotion_client_id,usage_limit=EXCLUDED.usage_limit,used_count=EXCLUDED.used_count,is_active=EXCLUDED.is_active,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(v.id as i32).bind(&v.code)
            .bind(v.promotion_id.map(|x| x as i32).unwrap_or(0))
            .bind(v.usage_limit.unwrap_or(1)).bind(v.used_count.unwrap_or(0))
            .bind(v.is_active.unwrap_or(true)).bind(&v.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "vouchers", "synced_vouchers", v.id).await?;
    }
    counts.insert("vouchers".into(), json!(payload.vouchers.len()));

    // 20. Daily Closings
    for dc in &payload.daily_closings {
        sqlx::query("INSERT INTO synced_daily_closings (store_id,client_id,closing_date,total_revenue,total_invoices,total_cash_sales,total_debt_sales,total_transfer_sales,expected_cash,actual_cash,difference,total_returns,total_debt_collected,note,created_by,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET closing_date=EXCLUDED.closing_date,total_revenue=EXCLUDED.total_revenue,total_invoices=EXCLUDED.total_invoices,total_cash_sales=EXCLUDED.total_cash_sales,total_debt_sales=EXCLUDED.total_debt_sales,total_transfer_sales=EXCLUDED.total_transfer_sales,expected_cash=EXCLUDED.expected_cash,actual_cash=EXCLUDED.actual_cash,difference=EXCLUDED.difference,total_returns=EXCLUDED.total_returns,total_debt_collected=EXCLUDED.total_debt_collected,note=EXCLUDED.note,created_by=EXCLUDED.created_by,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(dc.id as i32).bind(&dc.closing_date)
            .bind(dc.total_revenue.unwrap_or(0.0)).bind(dc.total_invoices.unwrap_or(0))
            .bind(dc.total_cash_sales).bind(dc.total_debt_sales)
            .bind(dc.total_transfer_sales).bind(dc.expected_cash)
            .bind(dc.actual_cash).bind(dc.difference)
            .bind(dc.total_returns).bind(dc.total_debt_collected)
            .bind(&dc.note).bind(&dc.created_by).bind(&dc.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "daily_closings", "synced_daily_closings", dc.id).await?;
    }
    counts.insert("daily_closings".into(), json!(payload.daily_closings.len()));

    // 21. Returns + Return Items
    for ret in &payload.returns {
        sqlx::query("INSERT INTO synced_returns (store_id,client_id,original_invoice_client_id,return_number,customer_client_id,customer_name,total_refund,refund_method,reason,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET original_invoice_client_id=EXCLUDED.original_invoice_client_id,return_number=EXCLUDED.return_number,customer_client_id=EXCLUDED.customer_client_id,customer_name=EXCLUDED.customer_name,total_refund=EXCLUDED.total_refund,refund_method=EXCLUDED.refund_method,reason=EXCLUDED.reason,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(ret.id as i32)
            .bind(ret.original_invoice_id.map(|x| x as i32).unwrap_or(0))
            .bind(&ret.return_number)
            .bind(ret.customer_id.map(|x| x as i32)).bind(&ret.customer_name)
            .bind(ret.total_refund.unwrap_or(0.0)).bind(&ret.refund_method)
            .bind(&ret.reason).bind(&ret.note).bind(&ret.created_at)
            .execute(&mut *tx).await?;
        v1_journal_log(&mut tx, store_id, "returns", "synced_returns", ret.id).await?;
        // Nested return items
        for item in &ret.items {
            sqlx::query("INSERT INTO synced_return_items (store_id,client_id,return_client_id,product_client_id,product_name,unit_name,exchange_value,quantity,unit_price,subtotal,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NOW()) ON CONFLICT (store_id,client_id) DO UPDATE SET return_client_id=EXCLUDED.return_client_id,product_client_id=EXCLUDED.product_client_id,product_name=EXCLUDED.product_name,unit_name=EXCLUDED.unit_name,exchange_value=EXCLUDED.exchange_value,quantity=EXCLUDED.quantity,unit_price=EXCLUDED.unit_price,subtotal=EXCLUDED.subtotal,synced_at=NOW()")
                .bind(store_id).bind(item.id as i32)
                .bind(item.return_id.map(|x| x as i32).unwrap_or(ret.id as i32))
                .bind(item.product_id.map(|x| x as i32).unwrap_or(0))
                .bind(&item.product_name).bind(&item.unit_name)
                .bind(item.exchange_value.unwrap_or(1.0))
                .bind(item.quantity).bind(item.unit_price).bind(item.subtotal)
                .execute(&mut *tx).await?;
            v1_journal_log(&mut tx, store_id, "return_items", "synced_return_items", item.id).await?;
        }
    }
    counts.insert("returns".into(), json!(payload.returns.len()));

    tx.commit().await?;

    // Broadcast sync event to WebSocket clients (real-time sync)
    let changed: Vec<&str> = counts.iter()
        .filter(|(_, v)| v.as_u64().unwrap_or(0) > 0)
        .map(|(k, _)| k.as_str())
        .collect();
    if !changed.is_empty() {
        crate::routes::ws_sync::broadcast_sync_event(
            &state.sync_rooms, store_id, &changed
        ).await;
    }

    tracing::info!("✅ Sync complete for store_id={}, counts={:?}", store_id, counts);

    Ok(Json(json!({
        "success": true,
        "message": "",
        "synced_count": counts
    })))
}

// ============================================================
// GET /api/sync/pull — Download all store data to client
// ============================================================

async fn handle_pull(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    let store_id = identify_store(&headers, &state.pool, &state.config.jwt_secret).await?;

    // 1. Customers (synced_at as created_at for mobile SQLite NOT NULL)
    let customers: Vec<Value> = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<String>, Option<String>, Option<String>, Option<i32>, Option<f64>, Option<String>, Option<String>)>(
        "SELECT local_id, name, phone, address, total_debt::FLOAT8, credit_limit, cccd, cccd_front_img, cccd_back_img, loyalty_points, total_spent::FLOAT8, loyalty_tier, synced_at::text FROM synced_customers WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "name": r.1, "phone": r.2, "address": r.3,
        "current_debt": r.4, "credit_limit": r.5, "cccd": r.6,
        "cccd_front_img": r.7, "cccd_back_img": r.8,
        "loyalty_points": r.9, "total_spent": r.10, "loyalty_tier": r.11,
        "created_at": r.12
    })).collect();

    // 2. Products (synced_at as created_at for mobile SQLite NOT NULL)
    let products: Vec<Value> = sqlx::query_as::<_, (i32, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>, Option<String>, Option<String>)>(
        "SELECT local_id, name, barcode, sku, description, category, manufacturer, base_unit, stock_quantity::FLOAT8, cost_price, sell_price, expiry_date, updated_at, synced_at::text FROM synced_products WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "name": r.1, "barcode": r.2, "sku": r.3,
        "description": r.4, "category": r.5, "manufacturer": r.6, "base_unit": r.7,
        "stock_quantity": r.8, "cost_price": r.9, "sell_price": r.10,
        "expiry_date": r.11, "updated_at": r.12, "created_at": r.13
    })).collect();

    // 3. Orders (invoices)
    let orders: Vec<Value> = sqlx::query_as::<_, (i32, Option<String>, Option<i32>, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT local_id, invoice_number, customer_id, customer_name, customer_phone, total_amount::FLOAT8, discount_amount, final_amount, customer_pay, change_money, payment_method, status, notes, created_at::text FROM synced_invoices WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "invoice_number": r.1, "customer_id": r.2,
        "customer_name": r.3, "customer_phone": r.4,
        "total_amount": r.5, "discount_amount": r.6, "final_amount": r.7,
        "customer_pay": r.8, "change_money": r.9, "payment_method": r.10,
        "status": r.11, "notes": r.12, "created_at": r.13
    })).collect();

    // 4. Suppliers (synced_at as created_at for mobile SQLite NOT NULL)
    let suppliers: Vec<Value> = sqlx::query_as::<_, (i32, Option<String>, String, Option<String>, Option<String>, Option<f64>, Option<String>)>(
        "SELECT local_id, company, name, phone, address, total_debt::FLOAT8, synced_at::text FROM synced_suppliers WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "code": r.1, "name": r.2, "phone": r.3,
        "address": r.4, "current_debt": r.5, "created_at": r.6
    })).collect();

    // 5. Purchase Orders
    let purchase_orders: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<String>, Option<f64>, Option<String>, Option<String>, Option<bool>, Option<String>, Option<String>, Option<String>)>(
        "SELECT local_id, supplier_id, supplier_name, total_amount, status, import_date, is_tax_invoice, invoice_type, note, created_at::text FROM synced_purchase_orders WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "supplier_id": r.1, "supplier_name": r.2,
        "total_amount": r.3, "status": r.4, "import_date": r.5,
        "is_tax_invoice": r.6, "invoice_type": r.7, "note": r.8, "created_at": r.9
    })).collect();

    // 6. Customer Transactions
    let customer_transactions: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<f64>, Option<String>, Option<String>, Option<String>, Option<i64>, Option<String>)>(
        "SELECT local_id, customer_id, amount, transaction_type, note, season, invoice_id, created_at::text FROM synced_customer_transactions WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "customer_id": r.1, "amount": r.2,
        "transaction_type": r.3, "note": r.4, "season": r.5,
        "invoice_id": r.6, "created_at": r.7
    })).collect();

    // 7. Supplier Transactions
    let supplier_transactions: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<f64>, Option<String>, Option<i64>, Option<String>, Option<f64>, Option<String>)>(
        "SELECT local_id, supplier_id, amount, ref_type, ref_id, note, balance_after, created_at::text FROM synced_supplier_transactions WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "supplier_id": r.1, "amount": r.2,
        "ref_type": r.3, "ref_id": r.4, "note": r.5,
        "balance_after": r.6, "created_at": r.7
    })).collect();

    // 8. Product Units
    let product_units: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<String>, Option<f64>, Option<f64>, Option<bool>, Option<String>)>(
        "SELECT local_id, product_id, unit_name, exchange_value, price, is_active, created_at::text FROM synced_product_units WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "product_id": r.1, "unit_name": r.2,
        "exchange_value": r.3, "price": r.4, "is_active": r.5, "created_at": r.6
    })).collect();

    // 9. Store Funds
    let store_funds: Option<Value> = sqlx::query_as::<_, (Option<i64>, Option<String>)>(
        "SELECT current_balance, updated_at FROM synced_store_funds WHERE store_id = $1"
    ).bind(store_id).fetch_optional(&state.pool).await?
    .map(|r| json!({ "current_balance": r.0, "updated_at": r.1 }));

    // 10. Cash Transactions
    let cash_transactions: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<String>, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>)>(
        "SELECT local_id, amount, flow_type, category, ref_id, balance_after, note, created_at::text FROM synced_cash_transactions WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "amount": r.1, "flow_type": r.2,
        "category": r.3, "ref_id": r.4, "balance_after": r.5,
        "note": r.6, "created_at": r.7
    })).collect();

    // 11. Product Batches
    let product_batches: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<i64>, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<String>, Option<String>)>(
        "SELECT local_id, product_id, purchase_item_id, expiry_date, import_date, quantity, remaining_quantity, created_at::text, updated_at::text FROM synced_product_batches WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "product_id": r.1, "purchase_item_id": r.2,
        "expiry_date": r.3, "import_date": r.4, "quantity": r.5,
        "remaining_quantity": r.6, "created_at": r.7, "updated_at": r.8
    })).collect();

    // 12. Payment Vouchers
    let payment_vouchers: Vec<Value> = sqlx::query_as::<_, (i64, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, Option<String>)>(
        "SELECT local_id, voucher_code, supplier_id, amount, payment_method, note, created_at::text FROM synced_payment_vouchers WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "voucher_code": r.1, "supplier_id": r.2,
        "amount": r.3, "payment_method": r.4, "note": r.5, "created_at": r.6
    })).collect();

    // 13. Store Settings
    let store_settings: Option<Value> = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT name, address, phone, security_pin, updated_at FROM synced_store_settings WHERE store_id = $1"
    ).bind(store_id).fetch_optional(&state.pool).await?
    .map(|r| json!({
        "name": r.0, "address": r.1, "phone": r.2,
        "security_pin": r.3, "updated_at": r.4
    }));

    // 14. Product Transactions
    let product_transactions: Vec<Value> = sqlx::query_as::<_, (i64, Option<i64>, Option<String>, Option<f64>, Option<String>, Option<i64>, Option<String>, Option<f64>, Option<String>)>(
        "SELECT local_id, product_id, transaction_type, quantity, reference_type, reference_id, note, balance_after, created_at::text FROM synced_product_transactions WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "product_id": r.1, "transaction_type": r.2,
        "quantity": r.3, "reference_type": r.4, "reference_id": r.5,
        "note": r.6, "balance_after": r.7, "created_at": r.8
    })).collect();

    // 15. Staff Members
    let staff_members: Vec<Value> = sqlx::query_as::<_, (i64, Option<String>, Option<String>, Option<String>, Option<String>, Option<serde_json::Value>, Option<bool>, Option<String>)>(
        "SELECT id, username, display_name, role, pin, permissions, is_active, created_at::text FROM sync_staff_members WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "username": r.1, "display_name": r.2,
        "role": r.3, "pin": r.4, "permissions": r.5.map(|v| v.to_string()),
        "is_active": r.6, "created_at": r.7
    })).collect();

    // 16. Loyalty Transactions (v2)
    let loyalty_transactions: Vec<Value> = sqlx::query_as::<_, (i32, i32, Option<String>, i32, Option<i32>, Option<String>, String)>(
        "SELECT client_id, customer_client_id, type, points, ref_invoice_id, description, created_at FROM synced_loyalty_transactions_v2 WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "customer_id": r.1, "type": r.2,
        "points": r.3, "ref_invoice_id": r.4,
        "description": r.5, "created_at": r.6
    })).collect();

    // 17. Loyalty Settings (v2)
    let loyalty_settings: Option<Value> = sqlx::query_as::<_, (Option<bool>, i32, i32, i32, i32, i32, Option<String>)>(
        "SELECT is_enabled, points_per_amount, point_value, tier_silver, tier_gold, tier_diamond, updated_at FROM synced_loyalty_settings_v2 WHERE store_id = $1"
    ).bind(store_id).fetch_optional(&state.pool).await?
    .map(|r| json!({
        "is_enabled": r.0, "points_per_amount": r.1, "point_value": r.2,
        "tier_silver": r.3, "tier_gold": r.4, "tier_diamond": r.5, "updated_at": r.6
    }));

    // 18. Promotions
    let promotions: Vec<Value> = sqlx::query_as::<_, (i32, String, String, f64, Option<f64>, Option<f64>, String, String, Option<bool>, Option<String>, Option<String>)>(
        "SELECT client_id, name, type, value, min_order_value, max_discount, start_date, end_date, is_active, applies_to, created_at FROM synced_promotions WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "name": r.1, "type": r.2, "value": r.3,
        "min_order_value": r.4, "max_discount": r.5,
        "start_date": r.6, "end_date": r.7,
        "is_active": r.8, "applies_to": r.9, "created_at": r.10
    })).collect();

    // 19. Vouchers
    let vouchers: Vec<Value> = sqlx::query_as::<_, (i32, String, i32, Option<i32>, Option<i32>, Option<bool>, Option<String>)>(
        "SELECT client_id, code, promotion_client_id, usage_limit, used_count, is_active, created_at FROM synced_vouchers WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "code": r.1, "promotion_id": r.2,
        "usage_limit": r.3, "used_count": r.4,
        "is_active": r.5, "created_at": r.6
    })).collect();

    // 20. Daily Closings
    let daily_closings: Vec<Value> = sqlx::query_as::<_, (i32, String, f64, i32, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<f64>, Option<String>, Option<String>, Option<String>)>(
        "SELECT client_id, closing_date, total_revenue, total_invoices, total_cash_sales, total_debt_sales, total_transfer_sales, expected_cash, actual_cash, difference, total_returns, total_debt_collected, note, created_by, created_at FROM synced_daily_closings WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?
    .into_iter().map(|r| json!({
        "id": r.0, "closing_date": r.1, "total_revenue": r.2,
        "total_invoices": r.3, "total_cash_sales": r.4,
        "total_debt_sales": r.5, "total_transfer_sales": r.6,
        "expected_cash": r.7, "actual_cash": r.8,
        "difference": r.9, "total_returns": r.10,
        "total_debt_collected": r.11, "note": r.12,
        "created_by": r.13, "created_at": r.14
    })).collect();

    // 21. Returns with nested items
    let returns_rows: Vec<(i32, i32, String, Option<i32>, Option<String>, f64, Option<String>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT client_id, original_invoice_client_id, return_number, customer_client_id, customer_name, total_refund, refund_method, reason, note, created_at FROM synced_returns WHERE store_id = $1"
    ).bind(store_id).fetch_all(&state.pool).await?;

    let mut returns: Vec<Value> = Vec::new();
    for r in &returns_rows {
        let items: Vec<Value> = sqlx::query_as::<_, (i32, i32, i32, String, String, f64, f64, f64, f64)>(
            "SELECT client_id, return_client_id, product_client_id, product_name, unit_name, exchange_value, quantity, unit_price, subtotal FROM synced_return_items WHERE store_id = $1 AND return_client_id = $2"
        ).bind(store_id).bind(r.0).fetch_all(&state.pool).await?
        .into_iter().map(|i| json!({
            "id": i.0, "return_id": i.1, "product_id": i.2,
            "product_name": i.3, "unit_name": i.4,
            "exchange_value": i.5, "quantity": i.6,
            "unit_price": i.7, "subtotal": i.8
        })).collect();
        returns.push(json!({
            "id": r.0, "original_invoice_id": r.1, "return_number": r.2,
            "customer_id": r.3, "customer_name": r.4,
            "total_refund": r.5, "refund_method": r.6,
            "reason": r.7, "note": r.8, "created_at": r.9,
            "items": items
        }));
    }

    tracing::info!("📥 Pull complete for store_id={}, customers={}, products={}, orders={}, promotions={}, returns={}",
        store_id, customers.len(), products.len(), orders.len(), promotions.len(), returns.len());

    Ok(Json(json!({
        "success": true,
        "store_id": store_id,
        "data": {
            "customers": customers,
            "products": products,
            "orders": orders,
            "suppliers": suppliers,
            "purchase_orders": purchase_orders,
            "customer_transactions": customer_transactions,
            "supplier_transactions": supplier_transactions,
            "product_units": product_units,
            "store_funds": store_funds,
            "cash_transactions": cash_transactions,
            "product_batches": product_batches,
            "payment_vouchers": payment_vouchers,
            "store_settings": store_settings,
            "product_transactions": product_transactions,
            "staff_members": staff_members,
            "loyalty_transactions": loyalty_transactions,
            "loyalty_settings": loyalty_settings,
            "promotions": promotions,
            "vouchers": vouchers,
            "daily_closings": daily_closings,
            "returns": returns
        }
    })))
}
