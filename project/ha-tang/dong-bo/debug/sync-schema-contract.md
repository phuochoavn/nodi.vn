# 🔄 Sync Schema Contract — App SQLite ↔ VPS PostgreSQL

> **MỤC ĐÍCH**: Agent PHẢI đọc file này TRƯỚC KHI sửa bất kỳ bug sync nào.
> Tất cả mismatches giữa App và VPS được ghi nhận ở đây.

---

## Quy tắc chung

| Đặc điểm | App (SQLite) | VPS (PostgreSQL) |
|-----------|:------------:|:----------------:|
| **Type system** | Dynamic (TEXT, REAL, INTEGER) | Strict (timestamp, boolean, bigint, numeric) |
| **Boolean** | INTEGER 0/1 (hoặc REAL 0.0/1.0) | BOOLEAN true/false |
| **Timestamp** | TEXT `"2026-03-16T09:55:00"` | TIMESTAMP `2026-03-16 09:55:00` |
| **Tên bảng** | `invoices` | `synced_invoices` (prefix `synced_`) |
| **Primary key** | `id INTEGER AUTOINCREMENT` | `id BIGSERIAL` + `uuid TEXT UNIQUE` |
| **Store isolation** | Không có `store_id` | Mỗi bảng có `store_id` |
| **Device tracking** | `device_id TEXT` (nullable) | `device_id TEXT` + `local_id BIGINT` |

---

## Type Casting Rules (App → VPS)

| App type | App value example | VPS type | Cast needed |
|----------|-------------------|----------|:-----------:|
| TEXT datetime | `"2026-03-16T09:55:00"` | timestamp | `$N::timestamp` |
| INTEGER 0/1 | `0`, `1` | boolean | Rust coerce `0→false, 1→true` |
| REAL 0.0/1.0 | `0.0`, `1.0` | boolean | Rust coerce `0.0→false, 1.0→true` |
| TEXT | `"Khách lẻ"` | text | Không cần cast |
| REAL | `50000.0` | double precision | Không cần cast |
| INTEGER | `42` | integer/bigint | Không cần cast |

---

## Columns VPS-Only (App KHÔNG gửi, VPS tự tạo)

| Column | Bảng VPS | Mô tả |
|--------|----------|-------|
| `store_id` | Tất cả `synced_*` | VPS tự lấy từ JWT/session |
| `local_id` | Tất cả `synced_*` | VPS auto-generate nếu NULL |
| `updated_at_v2` | Tất cả `synced_*` | VPS timestamp khi merge |
| `synced_at` | Tất cả `synced_*` | VPS timestamp khi nhận |

---

## Columns App-Only (VPS KHÔNG có, App phải skip khi pull)

| Column | Bảng App | Mô tả |
|--------|----------|-------|
| `synced_to_cloud` | `invoices` | Flag sync V1 cũ |
| `capital_price` | `invoice_items` | Giá vốn tại thời điểm bán — VPS CHƯA có |
| `active_ingredient` | `products` | Hoạt chất — VPS CHƯA có |
| `cccd`, `cccd_front_img`, `cccd_back_img` | `customers` | CCCD info — VPS CHƯA có |
| `season` | `customer_transactions` | Vụ mùa — VPS CHƯA có |

---

## Column Name Mapping (App ≠ VPS)

| App column | VPS column | Bảng | Ghi chú |
|------------|-----------|------|---------|
| `id` | `local_id` | **Tất cả bảng** | ⚡ Catch-all: VPS local_id = App id |
| `type` | `transaction_type` | `customer_transactions`, `supplier_transactions` | VPS dùng tên dài hơn |
| `name` | `company` | `suppliers` | VPS gọi "company" thay "name" |
| `invoice_id` | `invoice_local_id` | `invoice_items` | VPS dùng `_local_id` suffix |
| `product_id` | `product_local_id` | `invoice_items`, `purchase_items`, `return_items`, `product_units`, `product_batches`, `product_transactions` | VPS dùng `_local_id` suffix |
| `subtotal` | `total` | `invoice_items` | VPS gọi `total` thay `subtotal` |
| `customer_id` | `customer_local_id` | `invoices` | VPS dùng `_local_id` suffix |
| `note` | `notes` | `invoices` | VPS plural form |
| `purchase_order_id` | `purchase_order_local_id` | `purchase_items` | VPS dùng `_local_id` suffix |
| `return_id` | `return_local_id` | `return_items` | VPS dùng `_local_id` suffix |

