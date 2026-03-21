# 📚 NC Cơ Sở Dữ Liệu #01: Kiến trúc SQLite cho Nodi POS Offline-First

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-19
> **Prompt**: Database Architect perspective — SQLite, Rust, offline-first, multi-device
> **Liên quan**: UUIDv7, Migration, Backup GFS, Sync Revision, Archiving, FTS5

## Tóm tắt

Blueprint Database toàn diện: Migration 2 chiều (Up/Down) + snapshot VACUUM INTO, UUIDv7 thay UUIDv4 (giảm 30-50% latency ghi), Backup rotation GFS (Daily/Weekly/Monthly/Archive), Sync dựa trên Monotonic Revision (không dùng timestamps), Archiving qua ATTACH DATABASE, FTS5 cho search tiếng Việt, case study Linear/Turso/Notion.

---

## 1. Migration Strategy — 2 chiều + Multi-device

### Vòng đời Migration Multi-device

```
Developer → .sql (UP + DOWN)
    ↓
CI/CD → sqlx compile-time check
    ↓
Distribution → Tauri binary (embedded migrations)
    ↓
Local Execution → CHECK user_version → IMMEDIATE TRANSACTION → migrate
    ↓
Sync Ack → Gửi schema version lên VPS → VPS điều chỉnh API
    ↓
Consolidation → Cleanup old sync messages
```

### Migration Checklist

| Hạng mục | Yêu cầu | Lý do |
|---------|--------|-------|
| Idempotent | `IF NOT EXISTS` | Chạy lại không lỗi |
| Giao dịch | `BEGIN IMMEDIATE TRANSACTION` | Không schema lấp lửng |
| Kiểm tra FK | `PRAGMA foreign_key_check` | Toàn vẹn công nợ |
| Snapshot | `VACUUM INTO 'backup_pre_migrate.db'` | Rollback tức thì |
| Tương thích ngược | Cột mới = NULL hoặc DEFAULT | Bản ghi cũ không lỗi |
| Log | Bảng `__migrations_log` | Chẩn đoán lỗi |

---

## 2. UUIDv7 vs UUIDv4

| Đặc tính | UUIDv4 ❌ | **UUIDv7** ✅ |
|---------|:--------:|:----------:|
| Thứ tự | Ngẫu nhiên (chaos) | **Time-ordered** |
| Hiệu năng ghi | Thấp (page splits liên tục) | **Giảm 30-50% latency** |
| Kích thước index | Phân mảnh | **Gọn, liên tục** |
| Riêng tư | Tuyệt đối | Trung bình (lộ timestamp) |
| Đồng bộ | ✅ Chống va chạm | ✅ Chống va chạm |
| Rust | `uuid` crate | `uuid` crate + feature `v7` |

> **Lưu trữ**: UUIDv7 dạng `BLOB 16 bytes` (không TEXT) → tiết kiệm 50% dung lượng index

### Áp dụng
- ✅ `transactions`, `orders`, `inventory_logs`, `customers` → UUIDv7
- ❌ Token nhạy cảm → Giữ UUIDv4 (không lộ timestamp)

---

## 3. Backup & Recovery — GFS Rotation

| Loại | Vị trí | Tần suất | Lưu giữ | Kịch bản |
|------|--------|:--------:|:-------:|---------|
| **Local Snapshot** | AppData/Backups | Mỗi 4 giờ | 24h | Sập nguồn |
| **Daily** | Ổ ngoài/USB | Cuối ngày (chốt sổ) | 7 ngày | Xóa nhầm HĐ |
| **Cloud Off-site** | S3/VPS | 2:00 AM | 30 ngày | Mất cắp, cháy |
| **Archive** | Cloud | Mỗi quý | 5 năm | Kiểm toán công nợ |

### Kỹ thuật Backup an toàn
- ✅ `VACUUM INTO` (compact + consistent)
- ✅ SQLite Backup API (`rusqlite` wrapper)
- ✅ `PRAGMA integrity_check` sau backup
- ❌ **KHÔNG** dùng `cp` khi app đang chạy WAL

### Mobile (Android)
- WorkManager cho scheduled backup
- SQLCipher → backup file đã mã hóa
- Key quản lý qua Android Keystore

---

## 4. Sync — Monotonic Revision (không dùng timestamps)

### Tại sao KHÔNG dùng `updated_at`
- Clock skew giữa thiết bị → ghi đè sai
- Đồng hồ nhanh hơn = "thắng" dù sai

### Giải pháp: Server-issued Revision

```
Device → "Tôi có revision 42" (sinceRev=42)
Server → "Đây là thay đổi 43, 44, 45..."
Device → Apply + update local revision
```

### Conflict Resolution

| Loại dữ liệu | Chiến lược |
|-------------|----------|
| Thông tin KH (tên, SĐT) | Last Write Wins (LWW) |
| **Công nợ, tồn kho** | **Delta-update** (+/- thay vì ghi đè) |
| Hóa đơn | Idempotent (client_id + request_id) |

### Optimistic Locking

```sql
UPDATE products
SET stock = ?, version = version + 1
WHERE id = ? AND version = ?
-- Nếu affected_rows = 0 → CONFLICT!
```

---

## 5. Scalability & Archiving

### Cấu hình Production-ready

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA busy_timeout = 5000;
PRAGMA mmap_size = 268435456;  -- 256MB memory-mapped I/O
```

### Archiving — ATTACH DATABASE

```
Main DB: nodi_pos.db (6-12 tháng gần nhất)
Archive: nodi_pos_2023.db, nodi_pos_2024.db

-- Báo cáo 5 năm:
ATTACH DATABASE 'nodi_pos_2023.db' AS archive_2023;
SELECT * FROM transactions
UNION ALL
SELECT * FROM archive_2023.transactions;
```

### FTS5 cho tìm kiếm SP tiếng Việt

```sql
CREATE VIRTUAL TABLE products_search USING fts5(name, code, category);
-- "phân bón NPK" → kết quả tức thì
```

---

## 6. Case Studies

| App | Bài học | Áp dụng Nodi |
|-----|--------|-------------|
| **Linear** | Mọi mutation = cục bộ → Outbox queue → sync | UI không bao giờ chờ VPS |
| **Turso/libSQL** | Database-per-user (mỗi store = 1 DB riêng) | Cô lập rủi ro, scale ngang |
| **Notion** | SQLite = cache layer, FTS cho search blocks | FTS5 cho 5,700 mã SP |

---

## 7. Blueprint — 5 Khuyến nghị

| # | Khuyến nghị | Ưu tiên |
|:-:|-----------|:------:|
| 1 | **UUIDv7** cho transactions, orders, customers | 🔴 Ngay |
| 2 | **WAL + synchronous=NORMAL** | 🔴 Ngay |
| 3 | **Revision-based sync** thay updated_at | 🟠 Cao |
| 4 | **Backup GFS tự động** (VACUUM INTO + integrity_check) | 🟠 Cao |
| 5 | **ATTACH DB archiving** khi > 12 tháng dữ liệu | 🟡 TB |

---

## Nguồn tham khảo

- SQLite documentation (WAL, VACUUM INTO, ATTACH)
- sqlx Rust crate (compile-time SQL checking)
- UUID RFC 9562 (UUIDv7 specification)
- Linear sync engine architecture
- Turso/libSQL edge database
- Notion offline-first architecture
- SQLite Backup API documentation
- GFS backup rotation strategy
