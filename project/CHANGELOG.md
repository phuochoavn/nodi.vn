# 🎨 CHANGELOG — Nodi POS

## 📅 Sprint 103-113 — VTNN Killer Features + Mobile Parity (12/03/2026)

### 🌾 Tính năng VTNN mới

#### Sprint 105B: Công nợ vụ mùa
- **Crop Seasons**: Bảng `crop_seasons` + 6 mùa vụ seed
- **Auto-season**: Tự gán mùa vụ khi tạo invoice nợ
- **Sao kê vụ mùa**: Tab mới trong Hồ sơ KH — sao kê kiểu ngân hàng + xuất Excel

#### Sprint 106: Auto-Lock thuốc cấm (3 tầng bảo vệ)
- **Tầng 1**: Badges 🔴/⚠️ trong Kho hàng + filter SP cấm
- **Tầng 2**: POS guard — HARD block / SOFT confirm modal khi thêm SP cấm vào giỏ
- **Tầng 3**: Backend fail-safe trước INSERT invoice
- **Admin**: Settings tab CRUD danh sách hoạt chất cấm

#### Sprint 107: Truy xuất nguồn gốc + QR
- 4 columns mới trên `product_batches` (batch_code, manufacturer_lot, production_date, qr_data)
- QR Code Service: tạo QR từ data JSON lô hàng
- 3 IPC commands: `trace_batch_forward`, `trace_invoice_batches`, `find_batch_by_code`
- `BatchTracePanel.vue` — tra cứu lô + hiện QR + danh sách "đã bán cho"

### 🎨 UX Improvements

#### Sprint 109: Đơn giản hóa tabs KH (6→4)
- Gộp 3 tabs nợ (Sổ nợ + Ghi nợ + Sao kê) → 1 tab "💰 Công nợ" với sub-tabs
- Nút "Cho mượn thêm" = button mở modal (bỏ tab riêng)

#### Sprint 111: Tách nợ + filter loại GD
- 3 cards: 🛒 Nợ mua hàng | 📝 Cho mượn | 💰 Tổng nợ
- Filter theo loại giao dịch (Mua nợ / Cho mượn / Trả nợ)

### 🐛 Bug Fixes

#### Sprint 104A: Fix hoạt chất cấm trong AI DB
- Thay 8 hoạt chất cấm: Chlorpyrifos→Emamectin, Carbofuran→Paecilomyces, Paraquat→Glyphosate

#### Sprint 105A: Fix active_ingredient bug
- Migration 040: ALTER TABLE products ADD active_ingredient
- Migration 041: CREATE TABLE banned_ingredients
- 2 IPC commands: `get_banned_ingredients`, `check_ingredient_banned`

#### Sprint 108: Fix Mutex deadlock
- Root cause: `create_invoice()` → `earn_loyalty_points()` → deadlock
- Fix: `drop(conn)` sau `tx.commit()` trong `invoices.rs`

#### Sprint 110: Fix category filter
- Tất cả 47 SP hiện "TẤT CẢ" → filter danh mục không hoạt động
- Fix: đồng bộ `mapCategory()` output với `CATEGORY_FILTERS` keys
- Migration 044: backfill product categories

### 📱 Mobile Parity (Sprint 113A-B-C)

#### Sprint 113A: Mobile Ban Check
- POS guard + HARD/SOFT modal trên `MobilePOSGrid.vue`
- Badges 🔴/⚠️ + banned stock alerts trên `MobileInventory.vue`
- Danh sách hoạt chất cấm trên `MobileSettings.vue`

#### Sprint 113B: Mobile Customer Detail
- **NEW** `MobileCustomerDetail.vue` — full page 4-tab (Info, Lịch sử, Công nợ, Tích điểm)
- Route `/m/customer/:id` + tap từ list → detail page

#### Sprint 113C: Mobile Batch Trace
- Tab "🔍 Truy xuất lô" trong `MobileInventory.vue`
- Search mã lô → chi tiết + QR code 100×100px + danh sách "đã bán cho"

### 📊 Số liệu
- Build: 0 errors ✅
- Tests: 1238/1238 (100%) ✅
- 14 sprints ship trong 1 ngày (~4 tiếng agent time)
- Mobile parity: 4 → 15 features ĐÃ CÓ

---

## 📅 Sprint 36 — Nodi AI Level 8.0: Complete Agriculture Intelligence (25/02/2026)

### 🧠 AI Engine
- **Hotfix**: HELP guard, product name regex tightened, intent-registry sync
- **Agri Guard**: Ngăn truy vấn nông nghiệp bị nhầm thành DEBT_COLLECTION
- **16 tonic patterns** mới cho CROP_ACTION (chống rụng, kích rễ, làm chín...)

