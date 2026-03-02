use axum::{Router, Json, routing::post, extract::State};
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
}

#[derive(Deserialize)]
pub struct SyncCustomer {
    pub id: i64, pub name: String,
    pub phone: Option<String>, pub address: Option<String>,
    pub current_debt: Option<f64>, pub credit_limit: Option<f64>,
    pub cccd: Option<String>, pub cccd_front_img: Option<String>,
    pub cccd_back_img: Option<String>, pub created_at: Option<String>,
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
    pub permissions: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
}

// ============================================================
// Router
// ============================================================

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/sync", post(handle_sync))
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

    // 1. Customers
    for c in &payload.customers {
        sqlx::query("INSERT INTO synced_customers (store_id,local_id,name,phone,address,total_debt,credit_limit,cccd,cccd_front_img,cccd_back_img,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET name=EXCLUDED.name,phone=EXCLUDED.phone,address=EXCLUDED.address,total_debt=EXCLUDED.total_debt,credit_limit=EXCLUDED.credit_limit,cccd=EXCLUDED.cccd,cccd_front_img=EXCLUDED.cccd_front_img,cccd_back_img=EXCLUDED.cccd_back_img,synced_at=NOW()")
            .bind(store_id).bind(c.id as i32).bind(&c.name).bind(&c.phone).bind(&c.address)
            .bind(c.current_debt).bind(c.credit_limit).bind(&c.cccd)
            .bind(&c.cccd_front_img).bind(&c.cccd_back_img)
            .execute(&mut *tx).await?;
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
        // Items
        for item in &o.items {
            sqlx::query("INSERT INTO synced_invoice_items (store_id,invoice_local_id,local_id,product_local_id,product_name,product_sku,unit_name,exchange_value,quantity,unit_price,total,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET invoice_local_id=EXCLUDED.invoice_local_id,product_local_id=EXCLUDED.product_local_id,product_name=EXCLUDED.product_name,product_sku=EXCLUDED.product_sku,unit_name=EXCLUDED.unit_name,exchange_value=EXCLUDED.exchange_value,quantity=EXCLUDED.quantity,unit_price=EXCLUDED.unit_price,total=EXCLUDED.total,synced_at=NOW()")
                .bind(store_id).bind(o.id as i32).bind(item.id)
                .bind(item.product_id.map(|v| v as i32)).bind(&item.product_name)
                .bind(&item.product_sku).bind(&item.unit_name).bind(item.exchange_value)
                .bind(item.quantity).bind(item.unit_price).bind(item.subtotal)
                .execute(&mut *tx).await?;
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
    }
    counts.insert("suppliers".into(), json!(payload.suppliers.len()));

    // 5. Purchase Orders + Items
    for po in &payload.purchase_orders {
        sqlx::query("INSERT INTO synced_purchase_orders (store_id,local_id,supplier_id,supplier_name,total_amount,status,import_date,is_tax_invoice,invoice_type,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET supplier_id=EXCLUDED.supplier_id,supplier_name=EXCLUDED.supplier_name,total_amount=EXCLUDED.total_amount,status=EXCLUDED.status,import_date=EXCLUDED.import_date,is_tax_invoice=EXCLUDED.is_tax_invoice,invoice_type=EXCLUDED.invoice_type,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(po.id).bind(po.supplier_id).bind(&po.supplier_name)
            .bind(po.total_amount).bind(&po.status).bind(&po.import_date)
            .bind(po.is_tax_invoice).bind(&po.invoice_type).bind(&po.note).bind(&po.created_at)
            .execute(&mut *tx).await?;
        for item in &po.items {
            sqlx::query("INSERT INTO synced_purchase_items (store_id,purchase_order_id,local_id,product_id,product_name,unit,exchange_value,quantity,import_price,subtotal,expiry_date,batch_number) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) ON CONFLICT (store_id,local_id) DO UPDATE SET purchase_order_id=EXCLUDED.purchase_order_id,product_id=EXCLUDED.product_id,product_name=EXCLUDED.product_name,unit=EXCLUDED.unit,exchange_value=EXCLUDED.exchange_value,quantity=EXCLUDED.quantity,import_price=EXCLUDED.import_price,subtotal=EXCLUDED.subtotal,expiry_date=EXCLUDED.expiry_date,batch_number=EXCLUDED.batch_number")
                .bind(store_id).bind(po.id).bind(item.id).bind(item.product_id)
                .bind(&item.product_name).bind(&item.unit).bind(item.exchange_value)
                .bind(item.quantity).bind(item.import_price).bind(item.subtotal)
                .bind(&item.expiry_date).bind(&item.batch_number)
                .execute(&mut *tx).await?;
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
    }
    counts.insert("customer_transactions".into(), json!(payload.customer_transactions.len()));

    // 7. Supplier Transactions
    for st in &payload.supplier_transactions {
        sqlx::query("INSERT INTO synced_supplier_transactions (store_id,local_id,supplier_id,amount,ref_type,ref_id,note,balance_after,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET supplier_id=EXCLUDED.supplier_id,amount=EXCLUDED.amount,ref_type=EXCLUDED.ref_type,ref_id=EXCLUDED.ref_id,note=EXCLUDED.note,balance_after=EXCLUDED.balance_after,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(st.id).bind(st.supplier_id).bind(st.amount)
            .bind(&st.ref_type).bind(st.ref_id).bind(&st.note)
            .bind(st.balance_after).bind(&st.created_at)
            .execute(&mut *tx).await?;
    }
    counts.insert("supplier_transactions".into(), json!(payload.supplier_transactions.len()));

    // 8. Product Units
    for pu in &payload.product_units {
        sqlx::query("INSERT INTO synced_product_units (store_id,local_id,product_id,unit_name,exchange_value,price,is_active,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (store_id,local_id) DO UPDATE SET product_id=EXCLUDED.product_id,unit_name=EXCLUDED.unit_name,exchange_value=EXCLUDED.exchange_value,price=EXCLUDED.price,is_active=EXCLUDED.is_active,created_at=EXCLUDED.created_at")
            .bind(store_id).bind(pu.id).bind(pu.product_id).bind(&pu.unit_name)
            .bind(pu.exchange_value).bind(pu.price).bind(pu.is_active).bind(&pu.created_at)
            .execute(&mut *tx).await?;
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
    }
    counts.insert("cash_transactions".into(), json!(payload.cash_transactions.len()));

    // 11. Product Batches
    for pb in &payload.product_batches {
        sqlx::query("INSERT INTO synced_product_batches (store_id,local_id,product_id,purchase_item_id,expiry_date,import_date,quantity,remaining_quantity,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) ON CONFLICT (store_id,local_id) DO UPDATE SET product_id=EXCLUDED.product_id,purchase_item_id=EXCLUDED.purchase_item_id,expiry_date=EXCLUDED.expiry_date,import_date=EXCLUDED.import_date,quantity=EXCLUDED.quantity,remaining_quantity=EXCLUDED.remaining_quantity,created_at=EXCLUDED.created_at,updated_at=EXCLUDED.updated_at")
            .bind(store_id).bind(pb.id).bind(pb.product_id).bind(pb.purchase_item_id)
            .bind(&pb.expiry_date).bind(&pb.import_date).bind(pb.quantity)
            .bind(pb.remaining_quantity).bind(&pb.created_at).bind(&pb.updated_at)
            .execute(&mut *tx).await?;
    }
    counts.insert("product_batches".into(), json!(payload.product_batches.len()));

    // 12. Payment Vouchers
    for pv in &payload.payment_vouchers {
        sqlx::query("INSERT INTO synced_payment_vouchers (store_id,local_id,voucher_code,supplier_id,amount,payment_method,note,created_at,synced_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,NOW()) ON CONFLICT (store_id,local_id) DO UPDATE SET voucher_code=EXCLUDED.voucher_code,supplier_id=EXCLUDED.supplier_id,amount=EXCLUDED.amount,payment_method=EXCLUDED.payment_method,note=EXCLUDED.note,created_at=EXCLUDED.created_at,synced_at=NOW()")
            .bind(store_id).bind(pv.id).bind(&pv.voucher_code).bind(pv.supplier_id)
            .bind(pv.amount).bind(&pv.payment_method).bind(&pv.note).bind(&pv.created_at)
            .execute(&mut *tx).await?;
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

    tx.commit().await?;

    tracing::info!("✅ Sync complete for store_id={}, counts={:?}", store_id, counts);

    Ok(Json(json!({
        "success": true,
        "message": "",
        "synced_count": counts
    })))
}
