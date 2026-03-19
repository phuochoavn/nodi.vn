# 📱 Truy Xuất Nguồn Gốc Nông Sản — QĐ 25/QĐ-BNNMT & Verigoods.vn (2026)

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Pháp lý**: QĐ 25/QĐ-BNNMT + TT 11/2026/TT-BCT + NĐ 13/2023 + Luật 78/2025/QH15
> **Trọng tâm**: Verigoods.vn, GS1 Digital Link, JSON-LD, Nhật ký canh tác, Kiến trúc POS 4 lớp

---

## 🚨 Breaking Changes — TXNG 2026

| Thay đổi | Văn bản | Hiệu lực |
|----------|---------|:--------:|
| 🆕 **Hệ thống TXNG quốc gia** bắt buộc | QĐ 25/QĐ-BNNMT | 07/01/2026 |
| 📱 **Verigoods.vn** chính thức vận hành | Bộ Công Thương | 23/12/2025 |
| 🔗 **"Một bước trước - Một bước sau"** bắt buộc | TT 11/2026/TT-BCT | 02/2026 |
| 📝 **"Hộ chiếu số"** (Digital Product Passport) | Luật 78/2025/QH15 | 2026 |
| 🥭 **Sầu riêng** = mặt hàng thí điểm đầu tiên | QĐ 25 | 2026 |
| 📅 **01/07/2026** = Hạn chót TXNG hầu hết nông sản | QĐ 25 | 01/07/2026 |

---

## 1. Khung Pháp Lý — 3 Trụ Cột

### 1.1 QĐ 25/QĐ-BNNMT — Tái cấu trúc chuỗi cung ứng

| Giai đoạn | Thời gian | Nội dung |
|-----------|:---------:|---------|
| Thí điểm | Đầu 2026 | Sầu riêng + Nhật ký canh tác điện tử |
| **Bắt buộc** | **01/07/2026** | Hầu hết nông sản thực phẩm |
| Mở rộng | 2027+ | Toàn bộ chuỗi cung ứng |

**Dữ liệu bắt buộc trên hệ thống:**
- Thông tin nông hộ (định danh)
- Thông tin vùng trồng + thời gian canh tác/thu hoạch
- Thông tin sơ chế, đóng gói
- Tem QR hoặc vật mang dữ liệu tương đương

> 🔑 **Đại lý VTNN = "Node" kiểm soát dữ liệu** trong mạng lưới TXNG

### 1.2 TT 11/2026/TT-BCT — Nguyên tắc "Một bước trước - Một bước sau"

```
NCC đầu vào → ĐẠI LÝ VTNN → Nông dân/HTX
     ↑                              ↓
  "1 bước trước"              "1 bước sau"
     ↑                              ↓
  Phải biết ai cung cấp      Phải biết ai mua
```

**Thu hồi khẩn cấp (Recall):**
- Truy vết bằng mã lô + ngày SX → Khoanh vùng lô rủi ro
- Auto xuất báo cáo thu hồi toàn chuỗi
- Sản phẩm vi phạm → **Cảnh báo CÔNG KHAI** trên Cổng quốc gia

> ⚠️ Tem TXNG = **Công cụ tuân thủ pháp luật bắt buộc**, KHÔNG phải marketing

### 1.3 NĐ 13/2023 — Bảo vệ dữ liệu cá nhân

| Yêu cầu | Chi tiết |
|----------|---------|
| **Consent** | Nông dân phải đồng ý rõ ràng (không im lặng = đồng ý) |
| Mã hóa | QR công khai chỉ hiển thị GLN, tên HTX (KHÔNG hiện SĐT/CMND) |
| Báo cáo rò rỉ | **72 giờ** → Cục A05 Bộ Công an |
| Vi phạm | Phạt tiền nặng, đình chỉ phần mềm, truy cứu hình sự |

### 1.4 Phân biệt: TXNG đầu RA vs Quản lý đầu VÀO

