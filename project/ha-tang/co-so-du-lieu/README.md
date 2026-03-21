# 🗄️ Cơ Sở Dữ Liệu — Database Architecture

> Schema, migration, backup, sync strategy, scalability

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ Nghiên Cứu

### 🔴 Ưu tiên Cao nhất

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 1 | **UUIDv7** thay UUIDv4 | BLOB 16 bytes, giảm 30-50% latency ghi, time-ordered | Lớn |
| 2 | **WAL + synchronous=NORMAL** | Bắt buộc production. +mmap_size 256MB | Nhỏ |
| 3 | **Migration 2 chiều** | UP + DOWN, snapshot VACUUM INTO trước migrate | TB |
| 4 | **Backup GFS tự động** | Daily (chốt sổ) → Weekly (VPS) → Monthly → Archive 5 năm | TB |

### 🟠 Ưu tiên Cao

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 5 | **Revision-based sync** | Monotonic revision thay updated_at (chống clock skew) | Lớn |
| 6 | **Optimistic Locking** | Cột `version` cho products, debts — detect conflict | TB |
| 7 | **Delta-update** cho công nợ/tồn kho | +/- thay vì ghi đè giá trị tuyệt đối | TB |
| 8 | **FTS5** cho tìm kiếm SP | Virtual table, search tiếng Việt tức thì | TB |

### 🟡 Ưu tiên Trung bình

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 9 | **ATTACH DB archiving** | Main DB (12 tháng) + Archive DB theo năm | TB |
| 10 | **`PRAGMA integrity_check`** sau backup | Đảm bảo backup khôi phục được | Nhỏ |
| 11 | **Migrations log table** | `__migrations_log` để chẩn đoán lỗi | Nhỏ |
| 12 | **Idempotent sync** | client_id + request_id chống duplicate | Nhỏ |

---

### 💎 5 Nguyên tắc Database

1. **"UUIDv7 mặc định, UUIDv4 chỉ cho token"** — Time-ordered = B-Tree hạnh phúc
2. **"Revision, không Timestamp"** — Server cấp revision, chống clock skew
3. **"Backup = VACUUM INTO + integrity_check"** — Không bao giờ `cp` khi WAL đang chạy
4. **"Delta cho tiền, LWW cho text"** — Công nợ +/- , tên KH last-write-wins
5. **"1 DB = 1 Đại lý"** — Cô lập rủi ro, scale ngang (Turso model)

---

## Nghiên Cứu (Google DeepSearch — 19/03/2026)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-sqlite-offline-first-architecture.md` | Migration, UUIDv7, Backup GFS, Sync Revision, Archiving, FTS5 | **95** |

---

## Hiện trạng Nodi POS

| Tính năng | Hiện tại | Mục tiêu |
|----------|:-------:|:-------:|
| Migrations | 46 files, 1 chiều | 2 chiều + VACUUM INTO snapshot |
| UUID | Một phần UUIDv4 | UUIDv7 toàn bộ entity |
| Backup | Thủ công | GFS tự động (Daily/Weekly/Monthly) |
| Sync | updated_at based | Revision-based + Delta-update |
| Search | LIKE query | FTS5 virtual table |
| Archiving | Chưa có | ATTACH DB theo năm |
| WAL mode | ✅ Đã bật | ✅ |
| busy_timeout | ✅ 5000ms | ✅ |
