# 📋 Kế Hoạch Triển Khai V2 Sync — App ↔ VPS

> **Ngày**: 18/03/2026
> **Mục tiêu**: Đưa V2 Sync từ trạng thái "fix xong bugs" → production-ready

---

## Phase 1: Verify (Hiện tại — 18/03/2026)

### 1.1 Mobile Test End-to-End
| Bước | Thao tác | Kết quả mong đợi |
|:----:|---------|-------------------|
| 1 | Cài APK mới | App khởi động OK |
| 2 | Cài đặt → 🔍 Sync Diagnostic | Hiện device_id, JWT, cursor |
| 3 | Bấm **🔄 Reset Cursor** | Cursor = 0 |
| 4 | Bấm **📥 Test Pull** | "V2 Pull: nhận X thay đổi" (X > 0) |
| 5 | Refresh Diagnostic | SP ≥ 2, NCC ≥ 1, ĐH ≥ 3 |
| 6 | Vào Kho → xem products | Hiện đúng tên, giá, đơn vị |
| 7 | Vào Đơn hàng → xem invoices | Hiện đúng invoice + items |
| 8 | Tạo 1 hóa đơn mới trên mobile | Tạo OK, không crash |
| 9 | Bấm **📤 Test Push** | "V2 Push: 1 thay đổi đã đẩy" |
| 10 | Trên PC → bấm Sync | Hóa đơn từ mobile xuất hiện |

### 1.2 VPS Cross-Check
> **Prompt cho VPS agent sau khi mobile test xong:**
>
> Kiểm tra store 1000004:
> 1. Mobile vừa push 1 invoice mới → có xuất hiện trong `synced_invoices` không?
> 2. `sync_journal` có entry mới cho invoice đó không?
> 3. PC pull cursor=X → nhận được invoice mới từ mobile không?
> 4. Confirm: không duplicate entries, không orphan UUIDs

---

## Phase 2: Fix Remaining Issues (nếu cần)

### 2.1 Suppliers code empty
- VPS gửi `company: ""` → App map thành `name: ""`
- App `code UNIQUE NOT NULL` → default ""
- **Vấn đề**: Nếu có >1 supplier với code="" → UNIQUE conflict
- **Fix**: VPS cần gửi `code` thực, hoặc auto-generate `SUP-{local_id}`

### 2.2 Product sell_price null
- VPS gửi `sell_price: null` cho 1/2 products
- App default: `sell_price = cost_price`
- **Verify**: Trên mobile, sản phẩm hiện giá bán = giá vốn (có thể sai)
- **Fix**: VPS hoặc user cập nhật giá bán đúng

### 2.3 Auto-sync 60s loop
- `syncStore.ts` auto chạy V2 push + pull mỗi 60s
- **Risk**: Nếu lỗi → crash loop
- **Verify**: Để mobile chạy 5 phút, xem console log không có error spam

---

## Phase 3: Multi-Device Sync Test

### 3.1 Scenario: Mobile A → VPS → PC
| Bước | Thiết bị | Thao tác |
|:----:|:--------:|---------|
| 1 | Mobile A | Tạo hóa đơn |
| 2 | Mobile A | Bấm Sync |
| 3 | VPS | Verify journal entry |
| 4 | PC | Bấm Sync → nhận hóa đơn |

### 3.2 Scenario: PC → VPS → Mobile A
| Bước | Thiết bị | Thao tác |
|:----:|:--------:|---------|
| 1 | PC | Thêm sản phẩm mới |
| 2 | PC | Bấm Sync |
| 3 | Mobile A | Bấm Sync → nhận sản phẩm mới |
| 4 | Mobile A | Xem Kho → sản phẩm xuất hiện |

---

## Phase 4: Production Readiness Checklist

| # | Hạng mục | Status |
|:-:|---------|:------:|
| 1 | V2 Push hoạt động (App + VPS) | ✅ |
| 2 | V2 Pull hoạt động (VPS + App) | 🟡 Chờ test |
| 3 | Column mappings đầy đủ | ✅ |
| 4 | NULL handling cho NOT NULL | ✅ |
| 5 | Journal dedup (VPS) | ✅ |
| 6 | FK fields populated | ✅ |
| 7 | End-to-end mobile test | ⬜ |
| 8 | Multi-device sync test | ⬜ |
| 9 | Auto-sync 60s stable | ⬜ |
| 10 | WebSocket V2 real-time | ⬜ |

---

## Timeline

| Ngày | Phase | Công việc |
|------|:-----:|----------|
| 18/03 | 1 | ✅ Fix bugs, build APK, cross-reference |
| 18/03 | 1 | 🟡 Mobile test end-to-end |
| 19/03 | 2 | Fix remaining (suppliers, sell_price) |
| 19/03 | 3 | Multi-device sync test |
| 20/03 | 4 | WebSocket V2 + auto-sync stability |
| 21/03 | — | **Production deploy** |

---

*Cập nhật: 18/03/2026*
