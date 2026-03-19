# 🖥️ Kiểm Kê Tính Năng — App PC (Desktop)

> **Cập nhật**: 13/03/2026 | **Source**: Router, Views, Components, Services, Backend Commands
> **Platform**: Tauri v2 + Vue 3 Composition API | **Layout**: `MainLayout.vue`

---

## 1. 🛒 BÁN HÀNG (POS)

### Màn hình chính — `POSView.vue`

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 1.1 | Lưới sản phẩm + Tìm kiếm | POSView | Search real-time, lọc theo category |
| 1.2 | ProductCard hiển thị | ProductCard.vue | Tên, giá, ảnh, badge hết hàng |
| 1.3 | Giỏ hàng (Cart) | Cart.vue | Thêm/xóa/sửa số lượng, tổng tiền |
| 1.4 | Chọn đơn vị tính | UnitSelectionModal.vue | Đa đơn vị (bao/kg/chai/lít), quy đổi tự động |
| 1.5 | Chọn lô hàng (Batch) | BatchSelectionModal.vue | FEFO auto-suggest, hiển thị HSD, tồn kho lô |
| 1.6 | Danh sách lô hàng | BatchListModal.vue | Xem tất cả lô của 1 sản phẩm |
| 1.7 | Quét mã vạch (Remote Scanner) | RemoteScannerModal.vue | Quét từ thiết bị khác qua mạng LAN |
| 1.8 | Auto-lock thuốc cấm | BanAlertModal.vue | Cảnh báo 3 trạng thái (xanh/vàng/đỏ), chặn bán |
| 1.9 | Áp dụng khuyến mãi tự động | PromotionEngine.ts | Giảm giá, mua X tặng Y, % chiết khấu |
| 1.10 | Simple Mode toggle | useSimpleMode composable | Ẩn bớt sidebar, chỉ hiện 6 mục thiết yếu |

### Thanh toán — `CheckoutModal.vue`

| # | Tính năng | Chi tiết |
|:-:|----------|---------|
| 1.11 | Chọn khách hàng | Tìm kiếm, tạo mới nhanh |
| 1.12 | Chọn phương thức thanh toán | Tiền mặt, Chuyển khoản, Kết hợp, Ghi nợ |
| 1.13 | QR thanh toán VietQR | QRCodeService.ts, hiển thị QR động |
| 1.14 | QR display màn hình phụ | QrDisplayPage.vue (standalone route) |
| 1.15 | Tính thuế VAT tự động | Per-item tax |
| 1.16 | In hóa đơn Bill 80mm | PrintService.ts |
| 1.17 | In hóa đơn A4 | PrintService.ts |
| 1.18 | Ghi nợ toàn phần/một phần | Tích hợp checkout flow |
| 1.19 | Tích điểm loyalty tự động | Post-commit pattern, LoyaltySettingsTab |
| 1.20 | Giảm giá thủ công | Theo % hoặc số tiền |

---

## 2. 📦 KHO HÀNG (Inventory)

### Màn hình chính — `InventoryView.vue`

| # | Tính năng | Chi tiết |
|:-:|----------|---------|
| 2.1 | Danh sách sản phẩm | Bảng đầy đủ: tên, tồn kho, giá, category |
| 2.2 | Tìm kiếm + Lọc category | SearchService.ts, filter dropdown |
| 2.3 | Thêm sản phẩm mới | AddProductModal.vue — form chi tiết |
| 2.4 | Sửa sản phẩm | Inline edit hoặc modal |
| 2.5 | Xóa sản phẩm | Confirm + xóa |
| 2.6 | Nhập hàng Excel/CSV | ImportProductsModal.vue — batch import |
| 2.7 | Kiểm kê (Stocktake) | StocktakeModal.vue — đối soát tồn kho |
| 2.8 | In tem/nhãn mã vạch | LabelPrintModal.vue + BarcodeGenerator.ts |
| 2.9 | Truy xuất lô (Batch Trace) | BatchTracePanel.vue — tra QR, lịch sử bán |
| 2.10 | Cảnh báo tồn kho thấp | Notification system |
| 2.11 | Cảnh báo hạn sử dụng | FEFO tracking |
| 2.12 | Master Data 5,700+ SP | AgriProductService.ts |

