# 🔍 Báo Cáo Nghiên Cứu Chuyên Sâu: Sapo POS / Omnichannel

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 12/03/2026
> **Mục đích**: Phân tích đối thủ cạnh tranh #2 của Nodi POS

---

## 1. TỔNG QUAN CÔNG TY

| Thông tin | Chi tiết |
|---|---|
| **Tên pháp lý** | Công ty Cổ phần Công nghệ Sapo |
| **Tiền thân** | Bizweb (nền tảng thiết kế website TMĐT) |
| **Thành lập** | 20/08/2008 |
| **Trụ sở** | Hà Nội + **24 chi nhánh** toàn quốc |
| **Nhân sự** | **~1,000 người** (Dev + Support + Sales) |
| **Số cửa hàng** | **230,000+** doanh nghiệp |
| **Liên kết tài chính** | VNLIFE sở hữu 99.99% VNPAY → Sapo tích hợp sâu hạ tầng thanh toán |
| **Doanh thu ước tính** | Hàng trăm tỷ VNĐ/năm (ARR từ 230K subscribers) |

### DNA khác biệt vs KiotViet
- **KiotViet**: Xuất phát từ POS offline → mạnh offline + bán lẻ truyền thống
- **Sapo**: Xuất phát từ Website TMĐT (Bizweb) → mạnh omnichannel + online

---

## 2. SẢN PHẨM — 3 Trụ Cột

### Sapo POS (Bán hàng tại quầy)
- Tính tiền nhanh, quét mã vạch, in bill
- Quản lý kho, khách hàng, NCC
- Mobile App quản lý từ xa 24/7
- **Offline Mode** ✅ (lưu local → auto sync khi có mạng)

### Sapo Web (Website TMĐT độc lập)
- Tạo web bán hàng từ template
- 5GB, không giới hạn sản phẩm
- SEO-ready, responsive
- Tích hợp thanh toán + vận chuyển

### Sapo Omnichannel (Hợp kênh — sản phẩm cao cấp nhất)
- Gộp POS + Web + Sàn TMĐT + MXH về 1 dashboard
- Đồng bộ tồn kho real-time xuyên kênh
- Loyalty đồng nhất online ↔ offline
- Chat hợp kênh AI (auto phân luồng tin nhắn)

### Tích hợp

| Loại | Chi tiết |
|---|---|
| **Sàn TMĐT** | Shopee, TikTok Shop, Lazada, Tiki (tối đa 15 gian hàng) |
| **MXH** | Facebook Fanpage, Zalo OA (tối đa 5 trang) |
| **Vận chuyển** | Sapo Express (aggregator): GHN, GHTK, Viettel Post, J&T |
| **Thanh toán** | VNPAY QR, VietQR, Momo, ZaloPay, máy quẹt thẻ |
| **Phần cứng** | Máy in, máy quét, ngăn tiền, màn hình phụ |

### Offline Mode — Chi tiết kỹ thuật
- ✅ Bán hàng, quét barcode, in bill, tạo KH mới
- ❌ **KHÔNG THỂ** khi offline: chuyển kho, tích điểm, đổi trả, báo cáo, áp KM
- ⚠️ Cấm dùng Private/Incognito mode (mất data)
- Dữ liệu lưu localStorage → auto sync khi có mạng

### Ngành hàng hỗ trợ
- Bán lẻ, tạp hóa, siêu thị mini
- Thời trang, giày dép
- **F&B** (có gói riêng Sapo F&B: quản lý bàn, in bếp, tách bill)
- Mỹ phẩm, điện tử
- **VTNN** (quản lý lô/date, hoạt chất, UOM, công nợ vụ mùa)

---

## 3. BẢNG GIÁ (2025-2026)

| Gói | Giá chuẩn/tháng | Giá ưu đãi (2 năm) | Phí khởi tạo | Kênh | NV tối đa |
|---|---|---|---|---|---|
| **F&B** | 229,000đ | 171,600đ | — | F&B chuyên biệt | — |
| **Start Up** | 249,000đ | **170,000đ** | 1,000,000đ* | 1 kênh (POS/MXH/TMĐT) | 3 |
| **Pro** | 399,000đ | **249,000đ** | 1,500,000đ* | 3 kênh | 5 |
| **Omni** | 899,000đ | **600,000đ** | 2,000,000đ* | 4 kênh (POS+MXH+TMĐT+Web) | 15 |
| **Growth** | 1,499,000đ | **999,000đ** | 3,000,000đ* | 4 kênh + AI Workflow | 30 |
| **Web Standard** | 899,000đ | **499,000đ** | — | Website TMĐT độc lập | — |

