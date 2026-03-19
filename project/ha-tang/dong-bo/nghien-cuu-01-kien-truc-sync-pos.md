# 📚 Nghiên Cứu #1: Kiến Trúc Đồng Bộ Cho Hệ Thống POS Offline-First

> **Nguồn**: Google DeepSearch | **Ngày**: 16/03/2026
> **Mục đích**: Tham khảo chiến lược sync từ các POS hàng đầu thế giới
> **Lưu ý**: Đây là tài liệu tham khảo, không phải chén thánh. Quyết định cuối cùng dựa trên bối cảnh Nodi POS.

---

## 1. Tổng Quan: Mô Hình Offline-First

### Tại sao offline-first?

Kiến trúc POS truyền thống phụ thuộc 100% vào internet → khi mất mạng = **dừng bán hàng**. Mô hình offline-first **đảo ngược**: coi database local (SQLite) là **nguồn chính**, cloud chỉ là **đích đồng bộ phụ**.

> **Nguyên tắc cốt lõi**: Mọi thao tác quan trọng (tạo hóa đơn, trừ kho, thu nợ) phải xảy ra **hoàn toàn tại local** với độ trễ gần 0ms, tách biệt khỏi trạng thái mạng.

### Thách thức

Khi nhiều nhân viên bán hàng đồng thời trên nhiều thiết bị riêng biệt mà không có server trung tâm điều phối:
- Phải **tạo ID** không trùng nhau
- Phải **quản lý trạng thái** phân tán
- Phải **giải quyết xung đột** khi kết nối lại

---

## 2. Các Mô Hình Sync Của POS Hàng Đầu

### 2.1 Cloud-Dependent — Toast POS & Shopify POS

**Cách hoạt động**: Các thiết bị giao tiếp qua cloud. Khi mất mạng → **chạy độc lập, không thấy nhau**.

**Toast POS**:
- Offline: mỗi thiết bị **không thấy đơn hàng** của thiết bị khác
- Tránh trùng ID bằng công thức: `(Device_ID + 2) × 1000`
  - Terminal 1 → hóa đơn bắt đầu từ 3000
  - Terminal 2 → bắt đầu từ 4000
- Khóa các thao tác nguy hiểm (chốt ca, resync) để tránh mất data
- **Đang chuyển sang** mô hình "local hub device" trên LAN

**Shopify POS**:
- Offline: chỉ **thanh toán tiền mặt** và queue thẻ (nếu bật)
- Không có sync ngang thiết bị qua LAN
- Khi online lại → background process tự đẩy dữ liệu lên cloud

### 2.2 Active/Passive LAN Server — Lightspeed Restaurant

**Cách hoạt động**: Một iPad làm "server local", các iPad khác là "thin client".

- **Active device**: Giữ database master, xử lý mọi yêu cầu
- **Passive devices**: Gửi thao tác đến Active device qua Wi-Fi
- Yêu cầu: DHCP reservation, không client isolation, mở cổng 22/80/443/7373/8080/9100
- **Khi mất internet**: Active device vẫn điều phối đơn hàng local → **nhà bếp tiếp tục nhận order**
- ⚠️ **Rủi ro SPOF**: Nếu Active device hỏng → TOÀN BỘ hệ thống ngừng hoạt động

### 2.3 Local Gateway Routing — Square POS

**Cách hoạt động**: Thiết bị ping router local để duy trì kết nối phần cứng, queue dữ liệu cloud.

- Offline: lưu thanh toán thẻ mã hóa trong **secure enclave**
- **Giới hạn thời gian nghiêm ngặt** (TTL): nếu không online lại trong **24-72 giờ** → giao dịch thẻ **tự hủy**
- Rủi ro tài chính chuyển sang merchant nếu thẻ bị từ chối

### So sánh tổng hợp

| Mô hình | Sync offline? | Ưu điểm | Nhược điểm |
|---------|:------------:|---------|-----------|
| **Cloud-Dependent** (Toast, Shopify) | ❌ | Đơn giản, không cần cấu hình LAN | Thiết bị bị cô lập, mất visibility |
| **Active/Passive** (Lightspeed) | ✅ Full | Nhà bếp/đơn hàng vẫn hoạt động | SPOF: Active device hỏng = chết hệ thống |
| **Gateway Routing** (Square) | ⚠️ Partial | Di động tốt, offline payment | TTL nghiêm ngặt (24-72h) |

