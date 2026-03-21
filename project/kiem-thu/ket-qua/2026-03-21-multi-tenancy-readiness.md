# VPS Multi-Tenancy Readiness Report

**Ngày:** 2026-03-21  
**Scope:** Toàn bộ VPS codebase (`/opt/nodi`)  
**Loại:** READ-ONLY — không sửa code

---

## A. Schema

### A1. Danh sách TẤT CẢ bảng `synced_*`

Tổng: **23 bảng synced_*** (theo `db.rs` → `sync_v2_init()` line 191-215)

| # | Bảng | Nguồn tạo |
|---|------|-----------|
| 1 | `synced_products` | `init.sql:43` |
| 2 | `synced_customers` | `init.sql:57` |
| 3 | `synced_invoices` | `init.sql:70` |
| 4 | `synced_invoice_items` | `init.sql:84` |
| 5 | `synced_suppliers` | `init.sql:97` |
| 6 | `synced_purchase_orders` | `migration_sprint3.sql:35` |
| 7 | `synced_purchase_items` | `migration_sprint3.sql:52` |
| 8 | `synced_customer_transactions` | `migration_sprint3.sql:70` |
| 9 | `synced_supplier_transactions` | `migration_sprint3.sql:86` |
| 10 | `synced_product_units` | `migration_sprint3.sql:102` |
| 11 | `synced_store_funds` | `migration_sprint3.sql:116` |
| 12 | `synced_cash_transactions` | `migration_sprint3.sql:124` |
| 13 | `synced_product_batches` | `migration_sprint3.sql:140` |
| 14 | `synced_payment_vouchers` | `migration_sprint3.sql:156` |
| 15 | `synced_store_settings` | `migration_sprint3.sql:171` |
| 16 | `synced_product_transactions` | `migration_sprint3.sql:182` |
| 17 | `synced_promotions` | `db.rs` (sync_v2_init) |
| 18 | `synced_vouchers` | `db.rs` (sync_v2_init) |
| 19 | `synced_daily_closings` | `db.rs` (sync_v2_init) |
| 20 | `synced_returns` | `db.rs` (sync_v2_init) |
| 21 | `synced_return_items` | `db.rs` (sync_v2_init) |
| 22 | `synced_loyalty_transactions_v2` | `db.rs` (sync_v2_init) |
| 23 | `synced_store_funds` | `db.rs` (sync_v2_init) |

Ngoài ra còn 2 bảng loyalty cũ: `synced_loyalty_transactions` (db.rs:161), `synced_loyalty_settings` (db.rs:172).

### A2. Bảng nào ĐÃ có `store_id`? Bảng nào CHƯA?

| Bảng | store_id? | Ghi chú |
|------|:---------:|---------|
| synced_products | ✅ | `NOT NULL REFERENCES stores(id)`, `init.sql:45` |
| synced_customers | ✅ | `NOT NULL REFERENCES stores(id)`, `init.sql:59` |
| synced_invoices | ✅ | `NOT NULL REFERENCES stores(id)`, `init.sql:72` |
| synced_invoice_items | ✅ | `NOT NULL REFERENCES stores(id)`, `init.sql:86` |
| synced_suppliers | ✅ | `NOT NULL REFERENCES stores(id)`, `init.sql:99` |
| synced_purchase_orders | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:37` |
| synced_purchase_items | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:54` |
| synced_customer_transactions | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:72` |
| synced_supplier_transactions | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:88` |
| synced_product_units | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:104` |
| synced_store_funds | ✅ | `NOT NULL REFERENCES stores(id) UNIQUE`, `migration_sprint3:118` |
| synced_cash_transactions | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:126` |
| synced_product_batches | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:142` |
| synced_payment_vouchers | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:158` |
| synced_store_settings | ✅ | `NOT NULL REFERENCES stores(id) UNIQUE`, `migration_sprint3:173` |
| synced_product_transactions | ✅ | `NOT NULL REFERENCES stores(id)`, `migration_sprint3:184` |
| synced_promotions | ✅ | Created via `db.rs` sync_v2_init (ALTER TABLE ADD uuid, device_id) |
| synced_vouchers | ✅ | (same as above) |
| synced_daily_closings | ✅ | (same as above) |
| synced_returns | ✅ | (same as above) |
| synced_return_items | ✅ | (same as above) |
| synced_loyalty_transactions_v2 | ✅ | (same as above) |
| synced_loyalty_transactions | ✅ | `store_id INTEGER NOT NULL`, `db.rs:162` |
| synced_loyalty_settings | ✅ | `store_id INTEGER NOT NULL UNIQUE`, `db.rs:173` |

> **Kết luận A2: 100% bảng synced_* ĐÃ CÓ `store_id`.** Tất cả có `NOT NULL` constraint. Các bảng trong `init.sql` và `migration_sprint3.sql` có FK `REFERENCES stores(id)`. Các bảng tạo muộn trong `db.rs` chỉ có `NOT NULL` (không có FK).

### A3. Bảng `stores` — ✅ TỒN TẠI

