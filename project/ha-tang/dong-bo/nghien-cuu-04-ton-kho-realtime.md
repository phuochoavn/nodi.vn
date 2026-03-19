# 📚 Nghiên Cứu #4: Tồn Kho Real-Time Multi-Device — Concurrency, Offline, và Case Studies

> **Nguồn**: Google DeepSearch | **Ngày**: 16/03/2026
> **Chủ đề**: Race conditions, locking, event sourcing, mesh network, so sánh Shopify/Square/Clover
> **Lưu ý**: Tài liệu tham khảo, quyết định cuối cùng dựa trên bối cảnh Nodi POS.

---

## 1. Vấn Đề Cốt Lõi: Race Condition Tồn Kho

### Kịch bản overselling

```
Tồn kho: Thuốc sâu XYZ = 5 chai

NV1 (PC): Đọc stock = 5 → bán 5 chai → UPDATE stock = 0  ✅
NV2 (Mobile): Đọc stock = 5 → bán 5 chai → UPDATE stock = 0  ✅

Kết quả thực tế: stock = -5! (bán 10 chai khi chỉ có 5)
```

**Nguyên nhân**: Lost update anomaly — 2 thread đọc cùng state cũ → cả 2 ghi đè → data SAI.

---

## 2. Chiến Lược Locking

### 2.1 Pessimistic Locking — ❌ KHÔNG PHÙ HỢP POS

- Khóa row **ngay khi đọc** → thread khác bị block chờ
- Đảm bảo 100% nhưng **bottleneck nghiêm trọng** khi nhiều giao dịch
- Rủi ro **deadlock** khi transaction phức tạp (nhiều sản phẩm)
- **Anti-pattern cho POS phân tán**

### 2.2 Optimistic Locking — ✅ CHUẨN CHO POS

- Không khóa khi đọc — cho phép đọc đồng thời
- Thêm `version` column → UPDATE chỉ thành công nếu version khớp

```sql
-- NV1: UPDATE products SET stock = 0, version = 2
--       WHERE id = 123 AND version = 1;  → ✅ 1 row affected

-- NV2: UPDATE products SET stock = 0, version = 2
--       WHERE id = 123 AND version = 1;  → ❌ 0 rows affected!
--       → App hiển thị: "Sản phẩm đã hết hàng"
```

**Ưu điểm**: Throughput cao, không deadlock, xử lý collision bằng retry
**Phù hợp**: Multi-device POS, hàng ngàn SKU, xung đột hiếm khi xảy ra

### So sánh

| | Pessimistic | Optimistic |
|-|:-----------:|:----------:|
| Giả định | Xung đột xảy ra thường xuyên | Xung đột hiếm khi |
| Cơ chế | Khóa row, block thread khác | Version check khi write |
| Hiệu năng | Chậm — bottleneck | Nhanh — retry khi fail |
| Deadlock | ⚠️ Cao | ✅ Không |
| Phù hợp POS | ❌ | ✅ |

---

## 3. Cart Reservation vs Payment-Time Verification

### 3.1 Cart Reservation — ⚠️ Phức Tạp

- Scan barcode → trừ kho ngay → "reserved" state
- **Vấn đề**: KH bỏ đi → phải timeout release → set bao lâu?
  - Quá ngắn: KH đang mua bị mất hàng
  - Quá dài: Hàng bị khóa vô lý, mất doanh số
- E-commerce: Bot có thể add-to-cart để phá đối thủ

### 3.2 Payment-Time Verification — ✅ CHUẨN NGÀNH

- Scan barcode → chỉ thêm vào cart, **KHÔNG trừ kho**
- Trừ kho **CHỈ khi thanh toán thật** (optimistic lock tại thời điểm payment)
- NV đầu tiên thanh toán → thắng. NV sau → thông báo hết hàng

> ⚡ **Kết luận cho Nodi**: Nodi hiện tại trừ kho khi tạo hóa đơn (payment-time) — **ĐÚNG approach**. Không cần reservation.

---

## 4. State Management: CRUD vs Event Sourcing

### 4.1 CRUD (Absolute State) — ❌ Hạn chế