| | Quản lý vật tư ĐẦU VÀO | TXNG nông sản ĐẦU RA |
|--|:----------------------:|:--------------------:|
| Đối tượng | Phân bón, thuốc BVTV, giống | Trái cây, gạo, nông sản |
| Ai quản lý | Đại lý VTNN (POS) | Trang trại/HTX |
| Nhiệm vụ POS | Ngăn hàng giả, lô, HSD, thuốc cấm | Cung cấp dữ liệu cho nhật ký |
| **Chiếc cầu nối** | → **Nhật ký canh tác điện tử** ← |

> 💡 POS đại lý = node cung cấp dữ liệu vật tư → Nhật ký canh tác → QR nông sản

---

## 2. Verigoods.vn — Hệ Thống TXNG Quốc Gia

### Kiến trúc kỹ thuật

| Đặc tính | Chi tiết |
|----------|---------|
| Nền tảng | **Cloud-based** (scalable, high availability) |
| Xác thực | SSO qua Cổng DVC Bộ Công Thương |
| API | RESTful, real-time data push |
| Bảo mật | OAuth 2.0 / JWT, TLS 1.3 |

### Quy trình đăng ký (4 bước KYC)

```
1️⃣ Đăng ký tài khoản (Verigoods.vn hoặc Cổng DVC)
    ↓
2️⃣ Khai báo tổ chức + danh mục sản phẩm
    ↓
3️⃣ Upload: GPKD + Công bố chất lượng + Nhãn mác
    ↓
4️⃣ Bộ Công Thương kiểm duyệt → Cấp Company Prefix
    → Được quyền sinh mã QR cho lô hàng
```

### So sánh Verigoods.vn vs Nền tảng tư nhân

| Tiêu chí | **Verigoods.vn** | iCheck (tư nhân) | TE-FOOD (Blockchain) |
|----------|:----------------:|:-----------------:|:--------------------:|
| Bản chất | Cổng dữ liệu **Nhà nước** | B2B2C thương mại | Blockchain phi tập trung |
| Giá trị pháp lý | **Bắt buộc** — minh chứng tối cao | Phụ thuộc API → Verigoods | Cần công nhận Nhà nước |
| Chi phí | **NS nhà nước** (có thể phí tem) | ~3.5M/năm | Rất cao |
| Ưu tiên | Quản trị rủi ro, thu hồi, chống gian lận | Marketing, review KH | Minh bạch tuyệt đối, XK EU |

> 🔑 **Dù dùng nền tảng nào, PHẢI liên thông API → Verigoods.vn**

### Triển khai tại ĐBSCL

- **Trọng điểm**: Sóc Trăng, Tiền Giang, Đồng Tháp
- **Mặt hàng tiên phong**: Sầu riêng (áp lực kiểm dịch TQ)
- **Mô hình**: HTX ↔ Đại lý VTNN = Liên kết chuỗi giá trị
- **Rào cản**: Internet yếu, thói quen "mua nợ ghi sổ", sức ỳ truyền thống

---

## 3. Chuẩn Kỹ Thuật Mã QR — GS1 Digital Link

### Cấu trúc thông tin bắt buộc ("Hộ chiếu số")

| Lớp | Thông tin | Ví dụ |
|:---:|----------|-------|
| **Định danh** | GTIN + GLN | Mã sản phẩm + Mã vùng trồng |
| **Dữ liệu lô** | Batch, Serial, HSD | Lô A2026-001, HSD 01/2028 |
| **Cấu thành** | Thành phần, vật tư đã dùng | Phân bón X, Thuốc Y |
| **Tuân thủ** | Hợp quy, kiểm nghiệm | Dấu hợp quy, kết quả phân tích |

### GS1 Digital Link — Thế hệ QR mới

```
Cấu trúc URI:
https://id.gs1.org/01/09506000134376/10/ABC123/21/12345

  01/ = GTIN (mã sản phẩm toàn cầu)
  10/ = Batch/Lot number
  21/ = Serial number
```

**Giải mã đa tầng (Multi-resolver):**