---

## 3. 📥 NHẬP HÀNG (Import)

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 3.1 | Tạo phiếu nhập hàng | ImportView.vue | Chọn NCC, thêm SP, số lượng, giá nhập |
| 3.2 | Nhập theo lô (Batch) | Import flow | Mã lô, HSD, giá nhập |
| 3.3 | Lịch sử nhập hàng | ImportHistory.vue | Danh sách phiếu nhập, lọc theo ngày/NCC |
| 3.4 | Chi tiết phiếu nhập | Modal detail | Xem lại từng sản phẩm đã nhập |

---

## 4. 👥 KHÁCH HÀNG

### Danh sách — `CustomerView.vue`

| # | Tính năng | Chi tiết |
|:-:|----------|---------|
| 4.1 | Danh sách khách hàng | Bảng + tìm kiếm |
| 4.2 | Tạo khách hàng mới | CreateCustomerModal.vue |
| 4.3 | Sửa/Xóa khách hàng | Inline actions |

### Chi tiết — `CustomerDetailModal.vue` (65KB — component lớn nhất)

| # | Tính năng | Tab | Chi tiết |
|:-:|----------|-----|---------|
| 4.4 | Thông tin cá nhân | Info | Tên, SĐT, địa chỉ, MST |
| 4.5 | Lịch sử mua hàng | Mua hàng | Danh sách đơn, tổng chi |
| 4.6 | Sổ nợ chi tiết | 💰 Công nợ | Nợ mua hàng + Cho mượn + Tổng |
| 4.7 | Ghi nợ mới | 💰 Công nợ | Modal ghi nợ |
| 4.8 | Trả nợ | PayDebtModal.vue | Trả 1 phần hoặc toàn bộ |
| 4.9 | Cho mượn thêm | 💰 Công nợ | Modal cho mượn tiền/hàng |
| 4.10 | Sao kê vụ mùa | 💰 Công nợ | Báo cáo theo mùa vụ |
| 4.11 | Lọc giao dịch | 💰 Công nợ | Filter: Tất cả/Mua nợ/Cho mượn/Trả nợ |
| 4.12 | Tích điểm & hạng thành viên | Loyalty tab | Bronze → Diamond |

---

## 5. 🏭 NHÀ CUNG CẤP

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 5.1 | Danh sách NCC | SuppliersView.vue | Bảng + tìm kiếm |
| 5.2 | Thêm NCC | AddSupplierModal.vue | Tên, SĐT, địa chỉ, MST |
| 5.3 | Chi tiết NCC | SupplierDetail.vue | Lịch sử nhập, công nợ NCC |
| 5.4 | Sửa/Xóa NCC | Inline actions | |

---

## 6. 📋 ĐƠN HÀNG

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 6.1 | Lịch sử đơn hàng | OrderHistoryView.vue | Lọc ngày, trạng thái, tìm kiếm |
| 6.2 | Chi tiết hóa đơn | InvoiceDetailModal.vue | Danh sách SP, giá, thuế |
| 6.3 | Trả hàng/Hoàn tiền | ReturnModal.vue | Trả 1 phần hoặc toàn bộ |
| 6.4 | In lại hóa đơn | PrintService.ts | Bill 80mm hoặc A4 |
| 6.5 | Khuyến mãi (Sales) | SalesModal.vue | Xem KM đã áp dụng |

---

## 7. 💰 THU CHI (Cashflow)

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 7.1 | Sổ quỹ tiền mặt | CashflowView.vue | Thu/Chi/Số dư |
| 7.2 | Tạo phiếu thu/chi | CashTransactionModal.vue | Lý do, số tiền, loại |
| 7.3 | Lọc theo ngày/loại | Filter controls | |
| 7.4 | Tổng kết dòng tiền | Summary cards | |

---