```sql
UPDATE products SET stock_quantity = 95 WHERE id = 123;
-- Ai trừ? Tại sao? Bán hay hư? → KHÔNG BIẾT nếu không có audit log
```

- Nhanh, đơn giản
- Nhưng: **Mất lịch sử**, không biết TẠI SAO stock thay đổi
- Race condition khi nhiều device UPDATE đồng thời

### 4.2 Event Sourcing (Transaction Ledger) — ✅ TỐI ƯU

```
StockReceived:  +100  (08:00)
ItemSold:       -3    (09:15)
ItemReturned:   +1    (11:30)
ItemDamaged:    -1    (14:00)
→ Stock hiện tại = 100 - 3 + 1 - 1 = 97
```

**Ưu điểm**:
- **Audit trail hoàn hảo** — biết chính xác mỗi thay đổi
- **Append-only** → không race condition, không cần lock
- **Tái tạo state** tại bất kỳ thời điểm nào (temporal query)
- **Scale tốt** — write cực nhanh (chỉ append)

**Nhược điểm**:
- Tính state = replay events → chậm nếu nhiều events → cần **snapshot** định kỳ
- Sửa lỗi = thêm compensating event (không UPDATE/DELETE)
- Cần **CQRS** cho read model nhanh

### 4.3 Hybrid (Square API Model) — 🏆 THỰC TẾ NHẤT

Square dùng **adjustment ledger** với các state transitions:
- `IN_STOCK` → `SOLD` (bán)
- `IN_STOCK` → `WASTE` (hư hỏng)
- `SOLD` → `RETURNED_BY_CUSTOMER` (trả hàng)

**Đặc biệt**: Có **PHYSICAL_COUNT** event = hard reset khi kiểm kê thực tế. Timestamp quyết định thứ tự — offline sales chèn đúng vị trí timeline.

> ⚡ **Kết luận cho Nodi**: Nodi **đã có** `product_transactions` (ledger) + `stock_quantity` (computed). Đây chính là hybrid model. Cần đảm bảo `stock_quantity` = server-computed từ SUM(transactions), không phải client-pushed.

---

## 5. Offline Resilience — Topology Đồng Bộ

### 5.1 Server-Authoritative (Shopify, Square) — Cloud dependency

- Mất internet → mỗi terminal hoạt động **độc lập, không thấy nhau**
- Cache offline → push khi online → server reconcile
- **Rủi ro**: Overselling giữa các terminal vì không sync local

### 5.2 Peer-to-Peer Mesh (Clover ServiceSync) — ✅ RESILIENT

- Mất internet → devices vẫn **sync với nhau qua LAN/BLE**
- NV1 bán hết → mesh propagate → NV2 biết ngay
- Online lại → node chính push consolidated ledger lên cloud

### 5.3 CRDT PN-Counter Cho Tồn Kho — Giải Quyết Offline Conflict

```
PN-Counter structure:
  Increments: { DeviceA: +100, DeviceB: +50 }    ← nhập kho, trả hàng
  Decrements: { DeviceA: -30,  DeviceB: -20 }    ← bán hàng, hư hỏng

Stock = SUM(increments) - SUM(decrements)
     = (100 + 50) - (30 + 20) = 100

→ Mỗi device chỉ modify counter CỦA NÓ → KHÔNG BAO GIỜ collision
→ Merge = cộng tất cả → LUÔN đúng, bất kể thứ tự
```

**Ưu điểm**: Không phụ thuộc đồng hồ, không cần server, hội tụ xác định
**Chính xác = mô hình Nodi đề xuất**: `stock = SUM(product_transactions)`

---

## 6. Case Studies: Shopify vs Square vs Clover

### 6.1 Shopify POS

| Điểm | Chi tiết |
|------|---------|
| **Kiến trúc** | Server-authoritative, cloud-first |
| **Flash sale** | Redis atomic Lua scripts → microsecond deduction |
| **Sharding** | "Pods" (MySQL shard + Redis) per merchant group |
| **Cart reservation** | ❌ Cấm — chống bot abuse |
| **Verification** | Payment-time → đầu tiên thanh toán thắng |
| **Offline** | Cache local, queue payments. **Không sync giữa terminals** |
| **Phù hợp** | E-commerce scale lớn, omnichannel |

### 6.2 Square POS