---

## 3. Chiến Lược Đồng Bộ Dữ Liệu

### 3.1 Full State Transfer (Full Dump) — ❌ KHÔNG KHẢ THI

> **"Trong bối cảnh bán lẻ hiện đại, phương pháp full dump hoàn toàn không khả thi."**

- Tốn bandwidth, khóa UI trong lúc sync
- **Phá hủy dữ liệu**: thiết bị push sau sẽ ghi đè mọi thay đổi của thiết bị trước
- Tất cả POS hiện đại đều **đã loại bỏ** phương pháp này

> ⚡ **Kết luận cho Nodi**: Đây chính xác là vấn đề hiện tại của `sync_commands.rs` — push/pull toàn bộ DB mỗi 60s.

### 3.2 Incremental (Delta) Sync — ✅ PHỔ BIẾN NHẤT

**Cách hoạt động**: Chỉ truyền **rows đã thay đổi** kể từ lần sync cuối.

**PowerSync & ElectricSQL**:
- Kết nối trực tiếp PostgreSQL WAL (Write-Ahead Log) qua logical replication
- Parse log real-time → filter theo "buckets/shapes" → stream delta xuống SQLite client
- Khi offline: local writes queue trong outbox → reconnect → push lên PG

**LiteSync**:
- Nhúng sync logic vào thư viện SQLite C
- Truyền ở mức byte → sync đạt **vài millisecond**

**Hạn chế**: Mặc định dùng **Last-Write-Wins (LWW)** theo timestamp vật lý. Nếu đồng hồ thiết bị lệch → **mất data im lặng**.

### 3.3 Event Sourcing + CQRS — ⚠️ PHỨC TẠP

**Cách hoạt động**: Không lưu trạng thái hiện tại, mà lưu **chuỗi sự kiện bất biến**.

Ví dụ thay vì lưu "Đơn hàng = 500K":
```
OrderCreated
ItemAdded(Phân NPK, 200K)
ItemAdded(Thuốc sâu, 300K)
DiscountApplied(0)
```

**Ưu điểm cho sync**:
- Khi offline: thiết bị thêm events vào log local
- Reconnect: merge tất cả event logs → **không mất data vì event là bất biến**
- Audit trail hoàn hảo

**Nhược điểm**:
- Cần CQRS (tách read/write model) để đọc hiệu quả
- Logic phức tạp khi events đến không đúng thứ tự
- Chi phí migration cực cao

### 3.4 CRDT (Conflict-Free Replicated Data Types) — 🏆 GOLD STANDARD

**Cách hoạt động**: Cấu trúc dữ liệu toán học đảm bảo **hội tụ xác định** — 2 replica nhận cùng tập updates (bất kể thứ tự) → **luôn ra cùng kết quả**, không cần server trọng tài.

**Tại sao tốt hơn LWW**:
- LWW dựa vào đồng hồ vật lý → nếu 1 tablet lệch giờ 24h → **mọi edit của nó sẽ ghi đè tất cả**
- CRDT dùng **Vector Clocks** (bộ đếm logic tăng đơn điệu) → xác định chính xác quan hệ nhân quả

**Các loại CRDT**:
- **State-based**: Gửi toàn bộ cấu trúc dữ liệu → merge bằng phép **semilattice join** (tốn bandwidth)
- **Op-based**: Chỉ gửi **thao tác** ("tăng counter thêm 1") (tiết kiệm bandwidth)
- **Delta-State**: Tối ưu — chỉ gửi phần thay đổi

**Ví dụ thực tế**: Chick-fil-A dùng **Ditto** (CRDT + BLE mesh) → POS giao tiếp trực tiếp qua Bluetooth, không cần Wi-Fi hay internet

### So sánh chiến lược sync

| Chiến lược | Truyền gì | Giải quyết xung đột | Trade-off |
|-----------|----------|---------------------|-----------|
| **Full Dump** | Toàn bộ DB | Ghi đè → mất data | ❌ Không khả thi |
| **Delta Sync** | Chỉ rows thay đổi (WAL) | Last-Write-Wins (timestamp) | ✅ Tiết kiệm bandwidth, ⚠️ mất data nếu clock lệch |
| **Event Sourcing** | Log sự kiện bất biến | Không xung đột ở DB, xử lý ở application | ✅ Audit tốt, ❌ phức tạp cực kỳ |
| **CRDT** | Thao tác logic / delta-state | Hội tụ toán học (Vector Clocks) | ✅ Đảm bảo 100%, ⚠️ tốn storage cho metadata |

