# 🐛 Sync Known Issues — Danh Sách Bugs Đã Fix

> **MỤC ĐÍCH**: Agent mới đọc file này để KHÔNG lặp lại lỗi cũ.
> Mỗi bug ghi: triệu chứng → root cause → fix → file thay đổi.

---

## Bug #1: Push gửi 0 records dù có data mới
- **Ngày**: 16/03/2026
- **Triệu chứng**: `[V2Push] Total changes built: 0`
- **Root cause**: `build_changes_from_journal()` skip entries có `record_uuid = NULL`. Records cũ (trước migration 049) không có UUID.
- **Fix**: Generate UUID inline khi gặp NULL, update cả record gốc và journal entry.
- **File**: `sync_v2_push.rs` (line 258-287)

---

## Bug #2: Pull crash — VPS-only columns không tồn tại trong SQLite
- **Ngày**: 16/03/2026
- **Triệu chứng**: UPSERT fail cho mọi record từ VPS
- **Root cause**: VPS gửi columns `local_id`, `store_id`, `device_id`, `updated_at_v2` — App SQLite không có.
- **Fix**: PRAGMA table_info() check → skip columns không tồn tại, log diagnostic.
- **File**: `sync_v2_pull.rs` (upsert_by_uuid)

---

## Bug #3: Pull — Column name mismatch
- **Ngày**: 16/03/2026
- **Triệu chứng**: Insert fail do column không khớp
- **Root cause**: VPS dùng `transaction_type`, App dùng `type`. VPS dùng `company`, App dùng `name`.
- **Fix**: Column mapping trong `upsert_by_uuid`.
- **File**: `sync_v2_pull.rs`
- **Tham khảo**: `sync-schema-contract.md` → Column Name Mapping

---

## Bug #4: Pull — NOT NULL constraint failures
- **Ngày**: 16/03/2026
- **Triệu chứng**: Insert fail do `created_at`, `stock_quantity`, `code` bị NULL
- **Root cause**: VPS không gửi đầy đủ NOT NULL fields cho mọi bảng.
- **Fix**: Auto-fill defaults: `created_at` = now(), `stock_quantity` = 0, `code` = UUID prefix, etc.
- **File**: `sync_v2_pull.rs`

---

## Bug #5: Pull — Foreign Key constraint failures
- **Ngày**: 16/03/2026
- **Triệu chứng**: Insert invoice_items fail do `invoice_id` FK constraint
- **Root cause**: Child records insert trước parent records.
- **Fix 1**: Sort insert order: parent tables trước child tables.
- **Fix 2**: `PRAGMA foreign_keys = OFF` trước pull, `ON` sau commit.
- **File**: `sync_v2_pull.rs`

---

## Bug #6: Pull — UNIQUE constraint failures (barcode, code)
- **Ngày**: 16/03/2026
- **Triệu chứng**: UPSERT fail do trùng barcode (products) hoặc code (suppliers)
- **Root cause**: Demo seed data (migration 048) tạo records trùng với VPS data.
- **Fix 1**: DELETE conflicting records trước UPSERT.
- **Fix 2**: Disable demo seed migration 048 cho production.
- **File**: `sync_v2_pull.rs`, `migrations.rs`

---

## Bug #7: VPS Push — `local_id NOT NULL` constraint
- **Ngày**: 16/03/2026
- **Triệu chứng**: `processed: 0` — tất cả records reject
- **Root cause**: 15/16 bảng PG có `local_id NOT NULL`, App V2 không gửi local_id.
- **Fix**: VPS auto-generate local_id khi NULL.
- **File**: VPS `merge_engine.rs`

---

## Bug #8: VPS debts API 500 error
- **Ngày**: 16/03/2026
- **Triệu chứng**: `/api/dashboard/debts` trả 500
- **Root cause**: 3 SQL bugs: `current_debt` → `total_debt` column rename, `supplier_type` → `company`, `NUMERIC` → `::FLOAT8` cast.
- **Fix**: VPS SQL query sửa 6 chỗ.
- **File**: VPS `dashboard.rs`