```sql
-- init.sql:4
CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    license_key VARCHAR(50) UNIQUE NOT NULL,
    owner_name VARCHAR(255),
    phone VARCHAR(20),
    address TEXT,
    province VARCHAR(100),
    district VARCHAR(100),
    license_type VARCHAR(50) DEFAULT 'basic',  -- basic, pro, lifetime
    license_expires_at TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

Cột bổ sung (via `db.rs` seed_admin):
- `revoked_at TIMESTAMP` (db.rs:66)
- `duration_days INTEGER DEFAULT 30` (db.rs:68)
- `app_version TEXT` (db.rs:122)
- `last_sync_at TIMESTAMP` (db.rs:123)
- `hwid TEXT` (implicit, used in `identify_store`)

### A4. Bảng `sync_devices` — ✅ TỒN TẠI

```sql
-- db.rs:306
CREATE TABLE IF NOT EXISTS sync_devices (
    id BIGSERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL,
    device_id TEXT NOT NULL,
    device_name TEXT,
    last_push_at TIMESTAMP,
    last_pull_at TIMESTAMP,
    pull_cursor BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, device_id)
);
```

> ⚠️ **Không có FK** `REFERENCES stores(id)` — khác với bảng `devices` (db.rs:127) có FK.

### A5. Tổng số bảng

| Loại | Số lượng | Chi tiết |
|------|:--------:|----------|
| Bảng từ `init.sql` | 9 | stores, users, backup_files, synced_products, synced_customers, synced_invoices, synced_invoice_items, synced_suppliers, einvoice_config, tax_config |
| Bảng từ `migration_sprint3.sql` | 11 | 11 bảng synced_* bổ sung |
| Bảng từ `migration_sprint10.sql` | 2 | employees, store_groups |
| Bảng từ `migration_sprint89.sql` | 1 | sync_staff_members |
| Bảng từ `db.rs` (runtime migration) | 13 | license_payments, support_tickets, support_messages, audit_log, notifications, devices, synced_loyalty_transactions, synced_loyalty_settings, sync_inbox, sync_journal, sync_conflicts, sync_devices, orders |
| Bảng từ `payment.rs` | 1 | orders |
| **Tổng ước tính** | **~35-37** | |

---

## B. Auth & Middleware

### B6. JWT Auth — ✅ CÓ

- **Library:** `jsonwebtoken` crate (`middleware/auth.rs:1`)
- **Access Token:** 24h (`auth.rs:22`)
- **Refresh Token:** 30d (`auth.rs:45`)
- **Claims struct:** `{ sub: i32, store_id: i32, role: String, token_type: String, exp, iat }` (`auth.rs:8-16`)

### B7. `store_id` extract từ đâu?

Hàm `identify_store()` trong `sync.rs:298-335` — dùng cho cả V1 và V2 sync:

| Ưu tiên | Nguồn | Cách lấy |
|:--------:|-------|----------|
| 1️⃣ | **JWT** `Authorization: Bearer` header | `claims.store_id` — trực tiếp từ JWT payload |
| 2️⃣ | **X-HWID** header | Lookup `stores.id WHERE hwid = $1` |
| 3️⃣ | **X-HWID** (tự tạo) | Auto-create anonymous store nếu HWID mới |

> ⚠️ **Không có middleware global** để inject `store_id` vào request context. Mỗi handler phải gọi `identify_store()` thủ công.

### B8. RLS Policy — ❌ KHÔNG CÓ

Không tìm thấy `ROW LEVEL SECURITY`, `ENABLE RLS`, hay `CREATE POLICY` trong bất kỳ file nào.  
Chỉ có 1 mention trong tài liệu nghiên cứu (`nghien-cuu-02-server-merge-engine.md:304`) như suggestion, chưa triển khai.

---

## C. Connection & Infrastructure

### C9. SQLx Pool Config

```rust
// db.rs:4-9
pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)  // ← hardcoded
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}
```

| Setting | Value |
|---------|-------|
| `max_connections` | **10** |
| `min_connections` | default (0) |
| `idle_timeout` | default |
| `max_lifetime` | default |

### C10. Docker Compose — ❌ KHÔNG CÓ PgBouncer

```
services:
  nginx          — Nginx reverse proxy
  postgres       — PostgreSQL 16 Alpine (direct)
  axum-api       — Rust API (connects to postgres directly)
  nuxt-web       — Nuxt frontend
