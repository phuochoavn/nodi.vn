# 🔄 Đồng Bộ Dữ Liệu — Sync Architecture

> Nghiên cứu & thiết kế kiến trúc đồng bộ dữ liệu cho multi-device (PC + Mobile, 5-6 NV/cửa hàng)

## Tài Liệu

| # | File | Nội dung | Trạng thái |
|---|------|----------|:----------:|
| 1 | [nghien-cuu-01-kien-truc-sync-pos.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/nghien-cuu-01-kien-truc-sync-pos.md) | Kiến trúc sync POS offline-first (Toast, Shopify, Square, Lightspeed, CRDT, UUIDv7) | ✅ |
| 2 | [nghien-cuu-02-server-merge-engine.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/nghien-cuu-02-server-merge-engine.md) | Server-side merge engine (LWW vs Vector Clocks, CQRS, field-level merge, sync journal, tombstone GC) | ✅ |
| 3 | [nghien-cuu-03-sqlite-cloud-sync.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/nghien-cuu-03-sqlite-cloud-sync.md) | SQLite ↔ Cloud sync (migration, change tracking, Turso/PowerSync/ElectricSQL/cr-sqlite, bandwidth) | ✅ |
| 4 | [nghien-cuu-04-ton-kho-realtime.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/nghien-cuu-04-ton-kho-realtime.md) | Tồn kho real-time (race condition, locking, event sourcing, Shopify/Square/Clover) | ✅ |
| 5 | [nghien-cuu-05-pos-viet-nam.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/nghien-cuu-05-pos-viet-nam.md) | POS Việt Nam (KiotViet, Sapo, MISA CukCuk, iPOS — kiến trúc, offline, bài học) | ✅ |
| 6 | `kien-truc-moi.md` | Thiết kế kiến trúc sync mới cho Nodi | ⬜ Sau khi duyệt kế hoạch |
| 7 | [ke-hoach-trien-khai.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/ke-hoach-trien-khai.md) | 6 Phase rollout + phân công Agent App/VPS | ✅ |

---

## Tổng Hợp Phát Hiện Từ 5 Nghiên Cứu

### Xác nhận hướng đi đúng ✅

| Quyết định đã đề xuất | Nguồn xác nhận |
|----------------------|----------------|
| Full dump sync = SAI | NC #1, #2, #3 — "entirely unviable" |
| Tồn kho = SUM(transactions) | NC #1 (PN-Counter), #4 (Event Sourcing) |
| Công nợ = SUM(transactions) | NC #1 (Intent-based), #2 (CQRS) |
| Server-side computed fields | NC #2 — "client tính = anti-pattern" |
| Payment-time verification | NC #4 — chuẩn ngành (Shopify/Square/Clover) |
| SQLite trong Tauri = an toàn | NC #5 — an toàn hơn IndexedDB (Sapo/KiotViet) |
| Hybrid model (local SQLite + cloud PG) | NC #5 — mô hình tối ưu tại VN |

### Phát hiện MỚI cần áp dụng 📌

| Phát hiện | Nguồn | Ưu tiên |
|-----------|-------|:-------:|
| **UUIDv7** thay vì UUIDv4 (30-50% nhanh hơn) | NC #1, #3 | 🔴 |
| **Dual-ID Pattern** (client UUID + server INTEGER) | NC #1, #3 | 🔴 |
| **Sync journal** (operation log, sequence ID) | NC #2 | 🔴 |
| **Field-level merge** (không row-level LWW) | NC #2, #3 | 🔴 |
| **Tombstone + GC** (soft delete, gc_grace 30 ngày) | NC #2, #3 | 🟠 |
| **Staging table** (sync_inbox buffer) | NC #2 | 🟠 |
| **Device sync status** tracking | NC #2 | 🟠 |
| **Optimistic locking** (version column) | NC #4 | 🟠 |
| **PHYSICAL_COUNT** event (kiểm kê override) | NC #4 | 🟡 |
| **Idempotency** (UUID dedup trên server) | NC #5 | 🔴 |
| **Tồn kho âm = OK** (flag alert, không reject) | NC #5 | 🟡 |
| **Reconciliation report** (đối soát chéo) | NC #5 | 🟡 |
| **VACUUM scheduling** | NC #3 | 🟡 |
| **Binary serialization** (MessagePack, tương lai) | NC #3 | ⚪ |

### Công nghệ KHÔNG cần cho Nodi hiện tại

| Công nghệ | Lý do không cần |
|-----------|----------------|
| CRDT (Ditto, cr-sqlite) | Overkill cho 5-6 NV, insert chậm 2.5x |
| Peer-to-peer mesh (BLE) | Chỉ cần khi >20 devices, Nodi dùng cloud sync |
| Cart reservation | Anti-pattern, Nodi đã đúng (payment-time) |
| Kafka/RabbitMQ | Quá lớn, PG LISTEN/NOTIFY đủ dùng |
| SQLite Session Extension | Upgrade tương lai, `updated_at` triggers đủ giai đoạn đầu |

## Vấn Đề Cốt Lõi Hiện Tại

- 🔴 ID collision giữa các devices (auto-increment)
- 🔴 Full-dump sync (push/pull toàn bộ DB mỗi 60s)
- 🔴 Last-write-wins (INSERT OR REPLACE, không có conflict detection)
- 🔴 Không có idempotency trên VPS API
- 🟠 Computed fields sync trực tiếp thay vì tính từ transactions
- 🟡 WebSocket chỉ thông báo, không có selective pull
