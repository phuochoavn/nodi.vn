# 📚 Nghiên Cứu #3: Kiến Trúc Đồng Bộ SQLite ↔ Cloud Cho Ứng Dụng Mobile & Desktop

> **Nguồn**: Google DeepSearch | **Ngày**: 16/03/2026
> **Chủ đề**: Schema migration, change tracking, sync engines, tombstone/GC, và tối ưu bandwidth
> **Lưu ý**: Tài liệu tham khảo, quyết định cuối cùng dựa trên bối cảnh Nodi POS.

---

## 1. Migration Schema: Từ Auto-Increment Sang UUID

### 1.1 Tại Sao Auto-Increment Thất Bại

Trong hệ thống phân tán, nhiều thiết bị offline tạo record đồng thời → **trùng ID** → collision khi sync. Giải pháp duy nhất: UUID/ULID tạo tại client.

### 1.2 So Sánh Hiệu Năng ID

| Metric (10,000 records) | Integer PK | UUIDv4 (Random) | ULID/UUIDv7 (Time-sorted) |
|--------------------------|:---------:|:---------------:|:-------------------------:|
| Tốc độ INSERT | Baseline | **Chậm 5-8%** | Chậm 3-5% |
| Tốc độ Lookup | Baseline | Chậm 3-5% | Chậm 2-4% |
| Dung lượng (1M rows) | ~8 MB | ~34 MB | ~25 MB |

> **Kết luận**: ULID/UUIDv7 là **thỏa hiệp tối ưu** — gần bằng integer về tốc độ, ít fragmentation hơn UUIDv4, dung lượng nhỏ hơn UUID string.

### 1.3 Shadow-Table Migration — Cách Migrate SQLite An Toàn

SQLite **KHÔNG hỗ trợ** ALTER COLUMN. Phải dùng pattern "tạo bảng mới → copy data → swap":

| Phase | Thao tác | Chi tiết |
|:-----:|----------|----------|
| 1 | **Tắt FK constraints** | `PRAGMA foreign_keys = OFF` — **PHẢI** chạy ngoài transaction |
| 2 | **Tạo shadow table** | Schema mới, PK là TEXT thay vì INTEGER |
| 3 | **Copy + chuyển đổi** | `INSERT INTO new_table SELECT ... printf('%012x', id)` — deterministic! |
| 4 | **Update FK con** | Dùng **cùng công thức** `printf('%012x', parent_id)` cho child tables |
| 5 | **Swap** | `DROP TABLE old; ALTER TABLE new RENAME TO old;` |
| 6 | **Bật lại FK** | `PRAGMA foreign_keys = ON`, rebuild indexes |

> ⚠️ **QUAN TRỌNG**: Phải dùng **deterministic generation** (integer → hex cố định), KHÔNG dùng random UUID trong quá trình copy — nếu random, FK child tables không thể map lại.

> ⚡ **Kết luận cho Nodi**: Nodi có **46 migrations** dùng INTEGER PK. Migration sang UUID cần shadow-table pattern cho từng bảng, làm **tuần tự** theo dependency order.

### 1.4 Dual-Column Hybrid — Chiến Lược Thay Thế

Thay vì rebuild toàn bộ table, **giữ integer PK** cho local queries + **thêm column UUID** cho sync:

```
┌─────────────────────────────────────────┐
│ products                                 │
│                                          │
│ id INTEGER PK    ← Local queries, FK     │
│ uuid TEXT UNIQUE ← Sync layer identifier │
│ name TEXT                                │
│ ...                                      │
└─────────────────────────────────────────┘
```

**Ưu điểm**: Không cần rebuild table, giữ hiệu năng B-tree integer, ít rủi ro
**Nhược điểm**: Sync middleware phải translate giữa local integer ↔ global UUID

> ⚡ **Kết luận cho Nodi**: Dual-column là **chiến lược an toàn nhất** cho giai đoạn migration — giữ integer PK hiện tại, thêm UUID column. Đúng hướng đề xuất Phase 1.