---

## Bug #9: ⚡ CRITICAL — `mark_journal_synced` marks ALL entries
- **Ngày**: 16/03/2026
- **Triệu chứng**: VPS processed 4/7 records nhưng App mark cả 7 = synced → 3 records mất vĩnh viễn
- **Root cause**: `mark_journal_synced()` chạy `UPDATE sync_journal SET synced = 1 WHERE synced = 0` TRƯỚC KHI đọc VPS response.
- **Fix**: Đọc `processed` count trước → chỉ mark synced khi `processed > 0`.
- **File**: `sync_v2_push.rs` (line 156-192)
- **⚠️ VẪN CÒN**: Mark all khi processed > 0, nên mark từng record cụ thể.

---

## Bug #10: VPS — Type casting text→timestamp, float→boolean
- **Ngày**: 16/03/2026
- **Triệu chứng**: `synced_invoices` trống dù processed > 0
- **Root cause**: `bind_json_value()` bind tất cả dưới dạng text. PG từ chối:
  - `created_at` text → timestamp
  - `is_tax_invoice` float 1.0 → boolean
  - `is_active` float 0.0/1.0 → boolean
- **Fix**: VPS `get_column_info()` + `coerce_value_for_type()` + `get_cast_suffix()`.
- **File**: VPS `merge_engine.rs`
- **Tham khảo**: `sync-schema-contract.md` → Type Casting Rules

---

## Bug #11: VPS — Missing columns + NOT NULL violations
- **Ngày**: 16/03/2026
- **Triệu chứng**: invoice_items/invoice_payments insert fail
- **Root cause**: 
  - `synced_invoice_payments` thiếu column `local_id`
  - `synced_invoice_items.invoice_local_id` NOT NULL nhưng App không gửi
- **Fix**: ALTER TABLE ADD COLUMN + DROP NOT NULL + auto-resolve từ invoice_uuid.
- **File**: VPS migration + `merge_engine.rs`

---

## Bug #12: VPS — Unknown columns (capital_price, active_ingredient)
- **Ngày**: 16/03/2026
- **Triệu chứng**: Products push fail
- **Root cause**: App gửi `capital_price` (invoice_items) và `active_ingredient` (products) — VPS PG không có columns này.
- **Fix**: VPS query `information_schema.columns` → whitelist valid columns → skip unknown.
- **File**: VPS `merge_engine.rs`

---

## Bug #13: V2 Pull không thấy data V1 — sync_journal trống cho V1-pushed data
- **Ngày**: 18/03/2026
- **Triệu chứng**: Mobile login OK, products hiện, nhưng invoices trống. Web dashboard hiện 2 đơn hàng.
- **Root cause**: V1 sync handler (`/api/sync`) ghi data vào `synced_*` tables nhưng **KHÔNG tạo `sync_journal` entries**. V2 Pull (`/api/v2/sync/pull`) đọc **chỉ từ `sync_journal`** → data V1 invisible.
- **Fix**: VPS thêm `v1_journal_log()` helper — gọi sau mỗi V1 UPSERT cho 20 bảng:
  1. Generate UUID: `v1-{table}-{store_id}-{local_id}`
  2. Backfill UUID trên row synced
  3. INSERT vào `sync_journal` với `source_device_id = NULL` (visible to ALL devices)
- **File**: VPS `sync.rs` (thêm `v1_journal_log()` cho 20 operations)
- **Backfill**: Chạy SQL backfill cho data cũ thiếu journal entries
- **Bài học**: Khi có V1 + V2 chạy song song, V1 handler PHẢI cũng ghi sync_journal

---