*\* Phí khởi tạo **MIỄN PHÍ** nếu ký hợp đồng ≥ 2 năm*

### So sánh giá với KiotViet

| | Sapo (Start Up) | KiotViet (Cơ bản) | Nodi POS |
|---|---|---|---|
| Giá/tháng | 170,000đ (2 năm) | ~200,000đ | ~166,000đ (1.99M/năm) |
| Giá/năm | 2,040,000đ | ~2,400,000đ | **1,990,000đ** |
| Phí khởi tạo | 1M (hoặc miễn) | Linh hoạt | **0đ** |
| Trial | ? | 10-15 ngày | **Miễn phí mãi mãi (20 đơn/ngày)** |

### Phí ẩn & Add-ons
- **Mở rộng chi nhánh**: phí bổ sung
- **Thêm NV/gian hàng/fanpage**: phí bổ sung khi vượt hạn mức
- **Điểm AI**: 100 điểm/tháng (gói thường), 1000 điểm/tháng (Growth) — hết phải mua thêm
- **HĐĐT, SMS**: phí theo số lượng

---

## 4. ĐIỂM MẠNH & ĐIỂM YẾU

### ✅ Điểm mạnh

| # | Điểm mạnh | Chi tiết |
|---|---|---|
| 1 | **Omnichannel #1 Việt Nam** | Đồng bộ 15 gian hàng + 5 MXH + Website + POS |
| 2 | **UX thân thiện** | "Rất dễ dùng", "tiện lợi", "thích hợp mọi hoàn cảnh" |
| 3 | **Sapo Express** | Aggregator vận chuyển, so giá real-time |
| 4 | **VNPAY integration** | Thanh toán QR, ví điện tử, auto đối soát |
| 5 | **Support xuất sắc** | 2 tổng đài (1800 + 1900), 24/7, phản hồi nhanh |
| 6 | **24 chi nhánh** | Hỗ trợ trực tiếp tại cửa hàng KH |
| 7 | **OmniAI** | Chat AI, workflow automation, điểm AI |
| 8 | **Sapo Enterprise** | Giải pháp riêng cho doanh nghiệp lớn, tùy chỉnh API |

### ❌ Điểm yếu

| # | Điểm yếu | Chi tiết |
|---|---|---|
| 1 | **"Giao diện mới khó dùng"** | Phàn nàn phổ biến nhất — mỗi lần update UI gây ma sát |
| 2 | **Offline Mode hạn chế** | Không thể: chuyển kho, tích điểm, đổi trả, báo cáo |
| 3 | **Giá leo thang** | Phí ẩn nhiều: chi nhánh, NV, gian hàng, điểm AI |
| 4 | **Lock-in 2 năm** | Miễn phí khởi tạo = ép ký dài hạn |

---

## 5. CHIẾN LƯỢC MARKETING & BÁN HÀNG

### Inbound
- SEO + Blog kinh doanh
- YouTube đào tạo
- Khảo sát thường niên 15,000 nhà bán hàng → PR data

### Outbound
- **24 chi nhánh** với đội BD tại chỗ
- Telesales + demo trực tiếp tại cửa hàng
- **Sapo Enterprise**: BD cấp cao, tư vấn giải pháp cho tập đoàn

### Đại lý & Affiliate
- Affiliate v2 (300K/3 tháng) — biến KH thành đại sứ
- Đại lý ủy quyền theo vùng (miền Trung, miền Nam...)
- Hoa hồng chiết khấu định kỳ

---

## 6. CHIẾN LƯỢC NGÀNH VTNN

### Tính năng VTNN của Sapo

| # | Tính năng | Chi tiết |
|---|---|---|
| 1 | **Lô/Date** | ✅ Quản lý lô sản xuất + HSD, cảnh báo cận date, FIFO |
| 2 | **Hoạt chất** | ✅ Quản lý theo gốc hoạt chất, tìm SP thay thế |
| 3 | **UOM đa tầng** | ✅ Kiện→Thùng→Chai→ml, auto quy đổi |
| 4 | **Công nợ vụ mùa** | ✅ Phân tách theo vụ (ĐX, HT), credit limit |
| 5 | **Truy xuất nguồn gốc** | ⚠️ Cơ bản (lịch sử NCC→Đại lý→KH) — không QR |