---

## 2. Change Data Capture — Theo Dõi Thay Đổi Tại Client

### 2.1 Application-Layer Tracking — ❌ KHÔNG TIN CẬY

Dựa vào ORM/middleware đặt `updated_at` mỗi khi write → **KHÔNG đảm bảo**:
- Raw SQL, migrations, background processes → **bypass hoàn toàn**
- → Data divergence im lặng, không phát hiện được

> **"Application-level tracking bị deprecated trong kiến trúc offline-first cấp enterprise."**

### 2.2 SQLite Triggers + Bitmask Changelog — ✅ ĐẢM BẢO NHƯNG CHẬM

Tạo AFTER trigger cho mỗi bảng → ghi vào `_changelog` table:
- **Ưu điểm**: Đảm bảo 100% mọi mutation đều bị bắt
- **Tối ưu**: Dùng **bitmask** ghi field nào thay đổi thay vì copy cả row
  - Ví dụ: `name`=2, `age`=4, `weight`=8 → sửa name+weight = bitmask `10`
- **Nhược điểm nghiêm trọng**: Trigger chạy **đồng bộ** trong cùng DB lock → **gấp đôi disk I/O**. Thêm/xóa column → phải rewrite toàn bộ triggers

### 2.3 WAL Tailing — ⚡ CỰC NHANH NHƯNG KHÓ FILTER

Bật `PRAGMA journal_mode = WAL` → SQLite ghi thay đổi vào file WAL thay vì trực tiếp:
- Giảm latency từ **>30ms → <1ms** mỗi transaction
- Cho phép đọc concurrent trong khi đang ghi
- **Nhược điểm**: WAL chứa binary page data → khó filter theo row/tenant logic

### 2.4 SQLite Session Extension — 🏆 CHUẨN VÀNG

> **"Đây là tiêu chuẩn tuyệt đối cho offline-first data capture."**

Session Extension hoạt động **dưới SQL layer** — track thay đổi ở mức engine thấp nhất:

| Format | Chứa gì | Dùng khi |
|--------|---------|----------|
| **Changeset** | Giá trị cũ + mới (full context) | Server cần biết state trước khi sửa (conflict resolution phức tạp) |
| **Patchset** | Chỉ PK + fields đã sửa (compact) | Tối ưu bandwidth, đủ cho LWW |

- Tạo **binary blob** (như Git diff) — gọn, nhanh
- Không gấp đôi I/O như triggers
- Không bypass được như application-layer

> ⚡ **Kết luận cho Nodi**: Hiện tại Nodi dùng application-layer (`updated_at` triggers trong SQLite). **Chưa cần** chuyển sang Session Extension ngay, nhưng nên biết đây là hướng upgrade tương lai. Giai đoạn đầu, `updated_at` trigger + sync journal trên server là **đủ dùng**.

---

## 3. So Sánh Các Sync Engine

### 3.1 Turso (libSQL) — Physical WAL Replication

- Fork của SQLite → sync **toàn bộ DB file** qua WAL shipping
- **Ưu**: Đơn giản, developer dùng SQLite bình thường
- **Nhược**: Bắt buộc **1 DB per tenant** — không filter được row-level
- **Không phù hợp Nodi**: Nodi cần partial sync (mỗi store chỉ nhận data của store đó)

### 3.2 PowerSync — 🏆 PHÙ HỢP NHẤT

- Middleware giữa PostgreSQL và SQLite clients
- Đọc PG WAL → filter theo **"Buckets"** (SQL-based sync rules) → gửi delta xuống client
- Client SQLite schema **có thể khác** PG schema
- **Ưu**: Linh hoạt, partial replication, mỗi user/store nhận data riêng
- Conflict: LWW mặc định + hooks cho custom merge
- Scale tốt: Dùng MongoDB backend cho bucket state

