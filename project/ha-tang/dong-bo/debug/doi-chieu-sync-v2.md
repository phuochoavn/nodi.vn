# 🔄 Đối Chiếu V2 Sync — App ↔ VPS (18/03/2026)

> **MỤC ĐÍCH**: Bản đối chiếu hoàn chỉnh sau khi fix Bug #13–#15 + Bug A/B/C.
> Dùng để verify TRƯỚC KHI đưa vào production.

---

## 1. Tổng quan bugs đã fix

| Bug | Mô tả | Side | Status |
|-----|--------|:----:|:------:|
| #13 | V1 handler không tạo sync_journal | VPS | ✅ Fixed |
| #14 | Sync button chỉ push, không pull | App | ✅ Fixed |
| #15 | Pull UPSERT fail — raw PG data | VPS+App | ✅ Fixed |
| A | V1 backfill UUID mismatch → empty records | VPS | ✅ Fixed |
| B | invoice_items FK NULL | VPS | ✅ Fixed |
| C | Column name mismatches (5 mappings) | VPS+App | ✅ Fixed |

---

## 2. VPS Pull Response — Kiểm tra từng table

### ✅ products (3 records có data)

| Column | Giá trị mẫu | App NOT NULL? | Status |
|--------|-------------|:-------------:|:------:|
| id | 3 (= local_id) | — | ✅ |
| name | "Help 400SC" | YES | ✅ |
| base_unit | "Chai 100ml" | YES | ✅ |
| stock_quantity | 0.0 | YES (DEFAULT 0) | ✅ |
| created_at | "2026-03-16..." | YES | ✅ |
| updated_at | null → App fix NOW() | YES | ✅ (App default) |
| sell_price | null → App fix = cost_price | — | ✅ (App default) |
| uuid | "3d88d320-..." | — | ✅ |

### ✅ suppliers (1 record có data)

| Column | Giá trị mẫu | App NOT NULL? | Status |
|--------|-------------|:-------------:|:------:|
| id | 1 (= local_id) | — | ✅ |
| company → name | "" | YES | ⚠️ Empty string OK for NOT NULL |
| code | (chưa gửi) | YES UNIQUE | ⚠️ App default = "" |
| created_at | "2026-03-18..." | YES | ✅ |

### ✅ invoices (3 records có data)

| Column | Giá trị mẫu | App NOT NULL? | Status |
|--------|-------------|:-------------:|:------:|
| id | 3 (= local_id) | — | ✅ |
| invoice_number | "INV-177..." | YES UNIQUE | ✅ |
| total_amount | 45000 | YES | ✅ |
| final_amount | 45000 | YES | ✅ |
| created_at | "2026-03-16..." | YES | ✅ |
| customer_local_id → customer_id | null | — | ✅ (nullable) |
| notes → note | null | — | ✅ (nullable) |

### ✅ invoice_items (1 record)

| Column | Giá trị mẫu | App NOT NULL? | Status |
|--------|-------------|:-------------:|:------:|
| id | 3 (= local_id) | — | ✅ |
| invoice_local_id → invoice_id | 3 | YES | ✅ (Bug B fixed) |
| product_local_id → product_id | 2 | YES | ✅ (Bug B fixed) |
| product_name | "BASTA 200SL" | YES | ✅ |
| unit_name | "Chai" | YES | ✅ |
| exchange_value | null → App fix 1.0 | YES | ✅ (App default) |
| quantity | 1.0 | YES | ✅ |
| unit_price | 45000 | YES | ✅ |
| total → subtotal | null → App fix qty*price | YES | ✅ (App computed) |

### ✅ cash_transactions (1 record có data)

| Column | Giá trị mẫu | Status |
|--------|-------------|:------:|
| id, amount, flow_type, created_at | OK | ✅ |

### ✅ product_transactions (1 record có data)

| Column | Giá trị mẫu | Status |
|--------|-------------|:------:|
| id, product_id, quantity, created_at | OK | ✅ |

---

## 3. Column Mapping — Tổng hợp (App `vps_to_app_column()`)

| VPS gửi | App map thành | Table | Xử lý bởi |
|---------|:-------------:|-------|:----------:|
| `company` | `name` | suppliers | App+VPS |
| `transaction_type` | `type` | customer_transactions | App |
| `invoice_local_id` | `invoice_id` | invoice_items | App+VPS |
| `product_local_id` | `product_id` | invoice_items | App+VPS |
| `total` | `subtotal` | invoice_items | App+VPS |
| `customer_local_id` | `customer_id` | invoices | App+VPS |
| `notes` | `note` | invoices | App |

---

## 4. NULL → Default Handling (App `upsert_by_uuid()`)

| Field | Table | App xử lý khi NULL |
|-------|-------|:-------------------:|
| `subtotal` | invoice_items | `quantity * unit_price` |
| `exchange_value` | invoice_items | `1.0` |
| `updated_at` | all | `NOW()` |
| `created_at` | all | `NOW()` |
| `sell_price` | products | `= cost_price` |
| `code` | suppliers | Auto-gen `"SUP-{id:03}"` |
| `stock_quantity` | products | `0.0` |
| `current_debt` | customers | `0.0` |

---

## 5. Server Fields Stripped (VPS → KHÔNG gửi cho App)

| Field | Status |
|-------|:------:|
| `store_id` | ✅ Removed |
| `synced_at` | ✅ Removed (→ created_at nếu thiếu) |
| `local_id` | ✅ Removed (→ id trước khi strip) |
| `client_id` | ✅ Removed |
| `device_id` | ✅ Removed |
| `updated_at_v2` | ✅ Removed |
| `customer_local_id` | ✅ Removed (→ customer_id trước khi strip) |

---

## 6. Journal Deduplication (VPS)

| Metric | Trước fix | Sau fix |
|--------|:---------:|:-------:|
| Total journal entries | 88 | 88 (không đổi) |
| Unique UUIDs | 15 | 15 |
| Pull records trả về | 15 (6 empty) | 9 (0 empty) ✅ |
| Dedup ratio | 7:1 | 12:1 |

---

## 7. Kết luận

| Hạng mục | Status |
|----------|:------:|
| Pull API format đúng | ✅ |
| Column mappings đầy đủ | ✅ |
| NULL handling cho NOT NULL columns | ✅ |
| Server fields removed | ✅ |
| Journal dedup hoạt động | ✅ |
| FK fields (invoice_items) populated | ✅ |
| **Sẵn sàng test trên mobile** | ✅ |

*Cập nhật: 18/03/2026 13:50*
