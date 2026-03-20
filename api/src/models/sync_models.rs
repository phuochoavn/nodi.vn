//! Serde structs for VPS ↔ Desktop column mapping (Sprint 140).
//!
//! Each struct uses `#[serde(rename)]` / `#[serde(alias)]` to handle
//! column-name differences between PostgreSQL (VPS) and SQLite (Desktop)
//! at compile time, replacing the runtime `COLUMN_MAPPINGS` approach.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================
// 1. Products (synced_products)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncProduct {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub barcode: Option<String>,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
    pub stock_quantity: Option<f64>,
    pub sku: Option<String>,
    pub base_unit: Option<String>,
    pub cost_price: Option<f64>,
    pub sell_price: Option<f64>,
    pub expiry_date: Option<String>,
    pub updated_at: Option<String>,
    pub min_stock: Option<i64>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 2. Product Units (synced_product_units)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncProductUnit {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "product_id", alias = "product_local_id")]
    pub product_id: Option<i64>,
    pub unit_name: Option<String>,
    pub exchange_value: Option<f64>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 3. Customers (synced_customers)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncCustomer {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub tax_code: Option<String>,
    #[serde(rename = "current_debt", alias = "total_debt")]
    pub current_debt: Option<f64>,
    pub credit_limit: Option<f64>,
    pub cccd: Option<String>,
    pub cccd_front_img: Option<String>,
    pub cccd_back_img: Option<String>,
    pub loyalty_points: Option<i64>,
    pub total_spent: Option<f64>,
    pub loyalty_tier: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 4. Suppliers (synced_suppliers)
// ============================================================

/// Supplier — VPS has both "name" (contact) and "company" (business name).
/// Desktop COLUMN_MAPPINGS: desktop:"name" ↔ vps:"company".
/// On pull: VPS "company" → Desktop "name".
/// VPS "name" (contact) is not used by Desktop, so we skip it.
#[derive(Serialize, Deserialize, Debug)]
pub struct SyncSupplier {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    /// VPS "company" → serialized as Desktop "name"
    /// Deserializes from "company", serializes as "name".
    #[serde(rename(serialize = "name", deserialize = "company"))]
    pub company_as_name: Option<String>,
    #[serde(rename = "current_debt", alias = "total_debt")]
    pub current_debt: Option<f64>,
    /// VPS "name" field — not used by Desktop, skip on serialize
    #[serde(skip_serializing, default)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 5. Invoices (synced_invoices)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncInvoice {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub invoice_number: Option<String>,
    #[serde(rename = "customer_id", alias = "customer_local_id")]
    pub customer_id: Option<i64>,
    pub total_amount: Option<f64>,
    pub payment_method: Option<String>,
    pub payment_status: Option<String>,
    pub created_at: Option<String>,
    pub discount_amount: Option<f64>,
    pub final_amount: Option<f64>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub status: Option<String>,
    pub customer_pay: Option<f64>,
    pub change_money: Option<f64>,
    #[serde(rename = "note", alias = "notes")]
    pub note: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 6. Invoice Items (synced_invoice_items)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncInvoiceItem {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "invoice_id", alias = "invoice_local_id")]
    pub invoice_id: Option<i64>,
    #[serde(rename = "product_id", alias = "product_local_id")]
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub quantity: Option<f64>,
    pub unit_name: Option<String>,
    pub unit_price: Option<f64>,
    #[serde(rename = "subtotal", alias = "total")]
    pub subtotal: Option<f64>,
    pub product_sku: Option<String>,
    pub exchange_value: Option<f64>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 7. Invoice Payments (synced_invoice_payments)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncInvoicePayment {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "invoice_id", alias = "invoice_local_id")]
    pub invoice_id: Option<i64>,
    pub amount: Option<f64>,
    pub payment_method: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 8. Purchase Orders (synced_purchase_orders)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncPurchaseOrder {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub supplier_id: Option<i64>,
    pub supplier_name: Option<String>,
    pub total_amount: Option<f64>,
    pub status: Option<String>,
    pub import_date: Option<String>,
    pub is_tax_invoice: Option<bool>,
    pub invoice_type: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 9. Purchase Items (synced_purchase_items)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncPurchaseItem {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "purchase_order_id", alias = "purchase_order_local_id")]
    pub purchase_order_id: Option<i64>,
    #[serde(rename = "product_id", alias = "product_local_id")]
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub unit: Option<String>,
    pub exchange_value: Option<f64>,
    pub quantity: Option<f64>,
    pub import_price: Option<f64>,
    pub subtotal: Option<f64>,
    pub expiry_date: Option<String>,
    pub batch_number: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 10. Customer Transactions (synced_customer_transactions)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncCustomerTransaction {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "customer_id", alias = "customer_local_id")]
    pub customer_id: Option<i64>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub note: Option<String>,
    pub season: Option<String>,
    pub invoice_id: Option<i64>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 11. Supplier Transactions (synced_supplier_transactions)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncSupplierTransaction {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "supplier_id", alias = "supplier_local_id")]
    pub supplier_id: Option<i64>,
    pub amount: Option<f64>,
    pub ref_type: Option<String>,
    pub ref_id: Option<i64>,
    pub note: Option<String>,
    pub balance_after: Option<f64>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 12. Cash Transactions (synced_cash_transactions)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncCashTransaction {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub amount: Option<i64>,
    pub flow_type: Option<String>,
    pub category: Option<String>,
    pub ref_id: Option<i64>,
    pub balance_after: Option<i64>,
    pub note: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 13. Product Transactions (synced_product_transactions)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncProductTransaction {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "product_id", alias = "product_local_id")]
    pub product_id: Option<i64>,
    pub transaction_type: Option<String>,
    pub quantity: Option<f64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<i64>,
    pub note: Option<String>,
    pub balance_after: Option<f64>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 14. Product Batches (synced_product_batches)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncProductBatch {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "product_id", alias = "product_local_id")]
    pub product_id: Option<i64>,
    pub purchase_item_id: Option<i64>,
    pub expiry_date: Option<String>,
    pub import_date: Option<String>,
    pub quantity: Option<f64>,
    pub remaining_quantity: Option<f64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 15. Payment Vouchers (synced_payment_vouchers)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncPaymentVoucher {
    #[serde(rename = "id", alias = "local_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub voucher_code: Option<String>,
    pub supplier_id: Option<i64>,
    pub amount: Option<i64>,
    pub payment_method: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 16. Store Funds (synced_store_funds)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncStoreFund {
    pub current_balance: Option<i64>,
    pub updated_at: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
}

// ============================================================
// 17. Store Settings (synced_store_settings)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncStoreSetting {
    pub uuid: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub security_pin: Option<String>,
    pub updated_at: Option<String>,
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 18. Returns (synced_returns)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncReturn {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub original_invoice_client_id: Option<i64>,
    pub return_number: Option<String>,
    pub customer_client_id: Option<i64>,
    pub customer_name: Option<String>,
    pub total_refund: Option<f64>,
    pub refund_method: Option<String>,
    pub reason: Option<String>,
    pub note: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 19. Return Items (synced_return_items)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncReturnItem {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    #[serde(rename = "return_id", alias = "return_client_id")]
    pub return_id: Option<i64>,
    #[serde(rename = "product_id", alias = "product_client_id")]
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub unit_name: Option<String>,
    pub exchange_value: Option<f64>,
    pub quantity: Option<f64>,
    pub unit_price: Option<f64>,
    pub subtotal: Option<f64>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 20. Daily Closings (synced_daily_closings)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncDailyClosing {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub closing_date: Option<String>,
    pub total_revenue: Option<f64>,
    pub total_invoices: Option<i64>,
    pub total_cash_sales: Option<f64>,
    pub total_debt_sales: Option<f64>,
    pub total_transfer_sales: Option<f64>,
    pub expected_cash: Option<f64>,
    pub actual_cash: Option<f64>,
    pub difference: Option<f64>,
    pub total_returns: Option<f64>,
    pub total_debt_collected: Option<f64>,
    pub note: Option<String>,
    pub created_by: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 21. Promotions (synced_promotions)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncPromotion {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub promotion_type: Option<String>,
    pub value: Option<f64>,
    pub min_order_value: Option<f64>,
    pub max_discount: Option<f64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_active: Option<bool>,
    pub applies_to: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 22. Vouchers (synced_vouchers)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncVoucher {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub code: Option<String>,
    pub promotion_client_id: Option<i64>,
    pub usage_limit: Option<i64>,
    pub used_count: Option<i64>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// 23. Loyalty Transactions V2 (synced_loyalty_transactions_v2)
// ============================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncLoyaltyTransactionV2 {
    #[serde(rename = "id", alias = "client_id")]
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub customer_client_id: Option<i64>,
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    pub points: Option<i64>,
    pub ref_invoice_id: Option<i64>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    #[serde(skip_serializing)]
    pub store_id: Option<i32>,
    #[serde(skip_serializing)]
    pub synced_at: Option<String>,
    #[serde(skip_serializing)]
    pub device_id: Option<String>,
    #[serde(skip_serializing)]
    pub updated_at_v2: Option<String>,
}

// ============================================================
// Serde Transform Helper
// ============================================================

/// Attempt to transform a VPS JSON record into Desktop format using Serde structs.
/// Returns `Some(transformed)` if this table has a matching struct,
/// `None` if no struct exists (caller should fall back to COLUMN_MAPPINGS).
pub fn serde_transform_for_pull(record: &Value, table: &str) -> Option<Value> {
    // Helper macro to reduce boilerplate: deserialize → serialize round-trip
    macro_rules! transform {
        ($type:ty, $record:expr) => {{
            let typed: $type = serde_json::from_value($record.clone()).ok()?;
            serde_json::to_value(&typed).ok()
        }};
    }

    match table {
        "products" => transform!(SyncProduct, record),
        "product_units" => transform!(SyncProductUnit, record),
        "customers" => transform!(SyncCustomer, record),
        "suppliers" => transform!(SyncSupplier, record),
        "invoices" => transform!(SyncInvoice, record),
        "invoice_items" => transform!(SyncInvoiceItem, record),
        "invoice_payments" => transform!(SyncInvoicePayment, record),
        "purchase_orders" => transform!(SyncPurchaseOrder, record),
        "purchase_items" => transform!(SyncPurchaseItem, record),
        "customer_transactions" => transform!(SyncCustomerTransaction, record),
        "supplier_transactions" => transform!(SyncSupplierTransaction, record),
        "cash_transactions" => transform!(SyncCashTransaction, record),
        "product_transactions" => transform!(SyncProductTransaction, record),
        "product_batches" => transform!(SyncProductBatch, record),
        "payment_vouchers" => transform!(SyncPaymentVoucher, record),
        "store_funds" => transform!(SyncStoreFund, record),
        "store_settings" => transform!(SyncStoreSetting, record),
        "returns" => transform!(SyncReturn, record),
        "return_items" => transform!(SyncReturnItem, record),
        "daily_closings" => transform!(SyncDailyClosing, record),
        "promotions" => transform!(SyncPromotion, record),
        "vouchers" => transform!(SyncVoucher, record),
        "loyalty_transactions_v2" => transform!(SyncLoyaltyTransactionV2, record),
        _ => None,
    }
}

// ============================================================
// Unit Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Sprint 134 edge case: "local_id" in VPS → "id" in Desktop
    #[test]
    fn test_local_id_to_id() {
        let vps_record = json!({
            "local_id": 42,
            "uuid": "abc-123",
            "name": "Test Product",
            "store_id": 1
        });

        let result = serde_transform_for_pull(&vps_record, "products").unwrap();
        // "local_id" should become "id"
        assert_eq!(result["id"], json!(42));
        // "local_id" key should NOT appear in output
        assert!(result.get("local_id").is_none());
        // "store_id" should be stripped (skip_serializing)
        assert!(result.get("store_id").is_none());
    }

    /// Sprint 134 bug case: customer_transactions has "transaction_type" in VPS
    /// which should stay as "transaction_type" in Desktop (NOT renamed to "type").
    /// The old COLUMN_MAPPINGS had a buggy ALL-group entry that mapped this wrong.
    #[test]
    fn test_transaction_type_not_renamed() {
        let vps_record = json!({
            "local_id": 10,
            "uuid": "tx-001",
            "customer_id": 5,
            "amount": 100000.0,
            "transaction_type": "DEBIT",
            "store_id": 1
        });

        let result = serde_transform_for_pull(&vps_record, "customer_transactions").unwrap();
        // "transaction_type" should remain as "transaction_type"
        assert_eq!(result["transaction_type"], json!("DEBIT"));
        // Should NOT have a "type" key (that was the bug)
        assert!(result.get("type").is_none());
        // "customer_id" stays (no customer_local_id in VPS for this table)
        assert_eq!(result["customer_id"], json!(5));
    }

    /// Supplier: VPS "company" → Desktop "name" via COLUMN_MAPPINGS
    /// The Serde struct renames "company" to "name" on serialize.
    #[test]
    fn test_company_rename_suppliers() {
        let vps_record = json!({
            "local_id": 3,
            "uuid": "sup-001",
            "name": "Nguyễn Văn A",
            "company": "Công ty TNHH ABC",
            "total_debt": 5000000.0,
            "store_id": 1
        });

        let result = serde_transform_for_pull(&vps_record, "suppliers").unwrap();
        // "company" → serialized as "name"
        assert_eq!(result["name"], json!("Công ty TNHH ABC"));
        // "total_debt" → "current_debt"
        assert_eq!(result["current_debt"], json!(5000000.0));
        assert!(result.get("total_debt").is_none());
        // "store_id" stripped
        assert!(result.get("store_id").is_none());
    }

    /// Invoice: VPS "notes" → Desktop "note"
    #[test]
    fn test_notes_to_note_invoices() {
        let vps_record = json!({
            "local_id": 100,
            "uuid": "inv-001",
            "invoice_number": "HD001",
            "customer_local_id": 5,
            "notes": "Giao hàng trước 5h",
            "total_amount": 250000.0,
            "store_id": 1
        });

        let result = serde_transform_for_pull(&vps_record, "invoices").unwrap();
        // "notes" → "note"
        assert_eq!(result["note"], json!("Giao hàng trước 5h"));
        assert!(result.get("notes").is_none());
        // "customer_local_id" → "customer_id"
        assert_eq!(result["customer_id"], json!(5));
        assert!(result.get("customer_local_id").is_none());
    }
}
