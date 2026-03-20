use serde_json::{json, Value};
use sqlx::PgPool;
use std::collections::HashMap;

use crate::models::sync_models::serde_transform_for_pull;
use crate::models::sync_v2::{
    ChangeRecord, ConflictInfo, MergeRule, PullChangeRecord, get_table_meta,
};

// ============================================================
// PUSH — Process incoming changes
// ============================================================

/// Process a batch of changes from a device push.
/// Returns (new_cursor, processed_count, conflicts, computed_updates).
pub async fn process_push(
    pool: &PgPool,
    store_id: i32,
    device_id: &str,
    batch_id: &str,
    changes: &HashMap<String, Vec<ChangeRecord>>,
) -> Result<(i64, usize, Vec<ConflictInfo>, HashMap<String, HashMap<String, Value>>), crate::error::AppError> {

    // ── 1. Idempotency: check if batch_id already processed ────────────────
    let existing: Option<i64> = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sync_inbox WHERE batch_id = $1 AND store_id = $2 AND status = 'PROCESSED'"
    )
    .bind(batch_id)
    .bind(store_id)
    .fetch_optional(pool)
    .await?;

    if let Some(count) = existing {
        if count > 0 {
            // Already processed — return cached cursor
            let cursor: Option<i64> = sqlx::query_scalar(
                "SELECT MAX(id) FROM sync_journal WHERE store_id = $1"
            ).bind(store_id).fetch_optional(pool).await?.flatten();

            tracing::info!("⏭️ Batch {} already processed for store_id={}", batch_id, store_id);
            return Ok((cursor.unwrap_or(0), 0, vec![], HashMap::new()));
        }
    }

    // ── 2. Register device ─────────────────────────────────────────────────
    sqlx::query(
        "INSERT INTO sync_devices (store_id, device_id, last_push_at) \
         VALUES ($1, $2, NOW()) \
         ON CONFLICT (store_id, device_id) DO UPDATE SET last_push_at = NOW()"
    )
    .bind(store_id)
    .bind(device_id)
    .execute(pool)
    .await?;

    // ── 3. Process each table's changes ────────────────────────────────────
    let mut processed = 0usize;
    let mut conflicts: Vec<ConflictInfo> = Vec::new();
    let mut max_cursor: i64 = 0;
    let mut affected_customer_uuids: Vec<String> = Vec::new();
    let mut affected_product_uuids: Vec<String> = Vec::new();
    let mut affected_supplier_uuids: Vec<String> = Vec::new();

    let mut tx = pool.begin().await?;

    for (table_name, records) in changes {
        let meta = match get_table_meta(table_name) {
            Some(m) => m,
            None => {
                tracing::warn!("Unknown table in push: {}", table_name);
                continue;
            }
        };

        for record in records {
            // Create a savepoint so one bad record doesn't abort the whole transaction
            sqlx::query("SAVEPOINT record_sp").execute(&mut *tx).await?;

            // Write to sync_inbox
            let inbox_result = sqlx::query(
                "INSERT INTO sync_inbox (store_id, device_id, batch_id, table_name, record_uuid, operation, payload) \
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(store_id)
            .bind(device_id)
            .bind(batch_id)
            .bind(table_name.as_str())
            .bind(&record.uuid)
            .bind(&record.operation)
            .bind(&record.data)
            .execute(&mut *tx)
            .await;

            if let Err(e) = inbox_result {
                tracing::error!("Inbox write error for {}.{}: {:?}", table_name, record.uuid, e);
                sqlx::query("ROLLBACK TO SAVEPOINT record_sp").execute(&mut *tx).await?;
                continue;
            }

            // Process based on merge rule
            let result = match record.operation.as_str() {
                "INSERT" => {
                    match meta.merge_rule {
                        MergeRule::UuidDedup => {
                            merge_insert_dedup(&mut tx, store_id, meta.pg_table, table_name, device_id, record).await
                        }
                        MergeRule::Lww => {
                            merge_insert_lww(&mut tx, store_id, meta.pg_table, table_name, device_id, record).await
                        }
                    }
                }
                "UPDATE" => {
                    merge_update_lww(&mut tx, store_id, meta.pg_table, table_name, device_id, record, &mut conflicts).await
                }
                "DELETE" => {
                    // Soft delete not implemented for most tables — skip
                    Ok(MergeOutcome::Skipped)
                }
                _ => {
                    tracing::warn!("Unknown operation: {}", record.operation);
                    Ok(MergeOutcome::Skipped)
                }
            };

            match result {
                Ok(MergeOutcome::Applied) => {
                    processed += 1;

                    // Log to sync_journal
                    let journal_result: Result<i64, _> = sqlx::query_scalar(
                        "SELECT log_sync_journal($1, $2, $3, $4, $5)"
                    )
                    .bind(store_id)
                    .bind(table_name.as_str())
                    .bind(&record.uuid)
                    .bind(&record.operation)
                    .bind(device_id)
                    .fetch_one(&mut *tx)
                    .await;

                    match journal_result {
                        Ok(cursor) => {
                            if cursor > max_cursor {
                                max_cursor = cursor;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Journal error for {}.{}: {:?}", table_name, record.uuid, e);
                            sqlx::query("ROLLBACK TO SAVEPOINT record_sp").execute(&mut *tx).await?;
                            continue;
                        }
                    }

                    // Track affected UUIDs for computed fields
                    if meta.affects_computed {
                        track_affected_uuids(
                            table_name,
                            record,
                            &mut affected_customer_uuids,
                            &mut affected_product_uuids,
                            &mut affected_supplier_uuids,
                        );
                    }

                    // Mark inbox as processed
                    let _ = sqlx::query(
                        "UPDATE sync_inbox SET status = 'PROCESSED', processed_at = NOW() \
                         WHERE batch_id = $1 AND record_uuid = $2 AND table_name = $3 AND store_id = $4"
                    )
                    .bind(batch_id)
                    .bind(&record.uuid)
                    .bind(table_name.as_str())
                    .bind(store_id)
                    .execute(&mut *tx)
                    .await;

                    // Release savepoint on success
                    sqlx::query("RELEASE SAVEPOINT record_sp").execute(&mut *tx).await?;
                }
                Ok(MergeOutcome::Skipped) => {
                    let _ = sqlx::query(
                        "UPDATE sync_inbox SET status = 'SKIPPED', processed_at = NOW() \
                         WHERE batch_id = $1 AND record_uuid = $2 AND table_name = $3 AND store_id = $4"
                    )
                    .bind(batch_id)
                    .bind(&record.uuid)
                    .bind(table_name.as_str())
                    .bind(store_id)
                    .execute(&mut *tx)
                    .await;

                    sqlx::query("RELEASE SAVEPOINT record_sp").execute(&mut *tx).await?;
                }
                Err(e) => {
                    tracing::error!("Merge error for {}.{}: {:?}", table_name, record.uuid, e);
                    // Rollback to savepoint to recover from PG error state
                    sqlx::query("ROLLBACK TO SAVEPOINT record_sp").execute(&mut *tx).await?;

                    // Re-create savepoint and mark as failed
                    sqlx::query("SAVEPOINT record_sp").execute(&mut *tx).await?;
                    let _ = sqlx::query(
                        "UPDATE sync_inbox SET status = 'FAILED', error_message = $5, processed_at = NOW() \
                         WHERE batch_id = $1 AND record_uuid = $2 AND table_name = $3 AND store_id = $4"
                    )
                    .bind(batch_id)
                    .bind(&record.uuid)
                    .bind(table_name.as_str())
                    .bind(store_id)
                    .bind(format!("{:?}", e))
                    .execute(&mut *tx)
                    .await;
                    sqlx::query("RELEASE SAVEPOINT record_sp").execute(&mut *tx).await?;
                }
            }
        }
    }

    // ── 4. Recompute computed fields ───────────────────────────────────────
    let mut computed_updates: HashMap<String, HashMap<String, Value>> = HashMap::new();

    if !affected_product_uuids.is_empty() {
        let product_computed = recompute_stock(&mut tx, store_id, &affected_product_uuids).await?;
        if !product_computed.is_empty() {
            computed_updates.insert("products".to_string(), product_computed);
        }
    }

    if !affected_customer_uuids.is_empty() {
        let customer_computed = recompute_customer_debt(&mut tx, store_id, &affected_customer_uuids).await?;
        if !customer_computed.is_empty() {
            computed_updates.insert("customers".to_string(), customer_computed);
        }
    }

    if !affected_supplier_uuids.is_empty() {
        let supplier_computed = recompute_supplier_debt(&mut tx, store_id, &affected_supplier_uuids).await?;
        if !supplier_computed.is_empty() {
            computed_updates.insert("suppliers".to_string(), supplier_computed);
        }
    }

    tx.commit().await?;

    // If no journal entries were created, get current max cursor
    if max_cursor == 0 {
        let cursor: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(id) FROM sync_journal WHERE store_id = $1"
        ).bind(store_id).fetch_optional(pool).await?.flatten();
        max_cursor = cursor.unwrap_or(0);
    }

    Ok((max_cursor, processed, conflicts, computed_updates))
}