### 📊 Disease Expert DB: 100 → 146 entries
- +12 bệnh lúa (lúa von, hoa cúc, lùn sọc đen, sâu gai, sâu năn...)
- +9 sầu riêng (đốm mắt cua, rầy phấn, bọ xít muỗi, sâu đục thân...)
- +15 phân bón — category mới `phan_bon` (URê, NPK, DAP, hữu cơ vi sinh...)
- +10 thuốc dưỡng — category mới `duong_cay` (kích rễ, paclobutrazol, GA3...)
- +8 symptom chain rules (R046–R053)

### 🧪 Testing: 1123 → 1183 tests (100% pass)
- +60 data upgrade tests (bệnh mới, phân bón, thuốc dưỡng, boundary)
- +15 Sprint 36 hotfix tests

---

## 📅 Phiên bản 2.1.0 - Nâng cấp Hỗ trợ & Interactive Walkthrough (21/02/2026)

### ✨ Tính năng mới

#### 1. **Trang Hỗ trợ & Trợ giúp nâng cấp**
- **4 tab**: Hướng dẫn | FAQ | Phím tắt | Liên hệ
- **6 chủ đề hướng dẫn** với step-by-step (Bán hàng, Thêm SP, KH, Nhập hàng, Báo cáo, Sao lưu)
- **14 câu hỏi FAQ** chia 4 nhóm, dạng accordion click mở/đóng
- **Bảng phím tắt** với 10 keyboard shortcuts
- **Giờ làm việc** & nút "Chat KTV"
- Mẹo & cảnh báo xen kẽ trong hướng dẫn

#### 2. **Interactive Walkthrough (Tour)**
- Composable `useTour.ts` — Tour engine cho 3 tours (POS, Kho, Khách hàng)
- Component `TourOverlay.vue` — SVG overlay + tooltip + progress dots
- Nút **"▶ Thực hành"** trên mỗi hướng dẫn → chạy tour trực tiếp trên UI thật
- Auto-navigate đến trang đích khi bắt đầu tour
- Highlight phần tử đang hướng dẫn với viền xanh + overlay tối

#### 3. **Modal "Thêm khách hàng" sửa layout**
- `max-h-[90vh]` — không tràn màn hình
- Form cuộn được, nút Hủy/Lưu luôn hiển thị (sticky footer)

---

## 📅 Phiên bản 2.0 - Giao diện POS Chuyên nghiệp

### ✨ Tính năng mới

#### 1. **Layout 2 cột hiện đại**
- **Trái (70%)**: Danh sách sản phẩm dạng grid responsive
  - 2 cột (màn hình nhỏ) → 3 cột (HD) → 4 cột (4K)
  - Card sản phẩm đẹp mắt với icon, giá, tồn kho
  - Cảnh báo "sắp hết hàng" khi tồn < 50 đơn vị
  
- **Phải (30%)**: Giỏ hàng cố định
  - Hiển thị tổng số lượng sản phẩm
  - Điều chỉnh số lượng trực tiếp (+/- buttons)
  - Tự động tính tổng tiền
  - Nút thanh toán lớn, dễ nhấn

#### 2. **Tìm kiếm sản phẩm nâng cao**
- Ô tìm kiếm nổi bật ở đầu trang
- Tìm theo tên hoặc mã vạch
- Real-time filtering (không cần nhấn Enter)
- Hiển thị số lượng kết quả
- Keyboard shortcut: **Ctrl + F**

#### 3. **Modal chọn đơn vị (Unit Selection)**
- Popup đẹp mắt khi click vào sản phẩm
- Chọn đơn vị bán (Thùng/Chai/Lốc...)
- Hiển thị giá và hệ số quy đổi rõ ràng
- Điều chỉnh số lượng với +/- buttons
- Tính tạm tính tự động
- Animation mượt mà

#### 4. **Modal thanh toán (Checkout)**
- Form nhập thông tin khách hàng
- Chọn phương thức thanh toán (3 options):
  - 💵 Tiền mặt
  - 🏦 Chuyển khoản
  - 📋 Công nợ
- Nhập giảm giá
- Ghi chú đơn hàng
- Hiển thị tổng kết rõ ràng
- Validation đầy đủ

#### 5. **Thông báo (Toast Notifications)**
- Thông báo thành công (màu xanh)
- Thông báo lỗi (màu đỏ)
- Tự động ẩn sau 3 giây
- Animation slide-up từ dưới lên

#### 6. **UX Improvements**
- Loading state khi khởi tạo
- Empty state khi không có sản phẩm
- Hover effects trên tất cả buttons
- Transitions mượt mà
- Responsive design
- Keyboard shortcuts:
  - **F9**: Thanh toán
  - **Ctrl + F**: Focus tìm kiếm

### 🎨 Design System

#### Colors
- **Primary (Green)**: `#10b981` - Màu chủ đạo (nông nghiệp)
- **Secondary (Amber)**: `#f59e0b` - Màu nhấn mạnh
- **Success**: Xanh lá
- **Error**: Đỏ
- **Gray scale**: 50, 100, 200, 300, 400, 500, 600, 700, 800

