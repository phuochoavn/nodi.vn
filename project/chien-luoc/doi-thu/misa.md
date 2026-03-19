# 🔍 Báo Cáo Nghiên Cứu Chuyên Sâu: MISA eShop & MISA AMIS

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 12/03/2026
> **Mục đích**: Phân tích đối thủ #3 — Học hỏi chiến lược xây dựng hệ sinh thái

---

## 1. TỔNG QUAN CÔNG TY

| Thông tin | Chi tiết |
|---|---|
| **Tên pháp lý** | Công ty Cổ phần MISA (MISA JSC) |
| **Thành lập** | **1994** (hơn 30 năm) |
| **Trụ sở** | Hà Nội + VP TP.HCM, Đà Nẵng, Đắk Lắk |
| **Lãnh đạo** | TGĐ Đinh Thị Thúy |
| **Nhân sự** | **Hàng ngàn người** (Dev + Support + Sales + Marketing) |
| **Khách hàng** | **250,000+ doanh nghiệp** (AMIS Quy trình) |
| **Nhà đầu tư** | TA Associates, Mekong Capital (Private Equity) |
| **IPO** | Chưa niêm yết — vẫn là công ty tư nhân |
| **Doanh thu** | Ước tính **hàng ngàn tỷ VNĐ/năm** |
| **Vinh danh** | Bộ TT&TT: #1 trong Chương trình SMEdx 2021+ |

### Lịch sử phát triển
- **1994-2010**: Phần mềm kế toán đóng gói (CD cài đặt)
- **2011-2019**: Chuyển lên Cloud (SaaS)
- **2020-nay**: AI + Big Data + Hệ sinh thái hội tụ

---

## 2. HỆ SINH THÁI SẢN PHẨM

### Triết lý: "Hệ điều hành doanh nghiệp"
- Dữ liệu nhập 1 lần → tự động lan truyền toàn bộ phòng ban
- Xóa bỏ hoàn toàn silo dữ liệu

### Danh sách sản phẩm

| # | Sản phẩm | Mục đích | Đặc điểm |
|---|---|---|---|
| 1 | **MISA AMIS Kế toán** | Kế toán Cloud | **Trái tim hệ sinh thái** — mọi dữ liệu đổ về đây |
| 2 | **MISA SME** | Kế toán on-premise | Cài máy tính, doanh nghiệp thích local |
| 3 | **MISA eShop** | Quản lý bán hàng POS | Omnichannel, TMĐT, VTNN |
| 4 | **MISA AMIS CRM** | Quản lý khách hàng B2B | Lead scoring, pipeline, auto marketing |
| 5 | **MISA AMIS HRM** | Nhân sự + tính lương | Chấm công sinh trắc, KPI, bảng lương auto |
| 6 | **MISA AMIS Quy trình** | Phê duyệt không giấy tờ | 250K DN dùng, mobile approval |
| 7 | **MISA Mimosa** | Kế toán hành chính sự nghiệp | Trường học, bệnh viện, UBND — **gần như 0 đối thủ** |
| 8 | **MISA FinGPS** | Tài chính cá nhân | Thu chi, ngân sách cá nhân |
| 9 | **MISA ASP** | Nền tảng kế toán dịch vụ | Kết nối kế toán freelance ↔ doanh nghiệp |
| 10 | **MISA AVA** | Trợ lý AI | Voice + Text, thực thi tác vụ sâu |
| 11 | **CukCuk** | POS F&B | Nhà hàng, quán cafe |

### Cơ chế liên kết
```
eShop (Bán hàng) ──→ AMIS Kế toán ←── HRM (Lương)
                            ↑
CRM (Hợp đồng) ────────────┘
Quy trình (Phê duyệt) ─────┘
```
→ **Càng dùng nhiều module → chi phí chuyển đổi càng KHỔNG LỒ**

---

## 3. MISA eShop — Chi tiết

### Tính năng
- **POS tại quầy**: Quét barcode, tính tiền, in bill, kết nối cân điện tử
- **Omnichannel**: Shopee, Lazada, TikTok Shop, Tiki, Sendo, Facebook, Zalo, Instagram
- **Vận chuyển**: GHN, GHTK, Viettel Post, AhaMove, J&T Express — auto đối soát COD
- **Thanh toán**: QR Code, thẻ POS, ví điện tử
- **Kho**: 40,000 SKU, vị trí lưu kho (tủ/quầy/kệ) — tiết kiệm 85% thời gian nhặt hàng
- **HĐĐT**: Tích hợp sẵn, tặng 5,000 hóa đơn + 1 năm chữ ký số
- **Kê khai thuế**: Lập tờ khai trực tiếp trên điện thoại (Nghị định 70/2025)
- **AI bán hàng**: Auto copywriting, chỉnh ảnh, chốt đơn livestream