## 8. 📊 BÁO CÁO

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 8.1 | Doanh thu theo ngày/tuần/tháng | ReportView.vue | Biểu đồ + bảng |
| 8.2 | Top sản phẩm bán chạy | Report tab | Top 10/20 |
| 8.3 | Lợi nhuận gộp | Report tab | Doanh thu - Giá vốn |
| 8.4 | Báo cáo công nợ | Report tab | Tổng nợ, nợ quá hạn |
| 8.5 | Xuất Excel/PDF | ExportService.ts | Export báo cáo |
| 8.6 | DateRange picker | DateRangePicker.vue | Chọn khoảng ngày linh hoạt |

---

## 9. 🧾 THUẾ & KẾ TOÁN

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 9.1 | Tính thuế TNCN tự động | TaxView.vue + PIT_Calculator.ts | 0.5% HKD |
| 9.2 | Báo cáo thuế theo kỳ | TaxView tabs | Tháng/Quý/Năm |
| 9.3 | Hóa đơn đầu vào | InputInvoiceGuardView.vue | Guard phân quyền |
| 9.4 | Chi tiết HĐ đầu vào | InputInvoiceDetailModal.vue | |

---

## 10. 🧾 HÓA ĐƠN ĐIỆN TỬ (E-Invoice)

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 10.1 | Cấu hình HĐĐT | EInvoiceSettingsTab.vue | VNPT/Viettel/MISA provider |
| 10.2 | Phát hành HĐĐT | einvoice_commands.rs | 8 Tauri commands |
| 10.3 | XML export | ExportService.ts | Xuất XML chuẩn thuế |

---

## 11. 📅 CHỐT CA

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 11.1 | Chốt sổ cuối ngày | DailyClosingView.vue | Tổng doanh thu, tiền mặt, CK |
| 11.2 | Đối soát tiền đầu/cuối ca | Closing flow | So sánh thực tế vs hệ thống |
| 11.3 | In phiếu chốt ca | PrintService.ts | Bill tổng kết |

---

## 12. 🎁 KHUYẾN MÃI

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 12.1 | Tạo chương trình KM | PromotionsView.vue | Giảm giá %, mua X tặng Y |
| 12.2 | Thời hạn KM | Date range | Bắt đầu/Kết thúc |
| 12.3 | Áp dụng tự động | PromotionEngine.ts | Auto-apply khi checkout |
| 12.4 | Danh sách KM | List view | Active/Expired |

---

## 13. ⚙️ CÀI ĐẶT (Settings)

| # | Tính năng | Tab | Chi tiết |
|:-:|----------|-----|---------|
| 13.1 | Thông tin cửa hàng | GeneralSettingsTab | Tên, địa chỉ, logo, SĐT |
| 13.2 | Quản lý nhân viên | StaffManagementTab | CRUD nhân viên, phân quyền RBAC |
| 13.3 | Mời nhân viên (QR) | StaffInviteModal | QR code invite |
| 13.4 | Hóa đơn điện tử | EInvoiceSettingsTab | Cấu hình provider |
| 13.5 | Máy in & Mẫu in | PrinterSettingsTab | Chọn máy in, mẫu bill |
| 13.6 | Dữ liệu & Sao lưu | DatabaseSettingsTab | Backup/Restore, clear data |
| 13.7 | Tích điểm loyalty | LoyaltySettingsTab | Cấu hình tỷ lệ tích điểm, hạng |
| 13.8 | Hoạt chất cấm | BannedIngredientsTab | CRUD danh sách 31+2 chất |
| 13.9 | Giới thiệu + Cập nhật | About inline | Version, check update |
| 13.10 | AI Settings | AISettingsTab | Cấu hình AI engine |
| 13.11 | AI Training | AITrainingTab | Train/test chatbot |
| 13.12 | Giao diện | InterfaceSettingsTab | Dark mode, font size |

---