// ============================================================
// Merge outcomes
// ============================================================

enum MergeOutcome {
    Applied,
    Skipped,
}

// ============================================================
// Column Mapping — DEPRECATED (Sprint 140)
// Replaced by Serde structs in models/sync_models.rs for pull.
// Kept as fallback for push flow and any unrecognized tables.
// ============================================================

struct ColumnMap {
    desktop: &'static str,
    vps: &'static str,
}

/// All column mappings between Desktop (SQLite) and VPS (PostgreSQL).
/// "ALL" entries apply globally; table-specific entries override.
const COLUMN_MAPPINGS: &[(&str, &[ColumnMap])] = &[
    ("ALL", &[
        ColumnMap { desktop: "current_debt", vps: "total_debt" },
    ]),
    ("invoices", &[
        ColumnMap { desktop: "note", vps: "notes" },
        ColumnMap { desktop: "customer_id", vps: "customer_local_id" },
    ]),
    ("invoice_items", &[
        ColumnMap { desktop: "subtotal", vps: "total" },
        ColumnMap { desktop: "invoice_id", vps: "invoice_local_id" },
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("invoice_payments", &[
        ColumnMap { desktop: "invoice_id", vps: "invoice_local_id" },
    ]),
    ("purchase_items", &[
        ColumnMap { desktop: "purchase_order_id", vps: "purchase_order_local_id" },
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("product_units", &[
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("product_batches", &[
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("customer_transactions", &[
        ColumnMap { desktop: "customer_id", vps: "customer_local_id" },
    ]),
    ("supplier_transactions", &[
        ColumnMap { desktop: "supplier_id", vps: "supplier_local_id" },
    ]),
    ("product_transactions", &[
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("return_items", &[
        ColumnMap { desktop: "return_id", vps: "return_local_id" },
        ColumnMap { desktop: "product_id", vps: "product_local_id" },
    ]),
    ("suppliers", &[
        ColumnMap { desktop: "name", vps: "company" },
    ]),
];

/// Push: map desktop column name → VPS column name
fn map_column_push(table: &str, desktop_key: &str) -> String {
    // Check table-specific first
    if let Some((_, maps)) = COLUMN_MAPPINGS.iter().find(|(t, _)| *t == table) {
        if let Some(m) = maps.iter().find(|m| m.desktop == desktop_key) {
            return m.vps.to_string();
        }
    }
    // Check global
    if let Some((_, maps)) = COLUMN_MAPPINGS.iter().find(|(t, _)| *t == "ALL") {
        if let Some(m) = maps.iter().find(|m| m.desktop == desktop_key) {
            return m.vps.to_string();
        }
    }
    desktop_key.to_string()
}

/// Pull: map VPS column name → desktop column name
fn map_column_pull(table: &str, vps_key: &str) -> String {
    // Check table-specific first
    if let Some((_, maps)) = COLUMN_MAPPINGS.iter().find(|(t, _)| *t == table) {
        if let Some(m) = maps.iter().find(|m| m.vps == vps_key) {
            return m.desktop.to_string();
        }
    }
    // Check global
    if let Some((_, maps)) = COLUMN_MAPPINGS.iter().find(|(t, _)| *t == "ALL") {
        if let Some(m) = maps.iter().find(|m| m.vps == vps_key) {
            return m.desktop.to_string();
        }
    }
    vps_key.to_string()
}

// ============================================================
// UUID Dedup — Append-only tables
// ============================================================

async fn merge_insert_dedup(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    pg_table: &str,
    table_name: &str,
    device_id: &str,
    record: &ChangeRecord,
) -> Result<MergeOutcome, crate::error::AppError> {
    // Check if UUID already exists
    let sql = format!(
        "SELECT COUNT(*)::bigint FROM {} WHERE store_id = $1 AND uuid = $2",
        pg_table
    );
    let count: Option<i64> = sqlx::query_scalar(&sql)
        .bind(store_id)
        .bind(&record.uuid)
        .fetch_optional(&mut **tx)
        .await?;

    if count.unwrap_or(0) > 0 {
        return Ok(MergeOutcome::Skipped); // Idempotent — already exists
    }

    // Build dynamic INSERT from data payload
    insert_record_from_json(tx, store_id, pg_table, table_name, device_id, record).await?;
    Ok(MergeOutcome::Applied)
}

// ============================================================
// LWW — Mutable tables (INSERT path)
// ============================================================

async fn merge_insert_lww(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    pg_table: &str,
    table_name: &str,
    device_id: &str,
    record: &ChangeRecord,
) -> Result<MergeOutcome, crate::error::AppError> {
    // Check if UUID already exists
    let sql = format!(
        "SELECT COUNT(*)::bigint FROM {} WHERE store_id = $1 AND uuid = $2",
        pg_table
    );
    let count: Option<i64> = sqlx::query_scalar(&sql)
        .bind(store_id)
        .bind(&record.uuid)
        .fetch_optional(&mut **tx)
        .await?;

    if count.unwrap_or(0) > 0 {
        // Exists — treat as UPDATE (LWW path)
        return merge_update_record(tx, store_id, pg_table, table_name, device_id, record).await;
    }

    // New record — INSERT
    insert_record_from_json(tx, store_id, pg_table, table_name, device_id, record).await?;
    Ok(MergeOutcome::Applied)
}

// ============================================================
// LWW — UPDATE path
// ============================================================

async fn merge_update_lww(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    pg_table: &str,
    table_name: &str,
    device_id: &str,
    record: &ChangeRecord,
    conflicts: &mut Vec<ConflictInfo>,
) -> Result<MergeOutcome, crate::error::AppError> {
    // Check if record exists
    let sql = format!(
        "SELECT updated_at_v2::text FROM {} WHERE store_id = $1 AND uuid = $2",
        pg_table
    );
    let server_updated: Option<Option<String>> = sqlx::query_scalar(&sql)
        .bind(store_id)
        .bind(&record.uuid)
        .fetch_optional(&mut **tx)
        .await?;

    match server_updated {
        None => {
            // Record doesn't exist — INSERT it
            insert_record_from_json(tx, store_id, pg_table, table_name, device_id, record).await?;
            Ok(MergeOutcome::Applied)
        }
        Some(server_ts) => {
            let client_ts = record.data.get("updated_at")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let server_ts_str = server_ts.as_deref().unwrap_or("");

            if client_ts > server_ts_str {
                // Client wins — update
                merge_update_record(tx, store_id, pg_table, table_name, device_id, record).await
            } else {
                // Server wins — log conflict
                sqlx::query(
                    "INSERT INTO sync_conflicts (store_id, table_name, record_uuid, device_a, device_b, resolution) \
                     VALUES ($1, $2, $3, 'SERVER', $4, 'SERVER_WINS')"
                )
                .bind(store_id)
                .bind(table_name)
                .bind(&record.uuid)
                .bind(device_id)
                .execute(&mut **tx)
                .await?;

                conflicts.push(ConflictInfo {
                    table_name: table_name.to_string(),
                    record_uuid: record.uuid.clone(),
                    resolution: "SERVER_WINS".to_string(),
                });

                Ok(MergeOutcome::Skipped)
            }
        }
    }
}

// ============================================================
// Generic UPDATE for LWW
// ============================================================

async fn merge_update_record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    pg_table: &str,
    table_name: &str,
    device_id: &str,
    record: &ChangeRecord,
) -> Result<MergeOutcome, crate::error::AppError> {
    let data = match record.data.as_object() {
        Some(obj) => obj,
        None => return Ok(MergeOutcome::Skipped),
    };

    // Build SET clause from JSON keys (skip computed fields)
    let skip_fields = ["id", "uuid", "store_id", "local_id", "items", "payments", "device_id", "updated_at", "updated_at_v2", "synced_at"];
    let mut set_parts: Vec<String> = Vec::new();
    let mut values: Vec<Value> = Vec::new();
    let mut param_idx = 2; // $1=store_id, $2=uuid

    // Query column info (name + data_type) for type-aware casting
    let col_info = get_column_info(&mut **tx, pg_table).await;

    for (key, val) in data {
        if skip_fields.contains(&key.as_str()) {
            continue;
        }
        // Map client field names to PG column names via COLUMN_MAPPINGS
        let col = map_column_push(table_name, key);
        // Skip columns that don't exist in PG
        if !col_info.is_empty() && !col_info.contains_key(&col) {
            continue;
        }
        param_idx += 1;
        let col_type = col_info.get(&col).map(|s| s.as_str()).unwrap_or("");
        let cast = get_cast_suffix(col_type);
        set_parts.push(format!("{} = ${}{}", col, param_idx, cast));
        let coerced = coerce_value_for_type(val, col_type);
        values.push(coerced);
    }

    // Always update device_id and updated_at_v2
    param_idx += 1;
    set_parts.push(format!("device_id = ${}", param_idx));
    values.push(Value::String(device_id.to_string()));

    set_parts.push("updated_at_v2 = NOW()".to_string());

    if set_parts.is_empty() {
        return Ok(MergeOutcome::Skipped);
    }

    let sql = format!(
        "UPDATE {} SET {} WHERE store_id = $1 AND uuid = $2",
        pg_table,
        set_parts.join(", ")
    );

    let mut query = sqlx::query(&sql)
        .bind(store_id)
        .bind(&record.uuid);

    for val in &values {
        query = bind_json_value(query, val);
    }

    query.execute(&mut **tx).await?;
    Ok(MergeOutcome::Applied)
}

// ============================================================
// Generic INSERT from JSON payload
// ============================================================

async fn insert_record_from_json(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    pg_table: &str,
    table_name: &str,
    device_id: &str,
    record: &ChangeRecord,
) -> Result<(), crate::error::AppError> {
    let data = match record.data.as_object() {
        Some(obj) => obj,
        None => return Ok(()),
    };

    let skip_fields = ["id", "items", "payments", "uuid", "store_id", "device_id", "updated_at", "updated_at_v2", "synced_at", "local_id"];

    let mut col_names: Vec<String> = vec!["store_id".to_string(), "uuid".to_string(), "device_id".to_string(), "updated_at_v2".to_string()];
    let mut placeholders: Vec<String> = vec!["$1".to_string(), "$2".to_string(), "$3".to_string(), "NOW()".to_string()];
    let mut values: Vec<Value> = Vec::new();
    let mut param_idx = 3;

    // Auto-generate local_id: use value from JSON if provided, otherwise generate from MAX+1
    let has_local_id = data.get("local_id").and_then(|v| {
        if v.is_null() { None } else { Some(v) }
    });
    if let Some(lid) = has_local_id {
        param_idx += 1;
        col_names.push("local_id".to_string());
        placeholders.push(format!("${}", param_idx));
        values.push(lid.clone());
    } else {
        // Auto-generate: use MAX(local_id) + 1 for this store
        col_names.push("local_id".to_string());
        placeholders.push(format!(
            "(SELECT COALESCE(MAX(local_id), 0) + 1 FROM {} WHERE store_id = $1)",
            pg_table
        ));
    }

    // Query column info (name + data_type) for type-aware casting
    let col_info = get_column_info(&mut **tx, pg_table).await;

    // Auto-generate invoice_local_id for invoice_items/invoice_payments when null
    let needs_invoice_local_id = (pg_table == "synced_invoice_items" || pg_table == "synced_invoice_payments")
        && data.get("invoice_local_id").map_or(true, |v| v.is_null());
    if needs_invoice_local_id && col_info.contains_key("invoice_local_id") {
        // Try to resolve from the invoice's uuid if available
        let invoice_uuid = data.get("invoice_uuid").or_else(|| data.get("invoiceUuid")).and_then(|v| v.as_str());
        if let Some(iuuid) = invoice_uuid {
            param_idx += 1;
            col_names.push("invoice_local_id".to_string());
            placeholders.push(format!(
                "COALESCE((SELECT local_id FROM synced_invoices WHERE store_id = $1 AND uuid = ${}), 0)",
                param_idx
            ));
            values.push(Value::String(iuuid.to_string()));
        }
        // If no invoice_uuid available, let it be null (column is now nullable)
    }

    for (key, val) in data {
        if skip_fields.contains(&key.as_str()) {
            continue;
        }
        // Skip invoice_local_id if we already handled it above
        if needs_invoice_local_id && key == "invoice_local_id" {
            continue;
        }
        let col = map_column_push(table_name, key);
        // Skip columns that don't exist in PG
        if !col_info.is_empty() && !col_info.contains_key(&col) {
            continue;
        }
        param_idx += 1;
        let col_type = col_info.get(&col).map(|s| s.as_str()).unwrap_or("");
        let cast = get_cast_suffix(col_type);
        col_names.push(col);
        placeholders.push(format!("${}{}", param_idx, cast));
        let coerced = coerce_value_for_type(val, col_type);
        values.push(coerced);
    }

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        pg_table,
        col_names.join(", "),
        placeholders.join(", ")
    );

    let mut query = sqlx::query(&sql)
        .bind(store_id)
        .bind(&record.uuid)
        .bind(device_id);

    for val in &values {
        query = bind_json_value(query, val);
    }

    query.execute(&mut **tx).await?;
    Ok(())
}

// ============================================================
// Computed Fields
// ============================================================

async fn recompute_stock(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    product_uuids: &[String],
) -> Result<HashMap<String, Value>, crate::error::AppError> {
    let mut result = HashMap::new();

    for uuid in product_uuids {
        // Get the product's local_id by uuid
        let local_id: Option<i32> = sqlx::query_scalar(
            "SELECT local_id FROM synced_products WHERE store_id = $1 AND uuid = $2"
        )
        .bind(store_id)
        .bind(uuid)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(pid) = local_id {
            let stock: Option<f64> = sqlx::query_scalar(
                "SELECT COALESCE(SUM(\
                    CASE WHEN transaction_type = 'IN' THEN quantity \
                         WHEN transaction_type = 'ADJUST' THEN quantity \
                         ELSE -quantity END \
                 ), 0)::float8 \
                 FROM synced_product_transactions \
                 WHERE store_id = $1 AND product_id = $2"
            )
            .bind(store_id)
            .bind(pid as i64)
            .fetch_optional(&mut **tx)
            .await?;

            if let Some(qty) = stock {
                sqlx::query(
                    "UPDATE synced_products SET stock_quantity = $3 WHERE store_id = $1 AND uuid = $2"
                )
                .bind(store_id)
                .bind(uuid)
                .bind(qty)
                .execute(&mut **tx)
                .await?;

                result.insert(uuid.clone(), json!({ "stock_quantity": qty }));
            }
        }
    }

    Ok(result)
}

async fn recompute_customer_debt(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    customer_uuids: &[String],
) -> Result<HashMap<String, Value>, crate::error::AppError> {
    let mut result = HashMap::new();

    for uuid in customer_uuids {
        let local_id: Option<i32> = sqlx::query_scalar(
            "SELECT local_id FROM synced_customers WHERE store_id = $1 AND uuid = $2"
        )
        .bind(store_id)
        .bind(uuid)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(cid) = local_id {
            let debt: Option<f64> = sqlx::query_scalar(
                "SELECT COALESCE(SUM(\
                    CASE WHEN transaction_type = 'DEBIT' THEN amount ELSE -amount END \
                 ), 0)::float8 \
                 FROM synced_customer_transactions \
                 WHERE store_id = $1 AND customer_id = $2"
            )
            .bind(store_id)
            .bind(cid as i64)
            .fetch_optional(&mut **tx)
            .await?;

            if let Some(d) = debt {
                sqlx::query(
                    "UPDATE synced_customers SET total_debt = $3 WHERE store_id = $1 AND uuid = $2"
                )
                .bind(store_id)
                .bind(uuid)
                .bind(d)
                .execute(&mut **tx)
                .await?;

                result.insert(uuid.clone(), json!({ "current_debt": d }));
            }
        }
    }

    Ok(result)
}

async fn recompute_supplier_debt(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    store_id: i32,
    supplier_uuids: &[String],
) -> Result<HashMap<String, Value>, crate::error::AppError> {
    let mut result = HashMap::new();

    for uuid in supplier_uuids {
        let local_id: Option<i32> = sqlx::query_scalar(
            "SELECT local_id FROM synced_suppliers WHERE store_id = $1 AND uuid = $2"
        )
        .bind(store_id)
        .bind(uuid)
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(sid) = local_id {
            let debt: Option<f64> = sqlx::query_scalar(
                "SELECT COALESCE(SUM(amount), 0)::float8 \
                 FROM synced_supplier_transactions \
                 WHERE store_id = $1 AND supplier_id = $2"
            )
            .bind(store_id)
            .bind(sid as i64)
            .fetch_optional(&mut **tx)
            .await?;

            if let Some(d) = debt {
                sqlx::query(
                    "UPDATE synced_suppliers SET total_debt = $3 WHERE store_id = $1 AND uuid = $2"
                )
                .bind(store_id)
                .bind(uuid)
                .bind(d)
                .execute(&mut **tx)
                .await?;

                result.insert(uuid.clone(), json!({ "current_debt": d }));
            }
        }
    }

    Ok(result)
}

// ============================================================
// PULL — Build changes from sync_journal
// ============================================================

pub async fn build_pull_changes(
    pool: &PgPool,
    store_id: i32,
    device_id: &str,
    cursor: i64,
    limit: i64,
) -> Result<(i64, bool, HashMap<String, Vec<PullChangeRecord>>, HashMap<String, HashMap<String, Value>>), crate::error::AppError> {
    // Register / update device
    sqlx::query(
        "INSERT INTO sync_devices (store_id, device_id, last_pull_at) \
         VALUES ($1, $2, NOW()) \
         ON CONFLICT (store_id, device_id) DO UPDATE SET last_pull_at = NOW()"
    )
    .bind(store_id)
    .bind(device_id)
    .execute(pool)
    .await?;

    // Query sync_journal — exclude records from the requesting device
    // Deduplicate: keep only the LATEST journal entry per (table_name, record_uuid)
    let rows: Vec<(i64, String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT DISTINCT ON (table_name, record_uuid) id, table_name, record_uuid, operation, source_device_id \
         FROM sync_journal \
         WHERE store_id = $1 AND id > $2 AND (source_device_id IS NULL OR source_device_id != $3) \
         ORDER BY table_name, record_uuid, id DESC"
    )
    .bind(store_id)
    .bind(cursor)
    .bind(device_id)
    .fetch_all(pool)
    .await?;

    // Sort by id ASC for cursor calculation, then apply limit
    let mut sorted_rows = rows;
    sorted_rows.sort_by_key(|r| r.0);

    let has_more = sorted_rows.len() as i64 > limit;
    let rows_to_process: Vec<_> = if has_more {
        sorted_rows[..limit as usize].to_vec()
    } else {
        sorted_rows
    };

    let new_cursor = rows_to_process.last().map(|r| r.0).unwrap_or(cursor);

    // Group by table and fetch actual data
    let mut changes: HashMap<String, Vec<PullChangeRecord>> = HashMap::new();
    let mut affected_product_uuids: Vec<String> = Vec::new();
    let mut affected_customer_uuids: Vec<String> = Vec::new();
    let mut affected_supplier_uuids: Vec<String> = Vec::new();

    for (_, table_name, record_uuid, operation, _) in &rows_to_process {
        let meta = match get_table_meta(table_name) {
            Some(m) => m,
            None => continue,
        };

        // Fetch the full record as JSON
        let sql = format!(
            "SELECT row_to_json(t) FROM (SELECT * FROM {} WHERE store_id = $1 AND uuid = $2) t",
            meta.pg_table
        );

        let record_data: Option<Value> = sqlx::query_scalar(&sql)
            .bind(store_id)
            .bind(record_uuid)
            .fetch_optional(pool)
            .await?;

        let mut data = record_data.unwrap_or(json!({}));

        // ── Transform data for client compatibility ──────────────────
        transform_record_for_pull(&mut data, table_name);

        let entry = changes.entry(table_name.clone()).or_insert_with(Vec::new);
        entry.push(PullChangeRecord {
            uuid: record_uuid.clone(),
            operation: operation.clone(),
            data,
        });

        // Track affected UUIDs for computed fields
        if meta.affects_computed {
            match table_name.as_str() {
                "product_transactions" | "products" => affected_product_uuids.push(record_uuid.clone()),
                "customer_transactions" | "customers" => affected_customer_uuids.push(record_uuid.clone()),
                "supplier_transactions" | "suppliers" => affected_supplier_uuids.push(record_uuid.clone()),
                _ => {}
            }
        }
    }

    // Build computed updates for pull
    let mut computed_updates: HashMap<String, HashMap<String, Value>> = HashMap::new();

    if !affected_product_uuids.is_empty() {
        // Get product UUIDs that these transactions reference
        let product_updates = pull_product_computed(pool, store_id).await?;
        if !product_updates.is_empty() {
            computed_updates.insert("products".to_string(), product_updates);
        }
    }

    if !affected_customer_uuids.is_empty() {
        let customer_updates = pull_customer_computed(pool, store_id).await?;
        if !customer_updates.is_empty() {
            computed_updates.insert("customers".to_string(), customer_updates);
        }
    }

    // Update device pull_cursor
    sqlx::query(
        "UPDATE sync_devices SET pull_cursor = $3 WHERE store_id = $1 AND device_id = $2"
    )
    .bind(store_id)
    .bind(device_id)
    .bind(new_cursor)
    .execute(pool)
    .await?;

    Ok((new_cursor, has_more, changes, computed_updates))
}

// Pull-time computed field helpers
async fn pull_product_computed(
    pool: &PgPool,
    store_id: i32,
) -> Result<HashMap<String, Value>, crate::error::AppError> {
    let mut result = HashMap::new();

    let rows: Vec<(String, Option<f64>)> = sqlx::query_as(
        "SELECT p.uuid, p.stock_quantity::float8 \
         FROM synced_products p \
         WHERE p.store_id = $1 AND p.uuid IS NOT NULL"
    )
    .bind(store_id)
    .fetch_all(pool)
    .await?;

    for (uuid, stock) in rows {
        result.insert(uuid, json!({ "stock_quantity": stock }));
    }

    Ok(result)
}

async fn pull_customer_computed(
    pool: &PgPool,
    store_id: i32,
) -> Result<HashMap<String, Value>, crate::error::AppError> {
    let mut result = HashMap::new();

    let rows: Vec<(String, Option<f64>)> = sqlx::query_as(
        "SELECT c.uuid, c.total_debt::float8 \
         FROM synced_customers c \
         WHERE c.store_id = $1 AND c.uuid IS NOT NULL"
    )
    .bind(store_id)
    .fetch_all(pool)
    .await?;

    for (uuid, debt) in rows {
        result.insert(uuid, json!({ "current_debt": debt }));
    }

    Ok(result)
}

// ============================================================
// Pull transform helper — shared by build_pull_changes & build_snapshot
// ============================================================

/// Transform a row_to_json record into the format Desktop expects.
///
/// Sprint 140: tries Serde-based transform first (compile-time column mapping
/// via sync_models.rs). Falls back to COLUMN_MAPPINGS for unknown tables.
///
/// Applies: local_id/client_id→id, created_at fallback, column renames,
/// and removes server-internal fields.
fn transform_record_for_pull(data: &mut Value, table_name: &str) {
    // ── Pre-step: Add created_at from synced_at if missing ─────────────
    // Must happen before Serde transform since structs may expect created_at.
    if let Some(obj) = data.as_object_mut() {
        if !obj.contains_key("created_at") || obj.get("created_at").map_or(false, |v| v.is_null()) {
            if let Some(synced_at) = obj.get("synced_at").cloned() {
                obj.insert("created_at".to_string(), synced_at);
            }
        }
    }

    // ── Sprint 140: Try Serde transform first ─────────────────────────
    // Serde structs handle: local_id→id, column renames, skip server fields.
    if let Some(transformed) = serde_transform_for_pull(data, table_name) {
        *data = transformed;
        return;
    }

    // ── Fallback: manual transform via COLUMN_MAPPINGS (deprecated) ───
    if let Some(obj) = data.as_object_mut() {
        // 1. Map local_id → id (client expects id = local SQLite rowid)
        if let Some(local_id) = obj.get("local_id").cloned() {
            obj.insert("id".to_string(), local_id);
        }
        // Also handle client_id (some tables use client_id instead of local_id)
        if let Some(client_id) = obj.get("client_id").cloned() {
            if !obj.contains_key("local_id") {
                obj.insert("id".to_string(), client_id);
            }
        }

        // 2. Column name renames — via COLUMN_MAPPINGS (deprecated fallback)
        let vps_keys: Vec<String> = obj.keys().cloned().collect();
        let mut renames: Vec<(String, Value)> = Vec::new();
        for vps_key in &vps_keys {
            let desktop_key = map_column_pull(table_name, vps_key);
            if desktop_key != *vps_key {
                if let Some(v) = obj.remove(vps_key.as_str()) {
                    if !obj.contains_key(&desktop_key) {
                        renames.push((desktop_key, v));
                    } else {
                        // Target already exists, put back
                        obj.insert(vps_key.clone(), v);
                    }
                }
            }
        }
        for (desktop_key, val) in renames {
            obj.insert(desktop_key, val);
        }

        // 3. Remove server-internal fields
        for key in &["store_id", "synced_at", "device_id", "updated_at_v2",
                     "local_id", "client_id"] {
            obj.remove(*key);
        }
    }
}

// ============================================================
// SNAPSHOT — Full state dump for fresh devices (Sprint 137)
// ============================================================

/// All tables to include in snapshot: (client_name, pg_table).
const SNAPSHOT_TABLES: &[(&str, &str)] = &[
    ("products", "synced_products"),
    ("product_units", "synced_product_units"),
    ("customers", "synced_customers"),
    ("suppliers", "synced_suppliers"),
    ("invoices", "synced_invoices"),
    ("invoice_items", "synced_invoice_items"),
    ("invoice_payments", "synced_invoice_payments"),
    ("purchase_orders", "synced_purchase_orders"),
    ("purchase_items", "synced_purchase_items"),
    ("customer_transactions", "synced_customer_transactions"),
    ("supplier_transactions", "synced_supplier_transactions"),
    ("cash_transactions", "synced_cash_transactions"),
    ("product_transactions", "synced_product_transactions"),
    ("product_batches", "synced_product_batches"),
    ("payment_vouchers", "synced_payment_vouchers"),
    ("store_funds", "synced_store_funds"),
    ("store_settings", "synced_store_settings"),
    ("returns", "synced_returns"),
    ("return_items", "synced_return_items"),
    ("daily_closings", "synced_daily_closings"),
    ("promotions", "synced_promotions"),
    ("vouchers", "synced_vouchers"),
    ("loyalty_transactions_v2", "synced_loyalty_transactions_v2"),
];

/// Build a full snapshot of all synced tables for a store.
/// Returns (snapshot_map, watermark_cursor).
pub async fn build_snapshot(
    pool: &PgPool,
    store_id: i32,
) -> Result<(HashMap<String, Vec<Value>>, i64), crate::error::AppError> {
    let mut snapshot: HashMap<String, Vec<Value>> = HashMap::new();

    for &(client_name, pg_table) in SNAPSHOT_TABLES {
        let sql = format!(
            "SELECT row_to_json(t) FROM (SELECT * FROM {} WHERE store_id = $1) t",
            pg_table
        );

        let rows: Vec<Value> = sqlx::query_scalar(&sql)
            .bind(store_id)
            .fetch_all(pool)
            .await
            .unwrap_or_default();

        let mut transformed: Vec<Value> = Vec::with_capacity(rows.len());
        for mut row in rows {
            transform_record_for_pull(&mut row, client_name);
            transformed.push(row);
        }

        snapshot.insert(client_name.to_string(), transformed);
    }

    // Watermark cursor: MAX(id) from sync_journal for this store
    let watermark: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(id), 0) FROM sync_journal WHERE store_id = $1"
    )
    .bind(store_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    tracing::info!(
        "📸 Snapshot: store_id={}, tables={}, watermark={}",
        store_id, SNAPSHOT_TABLES.len(), watermark
    );

    Ok((snapshot, watermark))
}

// ============================================================
// Helpers
// ============================================================

/// Track which business entities are affected by a change
fn track_affected_uuids(
    table_name: &str,
    record: &ChangeRecord,
    customer_uuids: &mut Vec<String>,
    product_uuids: &mut Vec<String>,
    supplier_uuids: &mut Vec<String>,
) {
    match table_name {
        "customers" => customer_uuids.push(record.uuid.clone()),
        "products" => product_uuids.push(record.uuid.clone()),
        "suppliers" => supplier_uuids.push(record.uuid.clone()),
        "customer_transactions" => {
            // The customer UUID affected is referenced by customer_id in the data
            // For now, we track via the customer_id_uuid field if available
            if let Some(cust_uuid) = record.data.get("customer_id_uuid").and_then(|v| v.as_str()) {
                customer_uuids.push(cust_uuid.to_string());
            }
        }
        "product_transactions" => {
            if let Some(prod_uuid) = record.data.get("product_id_uuid").and_then(|v| v.as_str()) {
                product_uuids.push(prod_uuid.to_string());
            }
        }
        "supplier_transactions" => {
            if let Some(sup_uuid) = record.data.get("supplier_id_uuid").and_then(|v| v.as_str()) {
                supplier_uuids.push(sup_uuid.to_string());
            }
        }
        _ => {}
    }
}

// map_column_name removed — replaced by map_column_push/map_column_pull (Sprint 134B)

/// Query column info from information_schema for type-aware casting
async fn get_column_info(
    conn: &mut sqlx::PgConnection,
    pg_table: &str,
) -> HashMap<String, String> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT column_name::text, data_type::text FROM information_schema.columns WHERE table_name = $1"
    )
    .bind(pg_table)
    .fetch_all(&mut *conn)
    .await
    .unwrap_or_default();

    rows.into_iter().collect()
}

/// Return the SQL cast suffix for a given PG data_type.
/// This ensures JSON string values are properly cast to timestamps, etc.
/// Note: boolean is NOT cast via SQL because PG can't cast double→boolean;
/// instead, we coerce the value in Rust via coerce_value_for_type().
fn get_cast_suffix(data_type: &str) -> String {
    match data_type {
        "timestamp without time zone" => "::timestamp".to_string(),
        "timestamp with time zone" => "::timestamptz".to_string(),
        "numeric" => "::numeric".to_string(),
        "integer" | "smallint" => "::integer".to_string(),
        "bigint" => "::bigint".to_string(),
        "date" => "::date".to_string(),
        _ => String::new(), // text, double precision, boolean, etc.
    }
}

/// Pre-convert a JSON value to match the target PG column type.
/// Handles: float 0.0/1.0 → boolean, string "0"/"1"/"true"/"false" → boolean,
/// and integer/bigint conversion from floats.
fn coerce_value_for_type(val: &Value, data_type: &str) -> Value {
    match data_type {
        "boolean" => {
            match val {
                Value::Number(n) => {
                    let b = n.as_f64().map(|f| f != 0.0).unwrap_or(false);
                    Value::Bool(b)
                }
                Value::String(s) => {
                    let b = matches!(s.as_str(), "1" | "true" | "TRUE" | "True" | "yes");
                    Value::Bool(b)
                }
                _ => val.clone(),
            }
        }
        "integer" | "smallint" | "bigint" => {
            match val {
                Value::Number(n) if n.is_f64() && !n.is_i64() => {
                    // Convert float to integer (e.g., 5.0 → 5)
                    Value::Number(serde_json::Number::from(n.as_f64().unwrap_or(0.0) as i64))
                }
                _ => val.clone(),
            }
        }
        _ => val.clone(),
    }
}

/// Bind a serde_json::Value to a SQLx query
fn bind_json_value<'q>(
    query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    val: &'q Value,
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match val {
        Value::Null => query.bind(None::<String>),
        Value::Bool(b) => query.bind(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                query.bind(i)
            } else if let Some(f) = n.as_f64() {
                query.bind(f)
            } else {
                query.bind(n.to_string())
            }
        }
        Value::String(s) => query.bind(s.as_str()),
        Value::Array(_) | Value::Object(_) => query.bind(val.to_string()),
    }
}