---

## 4. Giải Quyết Xung Đột Business Logic

### 4.1 CAP Theorem trong POS

> Hệ thống phân tán chỉ đảm bảo **2/3**: Consistency, Availability, Partition Tolerance.
> POS bán lẻ **bắt buộc** Partition Tolerance (mất mạng là thực tế) → phải chọn giữa Consistency hoặc Availability.
> → **Tất cả POS hiện đại chọn Availability** (cho bán hàng tiếp) → chấp nhận "eventual consistency".

### 4.2 Xung đột tồn kho — Bán vượt (Overselling)

**Kịch bản**: Kho còn 5 chai thuốc sâu. Mất mạng. NV1 bán 5 chai trên PC. NV2 bán 5 chai trên mobile. → Tồn kho thực = **-5**.

**Giải pháp CRDT**: Dùng **PN-Counter** (Positive-Negative Counter)
- Không lưu `stock_quantity = 5` (sẽ bị ghi đè)
- Lưu riêng: `total_additions` và `total_subtractions` trên mỗi node
- Tồn kho = `SUM(additions) - SUM(subtractions)` → **không bao giờ mất giao dịch**

**Xử lý business**: Khi sync phát hiện tồn kho âm → **flag ngay** → chuyển kho khẩn cấp hoặc thông báo quản lý. Một số POS cao cấp cấp phát **"offline safety stock"** riêng cho từng terminal để dừng bán trước khi về 0.

> ⚡ **Kết luận cho Nodi**: Tồn kho **KHÔNG NÊN** sync trực tiếp qua `stock_quantity`. Nên tính từ `SUM(product_transactions)` — đây chính xác là hướng đi đã đề xuất.

### 4.3 Xung đột công nợ — Bán nợ khi offline

**Kịch bản**: KH Minh có giới hạn nợ 10 triệu. Mất mạng. NV1 bán nợ 8 triệu trên PC. NV2 bán nợ 8 triệu trên mobile. → Nợ thực = 16 triệu (vượt 6 triệu).

**Giải pháp**: **Ghi nhận Intent (Ý định)**
- Thay vì tranh nhau ghi đè field `current_debt`, ghi mỗi giao dịch là **ledger entry bất biến**
- Cả 2 khoản nợ 8 triệu đều được sync thành công lên cloud như **sự thật**
- Application layer đánh giá hậu kỳ → flag vượt giới hạn → thông báo quản lý

> **Nguyên tắc vàng**: Ưu tiên **ghi nhận sự thật** (intent), chuyển xung đột từ **lỗi DB không khôi phục được** sang **quy trình kế toán xử lý được**.

> ⚡ **Kết luận cho Nodi**: `current_debt` phải là computed field (tính từ `SUM(customer_transactions)`), không phải field sync trực tiếp — đúng hướng đã đề xuất.

---

## 5. Chiến Lược Tạo Khóa Chính (Primary Key)

### 5.1 Auto-Increment Integer — ❌ THẤT BẠI

> **"Trong hệ thống POS phân tán, dựa vào auto-increment = tạo Single Point of Failure chết người."**

- Thiết bị offline không thể hỏi server cho ID tiếp theo
- 2 thiết bị tạo cùng ID → **collision khi sync**
- Composite key (Device_ID + Local_ID) → phức tạp schema, khó migrate

> ⚡ **Đây chính xác là vấn đề hiện tại của Nodi**: 46 migrations đều dùng `INTEGER AUTOINCREMENT`.

### 5.2 UUIDv4 — ⚠️ HIỆU NĂNG KÉM

- Hoàn toàn random → insert vào B-tree ở vị trí ngẫu nhiên
- Gây **Index Fragmentation và Page Splitting** → chậm dần khi data lớn
- Tốn 16 bytes (gấp đôi BigInt 8 bytes) → tốn RAM cho index
- Khi DB có hàng triệu giao dịch → **tốc độ insert giảm theo cấp số nhân**

### 5.3 UUIDv7 — 🏆 TỐI ƯU NHẤT

> **"UUIDv7 là best practice cho hệ thống POS offline-first hiện đại."**