| Điểm | Chi tiết |
|------|---------|
| **Kiến trúc** | Cloud + timestamp-based reconciliation |
| **Inventory model** | Adjustment ledger (state transitions: IN_STOCK → SOLD → RETURNED) |
| **PHYSICAL_COUNT** | Hard reset khi kiểm kê — override tất cả offline edits cũ hơn |
| **Offline** | Queue + timestamp → chèn retroactive vào đúng timeline |
| **Peer sync** | ❌ Không có — offline terminals vẫn bị cô lập |
| **Phù hợp** | SMB, cần audit trail, flexible API |

### 6.3 Clover POS — 🏆 RESILIENT NHẤT

| Điểm | Chi tiết |
|------|---------|
| **Kiến trúc** | Hybrid: Cloud API + Local Secure Network API |
| **ServiceSync** | Peer-to-peer mesh qua Wi-Fi + proprietary protocols |
| **Offline** | **Full sync giữa terminals** — NV1 bán = NV2 biết ngay |
| **Hardware** | Station Duo (cố định) + Flex (cầm tay) |
| **Conflict** | Mesh resolve local → node chính push lên cloud |
| **Phù hợp** | Nhà hàng, retail lớn, nơi internet không ổn định |

### Bảng so sánh

| Feature | Shopify | Square | Clover |
|---------|:-------:|:------:|:------:|
| Cross-device offline sync | ❌ | ❌ | ✅ |
| Inventory model | Absolute | Adjustment ledger | Hybrid |
| Cart reservation | ❌ | ❌ | Optional |
| Payment-time verification | ✅ | ✅ | ✅ |
| Flash sale handling | 🏆 Redis | Timestamp | N/A |
| Audit trail | Basic | 🏆 Excellent | Good |

---

## 7. Đề Xuất Kiến Trúc Theo Quy Mô

Nghiên cứu đề xuất 3 mô hình tùy theo nhu cầu:

| Nhu cầu chính | Mô hình | Công nghệ |
|--------------|---------|-----------|
| **Internet không ổn định** (rural, pop-up) | Local mesh + CRDT | Clover ServiceSync, Ditto |
| **Flash sale / e-commerce scale** | Server-authoritative + Redis | Shopify Pods |
| **Audit + compliance** | Event-sourced ledger | Square Adjustment API |

---

## 8. Kết Luận Cho Nodi POS

### Nodi thuộc nhóm nào?

> Nodi = **POS cho đại lý nông nghiệp Việt Nam** → internet không ổn định (nông thôn), 5-6 NV, KHÔNG flash sale.
> → Mô hình **#1 (Local mesh + CRDT logic)** nhưng ở quy mô nhỏ.
> → Cụ thể: **Event-sourced ledger** (product_transactions) + **server-computed stock** + **incremental sync**.

### Xác nhận từ nghiên cứu:

| Quyết định | Nghiên cứu nói gì | Cho Nodi |
|-----------|-------------------|----------|
| Payment-time verification | Chuẩn ngành (Shopify, Square, Clover) | ✅ Nodi đã làm đúng |
| stock = SUM(transactions) | PN-Counter / Event Sourcing model | ✅ Xác nhận approach |
| Optimistic locking | Chuẩn cho multi-device POS | 📌 Cần implement version column |
| PHYSICAL_COUNT event | Hard reset khi kiểm kê | 📌 Nodi có stocktake nhưng chưa có "reset event" |
| Adjustment ledger | State transitions (IN_STOCK → SOLD) | 📌 Enhance product_transactions type field |

### Phát hiện MỚI:

1. **Optimistic locking cần `version` column** — chưa có trong schema Nodi
2. **PHYSICAL_COUNT** (kiểm kê) nên là event đặc biệt override timeline — Nodi cần implement
3. **Square retroactive timestamp insertion** — offline sales chèn vào đúng timeline, không chỉ append
4. **Cart reservation = anti-pattern** — Nodi đúng khi không làm reservation
5. **Clover ServiceSync** = template cho tương lai nếu Nodi muốn LAN mesh (chưa cần giai đoạn này)

---

*Tài liệu tham khảo — chỉ để hỗ trợ quyết định, không phải chén thánh.*