### Offline Mode
- ✅ Quét barcode, tính tiền, in bill, tạo KH, lưu local → auto sync
- ❌ Không thể: chuyển kho, tích điểm, đổi trả, báo cáo
- ⚠️ Cấm dùng Incognito mode

---

## 4. NỀN TẢNG KỸ THUẬT

| Tiêu chí | Chi tiết |
|---|---|
| **Kiến trúc** | Cloud-first + Hybrid (on-premise MISA SME) |
| **Nền tảng** | Web, Windows/macOS, iOS, Android |
| **Offline** | ✅ Có (eShop POS) |
| **API mở** | MISA Open API — cho phép tích hợp bên thứ 3 |
| **Bảo mật** | **ISO/IEC 27001** + **CSA/CCM** + **SOC2** |

---

## 5. BẢNG GIÁ

### MISA eShop (Bán lẻ)

| Gói | Giá/tháng | Đặc điểm |
|---|---|---|
| **Cơ bản** | **99,000đ** | POS đơn giản, 1 cửa hàng |
| **Chuyên nghiệp** | **299,000đ** | Kho chuyên sâu, công nợ, quản lý NV |
| **Cao cấp** | **699,000đ** | Omnichannel, chuỗi, AI bán hàng |
| **Đặc biệt** | Đến 1,400,000đ | Tùy biến |
| **Dùng thử** | **3 THÁNG MIỄN PHÍ** | Rất hào phóng |

### MISA SME (Kế toán on-premise) — 3 user mặc định

| Gói | Giá/năm | User thêm |
|---|---|---|
| Standard | 4,650,000đ | +1,550,000đ/user/năm |
| Professional | 5,850,000đ | +1,550,000đ/user/năm |
| Enterprise | 7,050,000đ | +1,550,000đ/user/năm |

### MISA AMIS (Cloud quản trị) — Bundle

| Gói | Giá/năm | Số phần mềm |
|---|---|---|
| Starter | **Miễn phí** | 3 PM cơ bản |
| Standard | 18,800,000đ | 4 PM |
| Professional | 32,112,000đ | 7 PM |
| Enterprise | 39,112,000đ | 7 PM + mở rộng |

### Phí ẩn
- Mở rộng chi nhánh: phí bổ sung
- User thêm: 1,550,000đ/user/năm (SME)
- SMS: theo số tin nhắn
- HĐĐT: vượt 5,000 hóa đơn miễn phí → mua thêm
- Dung lượng Cloud: mua thêm khi hết

---

## 6. ĐIỂM MẠNH — BÀI HỌC CHIẾN LƯỢC

### 🏆 Bài học 1: "Chiếm lấy trái tim hệ thống — Kế toán"
- MISA chọn nghiệp vụ **nhàm chán nhất, khó nhất, pháp lý nhất**: Kế toán
- Khi kế toán tin tưởng → họ ép Giám đốc mua MISA
- Từ "lõi" kế toán → cross-sell CRM, HRM, eShop DỄ NHƯ ĂN CHÁO
- **Bài học**: Chiếm 1 nghiệp vụ cốt lõi → mở rộng ra toàn doanh nghiệp

### 🏆 Bài học 2: "Khóa chân bằng chi phí chuyển đổi vô hình"
- Dữ liệu 10 năm nằm trên MISA → không dám rời
- Nhân viên chỉ biết dùng MISA → đào tạo lại = tốn tiền
- 7 module liên kết → gỡ 1 module = phá vỡ toàn bộ
- **Bài học**: Hệ sinh thái hội tụ = rào cản phòng thủ mạnh nhất

### 🏆 Bài học 3: "Đào tạo = Bán hàng không tốn tiền"
- **MISA Academy (MIBI)**: Học viện đào tạo kế toán
- Đưa MISA vào **giáo trình đại học** → triệu sinh viên ra trường "chỉ biết dùng MISA"
- Kho biểu mẫu miễn phí (50 mẫu JD, 17 bảng chấm công, mẫu KPI...)
- Khóa học online cấp chứng nhận miễn phí
- **Bài học**: Biến thị trường thành học trò = bán hàng ở tầm thế kỷ

