# 🧪 Sync Test Protocol — Kiểm Tra End-to-End

> **MỤC ĐÍCH**: Mỗi lần fix sync, PHẢI chạy qua checklist này TRƯỚC KHI báo "xong".

---

## Pre-Test: Chuẩn bị

- [ ] App: Build 0 errors (`cmd /c npm run build`)
- [ ] VPS: Docker running, API healthy (`curl https://api.nodi.vn/health`)
- [ ] Console App: mở để theo dõi logs

---

## Test A: Push Invoice (App → VPS → Web)

### Bước 1: Tạo hóa đơn trên App
- Mở POS → thêm 1 SP → thanh toán tiền mặt
- ✅ Check: App hiện hóa đơn trong "Lịch sử đơn hàng"

### Bước 2: Verify Push trên console
- Đợi 60s (auto-sync) hoặc bấm "Đồng bộ ngay"
- ✅ Check console:
  ```
  [V2Push] sync_journal entries (synced=0): N   (N > 0)
  [V2Push] Total changes built: N
  📤 [SyncV2] Pushing N changes from device XXX
  ✅ [V2Push] VPS response: processed=N, conflicts=0, total_sent=N
  📝 [V2Push] Journal marked synced (N processed)
  ```
- ❌ FAIL nếu: `processed=0` hoặc `processed < total_sent`

### Bước 3: Verify trên VPS
- Query: `SELECT count(*) FROM synced_invoices WHERE store_id = <STORE_ID>;`
- ✅ Check: count tăng đúng số đơn vừa tạo

### Bước 4: Verify trên Web
- Mở `nodi.vn/dashboard/don-hang`
- ✅ Check: đơn hàng mới hiện, đúng giá, đúng khách hàng

---

## Test B: Pull Data (VPS → App)

### Bước 1: Fresh DB trên App
- Xóa DB: `del "%APPDATA%\vn.nodi.pos\agripos.db"`
- Restart app, login cùng tài khoản

### Bước 2: Verify Pull trên console
- ✅ Check:
  ```
  📥 [PullV2] Pulling since cursor 0 for device XXX
  📥 [PullV2] Applied N changes, cursor now C, has_more=false
  ```
- ❌ FAIL nếu: `Applied 0 changes`

### Bước 3: Verify data trên App
- Products: Kho hàng hiện đúng số SP
- Invoices: Lịch sử đơn hàng hiện đúng số đơn
- Customers: Khách hàng hiện đúng số KH

---

## Test C: Multi-Device Sync (PC ↔ Mobile)

### Bước 1: PC tạo hóa đơn
- ✅ Check: Mobile nhận qua pull (sau 60s hoặc bấm Đồng bộ)

### Bước 2: Mobile tạo hóa đơn
- ✅ Check: Web dashboard hiện đơn mới
- ✅ Check: PC nhận qua pull

---

## Test D: Error Recovery

### D1: Push partial failure
- ✅ Check console: `processed < total_sent` → journal NOT mark all synced
- ✅ Check: next cycle retry failed records

### D2: Network offline
- Tắt wifi → tạo hóa đơn → bật wifi
- ✅ Check: auto-push khi online lại

---

## Kết quả Test

| Test | Ngày | Kết quả | Ghi chú |
|------|------|:-------:|---------|
| A | 16/03/2026 | ✅ | Sau 12 lần fix |
| B | 16/03/2026 | ✅ | Pull 92 changes |
| C | Chưa test | ⏳ | Cần test mobile |
| D | Chưa test | ⏳ | |

---

*Cập nhật lần cuối: 18/03/2026*
