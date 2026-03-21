# 📚 Nghiên cứu #08: Kiến trúc POS Offline-First cho Môi trường Bán lẻ Nông nghiệp Tải Cao

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-18
> **Prompt**: Cải tiến theo chuẩn 4 trụ cột (Solutions Architect perspective)
> **Liên quan**: SQLite WAL, Tauri IPC, Sync V2, Print system, UX bán hàng

## Tóm tắt

Blueprint kiến trúc toàn diện cho Nodi POS mùa cao điểm (200-300 hóa đơn/ngày). Phân tích 4 bottleneck (SQLite locks, Tauri IPC, Print, Network), chiến lược offline-first (WAL + Sync Queue + Conflict Resolution), case study Toast/Square/Clover, và thiết kế UX "tốc độ ánh sáng".

---

## 1. Bốn Bottleneck trong POS Tải Cao

| Bottleneck | Nguyên nhân | Giải pháp | Ưu tiên |
|-----------|-----------|----------|:-------:|
| **SQLite Locks** | Rollback Journal khóa toàn cục + fsync() sau mỗi COMMIT | WAL mode + `synchronous=NORMAL` + Transaction Batching | 🔴 Kritical |
| **Tauri IPC** | Serialize/Deserialize JSON 10,000+ sản phẩm → đóng băng JS Thread | Binary Data (Uint8Array) + Lazy-loading + Pagination | 🔴 Cao |
| **Print Queue** | `window.print()` chặn UI + driver Windows chậm | Rust Background Worker + ESC/POS raw printing | 🔴 Cao |
| **Network Sync** | API đồng bộ chờ timeout 30-60s khi mất mạng | Local-First 100% + Optimistic UI + Background Sync Queue | 🔴 Cao |
| **DOM Rendering** | Cập nhật DOM quá dày khi gõ nhanh/cập nhật giỏ | Debouncing + Virtual DOM tối ưu | 🟡 Trung bình |

---

## 2. SQLite Tối ưu cho High-Write

### PRAGMA bắt buộc (khởi tạo kết nối)

```sql
PRAGMA journal_mode = WAL;      -- Đọc-ghi song song
PRAGMA synchronous = NORMAL;    -- Tăng tốc 10-20x
PRAGMA busy_timeout = 5000;     -- Chờ 5s thay vì lỗi SQLITE_BUSY
```

### WAL Mode — Tại sao quan trọng

| Chế độ | Đọc khi đang ghi | Tốc độ ghi | Rủi ro |
|--------|:-:|:-:|--------|
| Rollback Journal | ❌ Bị chặn | Chậm (fsync mỗi COMMIT) | Không |
| **WAL** | ✅ Không chặn | **10-20x nhanh hơn** | Mất vài tx cuối nếu OS crash |

### Transaction Batching

```
❌ Sai: INSERT từng sản phẩm → 10 COMMIT = 10 fsync
✅ Đúng: BEGIN → INSERT 10 sản phẩm → 1 COMMIT = 1 fsync
```

---

## 3. Kiến trúc Sync Queue Offline-First

### Nguyên tắc cốt lõi

> Mọi tương tác → **Local SQLite trước** (< 100ms). Cloud = background task.

### UUIDv4 thay Auto-increment

```
❌ Auto-increment: Máy A = HĐ #1005, Máy B = HĐ #1005 → XUNG ĐỘT
✅ UUIDv4: Máy A = "a3f7...", Máy B = "b8c2..." → UNIQUE vĩnh viễn
```

### Sync Queue Flow

```
Thu ngân thao tác
  → Ghi vào SQLite local (Optimistic UI → báo thành công ngay)
    → Đẩy event vào Sync_Queue table (UUID, table, action, payload, timestamp)
      → Background Rust Worker ping server định kỳ
        → Mạng OK? → Bóc Sync_Queue FIFO → Push batch lên API
          → Fail? → Exponential Backoff (2s → 4s → 8s → 16s...)
            → Success? → Đánh dấu đã sync / xóa khỏi queue
```

### Chống mất dữ liệu 72 giờ mất mạng

- Sync_Queue lưu trong SQLite local → không mất khi tắt máy
- Exponential Backoff tránh nghẽn khi mạng yếu
- TTL cảnh báo: nhắc quản lý kết nối lại trong 24-72h

---

## 4. Conflict Resolution

| Chiến lược | Dùng cho | Cơ chế |
|-----------|---------|--------|
| **Last-Write-Wins (LWW)** | Thông tin tĩnh (tên KH, SĐT, địa chỉ) | So timestamp → bản mới nhất thắng |
| **Intent-Based (Additive)** | Tồn kho, công nợ | Gửi delta `{action: "decrement", amount: 10}` → atomic operation |
| **CRDTs** | Dữ liệu phân tán phức tạp | Hợp nhất mọi thứ tự → trạng thái đồng nhất |

### Ví dụ Intent-Based (tồn kho)

```
Kho = 50 bao Ure
Máy A bán 10 (09:00) → gửi {decrement: 10}
Máy B bán 5 (09:05)  → gửi {decrement: 5}

❌ LWW: Kho = 45 (mất 5 bao!)
✅ Intent: Kho = 50 - 10 - 5 = 35 ✓
```

---

## 5. Case Study: Toast, Square, Clover