- 48-bit timestamp (millisecond) ở **đầu** → giá trị **tăng đơn điệu theo thời gian**
- Insert vào B-tree chỉ cần **append ở cuối** (không page splitting)
- Cải thiện hiệu năng insert **30-50%** so với UUIDv4
- Tạo hoàn toàn tại client, không cần server
- Tương thích chuẩn UUID 128-bit

### 5.4 Các lựa chọn khác

| Chiến lược | Nơi tạo | Hiệu năng B-tree | Kích thước | Phù hợp POS offline? |
|-----------|---------|:------------------:|:----------:|:---------------------:|
| **Auto-Increment** | Server | ✅ Tốt | 8 bytes | ❌ Không thể tạo offline |
| **UUIDv4** | Client | ❌ Kém (random) | 16 bytes | ⚠️ Bloat khi data lớn |
| **Snowflake ID** | Client (cần Worker ID) | ✅ Tốt | 8 bytes | ⚠️ Cần phân phối Worker ID trước |
| **UUIDv7** | Client | ✅ Tốt (time-ordered) | 16 bytes | ✅ **Tối ưu** |
| **ULID** | Client | ✅ Tốt | 16 bytes | ✅ Tốt (Base32 dễ đọc) |

### Dual-ID Identity Pattern (Best Practice)

```
┌────────────────────────────────────────────┐
│ Mỗi record có 2 ID:                        │
│                                             │
│ client_id (UUIDv7): Tạo tại client         │
│   → Dùng làm PK local + FK + UI rendering  │
│   → Luôn có giá trị từ lúc tạo             │
│                                             │
│ server_id (INTEGER): Server gán sau sync    │
│   → NULL cho đến khi server xác nhận       │
│   → Dùng cho server-side queries            │
└────────────────────────────────────────────┘
```

---

## 6. Case Studies Thực Tế

### 6.1 Ditto + Chick-fil-A: Mesh CRDT

- Chuỗi fast-food lớn → hàng trăm giao dịch/phút
- POS giao tiếp trực tiếp qua **BLE / Wi-Fi Direct** (không cần router)
- CRDT merge concurrent edits ở edge → cloud chỉ là backup
- **Zero downtime** kể cả khi ISP chết hoàn toàn

### 6.2 PowerSync & ElectricSQL: PostgreSQL WAL → SQLite

- Kết nối trực tiếp PG WAL → parse real-time → stream delta xuống SQLite
- Developer dùng **SQL bình thường** trên SQLite local
- Không cần viết custom sync endpoint
- Phù hợp cho team đã có PostgreSQL backend

### 6.3 Shopify POS: Local-First Component

- iPad/Android giữ **product catalog, pricing, tax rules** local hoàn toàn
- Checkout tính toán offline → tạo order object hợp lệ → queue
- Background process tự push khi có mạng
- Kỹ thuật componentization chặt chẽ trong Rails backend

---

## 7. Kết Luận Cho Nodi POS

Dựa trên nghiên cứu này, các hướng đi đã thảo luận trước đó được **xác nhận là đúng**:

| Quyết định đã đề xuất | Nghiên cứu xác nhận | Mức độ phù hợp |
|----------------------|---------------------|:---------------:|
| Dùng UUID thay auto-increment | UUIDv7 là best practice | ✅ **Xác nhận** |
| Incremental push/pull | Delta sync là chuẩn ngành | ✅ **Xác nhận** |
| Server-side merge | PowerSync/ElectricSQL model | ✅ **Xác nhận** |
| Tồn kho = SUM(transactions) | PN-Counter / Event Sourcing | ✅ **Xác nhận** |
| Nợ KH = SUM(transactions) | Intent-based recording | ✅ **Xác nhận** |
| Full dump là sai | "Entirely unviable" | ✅ **Xác nhận** |

### Điều chỉnh sau nghiên cứu:

1. **Nên dùng UUIDv7** thay vì UUIDv4 — hiệu năng B-tree tốt hơn 30-50%
2. **Cân nhắc Dual-ID Pattern** (client_id UUIDv7 + server_id INTEGER) — linh hoạt hơn
3. **CRDT là overkill** cho giai đoạn hiện tại (5-6 NV/cửa hàng), nhưng nên **thiết kế schema tương thích** để có thể upgrade sau
4. **Event Sourcing cho transactions** (hóa đơn, nợ, tồn kho) là hướng đi tự nhiên vì data này vốn đã append-only

---

*Tài liệu tham khảo — chỉ để hỗ trợ quyết định, không phải chén thánh.*
