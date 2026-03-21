# Nodi POS — Danh Sách Tính Năng Đầy Đủ
> Cập nhật: 12/03/2026 16:50 | Version: 2.0.0
> **Chiến lược**: Tất cả tính năng mở cho mọi user. Giới hạn duy nhất: Free = 20 đơn/ngày.
> **Sprint 113**: 14 sprints VTNN features + Mobile Parity hoàn tất
> **📊 Verified by code audit 12/03/2026** — Build: 0 errors | Tests: 1238/1238 (100%)

---

## 🛒 1. Bán Hàng (POS)
- Giao diện bán hàng chuyên nghiệp cho cửa hàng vật tư nông nghiệp
- Quét barcode USB hoặc **quét bằng điện thoại** (Remote Scanner qua QR)
- Tìm kiếm sản phẩm nhanh (tên, mã, barcode)
- Giỏ hàng: thêm/xóa/sửa, chọn đơn vị tính (chai, gói, bao...)
- Thanh toán: Tiền mặt, Chuyển khoản, Kết hợp
- QR thanh toán ngân hàng (VietQR) trên màn hình khách
- Ghi nợ khách hàng khi thanh toán
- In hóa đơn nhiệt (receipt printer)
- Trả hàng / hoàn tiền

## 📦 2. Quản Lý Kho Hàng
- Danh sách sản phẩm lọc theo danh mục (thuốc trừ sâu, phân bón, giống...)
- Thêm / sửa / xóa sản phẩm
- Nhập hàng từ file Excel (.xlsx)
- Theo dõi tồn kho, giá vốn, giá bán
- **Cảnh báo tồn kho thấp** / **hạn dùng** / **hàng chậm bán**
- Thao tác hàng loạt: giá, tồn kho, xóa
- **🔴 Badges thuốc cấm/hạn chế** — hiện ngay trên danh sách SP ✨ MỚI
- **📦 Truy xuất lô hàng** — tra mã lô, xem nguồn gốc, QR code ✨ MỚI

## 🚛 3. Nhập Hàng & Nhà Cung Cấp
- Quản lý nhà cung cấp, phiếu nhập hàng, lịch sử nhập
- **Công nợ nhà cung cấp** — theo dõi nợ, trả nợ, phiếu chi

## 👥 4. Quản Lý Khách Hàng
- Danh sách khách, lịch sử mua hàng, CCCD
- **Công nợ khách hàng** — ghi nợ, thu nợ, sổ nợ chi tiết
- **Công nợ vụ mùa** — sao kê theo mùa vụ (HT2025, XH2026...) kiểu ngân hàng ✨ MỚI
- **Tách nợ**: 3 cards riêng (Nợ mua hàng / Cho mượn / Tổng nợ) ✨ MỚI
- **Filter loại GD**: Tất cả / Mua nợ / Cho mượn / Trả nợ ✨ MỚI
- **Hồ sơ KH 4 tabs**: Thông tin | Lịch sử | Công nợ (sub-tabs) | Tích điểm ✨ UX

## 💰 5. Quỹ Tiền Mặt (Thu Chi)
- Số dư quỹ, thu chi thủ công, trả nợ NCC từ quỹ

## 📊 6. Báo Cáo & Thống Kê
- Dashboard: doanh thu, đơn hàng, top SP bán chạy
- Báo cáo theo ngày/tuần/tháng, so sánh khoảng thời gian

## 📋 7. Chốt Sổ Cuối Ngày
- Tóm tắt hoạt động ngày, đóng sổ, lịch sử chốt

## 🧾 8. Thuế & Kế Toán
- Thuế khoán theo quý, tổng hợp năm, VAT breakdown (0%/5%/10%)
- **Thuế TNCN hộ kinh doanh** — tính thuế theo TT 40/2021 (0.5% phân phối, 1% dịch vụ) ✨ MỚI
- Dual-mode: bán có hóa đơn / bán lẻ không báo thuế