> ⚡ **Kết luận cho Nodi**: PowerSync model = **template** cho kiến trúc mới. VPS Axum đóng vai trò PowerSync: đọc PG, filter theo store_id, gửi delta xuống SQLite.

### 3.3 ElectricSQL (Next) — HTTP Shapes

- Ban đầu dùng CRDT nặng → **đã bỏ** vì không scale (memory tỉ lệ client × data)
- Rebuild thành HTTP sync engine — "Shapes" (table + WHERE clause)
- Gửi log thay đổi qua HTTP offset — stateless, scale tốt
- **Nhược**: Không quản lý SQLite local, conflict resolution do developer tự làm

### 3.4 cr-sqlite — Peer-to-Peer CRDT

- Extension C cho SQLite → biến table thành CRDT (Conflict-free Replicated Relations)
- Vector clocks + tombstone tracking nhúng vào SQLite layer
- 2 thiết bị sync trực tiếp qua WebRTC/BLE — **không cần server**
- **Nhược**: INSERT **chậm 2.5x**, tốn disk cho metadata
- **Không phù hợp Nodi**: Overkill cho 5-6 devices, chi phí hiệu năng quá cao

### Bảng so sánh tổng hợp

| Engine | Cách sync | Multi-tenancy | Conflict Resolution | Nhược điểm chính |
|--------|-----------|:-------------:|:-------------------:|-----------------|
| **Turso** | Physical WAL | 1 DB/tenant | LWW (WAL order) | Không partial replication |
| **PowerSync** | Logical (PG WAL → Buckets) | Dynamic filter | LWW + custom hooks | Cần manage middleware |
| **ElectricSQL** | HTTP Shapes | Dynamic filter | Developer tự làm | Không quản lý client SQLite |
| **cr-sqlite** | Peer-to-peer CRDT | Query filter | CRDT tự động | Insert chậm 2.5x |

---

## 4. Xóa Dữ Liệu — Tombstone & Garbage Collection

### 4.1 Hard DELETE = Zombie Data

```
Device A: DELETE customer Minh → row biến mất → KHÔNG CÓ payload để sync
Device B: Vẫn giữ customer Minh → push lên server → Minh SỐNG LẠI!
```

### 4.2 Soft Delete — ⚠️ Giải pháp tạm, ô nhiễm query

- Đổi `is_deleted = true` thay vì DELETE thật
- **MỌI query** phải thêm `WHERE is_deleted = 0` — dễ quên, tốn performance
- Bảng phình to với records "chết" → index chậm dần

### 4.3 Tombstone — ✅ Giải pháp đúng

```
1. DELETE thật khỏi bảng chính (giữ data sạch)
2. Sync layer tạo tombstone (PK + timestamp) trong bảng riêng
3. Server nhận tombstone → xóa canonical record
4. Server push tombstone xuống các client khác → client xóa local
```

**Quy tắc xung đột**: **"Delete Always Wins"** — nếu Device A gửi tombstone, Device B gửi update cho cùng row → server xóa, discard update.

### 4.4 Garbage Collection — gc_grace_seconds

- Tombstone tích lũy → DB phình to
- **GC policy**: Xóa tombstone sau **10-30 ngày** (gc_grace_seconds)
- **Edge case**: Device offline > 30 ngày → connect lại → push data cũ → zombie!
- **Giải pháp**: Nếu device epoch cũ hơn GC horizon → **reject delta, force full snapshot**

> ⚡ **Kết luận cho Nodi**: Cần implement tombstone + GC, đúng như nghiên cứu #2. Threshold 30 ngày hợp lý cho POS bán lẻ.

### 4.5 VACUUM — Tái Sử Dụng Disk

- SQLite **không tự giảm file size** khi xóa data — chỉ đánh dấu "free"
- `VACUUM` rebuild toàn bộ DB → thu gọn file
- ⚠️ **Cần 2x disk space** + **exclusive lock toàn DB** — KHÔNG chạy khi đang sync
- Chạy khi: app idle, startup, hoặc ratio deleted/active > threshold

