# 📚 Nghiên Cứu #2: Thiết Kế Server-Side Merge Engine — SQLite → PostgreSQL

> **Nguồn**: Google DeepSearch | **Ngày**: 16/03/2026
> **Chủ đề**: Kiến trúc merge engine phía server cho sync SQLite clients với PostgreSQL trung tâm
> **Lưu ý**: Tài liệu tham khảo, quyết định cuối cùng dựa trên bối cảnh Nodi POS.

---

## 1. Bối Cảnh

Khi nhiều thiết bị SQLite (PC + Mobile) hoạt động offline rồi sync lên PostgreSQL trung tâm, server phải:
- **Xác định ai thắng** khi 2 thiết bị sửa cùng record
- **Tính toán lại** các giá trị derived (nợ, tồn kho)
- **Đồng bộ tăng dần** (chỉ gửi thay đổi, không full dump)
- **Xử lý xóa** (tombstone) để không bị "zombie data"

> **CAP Theorem**: Hệ thống offline-first ưu tiên **Availability + Partition Tolerance** → chấp nhận **eventual consistency**.

---

## 2. Kiểm Soát Đồng Thời — LWW vs Vector Clocks

### 2.1 Last-Write-Wins (LWW) theo Timestamp

**Cách hoạt động**: Mỗi row có `updated_at`. Server so sánh → timestamp mới hơn thắng.

**Ưu điểm**: Đơn giản, ít metadata.

**Nhược điểm nghiêm trọng**:
- **Phụ thuộc hoàn toàn vào đồng hồ** của thiết bị
- Đồng hồ thạch anh trong thiết bị tiêu dùng **luôn bị trôi** (clock drift)
- Thiết bị offline lâu → không có NTP để hiệu chỉnh
- **Ví dụ nguy hiểm**: Nếu đồng hồ thiết bị A lệch nhanh 5 phút → mọi edit của A **tự động ghi đè** edit của B (dù B sửa sau)
- → **Mất data im lặng**, không có cảnh báo

> Google Spanner dùng đồng hồ nguyên tử để giải quyết. Thiết bị edge (SQLite) không có luxury này.

### 2.2 Vector Clocks — Xác Định Chính Xác Xung Đột

**Cách hoạt động**: Mỗi record mang **mảng counter logic** — 1 counter cho mỗi thiết bị.

**Ví dụ**:
```
Server ban đầu:  {NodeA: 1, NodeB: 1}

Mất mạng → 2 nhánh:
  Device A sửa → {NodeA: 2, NodeB: 1}
  Device B sửa → {NodeA: 1, NodeB: 2}

Reconnect → Server so sánh:
  NodeA: 2 vs 1 → A mới hơn ở chiều A
  NodeB: 1 vs 2 → B mới hơn ở chiều B
  → KHÔNG có bên nào "lớn hơn hoàn toàn"
  → XÁC ĐỊNH: Đây là XUNG ĐỘT ĐỒNG THỜI
```

**Ưu điểm**:
- **Không phụ thuộc đồng hồ vật lý** — dùng counter logic
- Phát hiện xung đột **chính xác 100%** — không bao giờ mất data im lặng
- Có thể giữ cả 2 phiên bản (siblings) cho intervention thủ công

**Nhược điểm**:
- Metadata **tăng tuyến tính** theo số thiết bị (mảng counter dài hơn)
- Vector clocks **chỉ phát hiện** xung đột, **không tự giải quyết** — cần logic ở application layer

### 2.3 Đề Xuất Cho POS: Chiến Lược Hybrid

> **Nghiên cứu kết luận**: POS bán lẻ (3-10 thiết bị) nên dùng **HYBRID** — phân loại data để chọn chiến lược:

| Loại dữ liệu | Chiến lược | Lý do |
|--------------|-----------|-------|
| **Giao dịch tài chính** (HĐ, nợ, thu chi, thẻ kho) | **Append-only / Event Sourcing** | Không bao giờ ghi đè — thêm record mới, xung đột KHÔNG THỂ xảy ra |
| **Master data** (KH, SP, NCC) | **Field-level LWW** (cẩn thận) | Mất tên KH tạm thời < chi phí quản lý vector clocks. Server reject timestamp tương lai |