## 📄 9. Hóa Đơn Điện Tử
- Kết nối NCC: **VNPT S-Invoice / Viettel S-Invoice / MISA** (3 nhà cung cấp)
- Cấu hình API URL, tài khoản, mật khẩu trong Settings
- **Xuất HĐĐT** tự động khi checkout (checkbox)
- Xem trước HĐ trước khi xuất (tên, MST, địa chỉ, items)
- Theo dõi trạng thái HĐĐT (Đã xuất / Đang chờ / Lỗi)
- **Thử lại HĐĐT** từ Lịch sử đơn hàng khi bị lỗi
- Nhập hóa đơn đầu vào (XML)
- ❌ **Chưa có**: Hủy / Điều chỉnh / Thay thế HĐ
- ❌ **Chưa có**: Gửi HĐ qua email/SMS cho khách

## 🤖 10. AI Chatbot Thông Minh
- Trợ lý AI tiếng Việt — hội thoại ngay trong app
- **Chẩn đoán bệnh cây trồng** → gợi ý thuốc từ kho hàng
- Tra cứu sản phẩm, hoạt chất, mùa vụ
- Truy vấn doanh thu, công nợ, tồn kho bằng ngôn ngữ tự nhiên
- Thêm vào giỏ hàng bằng chat, **46+ intents**
- **100% offline** cho tính năng core
- Self-learning: 👍/👎 feedback tự điều chỉnh AI
- Chain-of-thought Revenue Analysis

## ☁️ 11. Cloud & Đồng Bộ
- Cloud backup tự động, đồng bộ Smart-Sync
- Khôi phục khi đổi máy/cài lại
- Offline đầy đủ — đồng bộ khi có mạng

## 🔒 12. Bảo Mật
- **Multi-Device License**: 1 license = 1 cửa hàng = max 10 thiết bị ✨ MỚI
- License Key + HWID, auto-recovery khi cài lại
- Khóa màn hình khi rời quầy

## 👨‍💼 13. Quản Lý Nhân Viên
- Vai trò chủ/nhân viên, phân quyền 9 flags, PIN riêng

## 🔄 14. Cập Nhật Tự Động (OTA)
- Kiểm tra + tải + cài đặt bản mới ngay trong app

## 🆘 15. Hỗ Trợ Kỹ Thuật
- Chat trực tiếp qua WebSocket + Zalo

## 🖨️ 16. In Hóa Đơn & Barcode
- In nhiệt, chọn máy in, in thử
- **In barcode / nhãn sản phẩm** — SVG barcode Code 128, template 80mm ✨ MỚI

## 📱 17. Quét Mã Từ Xa
- Biến điện thoại thành máy quét barcode qua QR

## 🎁 18. Khuyến Mãi & Voucher ✨ MỚI
- Giảm giá %, giảm giá cố định, mua X tặng Y
- Voucher code: nhập mã giảm giá khi checkout
- Tự động áp dụng khuyến mãi đang hoạt động
- CRUD: tạo/sửa/xóa chương trình KM + voucher

## 🔒 19. Bảo Vệ Thuốc Cấm ✨ MỚI (Sprint 106)
- **3 tầng bảo vệ**: Inventory badges → POS guard → Backend fail-safe
- HARD ban: chặn hoàn toàn, không cho bán
- SOFT ban: cảnh báo, nhân viên xác nhận mới bán được
- Admin CRUD danh sách hoạt chất cấm (Settings)
- Tuân thủ TT 10/2020/TT-BNNPTNT, QĐ 4/2024/QĐ-BNNPTNT

## 📦 20. Truy Xuất Nguồn Gốc + QR ✨ MỚI (Sprint 107)
- Mã lô tự động (VD: L01-2026-T3)
- QR code chứa thông tin batch_code, NCC, ngày nhập, HSD
- Tra cứu lô: forward trace (đã bán cho ai)
- In phiếu giao hàng có QR + thông tin lô

## 📱 21. Mobile App (Full Parity) ✨ NÂNG CẤP
- **20+ components**, 8 routes (`/m/*`)
- Dashboard, POS, Cart, Checkout, Inventory, Orders, Customers, Customer Detail, Settings
- Touch-optimized: bottom nav, floating FAB, sheet cart
- Glassmorphism header, dark mode ready
- **Mobile Ban Check**: POS guard + badges + stock alerts ✨ MỚI
- **Mobile Customer Detail**: 4-tab full page (Info, LS, Công nợ, Tích điểm) ✨ MỚI
- **Mobile Batch Trace**: Tab truy xuất lô + QR code ✨ MỚI
- Stubs: camera scanner, bluetooth printer (Phase tương lai)

---

## 💎 Mô Hình Freemium (Chiến lược hiện tại)