#### Typography
- **Font Family**: System fonts (optimal rendering)
- **Sizes**: xs, sm, base, lg, xl, 2xl, 3xl
- **Weights**: 400 (normal), 500 (medium), 600 (semibold), 700 (bold)

#### Spacing
- **Padding/Margin**: 2, 3, 4, 6, 8, 12, 16, 24px
- **Gaps**: 2, 3, 4px cho grid/flex

#### Border Radius
- **Small**: 0.5rem (8px)
- **Medium**: 0.75rem (12px)
- **Large**: 1rem (16px)
- **XL**: 1.5rem (24px)

### 🔧 Cấu trúc Components

```
src/components/
├── ProductCard.vue       # Card hiển thị sản phẩm
├── Cart.vue              # Component giỏ hàng
├── UnitSelectionModal.vue # Modal chọn đơn vị
└── CheckoutModal.vue     # Modal thanh toán
```

### 📱 Responsive Breakpoints

- **2xl (2560px+)**: 4 cột sản phẩm
- **xl (1920px+)**: 3 cột sản phẩm
- **lg (1280px+)**: 2 cột sản phẩm
- **md (1024px-)**: Layout stack vertically (tương lai)

### 🎯 Workflow mới

1. **Chọn sản phẩm**:
   - Click vào card sản phẩm
   - Modal hiện lên

2. **Chọn đơn vị & số lượng**:
   - Chọn Thùng/Chai/Lốc...
   - Điều chỉnh số lượng
   - Click "Thêm vào giỏ"

3. **Giỏ hàng tự động cập nhật**:
   - Hiển thị sản phẩm vừa thêm
   - Tính tổng tiền real-time
   - Có thể +/- số lượng hoặc xóa

4. **Thanh toán**:
   - Click nút "Thanh toán" hoặc nhấn F9
   - Nhập thông tin khách hàng (optional)
   - Chọn phương thức thanh toán
   - Nhập giảm giá (nếu có)
   - Xác nhận

5. **Hoàn tất**:
   - Hóa đơn được tạo trong Rust
   - Tồn kho tự động trừ
   - Giỏ hàng được xóa
   - Thông báo thành công
   - Sản phẩm reload với tồn kho mới

### ⚡ Performance Optimizations

1. **Computed Properties**:
   - `filteredProducts`: Chỉ re-compute khi search query thay đổi
   - `cartTotal`: Chỉ re-compute khi cart thay đổi

2. **Virtual Scrolling** (future):
   - Để xử lý hàng nghìn sản phẩm

3. **Debounce Search** (future):
   - Giảm số lần filter khi typing

### 🐛 Bug Fixes (so với phiên bản cũ)

1. ✅ Không còn hiển thị JSON thô
2. ✅ Không còn dropdown xấu xí
3. ✅ Không còn alert() native
4. ✅ Giỏ hàng không còn bị overflow
5. ✅ Layout responsive hơn

### 📚 Dependencies mới

Không có! Chỉ dùng:
- Vue 3 built-in features
- TailwindCSS (đã có sẵn)
- Composition API (đã có sẵn)

### 🔜 Tính năng tương lai

- [ ] In hóa đơn (print)
- [ ] Xuất PDF
- [ ] Báo cáo doanh thu
- [ ] Quản lý khách hàng
- [ ] Lịch sử giao dịch
- [ ] Scan mã vạch
- [ ] Multi-tab (nhiều hóa đơn cùng lúc)
- [ ] Dark mode
- [ ] Multiple languages

### 📖 Hướng dẫn sử dụng

#### Thêm sản phẩm vào giỏ:
1. Click vào card sản phẩm
2. Chọn đơn vị trong modal
3. Nhập số lượng
4. Click "Thêm vào giỏ"

#### Chỉnh sửa giỏ hàng:
- Click `+` hoặc `-` để tăng/giảm
- Hoặc nhập trực tiếp vào ô số lượng
- Click `X` để xóa item
- Click "Xóa tất cả" để clear giỏ

#### Thanh toán:
1. Click "Thanh toán" hoặc nhấn **F9**
2. Điền form (khách hàng optional)
3. Chọn phương thức thanh toán
4. Nhập giảm giá nếu có
5. Click "Xác nhận thanh toán"

#### Tìm kiếm:
- Nhấn **Ctrl + F** để focus vào ô tìm kiếm
- Gõ tên sản phẩm hoặc mã vạch
- Kết quả tự động lọc

---

## 🎊 Kết luận

Giao diện mới đã được nâng cấp hoàn toàn, từ UI/UX đến workflow. Hệ thống giờ đây:
- ✅ Chuyên nghiệp hơn
- ✅ Dễ sử dụng hơn
- ✅ Nhanh hơn
- ✅ Đẹp hơn
- ✅ Responsive hơn

**Sẵn sàng triển khai cho production!** 🚀