> ⚡ **Kết luận cho Nodi**: Chính xác hướng đã đề xuất — append-only cho transactions, LWW cho master data.

---

## 3. Computed Fields — Client KHÔNG BAO GIỜ Được Tính

### 3.1 Tại Sao Client Tính = Anti-Pattern

**Kịch bản nguy hiểm**:
```
KH Minh: nợ ban đầu = 100K

Device A (offline): nhận thanh toán 50K → tính nợ = 50K → queue
Device B (offline): nhận thanh toán 20K → tính nợ = 80K → queue

Sync lên server:
  Device A gửi: current_debt = 50K
  Device B gửi: current_debt = 80K
  → Nợ đúng phải là 30K (100 - 50 - 20)
  → KHÔNG THỂ tính đúng từ 2 giá trị 50K và 80K!
```

**Vấn đề cốt lõi**: Client **không có toàn bộ dữ liệu** khi tính → kết quả SAI.

### 3.2 Server-Side Recalculation (CQRS)

> **Quy tắc tuyệt đối**: Client chỉ sync **raw events** (giao dịch). Server **tính lại** computed fields.

**Cách implement trong PostgreSQL**:

```sql
-- Trigger: Mỗi khi thêm transaction → recalculate debt
CREATE OR REPLACE FUNCTION recalculate_customer_debt()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE customers
    SET current_debt = (
        SELECT COALESCE(SUM(amount), 0)
        FROM customer_transactions
        WHERE customer_id = NEW.customer_id
    )
    WHERE id = NEW.customer_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_recalculate_debt
AFTER INSERT ON customer_transactions
FOR EACH ROW EXECUTE FUNCTION recalculate_customer_debt();
```

**Tối ưu hiệu năng**:
- Dùng **statement-level trigger** (`FOR EACH STATEMENT`) cho bulk sync thay vì row-level
- Hoặc **Materialized View** refresh concurrent
- Hoặc **API layer** xử lý trước khi ghi DB

**Push về client**: Giá trị computed field là **READ-ONLY** trên SQLite client — không được sửa từ UI.

> ⚡ **Kết luận cho Nodi**: `current_debt`, `stock_quantity`, `total_spent` phải là server-computed, push về client dạng read-only. Đúng hướng đề xuất.

---

## 4. Giải Quyết Xung Đột — Row-Level vs Field-Level

### 4.1 Row-Level LWW — ❌ PHÁ HỦY DATA

**Kịch bản**:
```
Device A: Sửa SĐT khách hàng Minh
Device B: Sửa Địa chỉ khách hàng Minh (cùng lúc)

Row-Level LWW:
  → Device B thắng (timestamp mới hơn)
  → SĐT mới của Device A BỊ MẤT!
```

### 4.2 Field-Level Merge — ✅ GIỮ CẢ 2 THAY ĐỔI

```
Field-Level Merge:
  → Phone = giá trị từ Device A ✅
  → Address = giá trị từ Device B ✅
  → Cả 2 thay đổi đều được giữ!
```

### 4.3 Implement Trong PostgreSQL

**Cách 1: JSON Patch + Staging Table**
```sql
-- 1. Client gửi partial update
-- { "id": "uuid-123", "updated_at": "...", "changes": {"phone": "0987..."} }

-- 2. Server insert vào staging
INSERT INTO sync_inbox (uuid, updated_at, changes_json) VALUES (...);

-- 3. Background worker dùng MERGE (PG 15+) apply từng field
MERGE INTO customers AS target
USING sync_inbox AS source
ON target.uuid = source.uuid
WHEN MATCHED AND source.updated_at > target.updated_at THEN
    UPDATE SET
        phone = COALESCE(source.changes_json->>'phone', target.phone),
        address = COALESCE(source.changes_json->>'address', target.address),
        updated_at = source.updated_at;
```