---

## Schema chi tiết từng bảng SYNC

### 1. products ↔ synced_products

| Column | App SQLite | VPS PostgreSQL | Ghi chú |
|--------|:----------:|:--------------:|---------|
| id | INTEGER PK | BIGSERIAL PK | OK |
| name | TEXT NOT NULL | text NOT NULL | OK |
| barcode | TEXT UNIQUE | text | App UNIQUE, VPS không |
| description | TEXT | text | OK |
| base_unit | TEXT NOT NULL | text | OK |
| stock_quantity | REAL NOT NULL DEFAULT 0 | double precision | OK |
| cost_price | REAL | double precision | OK |
| created_at | TEXT NOT NULL | timestamp | ⚠️ Cast cần |
| updated_at | TEXT NOT NULL | timestamp | ⚠️ Cast cần |
| uuid | TEXT UNIQUE | text UNIQUE | OK |
| device_id | TEXT | text | OK |
| active_ingredient | TEXT (migration 040) | ❌ KHÔNG CÓ | App-only, VPS skip |

### 2. invoices ↔ synced_invoices

| Column | App SQLite | VPS PostgreSQL | Ghi chú |
|--------|:----------:|:--------------:|---------|
| id | INTEGER PK | BIGSERIAL PK | OK |
| invoice_number | TEXT UNIQUE NOT NULL | text UNIQUE | OK |
| customer_name | TEXT | text | OK |
| customer_phone | TEXT | text | OK |
| total_amount | REAL NOT NULL | double precision | OK |
| discount_amount | REAL DEFAULT 0 | double precision | OK |
| final_amount | REAL NOT NULL | double precision | OK |
| payment_method | TEXT | text | OK |
| status | TEXT DEFAULT 'COMPLETED' | text | OK |
| note | TEXT | text | OK |
| created_at | TEXT NOT NULL | timestamp | ⚠️ Cast cần |
| synced_to_cloud | INTEGER DEFAULT 0 | ❌ KHÔNG CÓ | App-only |
| uuid | TEXT UNIQUE | text UNIQUE | OK |
| device_id | TEXT | text | OK |
| updated_at | TEXT | timestamp | ⚠️ Cast cần |

### 3. invoice_items ↔ synced_invoice_items

| Column | App SQLite | VPS PostgreSQL | Ghi chú |
|--------|:----------:|:--------------:|---------|
| capital_price | REAL DEFAULT 0 | ❌ KHÔNG CÓ | App-only! VPS phải skip |
| invoice_local_id | ❌ KHÔNG CÓ | bigint (nullable) | VPS-only, auto-resolve |
| product_local_id | ❌ KHÔNG CÓ | bigint (nullable) | VPS-only, auto-resolve |

### 4. purchase_orders ↔ synced_purchase_orders

| Column | App SQLite | VPS PostgreSQL | Ghi chú |
|--------|:----------:|:--------------:|---------|
| is_tax_invoice | INTEGER 0/1 | boolean | ⚠️ Coerce 0→false, 1→true |

### 5. product_units ↔ synced_product_units

| Column | App SQLite | VPS PostgreSQL | Ghi chú |
|--------|:----------:|:--------------:|---------|
| is_active | INTEGER DEFAULT 1 | boolean | ⚠️ Coerce 1→true |

---

## UNIQUE Constraints (App)

| Table | Constraint | Xung đột khi pull? |
|-------|-----------|:------------------:|
| products | `barcode UNIQUE` | ⚠️ Có — cần DELETE trùng trước UPSERT |
| suppliers | `code UNIQUE` | ⚠️ Có — cần DELETE trùng trước UPSERT |
| customers | `phone` — KHÔNG UNIQUE | ✅ OK |
| product_units | `UNIQUE(product_id, unit_name)` | ⚠️ Có — cần xử lý |
| invoices | `invoice_number UNIQUE` | ⚠️ Có |

---

*Cập nhật lần cuối: 18/03/2026 — sau 15 lần fix V2 sync*
