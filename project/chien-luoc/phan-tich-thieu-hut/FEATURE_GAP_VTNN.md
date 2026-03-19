# 📊 Phân Tích Thiếu Hụt Tính Năng — Nodi POS vs Yêu Cầu VTNN

> **Nguồn**: Báo cáo nghiên cứu VTNN 12/03/2026
> **Mục tiêu**: So sánh tính năng Nodi POS hiện có với yêu cầu thực tế của đại lý VTNN

---

## ✅ ĐÃ CÓ (Xác nhận đúng hướng)

| Yêu cầu | Nodi POS | Sprint |
|---|---|---|
| Dual-mode bán hàng | Toggle "Xuất HĐ VAT" | 86 |
| HĐĐT 3 NCC (VNPT/Viettel/MISA) | 8 IPC commands | 83+ |
| Thuế TNCN theo TT 40/2021 | PIT Calculator | 88-91 |
| Công nợ chi tiết từng GD | Sổ nợ, lịch sử mua/trả | 1+ |
| Quy đổi đơn vị tính đa tầng | Multi-unit system | 1+ |
| Batch tracking FEFO/FIFO | Cảnh báo hạn dùng | 1+ |
| AI chẩn đoán bệnh cây | 315+ bệnh, 8 cây | 82+ |
| VietQR miễn phí | Màn hình khách | 1+ |
| Offline 100% | Core architecture | Core |
| KB 5,700+ SP BVTV | Tra cứu hoạt chất | Core |

---

## 🟡 CÓ NHƯNG CẦN NÂNG CẤP

| Yêu cầu | Hiện có | Cần bổ sung | Ưu tiên |
|---|---|---|---|
| Công nợ theo VỤ MÙA | Nợ theo KH | Phân loại Đông Xuân/Hè Thu + đối soát kiểu sao kê | P1 |
| HĐĐT lifecycle | Tạo + retry | Hủy / Điều chỉnh / Thay thế / Gửi email | P1 |
| Hoạt chất cấm | DB 315 bệnh | Fix 23 hoạt chất cấm + auto-lock xuất kho | P0 |
| Báo cáo đối soát cuối vụ | Báo cáo cơ bản | "Sao kê ngân hàng" chi tiết ngày/SP/giá | P2 |
| Cảnh báo lệch tồn kho | Tồn kho realtime | So sánh HĐĐT vs kho thực, cảnh báo chênh lệch | P2 |

---

## ❌ CHƯA CÓ (Cần phát triển)

| Tính năng | Mô tả | Ưu tiên | Lý do |
|---|---|---|---|
| Truy xuất nguồn gốc lô hàng | Mã QR, NĐ 13/2020, lưu 12 tháng | P1 | Pháp lý bắt buộc |
| Khóa xuất kho thuốc cấm | Auto-lock khi TT mới ban hành | P1 | Bảo vệ đại lý |
| Quản lý giấy phép | Nhắc gia hạn BVTV, phân bón | P2 | Tiện ích |
| Combo sản phẩm | "Gói cho 1ha lúa giai đoạn X" | P2 | Tăng doanh thu |
| SMS/Zalo nhắc mùa vụ | Tự động theo chu kỳ cây trồng | P3 | Marketing |
| Tín dụng vi mô | Credit scoring từ lịch sử nợ | P4 | Tương lai xa |

---

*Cập nhật: 12/03/2026*
