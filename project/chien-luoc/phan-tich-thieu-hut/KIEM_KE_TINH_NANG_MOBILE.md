# 📱 Kiểm Kê Tính Năng — App Mobile

> **Cập nhật**: 13/03/2026 | **Source**: Router /m/*, Mobile Components, Shared Services
> **Platform**: Tauri v2 Android + Vue 3 | **Layout**: `MobileLayout.vue`
> **Navigation**: `BottomNavigation.vue` (5 tab) + `MobileMore.vue` (menu phụ)

---

## Bottom Navigation — 5 Tab Chính

```
[ 🏠 Trang chủ ] [ 🛒 Bán hàng ] [ 📦 Kho hàng ] [ 👥 Khách hàng ] [ ⚙️ Cài đặt ]
```

---

## 1. 🏠 TRANG CHỦ (Dashboard)

### `MobileDashboard.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 1.1 | Tổng quan doanh thu hôm nay | Cards: Doanh thu, Đơn hàng, Khách mới | ❌ PC không có Dashboard riêng |
| 1.2 | Biểu đồ doanh thu 7 ngày | Chart mini | ❌ |
| 1.3 | Sản phẩm bán chạy | Top list | ✅ (trong Báo cáo) |
| 1.4 | Quick actions | Nút tắt đến POS, Kho, Khách | ❌ |
| 1.5 | Trạng thái sync | Badge indicator | ✅ |

---

## 2. 🛒 BÁN HÀNG (POS)

### `MobilePOSGrid.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 2.1 | Lưới sản phẩm (Grid) | Cards hiển thị sản phẩm | ✅ |
| 2.2 | Tìm kiếm sản phẩm | MobileSearchBar.vue — real-time | ✅ |
| 2.3 | ProductCard mobile | MobileProductCard.vue — compact | ✅ (khác layout) |
| 2.4 | Quét mã vạch Camera | CameraScanner.vue — camera phone | ✅ (RemoteScanner) |
| 2.5 | Giỏ hàng (Bottom Sheet) | MobileCartSheet.vue — kéo lên | ✅ (Cart.vue sidebar) |
| 2.6 | Thêm/sửa số lượng | Trong cart sheet | ✅ |
| 2.7 | Auto-lock thuốc cấm | BanCheckService.ts (shared) | ✅ |
| 2.8 | Áp dụng KM tự động | PromotionEngine.ts (shared) | ✅ |

### `MobileCheckout.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 2.9 | Chọn khách hàng | Search + tạo nhanh | ✅ |
| 2.10 | Phương thức thanh toán | Tiền mặt, CK, Ghi nợ | ✅ |
| 2.11 | QR thanh toán VietQR | QRCodeService.ts (shared) | ✅ |
| 2.12 | Ghi nợ | Toàn phần/một phần | ✅ |
| 2.13 | In bill Bluetooth | BluetoothPrinterSettings.vue | ✅ (USB/Network) |
| 2.14 | Tích điểm loyalty | Post-commit (shared) | ✅ |

---

## 3. 📦 KHO HÀNG (Inventory)

### `MobileInventory.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 3.1 | Danh sách sản phẩm | Card list dọc | ✅ (bảng ngang) |
| 3.2 | Tìm kiếm | Search bar | ✅ |
| 3.3 | Lọc category | Filter chips | ✅ |
| 3.4 | Xem tồn kho | Badge số lượng | ✅ |
| 3.5 | Chi tiết sản phẩm | Tap to expand | ✅ (modal) |
| 3.6 | Truy xuất lô (Batch Trace) | Tab "Truy xuất lô" | ✅ (BatchTracePanel) |
| 3.7 | QR code lô hàng | Hiển thị QR | ✅ |
| 3.8 | Khách đã mua lô | Danh sách khách | ✅ |

---

## 4. 👥 KHÁCH HÀNG

### `MobileCustomers.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 4.1 | Danh sách khách hàng | Card list | ✅ |
| 4.2 | Tìm kiếm khách | Search bar | ✅ |
| 4.3 | Tạo khách mới | Form inline | ✅ (modal) |
| 4.4 | Gọi điện nhanh | Tap SĐT → call | ❌ Mobile-only |

### `MobileCustomerDetail.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 4.5 | Thông tin cá nhân | Header card | ✅ |
| 4.6 | Lịch sử mua hàng | Tab mua hàng | ✅ |
| 4.7 | Sổ nợ (Công nợ) | Tab 💰 Công nợ | ✅ |
| 4.8 | Nợ mua hàng / Cho mượn / Tổng | 3 cards tách biệt | ✅ |
| 4.9 | Lọc giao dịch | Filter: Tất cả/Mua nợ/Mượn/Trả | ✅ |
| 4.10 | Trả nợ | Nút trả nợ | ✅ |
| 4.11 | Sao kê vụ mùa | Sub-section | ✅ |

---

## 5. ⚙️ CÀI ĐẶT

### `MobileSettings.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 5.1 | User card (avatar + tên) | Header block | ❌ |
| 5.2 | Chế độ tối (Dark mode) | Toggle switch | ✅ (InterfaceSettingsTab) |
| 5.3 | Chế độ đơn giản | Toggle Simple Mode | ✅ (Header) |
| 5.4 | Đồng bộ lên Cloud | Push sync | ✅ (DatabaseSettingsTab) |
| 5.5 | Kéo dữ liệu từ Cloud | Pull sync | ✅ |
| 5.6 | Máy in Bluetooth | Placeholder "Sắp có" | ✅ (PrinterSettingsTab) |
| 5.7 | License | Placeholder | ✅ (ActivationView) |
| 5.8 | Danh sách hoạt chất cấm | Expandable list readonly | ✅ (BannedIngredientsTab — đầy đủ CRUD) |
| 5.9 | Đăng xuất | Logout button | ✅ |
| 5.10 | Version info | Footer | ✅ (About tab) |

---

## 6. 📋 TÍNH NĂNG PHỤ (MobileMore.vue)

### Menu "Tính năng" — 9 mục

| # | Tính năng | Route | Có trên PC? |
|:-:|----------|-------|:-----------:|
| 6.1 | Báo cáo | /m/reports | ✅ |
| 6.2 | Sổ quỹ | /m/cashflow | ✅ |
| 6.3 | Nhà cung cấp | /m/suppliers | ✅ |
| 6.4 | Đơn hàng | /m/orders | ✅ |
| 6.5 | Chốt ca | /m/daily-closing | ✅ |
| 6.6 | Khuyến mãi | /m/promotions | ✅ |
| 6.7 | Thông báo | /m/notifications | ✅ (alert system) |
| 6.8 | Bảo mật (→ Settings) | /m/settings | ✅ |
| 6.9 | Hỗ trợ (→ Settings) | /m/settings#support | ✅ (SupportView) |

---

## 7. 📊 BÁO CÁO

### `MobileReports.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 7.1 | Doanh thu theo ngày/tuần/tháng | Cards + list | ✅ |
| 7.2 | Top sản phẩm bán chạy | Top list | ✅ |
| 7.3 | Lợi nhuận gộp | Summary card | ✅ |
| 7.4 | Chọn khoảng ngày | Date range picker | ✅ |

---

## 8. 💰 SỔ QUỸ

### `MobileCashflow.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 8.1 | Danh sách thu/chi | Card list | ✅ |
| 8.2 | Tạo phiếu thu/chi | Form inline | ✅ |
| 8.3 | Lọc theo ngày/loại | Filter controls | ✅ |
| 8.4 | Tổng kết số dư | Summary header | ✅ |

---

## 9. 🏭 NHÀ CUNG CẤP

### `MobileSuppliers.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 9.1 | Danh sách NCC | Card list | ✅ |
| 9.2 | Tìm kiếm NCC | Search bar | ✅ |
| 9.3 | Thêm NCC mới | Form inline | ✅ |
| 9.4 | Chi tiết NCC | Expand card | ✅ (SupplierDetail page) |

---

## 10. 📋 ĐƠN HÀNG

### `MobileOrderHistory.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 10.1 | Lịch sử đơn hàng | Card list | ✅ |
| 10.2 | Lọc theo ngày | Date filter | ✅ |
| 10.3 | Chi tiết đơn | Expand/modal | ✅ |
| 10.4 | Tìm kiếm đơn | Search bar | ✅ |

---

## 11. 📅 CHỐT CA

### `MobileDailyClosing.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 11.1 | Chốt sổ cuối ngày | Summary + confirm | ✅ |
| 11.2 | Đối soát tiền | Thực tế vs hệ thống | ✅ |
| 11.3 | Xem lịch sử chốt ca | Past closings | ✅ |

---

## 12. 🎁 KHUYẾN MÃI

### `MobilePromotions.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 12.1 | Danh sách KM | Card list | ✅ |
| 12.2 | Tạo KM mới | Form | ✅ |
| 12.3 | Active/Expired filter | Tabs | ✅ |

---

## 13. 🔔 THÔNG BÁO

### `MobileNotifications.vue`

| # | Tính năng | Chi tiết | Có trên PC? |
|:-:|----------|---------|:-----------:|
| 13.1 | Cảnh báo tồn kho thấp | Auto-check | ✅ (alert system) |
| 13.2 | Cảnh báo hạn sử dụng | FEFO warning | ✅ |
| 13.3 | Thông báo thuốc cấm | Ban alert | ✅ |
| 13.4 | Mark as read | Toggle | ✅ |

---

## 14. 📷 PHẦN CỨNG MOBILE

| # | Tính năng | Component | Có trên PC? |
|:-:|----------|-----------|:-----------:|
| 14.1 | Camera quét mã vạch | CameraScanner.vue | ❌ (PC dùng RemoteScanner) |
| 14.2 | Bluetooth printer | BluetoothPrinterSettings.vue | ❌ (PC dùng USB/Network) |
| 14.3 | Auto-redirect mobile | Router guard | ❌ |

---

## TỔNG KẾ: Mobile có ~85 tính năng chi tiết across 14 module

| Module | Số tính năng |
|--------|:-----------:|
| Dashboard | 5 |
| Bán hàng + Checkout | 14 |
| Kho hàng | 8 |
| Khách hàng | 11 |
| Cài đặt | 10 |
| Tính năng phụ (More) | 9 |
| Báo cáo | 4 |
| Sổ quỹ | 4 |
| NCC | 4 |
| Đơn hàng | 4 |
| Chốt ca | 3 |
| Khuyến mãi | 3 |
| Thông báo | 4 |
| Phần cứng | 3 |
| **TỔNG** | **~86** |

---

## ⚠️ TÍNH NĂNG PC CÓ MÀ MOBILE CHƯA CÓ (Nhanh)

| # | Tính năng PC | Mobile Status |
|:-:|-------------|:------------:|
| 1 | Nhập hàng (ImportView) | ❌ Chưa có |
| 2 | Lịch sử nhập hàng (ImportHistory) | ❌ Chưa có |
| 3 | Thuế & Kế toán (TaxView) | ❌ Chưa có |
| 4 | Hóa đơn đầu vào (InputInvoice) | ❌ Chưa có |
| 5 | HĐĐT cấu hình (EInvoiceSettings) | ❌ Chưa có |
| 6 | AI Chatbot (ChatAssistant) | ❌ Chưa có |
| 7 | Onboarding Wizard | ❌ Chưa có |
| 8 | Tour Overlay | ❌ Chưa có |
| 9 | Support View | ❌ Chưa có |
| 10 | Test Dashboard | ❌ Dev-only |
| 11 | Quản lý nhân viên (Staff CRUD) | ❌ Chưa có |
| 12 | Mời nhân viên QR | ❌ Chưa có |
| 13 | In tem mã vạch | ❌ Chưa có |
| 14 | Kiểm kê (Stocktake) | ❌ Chưa có |
| 15 | Trả hàng/Hoàn tiền | ❌ Chưa có |
| 16 | Xuất Excel/PDF | ❌ Chưa có |
| 17 | DateRange picker nâng cao | ❌ Chưa có |
| 18 | Nhập hàng Excel/CSV | ❌ Chưa có |
| 19 | Loyalty settings CRUD | ❌ Chưa có |
| 20 | Banned ingredients CRUD | ⚠️ Chỉ readonly |
| 21 | AI Settings/Training | ❌ Chưa có |
| 22 | DB backup/restore | ❌ Chưa có |
| 23 | Chọn đơn vị tính | ❌ Chưa có |
| 24 | Chọn lô hàng (Batch selection) | ❌ Chưa có |
| 25 | QR display màn phụ | ❌ PC-only |

---

## ✅ TÍNH NĂNG MOBILE CÓ MÀ PC KHÔNG CÓ

| # | Tính năng | Chi tiết |
|:-:|----------|---------|
| 1 | Dashboard tổng quan | PC không có trang chủ riêng |
| 2 | Camera quét mã vạch | PC dùng scanner USB/remote |
| 3 | Gọi điện nhanh khách hàng | Tap SĐT → call |
| 4 | Bluetooth printer | PC dùng USB/Network |
| 5 | Quick actions | Nút tắt trên Dashboard |
