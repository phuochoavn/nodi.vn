# 🚀 Kế Hoạch Triển Khai Kiến Trúc Sync Mới — Nodi POS

> **Mục tiêu**: Chuyển từ full-dump sync sang incremental sync, đảm bảo multi-device an toàn
> **Tổng số Phase**: 6 | **Nguyên tắc**: Không vội, chỉnh chu, chính xác
> **Nguồn tham khảo**: 5 nghiên cứu DeepSearch (xem folder đồng cấp)

---

## Tổng Quan 6 Phase

```
Phase 1: Schema Foundation        → 🖥️ App Agent
Phase 2: VPS Merge Engine         → 🌐 VPS Agent
Phase 3: App Incremental Push     → 🖥️ App Agent
Phase 4: App Incremental Pull     → 🖥️ App Agent + 🌐 VPS Agent
Phase 5: Real-time WebSocket      → 🖥️ App Agent + 🌐 VPS Agent
Phase 6: Testing & Hardening      → 🖥️ App Agent + 🌐 VPS Agent + 👤 Anh kiểm tra
```

```
Timeline gợi ý:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 1 ████░░░░░░░░░░░░░░░░░░░░░░░░░░ (Tuần 1-2)
Phase 2 ░░░░████░░░░░░░░░░░░░░░░░░░░░░ (Tuần 3-4)
Phase 3 ░░░░░░░░████░░░░░░░░░░░░░░░░░░ (Tuần 5-6)
Phase 4 ░░░░░░░░░░░░████░░░░░░░░░░░░░░ (Tuần 7-8)
Phase 5 ░░░░░░░░░░░░░░░░████░░░░░░░░░░ (Tuần 9-10)
Phase 6 ░░░░░░░░░░░░░░░░░░░░████░░░░░░ (Tuần 11-12)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

> ⏰ Không gấp — mỗi Phase xong phải **verify kỹ** trước khi sang Phase tiếp.

---

## Phase 1: Schema Foundation — 🖥️ App Agent

> **Ai làm**: Agent App (Tauri + Rust + Vue)
> **Mục tiêu**: Chuẩn bị nền móng database — thêm UUID, device_id, updated_at cho tất cả bảng

### Công việc chi tiết:

| # | Task | File cần sửa | Chi tiết kỹ thuật |
|---|------|-------------|-------------------|
| 1.1 | Tạo migration thêm UUID + device_id + updated_at | `src-tauri/migrations/049_sync_foundation.sql` [MỚI] | ALTER TABLE cho ~20 bảng sync |
| 1.2 | Tạo `sync_metadata` table | Cùng migration 049 | device_id, last_push_cursor, last_pull_cursor, store_id |
| 1.3 | Tạo `sync_journal` table | Cùng migration 049 | sequence_id (INTEGER PK), table_name, record_uuid, operation (INSERT/UPDATE/DELETE), created_at |
| 1.4 | Tạo helper `generate_uuidv7()` trong Rust | `src-tauri/src/utils/uuid.rs` [MỚI] | Dùng crate `uuid` v1.x với feature `v7` |
| 1.5 | Backfill UUIDs cho records hiện có | `src-tauri/migrations/050_backfill_uuids.sql` [MỚI] | `UPDATE <table> SET uuid = lower(hex(randomblob(16)))` cho tất cả records NULL |
| 1.6 | Tạo triggers `updated_at` cho mutable tables | Cùng migration 049 | AFTER UPDATE → SET updated_at = datetime('now') |
| 1.7 | Tạo triggers ghi sync_journal | `src-tauri/migrations/051_sync_journal_triggers.sql` [MỚI] | AFTER INSERT/UPDATE/DELETE → ghi vào sync_journal |
| 1.8 | Sửa tất cả INSERT commands để kèm UUID + device_id | `src-tauri/src/commands.rs`, `db/*.rs` | Mỗi lệnh tạo record phải gọi `generate_uuidv7()` |
| 1.9 | Đăng ký device_id khi app khởi động lần đầu | `src/services/deviceService.ts` [MỚI] | Tạo UUID cho device, lưu vào sync_metadata |

### Verification Phase 1:
- [ ] `cmd /c npm run build` → 0 errors
- [ ] App khởi động bình thường, tất cả tính năng không bị ảnh hưởng
- [ ] Tạo 1 khách hàng mới → verify có UUID, device_id, updated_at
- [ ] Tạo 1 hóa đơn → verify có UUID, device_id
- [ ] Check sync_journal có log entry

### Đầu ra cho Phase tiếp:
- Mọi record mới đều có UUID + device_id + updated_at
- sync_journal ghi lại mọi thay đổi
- Device có unique ID

---

## Phase 2: VPS Merge Engine — 🌐 VPS Agent

> **Ai làm**: Agent VPS (Rust Axum + PostgreSQL)
> **Mục tiêu**: Xây dựng server-side merge engine nhận data từ nhiều device

### Công việc chi tiết:

| # | Task | File cần tạo/sửa | Chi tiết kỹ thuật |
|---|------|------------------|-------------------|
| 2.1 | Tạo `sync_inbox` staging table trên PG | Migration PG mới | device_id, table_name, operation, payload (JSONB), received_at, processed_at, status |
| 2.2 | API endpoint `POST /api/v2/sync/push` | `src/routes/sync_v2.rs` [MỚI] | Nhận batch changes từ client, ghi vào sync_inbox |
| 2.3 | Background worker: Merge engine | `src/workers/merge_engine.rs` [MỚI] | Consume sync_inbox → merge vào bảng chính |
| 2.4 | Merge rules cho append-only tables | Cùng merge_engine.rs | Invoice/transactions: UUID dedup (INSERT nếu chưa có, SKIP nếu trùng UUID) |
| 2.5 | Merge rules cho mutable tables | Cùng merge_engine.rs | Customer/Product: So sánh `updated_at`, record mới hơn thắng |
| 2.6 | Conflict logging | `src/models/sync_conflict.rs` [MỚI] | table, record_uuid, device_a, device_b, field, value_a, value_b, resolution, created_at |
| 2.7 | Tính toán computed fields | Trigger/query PG | `stock_quantity = SUM(product_transactions)`, `current_debt = SUM(customer_transactions)` |
| 2.8 | API endpoint `GET /api/v2/sync/pull` | `src/routes/sync_v2.rs` | Trả records thay đổi since cursor, EXCLUDE records từ device_id đang request |
| 2.9 | Idempotency key check | Middleware | Request ID → check đã xử lý chưa → skip nếu trùng |
| 2.10 | `sync_journal` table trên PG | Migration PG mới | Global sequence ID để client cursor follow |

### Verification Phase 2:
- [ ] Gửi mock push payload qua curl/Postman → verify merge đúng
- [ ] Gửi cùng payload 2 lần → verify idempotency (chỉ ghi 1 lần)
- [ ] Gửi 2 payload conflict (cùng UUID, khác updated_at) → verify record mới hơn thắng
- [ ] Pull API trả đúng data (exclude device gọi)
- [ ] Computed fields đúng sau merge

### Đầu ra cho Phase tiếp:
- API v2/sync/push nhận và merge data từ client
- API v2/sync/pull trả incremental changes
- Conflict log ghi nhận mọi xung đột

---

## Phase 3: App Incremental Push — 🖥️ App Agent

> **Ai làm**: Agent App (Tauri + Rust + Vue)
> **Mục tiêu**: Rewrite sync push — chỉ gửi records thay đổi thay vì toàn bộ DB

### Công việc chi tiết:

| # | Task | File cần sửa | Chi tiết kỹ thuật |
|---|------|-------------|-------------------|
| 3.1 | Tạo module `sync_v2_push.rs` | `src-tauri/src/sync/push.rs` [MỚI] | Query sync_journal since last_push_cursor → gom changes |
| 3.2 | Serialize payload theo format API v2 | Cùng push.rs | `{ device_id, cursor, changes: { customers: [...], invoices: [...] } }` |
| 3.3 | Gọi `POST /api/v2/sync/push` | Cùng push.rs | HTTP client với retry + exponential backoff |
| 3.4 | Xử lý response: cập nhật cursor | Cùng push.rs | Lưu new_cursor vào sync_metadata.last_push_cursor |
| 3.5 | Xử lý response: hiển thị conflicts | `src/components/SyncConflicts.vue` [MỚI] | Danh sách conflicts nếu có |
| 3.6 | Offline queue: push khi có mạng | `src/services/syncQueue.ts` [MỚI] | Monitor navigator.onLine → auto-push khi online lại |
| 3.7 | Giữ sync cũ (`/api/sync/push`) hoạt động song song | `sync_commands.rs` | KHÔNG XÓA code cũ — chạy song song cho đến khi verify xong |

### Verification Phase 3:
- [ ] Tạo 3 records → push → verify VPS chỉ nhận 3 records (không full DB)
- [ ] Tạo 0 changes → push → verify payload rỗng (không gửi gì)
- [ ] Tắt mạng → tạo 5 hóa đơn → bật mạng → verify auto-push đúng 5
- [ ] Sync cũ vẫn hoạt động song song (fallback)

### Đầu ra cho Phase tiếp:
- App push incremental thay vì full dump
- Offline queue hoạt động

---

## Phase 4: App Incremental Pull — 🖥️ App Agent + 🌐 VPS Agent

> **Ai làm**: Agent App (phần lớn) + Agent VPS (API tinh chỉnh)
> **Mục tiêu**: Pull chỉ records thay đổi từ server, UPSERT bằng UUID

### Công việc chi tiết:

**🖥️ App Agent**:

| # | Task | File cần sửa | Chi tiết kỹ thuật |
|---|------|-------------|-------------------|
| 4.1 | Tạo module `sync_v2_pull.rs` | `src-tauri/src/sync/pull.rs` [MỚI] | Gọi GET /api/v2/sync/pull?since=cursor&device_id=xxx |
| 4.2 | UPSERT bằng UUID | Cùng pull.rs | INSERT OR REPLACE WHERE uuid = ? (KHÔNG dùng integer ID) |
| 4.3 | Recalc computed fields sau pull | Cùng pull.rs | `UPDATE products SET stock_quantity = (SELECT SUM...)` |
| 4.4 | Cập nhật last_pull_cursor | Cùng pull.rs | Lưu vào sync_metadata |
| 4.5 | UI: Hiển thị trạng thái sync | `src/components/SyncStatus.vue` [MỚI] | Badge: "Đang đồng bộ...", "Đã đồng bộ ✅", "Offline ⚠️" |

**🌐 VPS Agent**:

| # | Task | Chi tiết |
|---|------|---------|
| 4.6 | Tinh chỉnh pull API: pagination | Trả tối đa 500 records/request, có next_cursor |
| 4.7 | Tinh chỉnh pull API: soft delete | Trả tombstone records (is_deleted = true) |

### Verification Phase 4:
- [ ] PC tạo khách hàng → Mobile pull → verify hiện trên mobile
- [ ] Mobile tạo hóa đơn → PC pull → verify hiện trên PC
- [ ] PC sửa tên KH → Mobile pull → verify tên mới
- [ ] Delete trên PC → Mobile pull → verify bị xóa (soft delete)
- [ ] Tạo 1000 records → pull → verify pagination hoạt động

---

## Phase 5: Real-time WebSocket — 🖥️ App Agent + 🌐 VPS Agent

> **Ai làm**: Cả 2 agent
> **Mục tiêu**: Push notification real-time — device nhận biết thay đổi ngay lập tức

### Công việc chi tiết:

**🌐 VPS Agent**:

| # | Task | Chi tiết |
|---|------|---------|
| 5.1 | Upgrade WS protocol: Gửi table hints | `{ "type": "sync_available", "tables": ["customers", "invoices"], "source_device": "xxx" }` |
| 5.2 | PG LISTEN/NOTIFY trigger | Khi merge_engine xử lý xong → NOTIFY → WS broadcast |

**🖥️ App Agent**:

| # | Task | Chi tiết |
|---|------|---------|
| 5.3 | Upgrade WebSocket listener | Nhận table hints → selective pull chỉ tables cần thiết |
| 5.4 | Auto-pull loop: Fallback polling | Mỗi 30s check nếu WS disconnect → pull thủ công |
| 5.5 | Ping/Pong heartbeat | Phát hiện WS disconnect nhanh → chuyển offline mode |

### Verification Phase 5:
- [ ] PC tạo record → Mobile nhận WS → auto-pull → hiện trong 2-3 giây
- [ ] Tắt WS server → App vẫn sync bằng polling (fallback)
- [ ] 3 devices online → 1 device tạo record → 2 devices còn lại cập nhật

---

## Phase 6: Testing & Hardening — 👥 Tất cả

> **Ai làm**: App Agent + VPS Agent + Anh kiểm tra thực tế
> **Mục tiêu**: Kiểm tra kịch bản thực tế, fix edge cases, benchmark hiệu năng

### Kịch bản test thực tế:

| # | Kịch bản | Kỳ vọng | Ai test |
|---|---------|---------|:-------:|
| 6.1 | 3 devices online đồng thời, tạo record song song | Không mất, không trùng, sync < 5s | 🖥️ + 👤 |
| 6.2 | 1 device offline 30 phút, tạo 20 hóa đơn → online lại | Tất cả 20 hóa đơn sync lên, không thiếu | 🖥️ + 👤 |
| 6.3 | 2 devices sửa cùng 1 khách hàng offline → online cùng lúc | Record mới hơn thắng, conflict log ghi nhận | 🖥️ + 🌐 |
| 6.4 | VPS restart giữa chừng sync | Client retry, không mất data | 🌐 + 👤 |
| 6.5 | Bandwidth test: 1000 records push | < 5 giây, payload < 500KB | 🖥️ + 🌐 |
| 6.6 | Database 50,000 records: Full app cycle test | App không lag, sync incremental nhanh | 🖥️ + 👤 |

### Hardening:

| # | Task | Agent |
|---|------|:-----:|
| 6.7 | Rate limiting trên sync API | 🌐 VPS |
| 6.8 | Retry với exponential backoff (max 5 lần) | 🖥️ App |
| 6.9 | Tombstone GC: Xóa soft-deleted records > 30 ngày | 🌐 VPS |
| 6.10 | VACUUM scheduling cho SQLite | 🖥️ App |
| 6.11 | Xóa code sync cũ (v1) sau khi v2 ổn định | 🖥️ App + 🌐 VPS |
| 6.12 | Monitoring: Sync success/fail rate dashboard | 🌐 VPS |

---

## Tổng Hợp Phân Công Agent

### 🖥️ Agent App (Tauri + Rust + Vue)

| Phase | Khối lượng | Mô tả |
|:-----:|:----------:|-------|
| 1 | ████████ **100%** | Schema migration, UUID generation, triggers, sync_journal |
| 2 | ░░░░░░░░ 0% | Không liên quan |
| 3 | ████████ **100%** | Push rewrite, offline queue, conflict UI |
| 4 | ██████░░ **70%** | Pull rewrite, UPSERT UUID, computed fields, sync status UI |
| 5 | ████░░░░ **50%** | WS listener upgrade, selective pull, fallback polling |
| 6 | ████░░░░ **50%** | Client testing, retry logic, VACUUM, cleanup sync v1 |

### 🌐 Agent VPS (Rust Axum + PostgreSQL)

| Phase | Khối lượng | Mô tả |
|:-----:|:----------:|-------|
| 1 | ░░░░░░░░ 0% | Không liên quan |
| 2 | ████████ **100%** | Merge engine, sync_inbox, conflict log, computed fields, idempotency |
| 3 | ░░░░░░░░ 0% | Không liên quan |
| 4 | ██░░░░░░ **30%** | Pagination, soft delete/tombstone trên pull API |
| 5 | ████░░░░ **50%** | PG LISTEN/NOTIFY, WS broadcast upgrade |
| 6 | ████░░░░ **50%** | Rate limit, tombstone GC, monitoring, cleanup sync v1 |

### 👤 Anh (Owner)

| Phase | Vai trò |
|:-----:|---------|
| 1 | Review migration, verify app hoạt động bình thường |
| 2 | Test merge API qua Postman (hoặc tôi viết prompt cho VPS agent) |
| 3 | Test offline → online flow |
| 4 | Test PC ↔ Mobile sync thực tế |
| 5 | Test real-time trên nhiều thiết bị |
| 6 | **Test thực tế tại cửa hàng** — edge cases thực |

---

## Quy Tắc Quan Trọng

### ❌ KHÔNG LÀM

- ❌ Xóa code sync cũ cho đến khi v2 chạy ổn (Phase 6 mới xóa)
- ❌ Merge Phase — mỗi Phase phải verify xong mới sang tiếp
- ❌ CRDT, MessagePack, Kafka — chưa cần ở giai đoạn này
- ❌ Sửa database migration cũ — chỉ thêm migration mới

### ✅ PHẢI LÀM

- ✅ Mỗi Phase kết thúc bằng `npm run build` → 0 errors
- ✅ Giữ backward-compatible — sync v1 và v2 chạy song song
- ✅ Mỗi Phase có verification checklist riêng
- ✅ Ghi lại kết quả test vào `roadmap/DEVELOPMENT_JOURNAL.md`

---

## Cách Viết Brief Cho Agent

### Brief cho 🖥️ Agent App (Phase 1 — ví dụ):

```
Nhiệm vụ: Phase 1 — Sync Foundation Schema
Đọc kỹ trước:
  - project/ha-tang/dong-bo/ke-hoach-trien-khai.md (Phase 1)
  - project/ha-tang/dong-bo/nghien-cuu-01-kien-truc-sync-pos.md (mục UUIDv7)
  
Công việc:
  1. Tạo migration 049_sync_foundation.sql
  2. Thêm uuid, device_id, updated_at cho ~20 bảng
  3. Tạo sync_metadata table
  4. Tạo sync_journal table + triggers
  5. Tạo generate_uuidv7() trong Rust
  6. Sửa tất cả INSERT commands kèm UUID
  7. Build verify: 0 errors

KHÔNG được:
  - Sửa migration cũ
  - Sửa VPS code
  - Xóa sync code hiện tại
```

### Brief cho 🌐 Agent VPS (Phase 2 — ví dụ):

```
Nhiệm vụ: Phase 2 — Server-Side Merge Engine
Đọc kỹ trước:
  - project/ha-tang/dong-bo/ke-hoach-trien-khai.md (Phase 2)
  - project/ha-tang/dong-bo/nghien-cuu-02-server-merge-engine.md
  
Công việc:
  1. Tạo sync_inbox staging table trên PostgreSQL
  2. API POST /api/v2/sync/push — nhận batch changes
  3. Background worker: merge engine (UUID dedup + timestamp merge)
  4. Conflict logging table
  5. Computed fields recalculation (stock, debt)
  6. API GET /api/v2/sync/pull — trả incremental changes
  7. Idempotency middleware
  
KHÔNG được:
  - Sửa API sync v1 hiện tại
  - Sửa client app code
  - Deploy lên production cho đến khi Phase 3-4 xong
```

---

## Upgrade Path Tương Lai (Sau Phase 6)

| Thời điểm | Upgrade | Lý do |
|-----------|---------|-------|
| Khi cần tốc độ hơn | JSON → **MessagePack/Protobuf** | Giảm 40% bandwidth |
| Khi cần sync P2P | LWW → **CRDT PN-Counter** | Cho inventory offline P2P |
| Khi > 50 cửa hàng | Single PG → **Read replicas** | Scale đọc |
| Khi cần IoT | WebSocket → **MQTT** | Thiết bị IoT nhẹ hơn |
| Khi cần AI predictive | Thêm analytics pipeline | Gợi ý nhập hàng theo mùa vụ |

> Tất cả upgrade trên **backward-compatible** nếu Phase 1 (UUID + sync_journal) đúng từ đầu.

---

*Kế hoạch này dựa trên 5 nghiên cứu DeepSearch. Có thể điều chỉnh linh hoạt theo thực tế.*