| Khía cạnh | Square | Toast | Clover |
|----------|--------|-------|--------|
| **Offline** | "Store-and-Forward" vô hình | Banner sau 40s, khóa tính năng nhạy cảm | Offline lên tới 7 ngày |
| **ID Shifting** | Client-side UUID | `(DeviceID + 2) * 1000` = không gian số độc lập | Pending flag + sync sau |
| **Local Sync** | Point-to-Cloud | **Local Hub**: LAN giữa các máy khi mất internet | ServiceSync đa kênh |
| **TTL** | 24h phải sync, 72h max | Khóa: chốt ca, lương, thẻ quà | Chủ tự cài hạn mức offline |
| **Hiệu năng** | **-48% checkout time, -43% crash** | F-pattern UI, minimal taps | Heatmap giờ cao điểm |

### Bài học cho Nodi POS

1. **Chuyển offline thầm lặng** (Square) — không làm gián đoạn thu ngân
2. **Khóa tính năng nhạy cảm** (Toast) — báo cáo tài chính, kiểm kho offline = tắt
3. **Local Hub qua LAN** (Toast) — máy quầy ↔ tablet kho khi mất internet
4. **ID không gian số** (Toast) — kết hợp UUID + DeviceID
5. **TTL 24-72h** (Square) — cảnh báo phải sync lại

---

## 6. UX "Tốc Độ Ánh Sáng"

### Keyboard-First

| Phím | Chức năng |
|:----:|----------|
| F1 | Tìm sản phẩm |
| F2 | Thanh toán nhanh |
| F4 | Tìm khách hàng |
| Enter | Xác nhận thanh toán |
| Esc | Hủy giao dịch |
| ↑↓ | Chọn trong autocomplete |
| Tab | Chấp nhận ghost text |

### Autocomplete 5 Quy tắc Vàng

1. ✅ Hiển thị gợi ý **ngay khi focus** (bán chạy nhất / theo mùa)
2. ✅ **In đậm** từ khóa khớp
3. ✅ **Ghost text** (đoán trước, Tab để chấp nhận)
4. ✅ Giới hạn **5-7 kết quả** (không thanh cuộn)
5. ✅ **Phím mũi tên** chọn + Enter xác nhận

### Error Recovery

- ❌ KHÔNG dùng Modal Error che màn hình
- ✅ Inline feedback (text đỏ cạnh trường lỗi)
- ✅ Nút "Hoàn tác" ngay lập tức

---

## 7. Training Mode & Simplified Mode

### Training Mode (Sandbox)

```
Feature Flag ON → Ngắt nodi.db → Gắn training.db (hoặc In-Memory)
                → Tắt Sync Queue Worker
                → Nhân viên thực hành tự do, 0 rủi ro
```

### Simplified Mode (RBAC)

```
Thu ngân đăng nhập → Ẩn: báo cáo, kiểm kê, cài đặt, KH thân thiết
                   → Hiện: Tìm SP → Nhập SL → Thanh toán (luồng tuyến tính)
                   → Kết quả: Sẵn sàng trong 5 phút
```

---

## 8. Tuân thủ Hóa đơn Điện tử Offline (NĐ 123/2020)

| Yêu cầu pháp lý | Giải pháp Nodi POS |
|-----------------|-------------------|
| HĐ từ MTT không cần chữ ký số | ✅ Sinh HĐ offline → không cần API Thuế |
| Lưu trữ cục bộ khi mất mạng | ✅ Sync Queue + timestamp chống chỉnh sửa |
| Gửi Thuế trong 24h sau có mạng | ✅ Rust Background Worker → XML chuẩn → T-VAN |
| Không cần thao tác thủ công | ✅ Tự động hoàn toàn |

---

## 9. Blueprint Tổng hợp — Nodi POS Mùa Cao Điểm

### Tầng Database (Rust Core)
- [x] WAL mode + synchronous=NORMAL ← **đã có trong code**
- [x] UUIDv4 cho mọi entity ← **đã có**
- [ ] Transaction Batching cho hóa đơn
- [ ] busy_timeout = 5000

### Tầng IPC (Tauri Bridge)
- [ ] Binary Data thay JSON cho dataset lớn
- [ ] Lazy-loading + Pagination từ Rust
- [ ] Debouncing cho autocomplete

### Tầng Sync (Background Worker)
- [x] Sync Queue architecture ← **đang triển khai (V2)**
- [x] Exponential Backoff ← **đã có**
- [ ] Intent-Based mutations cho tồn kho
- [ ] TTL cảnh báo 24-72h

### Tầng Print
- [ ] Rust ESC/POS raw printing (bỏ window.print)
- [ ] Hàng đợi in bất đồng bộ

### Tầng UI
- [x] Keyboard shortcuts ← **đã có cơ bản**
- [ ] Autocomplete 5 quy tắc vàng
- [ ] Training Mode (Feature Flag + training.db)
- [ ] Simplified Mode (RBAC)
- [ ] Inline error recovery

---

## Nguồn tham khảo

- SQLite WAL documentation
- Tauri IPC architecture
- Square POS Engineering Blog
- Toast POS Technical Architecture
- Clover Developer Documentation
- Nghị định 123/2020/NĐ-CP, Thông tư 78/2021/TT-BTC
- Nielsen Norman Group: Autocomplete design patterns