## Bug #14: Nút "Đồng bộ ngay" chỉ PUSH, không PULL
- **Ngày**: 18/03/2026
- **Triệu chứng**: Bấm "Đồng bộ ngay" trên mobile/PC → data push lên VPS OK nhưng không kéo data VPS về local. Mobile không thấy đơn hàng từ PC.
- **Root cause**: `triggerSync()` trong `syncStore.ts` chỉ gọi `sync_data_v2` (push). `handleSync()` trong `DatabaseSettingsTab.vue` gọi V1 `sync_data` (push only). Cả hai đều **KHÔNG** gọi `pull_sync_data_v2`.
- **Fix**: Thêm V2 pull (với V1 fallback) sau push trong cả `triggerSync()` và `handleSync()`.
- **File**: `src/stores/syncStore.ts`, `src/components/settings/DatabaseSettingsTab.vue`
- **Bài học**: Sync cycle PHẢI gồm cả Push + Pull. Push gửi data đi, Pull kéo data về.

---

## Bug #15: V2 Pull UPSERT thất bại — VPS gửi raw PG data không transform
- **Ngày**: 18/03/2026
- **Triệu chứng**: Diagnostic: journal products 85/85 synced, nhưng local DB chỉ có 1 product. Pull nói "nhận 0 thay đổi" vì cursor đã ở cuối.
- **Root cause** (3 issues xếp chồng):
  1. **`id` field**: VPS gửi PG auto-increment `id` (14929) + `local_id` (5). App skip `id` (line 414), App cũng skip `local_id` (không có column này) → INSERT không đặt `id` → auto-increment mới
  2. **`created_at` MISSING**: VPS table `synced_products` có `synced_at` nhưng KHÔNG có `created_at` → App SQLite `created_at NOT NULL` → INSERT fail silently
  3. **Journal duplicate**: 62 journal entries cho 3 products thực → cùng UUID UPSERT 62 lần → lãng phí
- **Fix cần**: VPS `merge_engine.rs::build_pull_changes()`:
  1. Map `local_id` → `id`
  2. Thêm `created_at` từ `synced_at`
  3. Loại bỏ `store_id`, `synced_at`, `device_id`, `updated_at_v2`
  4. Deduplicate journal — cùng UUID chỉ trả 1 lần
- **File**: VPS `merge_engine.rs`
- **Bài học**: Pull response PHẢI transform data cho client, KHÔNG gửi raw `row_to_json(SELECT *)`

---

## Bug #16: `local_id` mapping thiếu → tất cả bảng
- **Phát hiện**: 18/03/2026
- **Triệu chứng**: Invoices pull về 0, products pull OK (VPS đã fix riêng cho products)
- **Root cause**: `vps_to_app_column()` thiếu `(_, "local_id") => Some("id")`
- **Fix**: Thêm catch-all mapping + FK mappings cho child tables
- **File**: `sync_v2_pull.rs`

---

## Bug #17: invoice_number UNIQUE conflict
- **Phát hiện**: 18/03/2026
- **Triệu chứng**: UPSERT fail khi pull invoice có `invoice_number` trùng với record cũ (khác uuid)
- **Fix**: DELETE conflicting records trước UPSERT (pattern giống products/suppliers)
- **File**: `sync_v2_pull.rs`

---

## Bug #18: Cursor advance dù pull có lỗi
- **Phát hiện**: 18/03/2026
- **Triệu chứng**: UPSERT fail nhưng cursor advance → không retry → data mất vĩnh viễn
- **Fix**: Error counting, abort + giữ cursor nếu > 30% fail
- **File**: `sync_v2_pull.rs`

---

## Thống kê

| Loại | Số lượng |
|------|:--------:|
| App Push bugs | 2 (#1, #9) |
| App Pull bugs | 8 (#2, #3, #4, #5, #6, #14, #16, #17, #18) |
| VPS bugs | 8 (#7, #8, #10, #11, #12, #13, #15) |
| **Tổng** | **18** |

---

*Cập nhật lần cuối: 18/03/2026*
