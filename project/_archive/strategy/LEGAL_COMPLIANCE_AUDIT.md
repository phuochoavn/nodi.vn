# Báo Cáo Tư Vấn Pháp Lý: Nodi POS & Quy Định Pháp Luật Việt Nam

> **Ngày**: 2026-01-31
> **Đối tượng**: Phần mềm quản lý bán hàng Vật tư Nông nghiệp (SaaS/On-premise)

---

## 1. Kết Luận Tổng Quan
✅ **Phần mềm KHÔNG vi phạm pháp luật.**
Mô hình hoạt động của Nodi POS (Offline-first, Sync Cloud) là hoàn toàn hợp pháp và phù hợp với chủ trương chuyển đổi số của nhà nước.

Tuy nhiên, có **3 điểm nóng pháp lý** cần lưu ý để đảm bảo tuân thủ tuyệt đối, đặc biệt khi quy mô người dùng tăng lên.

---

## 2. Chi Tiết Các Quy Định Liên Quan

### 🔒 A. Bảo Vệ Dữ Liệu Cá Nhân (Nghị Định 13/2023/NĐ-CP)
Đây là quy định quan trọng nhất hiện nay.
*   **Quy định**: Việc thu thập xử lý dữ liệu cá nhân (Tên, SĐT, Địa chỉ, Số CCCD của khách mua hàng) phải có sự **đồng ý** của chủ thể dữ liệu.
*   **Hiện trạng App**: App có sync thông tin khách hàng lên VPS.
*   **Đánh giá**:
    *   Về mặt kỹ thuật, Nodi POS đóng vai trò là "Bên xử lý dữ liệu" cho chủ cửa hàng. Chủ cửa hàng là người thu thập data khách của họ.
    *   **Giải pháp đã làm**: File `EULA.rtf` trong bộ cài đã có mục "Sự Đồng Thuận". Việc người dùng bấm "I Agree" khi cài đặt được tính là sự đồng ý hợp lệ.
*   **Khuyến nghị**:
    *   Không bán/chia sẻ data khách hàng cho bên thứ 3 (đã cam kết trong EULA).
    *   Đảm bảo data được mã hóa khi truyền tải (HTTPS) - *Kỹ thuật đã làm*.

### ☁️ B. Luật An Ninh Mạng & Lưu Trữ Dữ Liệu (Nghị Định 53/2022/NĐ-CP)
*   **Quy định**: Doanh nghiệp cung cấp dịch vụ trên không gian mạng tại VN phải **lưu trữ dữ liệu người dùng tại Việt Nam**.
*   **Rủi ro**: Nếu VPS (Server) của bạn đặt tại nước ngoài (AWS Singapore, DigitalOcean...), về lý thuyết là chưa tối ưu về luật (dù các Doanh nghiệp nhỏ ít bị soi xét).
*   **Khuyến nghị**:
    *   Tốt nhất nên thuê VPS tại Việt Nam (Viettel IDC, VNPT, FPT, CMC...).
    *   Vừa tuân thủ luật 100%, vừa cho tốc độ sync nhanh nhất cho khách hàng trong nước.

### 🌱 C. Quy Định Chuyên Ngành (Kinh doanh Vật tư Nông nghiệp)
*   **Quy định**: Cửa hàng kinh doanh thuốc BVTV phải có sổ sách theo dõi xuất nhập, truy xuất nguồn gốc (Thông tư 21/2015/TT-BNNPTNT).
*   **Lợi thế của App**:
    *   Tính năng **"Thẻ Kho" (Product Transactions)** vừa thêm vào chính là công cụ giúp khách hàng tuân thủ quy định này!
    *   Việc in hóa đơn có chi tiết mặt hàng giúp minh bạch nguồn gốc.
*   **Kết luận**: App không chỉ không vi phạm, mà còn là **công cụ giúp khách hàng đúng luật** hơn so với ghi sổ tay.

---

## 3. Checklist Hành Động

| Hạng mục | Trạng thái | Hành động tiếp theo |
|----------|------------|-------------------|
| **EULA/Thỏa thuận** | ✅ Đã có | Giữ nguyên trong bộ cài v1.0.0 |
| **Bảo mật đường truyền** | ✅ Đã có | Luôn dùng HTTPS cho API Sync |
| **Vị trí Server (VPS)** | ❓ Cần check | **Nên thuê VPS Việt Nam** (Ưu tiên số 1) |
| **Quyền riêng tư** | ✅ Đã cam kết | Không bán data khách hàng |

## 💡 Lời Khuyên Của Chuyên Gia
Bạn hoàn toàn có thể tự tin triển khai. Để marketing "ngầu" hơn và đánh tan mọi lo ngại pháp lý, hãy dùng câu này khi chào khách:

> *"Nodi POS không chỉ giúp anh/chị quản lý nhàn hạ, mà còn <u>tự động tạo sổ sách điện tử</u> đúng chuẩn quy định của Bộ Nông nghiệp, khỏi lo mỗi khi có đoàn kiểm tra!"*