## 14. 🤖 TRỢ LÝ AI (Chatbot)

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 14.1 | Chat inline | ChatAssistant.vue | Panel bên phải |
| 14.2 | Chat popup window | ChatPopupView.vue | Cửa sổ riêng biệt |
| 14.3 | 48 intents NLP | PragmaticAIEngine.ts (222KB!) | Xử lý ngôn ngữ tự nhiên tiếng Việt |
| 14.4 | Chẩn đoán bệnh cây | DiseaseDiagnosisEngine.ts | 315+ bệnh |
| 14.5 | Tra cứu sản phẩm KB | ProductKnowledgeService.ts | 5,700+ SP |
| 14.6 | Cloud AI fallback | CloudAIService.ts | Khi offline NLP không đủ |
| 14.7 | Test Dashboard | ChatbotTestDashboard.vue | Admin test AI |
| 14.8 | Context engine | ContextEngine.ts | Ngữ cảnh hội thoại |
| 14.9 | Mining engine | MiningEngine.ts | Khai thác dữ liệu bán hàng |

---

## 15. 🛡️ BẢO MẬT & ĐĂNG NHẬP

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 15.1 | Đăng nhập OTP | LoginScreen.vue | Phone/OTP flow |
| 15.2 | Lock screen (PIN) | LockScreen.vue | PIN nhân viên |
| 15.3 | Chuyển nhân viên | StaffSwitchModal.vue | Switch staff PIN |
| 15.4 | Phân quyền RBAC | permissions.ts | Route + feature level |
| 15.5 | License/Trial check | ActivationView.vue | Activation flow |
| 15.6 | Store binding | Rust backend | 1 thiết bị = 1 cửa hàng |

---

## 16. 🔄 ĐỒNG BỘ & DỮ LIỆU

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 16.1 | Cloud sync (Push) | syncStore.ts | Đẩy dữ liệu lên server |
| 16.2 | Cloud sync (Pull) | syncStore.ts | Kéo dữ liệu về |
| 16.3 | Sync status indicator | SyncStatusIndicator.vue | Hiện trạng sync |
| 16.4 | Offline-first SQLite | Rust backend | Tất cả data local |
| 16.5 | Backup & Restore | DatabaseSettingsTab | Export/Import DB |

---

## 17. 🎓 ONBOARDING & HỖ TRỢ

| # | Tính năng | Component | Chi tiết |
|:-:|----------|-----------|---------|
| 17.1 | Onboarding Wizard 5 bước | OnboardingWizard.vue | Hướng dẫn lần đầu |
| 17.2 | Tour overlay | TourOverlay.vue | Highlight tính năng |
| 17.3 | Trung tâm hỗ trợ | SupportView.vue | FAQ, hướng dẫn, liên hệ |
| 17.4 | Toast notifications | ToastNotification.vue | Thông báo nhỏ |
| 17.5 | Update notification | UpdateNotificationModal.vue | Thông báo bản mới |
| 17.6 | Upgrade prompt | UpgradePromptModal.vue | Gợi ý nâng cấp |
| 17.7 | Confirmation dialogs | ConfirmationModal.vue | Xác nhận hành động |

---

## 18. 🖨️ IN ẤN

| # | Tính năng | Service | Chi tiết |
|:-:|----------|---------|---------|
| 18.1 | In bill 80mm nhiệt | PrintService.ts | Máy in USB/Network |
| 18.2 | In hóa đơn A4 | PrintService.ts | Laser/Inkjet |
| 18.3 | In tem mã vạch | LabelPrintModal.vue | Khổ tem tùy chỉnh |
| 18.4 | In phiếu nhập hàng | PrintService.ts | |
| 18.5 | In phiếu chốt ca | PrintService.ts | |

---

## TỔNG KẾ: PC có ~120 tính năng chi tiết across 18 module

| Module | Số tính năng |
|--------|:-----------:|
| Bán hàng + Checkout | 20 |
| Kho hàng | 12 |
| Nhập hàng | 4 |
| Khách hàng | 12 |
| NCC | 4 |
| Đơn hàng | 5 |
| Thu chi | 4 |
| Báo cáo | 6 |
| Thuế & Kế toán | 4 |
| HĐĐT | 3 |
| Chốt ca | 3 |
| Khuyến mãi | 4 |
| Cài đặt | 12 |
| AI Chatbot | 9 |
| Bảo mật | 6 |
| Đồng bộ | 5 |
| Onboarding & Hỗ trợ | 7 |
| In ấn | 5 |
| **TỔNG** | **~125** |