```

> **Không có PgBouncer**, không có connection pooler nào ở tầng infrastructure.

### C11. DATABASE_URL

```env
# .env:5
APP_DATABASE_URL=postgres://nodi_app:***@postgres:5432/nodi
```

```yaml
# docker-compose.yml:50
DATABASE_URL: ${APP_DATABASE_URL}
```

> **Trỏ thẳng PostgreSQL** qua Docker network (`postgres:5432`), không qua proxy/bouncer.

---

## D. Sync Protocol

### D12. Sync Endpoint Routes

**V1 routes** (`sync.rs:288-292`):
| Method | Path | Handler |
|--------|------|---------|
| POST | `/api/sync` | `handle_sync` — full push (14 collections) |
| GET | `/api/sync/pull` | `handle_pull` — download all store data |

**V2 routes** (`sync_v2.rs:15-19`):
| Method | Path | Handler |
|--------|------|---------|
| POST | `/api/v2/sync/push` | `handle_push` — delta push with merge engine |
| GET | `/api/v2/sync/pull` | `handle_pull` — delta pull with cursor |
| GET | `/api/v2/sync/snapshot` | `handle_snapshot` — full data dump for new devices |

### D13. Payload Format — **JSON only**

- Không tìm thấy `protobuf`, `prost`, hoặc bất kỳ binary serialization nào
- Tất cả endpoint dùng `Json<Value>` (Axum JSON extractor)
- Content-Type mentions chỉ có `text/csv` (admin export) và `application/octet-stream` (backup download)

### D14. Merge Engine — ✅ CÓ filter by `store_id`

`merge_engine.rs` sử dụng `store_id` **xuyên suốt** (100+ references):

| Chức năng | Cách dùng |
|-----------|-----------|
| `process_push()` | Nhận `store_id: i32` param (line 18), dùng cho mọi INSERT/UPDATE |
| `merge_insert_dedup()` | `WHERE store_id = $1 AND uuid = $2` (line 369) |
| `merge_insert_lww()` | `WHERE store_id = $1 AND uuid = $2` (line 401) |
| `merge_update_lww()` | `WHERE store_id = $1 AND uuid = $2` (line 435) |
| `merge_update_record()` | `UPDATE ... WHERE store_id = $1 AND uuid = $2` (line 540) |
| `insert_record_from_json()` | Includes `store_id` in INSERT column list (line 576) |
| `recompute_stock()` | Filter by `store_id` |
| `recompute_customer_debt()` | Filter by `store_id` |
| `recompute_supplier_debt()` | Filter by `store_id` |
| `build_snapshot()` | Filter by `store_id` |
| Pull transforms | All queries use `WHERE store_id = $1` |

---

## E. Backup

### E15. Backup Mechanism — **App-level only**

| Component | Status |
|-----------|--------|
| `pg_dump` cron | ❌ Không có |
| pgBackRest | ❌ Không có |
| App backup API | ✅ `backup.rs` — upload/list/download SQLite backup files |

App backup flow:
- `POST /api/backup/upload` — client uploads SQLite .db file
- `GET /api/backup/list` — list backups per store
- `GET /api/backup/download` — download latest backup
- Files stored in `/opt/nodi/backups/{store_id}/`
- Quota: Free = 3 backups × 50MB; Paid = 10 backups × 200MB
- Auto-cleanup via `cleanup_old_backups()` (backup.rs:229)

> ⚠️ Đây là backup **client SQLite**, KHÔNG phải backup PostgreSQL server.

### E16. WAL Archiving — ❌ CHƯA BẬT

Không tìm thấy `archive_mode` hoặc `archive_command` trong toàn bộ codebase.  
PostgreSQL chạy với custom config (`data/postgresql.conf`) nhưng không có WAL archiving config visible trong repo.

---

## Kết luận: Multi-Tenancy Readiness Score

| Hạng mục | Status | Điểm |
|----------|:------:|:----:|
| **A. Schema** — store_id trên mọi bảng | ✅ Hoàn tất | 10/10 |
| **A. Schema** — stores table | ✅ Có | — |
| **A. Schema** — sync_devices | ✅ Có | — |
| **B. Auth** — JWT với store_id | ✅ Có | 7/10 |
| **B. Auth** — RLS policies | ❌ Chưa có | 0/10 |
| **B. Auth** — Global middleware inject store_id | ❌ Manual per-handler | — |
| **C. Infra** — Connection pooler (PgBouncer) | ❌ Chưa có | 0/10 |
| **C. Infra** — Pool config | ⚠️ Hardcoded 10 | 5/10 |
| **D. Protocol** — Payload format | ⚠️ JSON only | 5/10 |
| **D. Protocol** — store_id isolation in merge_engine | ✅ Hoàn tất | 10/10 |
| **E. Backup** — Server-level backup | ❌ Chưa có | 0/10 |
| **E. Backup** — WAL archiving | ❌ Chưa có | 0/10 |

### Tổng: ~45% ready cho multi-tenancy production

**Điểm mạnh:**
- ✅ Schema đã sẵn sàng 100% — mọi bảng có `store_id NOT NULL`
- ✅ JWT chứa `store_id` claim
- ✅ Merge engine filter by `store_id` xuyên suốt
- ✅ V2 sync protocol có delta push/pull + snapshot

**Gaps cần bổ sung:**
1. 🔴 **RLS** — Chưa có row-level security, hoàn toàn phụ thuộc vào application-level filtering
2. 🔴 **PgBouncer** — Không có connection pooler, pool chỉ 10 connections (không scale được)
3. 🔴 **Server backup** — Không có pg_dump, pgBackRest, hay WAL archiving (risk mất data)
4. 🟡 **Protobuf** — JSON payload cho sync sẽ chậm khi data lớn
5. 🟡 **Middleware** — Không có global middleware extract store_id, dễ quên filter trong handler mới