### 🏆 Bài học 4: "Bán sự an toàn pháp lý, không bán tính năng"
- Không nói "phần mềm tốt" — nói "cán bộ thuế không phạt bạn"
- Luôn cập nhật thông tư mới NHANH NHẤT
- Tặng 5,000 HĐĐT + chữ ký số = giải quyết nỗi sợ pháp lý
- Hội thảo miễn phí về Luật Thuế mới → demo MISA luôn
- **Bài học**: Ở VN, bán tiện lợi = tiền lẻ, bán an toàn pháp lý = tiền tỷ

### 🏆 Bài học 5: "Mạng lưới kế toán = đội quân không lương"
- MISA ASP: Kế toán freelance giới thiệu khách → nhận 500K điểm
- Điểm KHÔNG đổi tiền mặt — chỉ dùng mua MISA
- Vòng tuần hoàn: Bán MISA → lấy điểm → dùng điểm mua MISA cho mình
- **Bài học**: Biến khách hàng thành lực lượng bán hàng tự nhiên

---

## 7. ĐIỂM YẾU — KHOẢNG TRỐNG KHAI THÁC

| # | Điểm yếu | Chi tiết |
|---|---|---|
| 1 | **Learning curve quá cao** | Giao diện "kế toán-hóa" — tiểu thương nông thôn hoa mắt |
| 2 | **Chậm giờ cao điểm** | Mùa quyết toán thuế (T3-T4) → server quá tải |
| 3 | **Chi phí dội lên** | Hết dung lượng → ép nâng cấp gói |
| 4 | **Yếu F&B chuyên sâu** | CukCuk thua iPOS về nghiệp vụ nhà hàng |
| 5 | **TMĐT không bằng Sapo** | Ít API đối tác, không mạnh cross-border |

---

## 8. MARKETING & BÁN HÀNG

### SEO + Content Marketing = BÁ CHỦ
- Tìm "cách tính thuế TNCN", "mẫu hợp đồng lao động" → **MISA top 1-3 Google**
- Đội Content Marketers khổng lồ viết bài pháp luật + tài chính
- Tải biểu mẫu → để lại SĐT → Lead → CRM → Telesales chốt

### Sự kiện offline
- Hợp tác Chi cục Thuế + Hội Kế toán → Hội thảo miễn phí
- "Đến nghe cập nhật Luật Thuế mới" → demo MISA luôn
- Khách đến vì sợ phạt → về mua MISA vì an tâm

### Đội Sales
- Telesales: tiếp cận DN nhỏ
- Field Sales: đàm phán tập đoàn
- ASP Network: hàng vạn kế toán freelance = chân rết
- **Đặc biệt**: Sales MISA phải thi kế toán cơ bản → tư vấn như chuyên gia

---

## 9. CHIẾN LƯỢC NGÀNH VTNN

### MISA eShop phiên bản VTNN — "Thiết kế riêng biệt, đơn giản và dễ dùng"

| # | Tính năng | Chi tiết |
|---|---|---|
| 1 | **40,000 SKU** | Lưu trữ hàng ngàn loại thuốc + vị trí kho (tủ/quầy) |
| 2 | **Lô/Date** | Số lô + HSD, cảnh báo cận date, FIFO |
| 3 | **Hoạt chất** | Tra cứu SP thay thế cùng hoạt chất |
| 4 | **UOM đa tầng** | Tấn→Bao→Ký, Thùng→Chai→ml, kết nối cân điện tử |
| 5 | **Công nợ vụ mùa** | Giá tiền mặt ≠ giá nợ, theo dõi dài hạn, tự động liên kết kế toán |

---

## 10. HƯỚNG ĐI TƯƠNG LAI

### 10.1. MISA AVA — Trợ lý AI
- Voice + Text command
- "Tạo báo cáo tài chính tháng 10" → AI tự truy xuất + vẽ biểu đồ
- AI bán hàng: auto copywriting, chỉnh ảnh, chốt đơn livestream
- Machine Learning dự báo nhu cầu thị trường

### 10.2. Tài chính nhúng — MISA Lending
- Credit Scoring từ data kế toán thực tế
- Vay tín chấp trong 24h ngay trên phần mềm
- MISA thu hoa hồng trên hàng ngàn tỷ dư nợ giải ngân
- **Chuyển từ công ty phần mềm → công ty Dữ liệu + Fintech**

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 12/03/2026*
*Lưu trữ bởi Nodi POS Executive Assistant*