### Nhận xét
- Sapo có tính năng VTNN **tương đương KiotViet**
- Cả hai đều KHÔNG CÓ: AI chẩn đoán bệnh, auto-lock thuốc cấm, QR truy xuất lô

---

## 7. HƯỚNG ĐI TƯƠNG LAI

### 7.1. OmniAI + Workflow Automation
- Chat AI phân luồng tin nhắn
- Điểm AI (tiền tệ nội bộ) — nguồn doanh thu mới
- IF-THEN automation: auto chọn vận chuyển, auto gửi Zalo ZNS

### 7.2. Thuế & HĐĐT
- API trực tiếp với Tổng Cục Thuế
- Auto: quét barcode → kế toán → HĐĐT → email/Zalo cho KH

### 7.3. Tài chính nhúng (Embedded Finance)
- Sapo Pay + Sapo Money (backed by VNPAY)
- Tương lai: Micro-lending, Merchant Cash Advance, bảo hiểm hàng hóa

### 7.4. Predictive Analytics
- Dự báo nhu cầu (Demand Forecasting)
- Auto Reorder Point
- Phân tích hành vi KH theo mùa vụ

---

## 8. SO SÁNH 3 CHIỀU: NODI vs KIOTVIET vs SAPO

| Tiêu chí | Nodi POS | KiotViet | Sapo |
|---|---|---|---|
| **Năm thành lập** | 2026 | 2014 | 2008 |
| **Khách hàng** | Pre-launch | 300,000+ | 230,000+ |
| **Nhân sự** | 1 + AI agents | 2,000+ | 1,000+ |
| **DNA** | Offline-first VTNN | POS offline retail | Omnichannel online |
| **Giá/năm** | **1.99M** ✅ | 2.4M | 2.04M |
| **Freemium** | **Mãi mãi (20 đơn)** ✅ | 10-15 ngày rồi khóa | ? |
| **Offline** | **100% (Tauri native)** ✅ | Rất mạnh | Khá (hạn chế tính năng) |
| **AI chatbot** | **315+ bệnh cây** ✅ | ❌ | Chat AI (phân luồng tin nhắn) |
| **Thuốc cấm** | **3 tầng auto-lock** ✅ | ❌ | ❌ |
| **QR truy xuất lô** | **✅ QR code** | Cơ bản | Cơ bản |
| **Công nợ vụ mùa** | **✅ Sao kê kiểu NH** | ✅ | ✅ |
| **UOM đa tầng** | ✅ | ✅ | ✅ |
| **Lô/Date** | ✅ | ✅ FEFO | ✅ FIFO |
| **Sàn TMĐT** | ❌ | Shopee, Lazada, TikTok | **15 gian hàng** ✅ |
| **Vận chuyển** | ❌ | GHN, GHTK, J&T | **Sapo Express** ✅ |
| **Thanh toán số** | VietQR cơ bản | VIB, VietinBank, VCB | **VNPAY full stack** ✅ |
| **Fintech** | ❌ | Hướng tới | **VNPAY/VNLIFE** ✅ |
| **F&B** | ❌ | ✅ (20+ ngành) | ✅ (gói F&B riêng) |
| **Mobile App** | ✅ Full parity | ✅ 2 app riêng | ✅ App quản lý |

### 🎯 Nodi POS thắng ở đâu (vs cả 2)?
1. **Offline tuyệt đối** — Native app, không cần browser, không sợ mất data
2. **AI nông nghiệp** — KHÔNG ĐỐI THỦ NÀO CÓ chẩn đoán bệnh cây
3. **Auto-lock thuốc cấm** — KHÔNG ĐỐI THỦ NÀO CÓ 3 tầng bảo vệ
4. **QR truy xuất lô** — Forward trace + QR code chuẩn
5. **Rẻ nhất** — 1.99M/năm vs 2.04M vs 2.4M
6. **Freemium thật** — Không khóa tính năng, không ép ký dài hạn

### ⚠️ Nodi POS thua ở đâu?
1. **0 khách vs 230K-300K** — Chưa có thị phần
2. **Không TMĐT / Omnichannel** — Nhưng VTNN ít cần
3. **Không Fintech** — Không có VNPAY/VIB partnership
4. **Không F&B** — Chỉ focus VTNN
5. **1 người** — vs 1,000-2,000 nhân sự

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 12/03/2026*
*Phân tích so sánh 3 chiều bởi Nodi POS Executive Assistant*