> **Tất cả tính năng mở cho TẤT CẢ users.**
> Khác biệt duy nhất: Free = 20 đơn/ngày, Pro = không giới hạn.

| Tính năng | Free (không login) | Đăng nhập (Trial/Pro) |
|-----------|:------------------:|:---------------------:|
| Bán hàng POS | ✅ 20 đơn/ngày | ✅ Không giới hạn |
| Quản lý SP, KH, NCC | ✅ | ✅ |
| Báo cáo & Thống kê | ✅ | ✅ |
| AI Chatbot | ✅ | ✅ |
| Quản lý công nợ | ✅ | ✅ |
| Thu chi / Quỹ tiền mặt | ✅ | ✅ |
| Chốt sổ cuối ngày | ✅ | ✅ |
| Thuế & Kế toán | ✅ | ✅ |
| Quản lý nhân viên | ✅ | ✅ |
| Trả hàng | ✅ | ✅ |
| In hóa đơn | ✅ | ✅ |
| Hóa đơn điện tử | ✅ | ✅ |
| Cloud backup | ✅ | ✅ |
| Đồng bộ dữ liệu | ✅ | ✅ |
| Cập nhật tự động | ✅ | ✅ |
| Quét barcode điện thoại | ✅ | ✅ |
| Hỗ trợ kỹ thuật | ✅ | ✅ |

---

## 📝 Brief cho Agent Web — Cần sửa trên nodi.vn

### Gói "Dùng thử" (Miễn phí)
Tất cả tính năng phải ghi ✅, không có ✗. Chỉ ghi chú "20 đơn/ngày" cho POS.

Bổ sung thêm:
- ✅ AI Chatbot thông minh (đang ghi ✗ → sửa)
- ✅ Hóa đơn điện tử (đang ghi ✗ → sửa)
- ✅ Cloud backup (đang ghi ✗ → sửa)
- ✅ Quản lý công nợ KH & NCC
- ✅ Quỹ tiền mặt, Thu Chi
- ✅ Chốt sổ cuối ngày
- ✅ Thuế & Kế toán
- ✅ Trả hàng, In hóa đơn
- ✅ Quét barcode bằng điện thoại
- ✅ Quản lý nhân viên & phân quyền
- ✅ Cảnh báo tồn kho & hạn dùng
- ✅ Chẩn đoán bệnh cây trồng (AI)
- ✅ Nhập hàng từ Excel

### Gói "Theo tháng" (299k)
Bổ sung tương tự — liệt kê đầy đủ tính năng.
Highlight: "Không giới hạn đơn hàng" + "Hỗ trợ ưu tiên"

### Gói "Theo năm" (1.990k)
OK — "Tất cả tính năng + Tiết kiệm 44% + Ưu tiên hỗ trợ"

---

## ⚠️ Tính năng CHƯA CÓ (so với KiotViet) — Verified 12/03/2026

> Rà soát codebase bằng grep/search xác nhận.

| # | Tính năng | Mức ưu tiên | Ghi chú |
|:-:|-----------|:----------:|--------|
| 1 | Kết nối sàn TMĐT (Shopee, Lazada, TikTok Shop) | 🔴 Chưa cần | VTNN ít bán online |
| 2 | Kết nối vận chuyển (GHN, GHTK, J&T) | 🟡 Tương lai | VTNN ít ship GHN |
| 3 | HĐĐT: Hủy / Điều chỉnh / Thay thế | 🟡 Cần làm | Đã có tạo + theo dõi, thiếu lifecycle |
| 4 | Gửi HĐ qua email/SMS | 🟡 Cần làm | Cần SMTP/SMS gateway |
| 5 | API mở cho tích hợp bên thứ 3 | 🟡 Tương lai | Cần có userbase trước |
| 6 | Báo cáo nâng cao (ABC, doanh thu/NV) | 🟡 Cần làm | Hiện chỉ có revenue/profit/top SP |
| 7 | Crop seasons CRUD trong Settings | 🟡 Cần làm | Cả PC lẫn mobile đều chưa có UI quản lý |
| 8 | Bluetooth print trên mobile | 🟡 Tương lai | Mobile dùng window.print() chung |
| 9 | Video đào tạo + Tài liệu | 🔴 Cần gấp | Marketing |
| 10 | Social proof / Reviews | 🔴 Cần gấp | Cần onboard khách hàng thật |