---

## 5. Tối Ưu Bandwidth — Binary Serialization & Compression

### 5.1 JSON = Lãng Phí

- Key text lặp lại ("customer_id" × 1000 lần)
- Dấu ngoặc, dấu phẩy, encoding overhead
- Parse JSON tốn CPU → tốn pin mobile

### 5.2 Binary Serialization — Giảm 40%+

| Format | Kích thước | So với JSON |
|--------|:---------:|:-----------:|
| **JSON đầy đủ** | 102 bytes | Baseline |
| **JSON rút gọn** | 86 bytes | -16% |
| **Protobuf** | 62 bytes | **-39%** |
| **MessagePack** | 61 bytes | **-40%** |

SQLite Session Extension tạo **patchset binary** tự nhiên — không cần convert thủ công.

### 5.3 Compression + Batching

- **Gzip/Brotli** trên binary payload → nén thêm
- **Batching**: KHÔNG gửi từng row — queue thay đổi → gửi batch theo schedule hoặc threshold
- **Exponential backoff**: Khi mất mạng, retry với delay tăng dần — tránh thundering herd

### 5.4 WebSocket vs HTTP Polling

| Phương thức | Hiệu quả | Dùng khi |
|------------|:---------:|----------|
| HTTP Polling | ❌ Lãng phí | Cần tương thích cũ |
| **WebSocket** | ✅ Tối ưu | Real-time push, persistent connection |
| **MQTT** | ✅ Cực tối ưu | IoT, bandwidth cực thấp |

> ⚡ **Kết luận cho Nodi**: Nodi đã có WebSocket (`/ws/sync`). Cần nâng cấp từ "notify → full pull" thành "push delta changes qua WS". Payload nên dùng JSON gọn (giai đoạn đầu), sau đó upgrade sang MessagePack nếu cần.

---

## 6. Kết Luận Cho Nodi POS

### Xác nhận từ nghiên cứu:

| Quyết định | Nghiên cứu nói gì | Cho Nodi |
|-----------|-------------------|----------|
| Dual-column (integer + UUID) | Chiến lược migration an toàn nhất | ✅ Xác nhận Phase 1 |
| Shadow-table migration | Pattern chuẩn cho SQLite ALTER PK | 📌 Cần dùng khi migrate |
| `updated_at` triggers | Đủ dùng — Session Extension là upgrade sau | ✅ Giữ approach hiện tại |
| PowerSync model | Template lý tưởng cho Nodi architecture | ✅ Xác nhận |
| Tombstone + GC | "Delete Always Wins", gc_grace 30 ngày | 📌 Cần implement |
| VACUUM scheduling | Chạy khi idle, không khi sync | 📌 Cần thêm vào app lifecycle |
| Binary serialization | MessagePack/Protobuf giảm 40% | 📌 Upgrade sau (JSON trước) |
| WebSocket push delta | Thay cho notify → full pull | ✅ Đúng hướng Phase 4 |

### Phát hiện MỚI:

1. **UUIDv7/ULID chỉ chậm 3-5%** so với integer (benchmark 10K records) — acceptable
2. **Deterministic UUID generation** khi migrate: `printf('%012x', id)` — đảm bảo FK mapping
3. **SQLite Session Extension** — future upgrade path cho change tracking (binary patchsets)
4. **cr-sqlite**: Peer-to-peer CRDT cho SQLite tồn tại, nhưng insert chậm 2.5x → overkill cho Nodi
5. **VACUUM** cần schedule riêng — exclusive lock, tốn 2x disk, không chạy khi sync
6. **Batching + Exponential backoff** — critical cho mobile cellular network

---

*Tài liệu tham khảo — chỉ để hỗ trợ quyết định, không phải chén thánh.*
