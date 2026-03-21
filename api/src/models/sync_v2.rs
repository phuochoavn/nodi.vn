use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================
// Push Request / Response
// ============================================================

#[derive(Deserialize)]
pub struct SyncV2PushRequest {
    pub device_id: String,
    pub batch_id: String,
    /// Map of table_name → list of change records
    pub changes: HashMap<String, Vec<ChangeRecord>>,
    /// Client sends MAX(sync_journal.id) from this batch; VPS echoes it back
    #[serde(default)]
    pub max_journal_id: Option<i64>,
}

#[derive(Deserialize, Clone)]
pub struct ChangeRecord {
    pub uuid: String,
    pub operation: String, // INSERT, UPDATE, DELETE
    pub data: Value,       // Full record data as JSON
}

#[derive(Serialize)]
pub struct SyncV2PushResponse {
    pub success: bool,
    pub message: String,
    pub data: PushResponseData,
}

#[derive(Serialize)]
pub struct PushResponseData {
    pub new_cursor: i64,
    pub processed: usize,
    pub conflicts: Vec<ConflictInfo>,
    pub computed_updates: HashMap<String, HashMap<String, Value>>,
}

#[derive(Serialize, Clone)]
pub struct ConflictInfo {
    pub table_name: String,
    pub record_uuid: String,
    pub resolution: String, // SERVER_WINS, CLIENT_WINS, MERGED
}

// ============================================================
// Pull Response
// ============================================================

#[derive(Serialize)]
pub struct SyncV2PullResponse {
    pub success: bool,
    pub data: PullResponseData,
}

#[derive(Serialize)]
pub struct PullResponseData {
    pub cursor: i64,
    pub has_more: bool,
    pub changes: HashMap<String, Vec<PullChangeRecord>>,
    pub computed_updates: HashMap<String, HashMap<String, Value>>,
}

#[derive(Serialize)]
pub struct PullChangeRecord {
    pub uuid: String,
    pub operation: String,
    pub data: Value,
}

// ============================================================
// Table metadata — merge rules
// ============================================================

#[derive(Clone, Copy, PartialEq)]
pub enum MergeRule {
    UuidDedup,  // Append-only: INSERT if UUID not exists, skip if exists
    Lww,        // Last-Write-Wins by updated_at
}

pub struct TableMeta {
    pub pg_table: &'static str,  // actual PG table name (synced_*)
    pub merge_rule: MergeRule,
    /// Does this table affect computed fields?
    pub affects_computed: bool,
}

/// Get metadata for a given client table name.
/// Returns None if the table name is not recognized.
pub fn get_table_meta(client_name: &str) -> Option<TableMeta> {
    match client_name {
        // ── Mutable (LWW) ──
        "customers" => Some(TableMeta { pg_table: "synced_customers", merge_rule: MergeRule::Lww, affects_computed: true }),
        "products" => Some(TableMeta { pg_table: "synced_products", merge_rule: MergeRule::Lww, affects_computed: true }),
        "suppliers" => Some(TableMeta { pg_table: "synced_suppliers", merge_rule: MergeRule::Lww, affects_computed: true }),
        "product_units" => Some(TableMeta { pg_table: "synced_product_units", merge_rule: MergeRule::Lww, affects_computed: false }),
        "product_batches" => Some(TableMeta { pg_table: "synced_product_batches", merge_rule: MergeRule::Lww, affects_computed: false }),
        "promotions" => Some(TableMeta { pg_table: "synced_promotions", merge_rule: MergeRule::Lww, affects_computed: false }),
        "vouchers" => Some(TableMeta { pg_table: "synced_vouchers", merge_rule: MergeRule::Lww, affects_computed: false }),
        "store_settings" => Some(TableMeta { pg_table: "synced_store_settings", merge_rule: MergeRule::Lww, affects_computed: false }),
        "store_funds" => Some(TableMeta { pg_table: "synced_store_funds", merge_rule: MergeRule::Lww, affects_computed: false }),

        // ── Append-only (UUID dedup) ──
        "invoices" => Some(TableMeta { pg_table: "synced_invoices", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "invoice_items" => Some(TableMeta { pg_table: "synced_invoice_items", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "invoice_payments" => Some(TableMeta { pg_table: "synced_invoice_payments", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "customer_transactions" => Some(TableMeta { pg_table: "synced_customer_transactions", merge_rule: MergeRule::UuidDedup, affects_computed: true }),
        "supplier_transactions" => Some(TableMeta { pg_table: "synced_supplier_transactions", merge_rule: MergeRule::UuidDedup, affects_computed: true }),
        "cash_transactions" => Some(TableMeta { pg_table: "synced_cash_transactions", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "product_transactions" => Some(TableMeta { pg_table: "synced_product_transactions", merge_rule: MergeRule::UuidDedup, affects_computed: true }),
        "purchase_orders" => Some(TableMeta { pg_table: "synced_purchase_orders", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "purchase_items" => Some(TableMeta { pg_table: "synced_purchase_items", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "payment_vouchers" => Some(TableMeta { pg_table: "synced_payment_vouchers", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "returns" => Some(TableMeta { pg_table: "synced_returns", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "return_items" => Some(TableMeta { pg_table: "synced_return_items", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "daily_closings" => Some(TableMeta { pg_table: "synced_daily_closings", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "loyalty_transactions" => Some(TableMeta { pg_table: "synced_loyalty_transactions_v2", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "input_invoices" => Some(TableMeta { pg_table: "synced_purchase_orders", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        "input_invoice_items" => Some(TableMeta { pg_table: "synced_purchase_items", merge_rule: MergeRule::UuidDedup, affects_computed: false }),
        _ => None,
    }
}