**Cách 2: BEFORE UPDATE Trigger**
```sql
-- Trigger so sánh từng field, reject thay đổi cũ hơn
CREATE OR REPLACE FUNCTION field_level_merge()
RETURNS TRIGGER AS $$
BEGIN
    -- Nếu payload cũ hơn data hiện tại
    IF NEW.updated_at < OLD.updated_at THEN
        -- Giữ từng field nào đã mới hơn
        IF NEW.phone IS DISTINCT FROM OLD.phone THEN
            NEW.phone := OLD.phone; -- Giữ giá trị hiện tại (mới hơn)
        END IF;
        -- ... tương tự cho các field khác
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

> ⚡ **Kết luận cho Nodi**: Cần implement **field-level merge** cho master data (customers, products, suppliers). Có thể bắt đầu với JSON patch + staging table.

---

## 5. Đồng Bộ Tăng Dần — Cursor & Sync Token

### 5.1 Cursor Protocol

**Nguyên tắc**: Client giữ `last_sync_cursor`. Push/Pull chỉ gửi/nhận records **SAU** cursor đó.

### 5.2 Tại Sao KHÔNG Dùng `xmin` (PostgreSQL)

- `xmin` = transaction ID nội bộ PG → bị **wraparound** ở high-volume DB
- `xmin` **không thấy DELETE** — row bị xóa biến mất, client không bao giờ biết
- → Gây **zombie data** (data đã xóa lại sống lại ở client)

### 5.3 Giải Pháp: Operation Log (Sync Journal)

```sql
CREATE TABLE sync_journal (
    seq_id BIGSERIAL PRIMARY KEY,     -- Tăng đơn điệu, không bao giờ gap
    table_name TEXT NOT NULL,          -- 'customers', 'invoices', ...
    record_uuid TEXT NOT NULL,         -- UUID của record
    operation TEXT NOT NULL,           -- 'INSERT', 'UPDATE', 'DELETE'
    payload JSONB,                     -- Data snapshot tại thời điểm
    device_id TEXT,                    -- Thiết bị gốc
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Client query: SELECT * FROM sync_journal WHERE seq_id > $client_cursor
-- → Nhận TẤT CẢ thay đổi theo đúng thứ tự, bao gồm cả DELETE
```

**Ưu điểm**:
- Đảm bảo **giao chính xác mọi thay đổi** theo đúng thứ tự
- Bao gồm DELETE (không zombie data)
- Cursor là integer đơn giản → so sánh nhanh

> ⚡ **Kết luận cho Nodi**: Cần tạo `sync_journal` table trên PostgreSQL thay vì dựa vào timestamp hoặc xmin.

---

## 6. Xử Lý Xóa — Tombstone & Garbage Collection

### 6.1 Vấn đề: Zombie Data

```
Server: DELETE customer Minh (xóa thật)

Device A (offline 3 ngày): Vẫn có customer Minh trong SQLite
Device A reconnect → push customer Minh lên server
→ Customer Minh SỐNG LẠI! (zombie data)
```

### 6.2 Giải pháp: Soft Delete + Tombstone

```sql
-- KHÔNG BAO GIỜ DELETE thật
-- Thay vào đó:
UPDATE customers SET
    _deleted = true,
    _deleted_at = NOW()
WHERE uuid = 'uuid-123';

-- Record này vẫn tồn tại → cursor protocol gửi cho client
-- Client nhận → xóa local → đồng bộ
```

### 6.3 Garbage Collection (GC)

**Vấn đề**: Tombstone tích lũy → DB phình to, query chậm.

**Giải pháp**: Xóa tombstone KHI VÀ CHỈ KHI tất cả devices đã xác nhận:

```sql
-- Bảng tracking cursor của từng device
CREATE TABLE device_sync_status (
    device_id TEXT PRIMARY KEY,
    last_confirmed_cursor BIGINT,
    last_sync_at TIMESTAMPTZ
);

-- GC: Xóa tombstone mà TẤT CẢ devices đã nhận
DELETE FROM customers
WHERE _deleted = true
AND sync_seq_id < (SELECT MIN(last_confirmed_cursor) FROM device_sync_status);
```

**Quy tắc**: Nếu device offline > 30 ngày → **bắt buộc full baseline replication** (không incremental nữa, tải lại toàn bộ DB).

---

## 7. Case Studies Thực Tế

### 7.1 CouchDB/PouchDB — Multi-Master + Revision Trees

- Document có `_rev` (revision ID) → tạo branching history như Git
- Xung đột → giữ **cả 2 revision** (siblings) → app logic chọn
- **Nhược điểm**: Metadata nặng, revision tree phình to trên thiết bị edge
- **Hướng mới**: Couchbase Mobile chuyển sang **version vectors + LWW** để đơn giản hơn

### 7.2 Supabase Realtime — PostgreSQL WAL Streaming

- Đọc WAL → broadcast qua WebSocket (Erlang processes)
- Row Level Security (RLS) filter trước khi gửi
- **Giới hạn**: Chỉ streaming, **không phải merge engine**. Client offline = bỏ lỡ → phải tự reconcile

### 7.3 MongoDB Realm Sync — CRDT tự động

- Embedded Realm DB trên device → sync tự động với MongoDB Atlas
- CRDT dưới hood → merge concurrent edits không cần manual logic
- **Giới hạn**: Lock-in vào MongoDB ecosystem, không relational

### 7.4 PowerSync — ĐẶC BIỆT ĐÁNG CHÚ Ý

> **PowerSync là mô hình gần nhất với kiến trúc Nodi cần xây.**

**Kiến trúc PowerSync**:
```
PostgreSQL (WAL) → PowerSync Service (cache) → SQLite (client)
                                                    │
                                                    ▼
SQLite writes → queue → Developer's Backend API → PostgreSQL
                         (validation, CQRS, conflict resolution)
```

**Đặc điểm quan trọng**:
- **Tách read/write path**: Read stream từ PG → SQLite. Write đi qua **backend API của developer**
- Backend API kiểm soát: validation, CQRS (tính computed fields), conflict resolution
- Nếu write bị reject → server trả success (clear queue) nhưng **rollback local change** → client tự hội tụ

> ⚡ **Đây là mô hình lý tưởng cho Nodi**: VPS (Rust/Axum) đóng vai trò PowerSync Service — đọc PG WAL, gửi delta xuống client. Client writes đi qua API, server validate + merge.

### 7.5 ElectricSQL — PostgreSQL là Control Plane

- PG là "control plane" duy nhất — access control qua schema + constraints
- Sync "shapes" (subset data) qua HTTP API
- **Giới hạn**: Không có application layer trung gian → validation phải ở DB level

---

## 8. Kết Luận Cho Nodi POS

### Xác nhận từ nghiên cứu:

| Quyết định | Nghiên cứu nói gì | Cho Nodi |
|-----------|-------------------|----------|
| Server-side computed fields | **"Client tính = anti-pattern nghiêm trọng"** | ✅ Xác nhận: nợ, tồn kho phải server tính |
| Field-level merge | **"Row-level LWW phá hủy data"** | ✅ Cần implement field-level cho master data |
| Sync journal / operation log | **"Dùng sequence ID, không dùng xmin hay timestamp"** | 📌 Cần tạo `sync_journal` trên PG |
| Soft delete + tombstone | **"Hard DELETE = zombie data"** | 📌 Cần thêm `_deleted` flag cho mọi bảng sync |
| Tombstone GC | **"Xóa chỉ khi tất cả devices xác nhận"** | 📌 Cần `device_sync_status` tracking |
| PowerSync model | **"Tách read/write, writes qua backend API"** | 📌 VPS Axum = merge engine, validate + CQRS |

### Phát hiện MỚI (chưa có trong đề xuất ban đầu):

1. **Staging table** (`sync_inbox`) — buffer sync payload trước khi merge vào main tables
2. **Sync journal** (`sync_journal`) — append-only log thay cho cursor timestamp
3. **Device sync status** tracking — biết device nào đã nhận update gì
4. **Tombstone GC** policy — device offline > 30 ngày → force full replication
5. **BEFORE UPDATE trigger** — field-level merge tại DB level (không cần app code)
6. **Statement-level trigger** — tối ưu performance cho bulk sync

---

*Tài liệu tham khảo — chỉ để hỗ trợ quyết định, không phải chén thánh.*