| Người quét | Thấy gì |
|-----------|---------|
| 🧑‍🌾 Nông dân | Hướng dẫn sử dụng, nguồn gốc, an toàn |
| 👮 QLTT (tài khoản công vụ) | Hải quan, kiểm định, logistics |
| 🛒 Người tiêu dùng | Xuất xứ, VietGAP, quy trình canh tác |

### JSON-LD — Định dạng dữ liệu chuẩn quốc tế

```jsonld
{
  "@context": "https://schema.org",
  "@type": "Product",
  "gtin": "09506000134376",
  "name": "Phân bón NPK 20-20-15",
  "batch": "A2026-001",
  "expiryDate": "2028-01-15",
  "manufacturer": {
    "@type": "Organization",
    "name": "Đạm Cà Mau"
  }
}
```

> 💡 JSON-LD = Máy EU đọc được dữ liệu từ POS Sóc Trăng **không cần chuyển đổi**

### GS1 Sunrise 2027
- Ngừng mã vạch 1D → Chuyển hoàn toàn sang 2D (QR)
- Nodi POS cần sẵn sàng trước 2027

---

## 4. Nhật Ký Canh Tác Điện Tử — Vai Trò Đại Lý

### Luồng dữ liệu tự động

```
Nông dân mua hàng → POS quét barcode → Xuất bán + Quản lý tồn kho
                                            ↓
                                    Push API → App nhật ký canh tác
                                            ↓
                              Nông dân chỉ cần: Chọn vật tư + Ngày phun + Diện tích
                                            ↓
                              Hệ thống đối chiếu: Mua gì? ↔ Dùng gì?
                                            ↓
                              Khớp → ✅ Cấp QR     Lệch → 🔴 Red Flag
```

### Khóa chặt gian lận bằng dữ liệu chéo

```
Nông dân khai: "Chỉ dùng thuốc sinh học" ✅
POS đại lý ghi: Mua 10L Chlorpyrifos (thuốc cấm) ❌
    ↓
AI phát hiện BẤT ĐỒNG NHẤT
    ↓
❌ TỪ CHỐI cấp mã QR cho lô nông sản
```

### Các app nhật ký canh tác tại VN

| App | Đặc điểm |
|-----|---------|
| MimosaTEK | IoT + tưới tự động |
| KidsGarden | Sổ tay nông nghiệp |
| AutoAgri | Tự động hóa |
| BNNMT app | Cơ quan nhà nước |

---

## 5. Tác Động Đến Đại Lý VTNN

### Hành động bắt buộc

| # | Hành động | Deadline |
|:-:|----------|:--------:|
| 1 | Trang bị POS có API Verigoods.vn | **Ngay** |
| 2 | Quét mã QR khi Nhập/Xuất mọi lô hàng | **Ngay** |
| 3 | Sẵn sàng thu hồi khẩn cấp | **Ngay** |
| 4 | Liên kết Push API → Nhật ký canh tác | 07/2026 |
| 5 | Đăng ký GS1 (GTIN, GLN) | 2026 |

### Rủi ro vs Cơ hội

| Không tuân thủ | Tuân thủ |
|:--------------:|:--------:|
| HTX từ chối mua | Trở thành "đối tác chiến lược" |
| Tồn kho ứ đọng | Thu hút khách hàng XK |
| Phạt theo Luật 78/2025 | Lợi thế cạnh tranh |
| Bị đào thải khỏi chuỗi | Định giá premium |

### Chi phí triển khai

| Hạng mục | Chi phí (VNĐ) | Tần suất |
|----------|:-------------:|:--------:|
| PM POS tích hợp API | 3.5M - 8M/năm | Hàng năm |
| Máy in tem QR | 2.5M - 5M | Một lần |
| Máy quét 2D (CMOS) | 1M - 2.5M | Một lần |
| Đăng ký GS1 VN | ~2M/năm | Hàng năm |
| Đào tạo nhân sự | Biến đổi | Ban đầu |
| **Tổng năm đầu** | **~10M - 20M** | |

