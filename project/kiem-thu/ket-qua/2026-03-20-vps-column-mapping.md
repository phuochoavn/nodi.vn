# QA Report: Sprint 134B — Column Mapping Fix + Schema Sync

**Ngày:** 2026-03-20  
**Build:** `cargo check` → ✅ 0 errors (15 warnings, không liên quan)

---

| # | Kiểm tra | KQ | Bằng chứng |
|---|----------|----|------------|
| 1 | `cargo check` → 0 errors | ✅ PASS | `Finished dev profile, 15 warnings` |
| 2 | `map_column_name()` ĐÃ XÓA hoàn toàn | ✅ PASS | `merge_engine.rs:1060` — comment xác nhận đã xóa. Grep toàn bộ `api/src/` chỉ tìm thấy comment này |
| 3 | `COLUMN_MAPPINGS` tồn tại, ≥10 table groups | ✅ PASS | `merge_engine.rs:274-317` — 12 groups: ALL, invoices, invoice_items, invoice_payments, purchase_items, product_units, product_batches, customer_transactions, supplier_transactions, product_transactions, return_items, suppliers |
| 4 | COLUMN_MAPPINGS: `"type"` → `"transaction_type"` (ALL group) | ✅ PASS | `merge_engine.rs:277` — `ColumnMap { desktop: "type", vps: "transaction_type" }` trong group "ALL" |
| 5 | COLUMN_MAPPINGS: `"current_debt"` → `"total_debt"` | ✅ PASS | `merge_engine.rs:276` — `ColumnMap { desktop: "current_debt", vps: "total_debt" }` trong group "ALL" |
| 6 | Không còn pattern `"type"` (quoted type) ngoài COLUMN_MAPPINGS | ✅ PASS | `grep '"type"' merge_engine.rs` → chỉ line 277 (trong COLUMN_MAPPINGS) |
| 7 | `map_column_push(table, key)` tồn tại, 2 params | ✅ PASS | `merge_engine.rs:320` — `fn map_column_push(table: &str, desktop_key: &str) -> String` |
| 8 | `map_column_pull(table, key)` tồn tại, 2 params | ✅ PASS | `merge_engine.rs:337` — `fn map_column_pull(table: &str, vps_key: &str) -> String` |
| 9 | Push functions có param `table_name` | ✅ PASS | `merge_insert_dedup` L357-361, `merge_insert_lww` L389-396, `merge_update_record` L486-492, `insert_record_from_json` L559-565 — tất cả có `table_name: &str` |
| 10 | Push functions gọi `map_column_push()` | ✅ PASS | `merge_engine.rs:513` — `merge_update_record` gọi `map_column_push(table_name, key)`. `merge_engine.rs:626` — `insert_record_from_json` gọi `map_column_push(table_name, key)`. Không có lời gọi `map_column_name` |
| 11 | Pull transform dùng `map_column_pull()` | ✅ PASS | `merge_engine.rs:901-920` — section "Column name renames — via COLUMN_MAPPINGS" gọi `map_column_pull(table_name, vps_key)` |
| 12 | Hardcoded `generic_renames` array ĐÃ XÓA | ✅ PASS | Grep `generic_renames` → 0 kết quả |
| 13 | Hardcoded table-specific renames ĐÃ XÓA → COLUMN_MAPPINGS | ✅ PASS | Không còn hardcoded renames. `invoices` L280 `note↔notes`, `invoice_items` L284 `subtotal↔total` |
| 14 | `synced_products` có column `sell_price` | ✅ PASS | `db.rs:246` — `ADD COLUMN IF NOT EXISTS sell_price DOUBLE PRECISION` |
| 15 | `synced_products` có column `sku` | ✅ PASS | `db.rs:247` — `ADD COLUMN IF NOT EXISTS sku TEXT` |
| 16 | `synced_products` có column `min_stock` | ✅ PASS | `db.rs:248` — `ADD COLUMN IF NOT EXISTS min_stock DOUBLE PRECISION` |
| 17 | `synced_invoice_items` có column `created_at` | ✅ PASS | `db.rs:249` — `ADD COLUMN IF NOT EXISTS created_at TEXT` |
| 18 | `synced_invoice_items` có column `product_sku` | ✅ PASS | `db.rs:250` — `ADD COLUMN IF NOT EXISTS product_sku TEXT` |
| 19 | `synced_store_funds` trong `synced_tables` array | ✅ PASS | `db.rs:214` — `"synced_store_funds"` trong array |
| 20 | `store_funds` trong `get_table_meta()` với `MergeRule::Lww` | ✅ PASS | `sync_v2.rs:101` — `"store_funds" => Some(TableMeta { ..., merge_rule: MergeRule::Lww, ... })` |

---

## Tổng kết

| Tổng | PASS | FAIL |
|------|------|------|
| 20   | **20** | 0  |

**Kết luận:** Sprint 134B — Column Mapping Fix + Schema Sync **ĐẠT YÊU CẦU** ✅