> So sánh: **10-20M đầu tư** vs **Mất khách HTX/XK + Phạt pháp lý** → ROI rõ ràng

---

## 6. Kiến Trúc POS 4 Lớp — Tích Hợp TXNG

### Layer 1: API Gateway → Verigoods.vn

| Tính năng | Chi tiết |
|-----------|---------|
| Giao thức | RESTful API, OAuth 2.0 / JWT |
| Mã hóa | TLS 1.3 (tuân thủ NĐ 13/2023) |
| Push data | Đẩy thông tin lô hàng real-time |
| **Pull data** | Tự động tải danh mục cấm, recall alerts |

### Layer 2: Batch-to-QR (Nhập kho)

```
Quét QR thùng NCC → Parse GS1: GTIN + Batch + Expiry
                        ↓
              Ghi vào sổ cái tồn kho
                        ↓
     Xé lẻ bán? → Sinh QR mới (kế thừa data lô gốc)
                        ↓
              Liên kết: QR con → QR mẹ → NCC
```

### Layer 3: Checkout thông minh

```
Thanh toán → In hóa đơn có "Transaction QR"
                        ↓
Nông dân quét QR → Xem nguồn gốc toàn bộ sản phẩm
                        ↓
         Nút "Nhập Nhật ký canh tác"
                        ↓
    Auto đóng gói JSON-LD → Push sang app trang trại
```

### Layer 4: Dashboard Compliance Analytics

| Chỉ số | Mô tả |
|--------|-------|
| 📊 **% hàng có QR** | Đã mã hóa vs Chưa mã hóa |
| 🔴 **Cảnh báo thu hồi** | Webhook từ Verigoods.vn → Highlight lô rủi ro |
| 🔐 **Tuân thủ NĐ 13** | Access logs + Tỷ lệ consent nông dân |
| 📦 **Tồn kho QR** | Khớp: Kho vật lý = Kho dữ liệu QR |

---

## 7. Nodi POS — Gap Analysis TXNG

### Đã có ✅

| Tính năng | Trạng thái |
|-----------|:----------:|
| Batch tracking FEFO/FIFO | ✅ |
| QR Code mỗi lô | ✅ (QRCodeService.ts) |
| Offline-first (SQLite) | ✅ |
| Auto-lock thuốc cấm | ✅ |

### Cần bổ sung ⚠️

| Tính năng | Ưu tiên | Deadline |
|-----------|:-------:|:--------:|
| API Gateway → Verigoods.vn | 🔴 P0 | 07/2026 |
| GS1 Digital Link parser | 🔴 P0 | 2026 |
| JSON-LD output format | 🟡 P1 | 2027 |
| Push API → Nhật ký canh tác | 🟡 P1 | 07/2026 |
| Batch-to-QR (xé lẻ kế thừa) | 🟡 P1 | 2026 |
| Transaction QR trên hóa đơn | 🟢 P2 | 2026 |
| Dashboard TXNG compliance | 🟢 P2 | 2026 |
| Consent management (NĐ 13) | 🟢 P2 | 2026 |
| Recall webhook listener | 🟢 P2 | 2026 |

---

## 8. Timeline Tổng Hợp TXNG 2025-2027

| Thời điểm | Sự kiện |
|-----------|---------|
| 23/12/2025 | Verigoods.vn chính thức vận hành |
| 01/01/2026 | TXNG bắt buộc cho hóa chất/BVTV |
| 07/01/2026 | QĐ 25 ban hành — Lộ trình TXNG nông sản |
| 02/2026 | TT 11/2026 có hiệu lực — "1 bước trước - 1 bước sau" |
| **01/07/2026** | **Hạn chót TXNG hầu hết nông sản** |
| 2027 | GS1 Sunrise — Chuyển toàn bộ sang mã 2D |

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Dành cho Nodi POS — Nút xác thực dữ liệu trong chuỗi cung ứng nông nghiệp*
