# 📋 DeepSearch Prompts Bổ Sung — Nghiên Cứu VTNN (Đợt 2)

> **Ngày tạo**: 13/03/2026
> **Số lượng**: 4 prompts → 4 file nghiên cứu
> **Mục tiêu**: Bổ sung dữ liệu cho Auto-lock database, QR truy xuất, competitive intelligence, user persona

---

## Prompt 5 → File: `05-danh-muc-hoat-chat-cam-tt75-2025.md`

```
Báo Cáo Nghiên Cứu Chi Tiết: Danh Mục Đầy Đủ 31 Hoạt Chất Bảo Vệ Thực Vật Bị Cấm Theo Thông Tư 75/2025/TT-BNNPTNT Và Lộ Trình Chuyển Tiếp Áp Dụng Cho Đại Lý Vật Tư Nông Nghiệp Tại Việt Nam

Thông tư số 75/2025/TT-BNNPTNT ban hành ngày 26/12/2025 (có hiệu lực từ 10/02/2026) đánh dấu bước ngoặt quan trọng trong quản lý thuốc bảo vệ thực vật tại Việt Nam, chính thức loại bỏ 31 hoạt chất ra khỏi Danh mục được phép sử dụng. Báo cáo này cần cung cấp phân tích đầy đủ và có hệ thống nhằm phục vụ trực tiếp cho việc xây dựng cơ sở dữ liệu tự động khóa (Auto-lock database) trong phần mềm quản lý bán hàng chuyên biệt cho ngành nông nghiệp.

Yêu cầu nghiên cứu chi tiết:

1. Bảng danh mục đầy đủ 31 hoạt chất bị cấm, bao gồm:
   - Tên hoạt chất (tiếng Việt + tên quốc tế/IUPAC)
   - Nhóm phân loại (thuốc trừ sâu, thuốc trừ cỏ, thuốc trừ bệnh, bảo quản lâm sản, v.v.)
   - Phân loại độc tính theo WHO (Ia, Ib, II, III)
   - Lý do cấm (Công ước Rotterdam, Stockholm, hoặc rủi ro nội địa)
   - Số lượng tên thương phẩm phổ biến trên thị trường Việt Nam gắn với hoạt chất đó

2. Danh sách các tên thương phẩm (tên thương mại) phổ biến nhất tại Việt Nam gắn với từng hoạt chất cấm:
   - Ví dụ: Hoạt chất Carbosulfan → Tên thương phẩm: Marshal 200SC, Sát Trùng Đan 95WP, v.v.
   - Nhà sản xuất/nhập khẩu chính
   - Dạng bào chế (EC, SC, WP, WG, GR)
   - Mã đăng ký lưu hành (nếu có)

3. Lộ trình chuyển tiếp chi tiết cho từng hoạt chất:
   - Thời hạn cấm nhập khẩu
   - Thời hạn cấm sản xuất trong nước
   - Thời hạn cho phép buôn bán, sử dụng (countdown từ ngày TT có hiệu lực)
   - Phân biệt rõ: hoạt chất nào cấm 1 năm, 2 năm, cấm ngay

4. Lịch sử cấm hoạt chất tại Việt Nam (2017-2025):
   - 14 hoạt chất đã cấm trước đó (Glyphosate, Chlorpyrifos ethyl, Paraquat, Fipronil, v.v.)
   - Bài học từ giai đoạn chuyển tiếp: đại lý nào bị phạt vì tồn kho thuốc cấm?
   - Tổng số hoạt chất hiện đang bị cấm hoàn toàn tại VN (14 cũ + 31 mới = ?)

5. Đặc thù tại ĐBSCL:
   - Hoạt chất cấm nào đang được sử dụng phổ biến nhất tại khu vực ĐBSCL?
   - Các loại cây trồng chịu ảnh hưởng lớn nhất khi mất các hoạt chất này (lúa, cây ăn trái, rau màu)
   - Giải pháp thay thế (hoạt chất sinh học, IPM) đang được Bộ NN khuyến cáo

6. Ứng dụng cho phần mềm POS nông nghiệp:
   - Cấu trúc dữ liệu cần thiết để Auto-lock: {active_ingredient, trade_names[], ban_date, transition_end_date, penalty_level}
   - Quy trình cập nhật khi Bộ NN ban hành thêm thông tư cấm bổ sung
   - UX/UI cảnh báo: màu sắc, âm thanh, thông báo khi quét barcode thuốc cấm

Định dạng output mong muốn: Bảng biểu có cấu trúc, sắp xếp theo nhóm phân loại, kèm timeline trực quan cho lộ trình cấm.
```

---

## Prompt 6 → File: `06-truy-xuat-nguon-goc-qd25-verigoods.md`

```
Báo Cáo Nghiên Cứu Chuyên Sâu: Quy Định Truy Xuất Nguồn Gốc Nông Sản Bắt Buộc Theo Quyết Định 25/QĐ-BNNPTNT, Hệ Thống Verigoods.vn Và Lộ Trình Tích Hợp Cho Phần Mềm Quản Lý Đại Lý Vật Tư Nông Nghiệp (Cập nhật 2026)

Quyết định số 25/QĐ-BNNPTNT ban hành ngày 07/01/2026 đặt ra mục tiêu khắt khe: đến 01/07/2026, các địa phương phải hoàn thiện Hệ thống truy xuất nguồn gốc nông sản. Song song, Bộ Công Thương đã chính thức vận hành nền tảng truy xuất quốc gia Verigoods.vn từ 23/12/2025. Báo cáo này phân tích toàn diện khung pháp lý, chuẩn kỹ thuật, và chiến lược tích hợp cho phần mềm POS chuyên biệt ngành nông nghiệp.

Yêu cầu nghiên cứu chi tiết:

1. Khung pháp lý truy xuất nguồn gốc tại Việt Nam (2026):
   - QĐ 25/QĐ-BNNPTNT: Phạm vi áp dụng, đối tượng bắt buộc, timeline
   - Thông tư 11/2026/TT-BCT (nếu có): Quy định chi tiết về tem truy xuất
   - Nghị định 13/2023: Bảo vệ dữ liệu cá nhân trong hệ thống truy xuất
   - Phân biệt: truy xuất nguồn gốc nông SẢN (đầu ra) vs quản lý vật tư đầu VÀO

2. Hệ thống Verigoods.vn:
   - Kiến trúc kỹ thuật: cloud-based, API specifications, data format
   - Chuẩn dữ liệu QR code: GS1, DataMatrix, hay chuẩn riêng VN?
   - Quy trình đăng ký doanh nghiệp/hộ kinh doanh trên hệ thống
   - Chi phí sử dụng (miễn phí hay thu phí?)
   - Tình hình triển khai thực tế tại các tỉnh ĐBSCL
   - So sánh với các hệ thống truy xuất khác: iCheck, TE-FOOD, Agriseco

3. Chuẩn kỹ thuật mã QR truy xuất:
   - Thông tin bắt buộc trong mã QR: mã lô, ngày SX, HSD, cơ sở SX, thành phần
   - Định dạng dữ liệu (JSON, XML, URI scheme?)
   - Kích thước tối thiểu tem QR trên bao bì VTNN
   - Quy trình xác thực: ai quét (QLTT, nông dân, HTX)?

4. Nhật ký canh tác điện tử (Digital Farm Diary):
   - Yêu cầu của QĐ 25 về nhật ký canh tác
   - Vai trò của đại lý VTNN: cung cấp dữ liệu vật tư đầu vào
   - Liên kết POS đại lý ↔ Nhật ký canh tác trang trại
   - Các app nhật ký canh tác đang có tại VN (KidsGarden, MimosaTEK, v.v.)

5. Tác động đến đại lý VTNN:
   - Đại lý phải làm gì cụ thể để tuân thủ?
   - Chi phí triển khai (tem QR, máy in, phần mềm)
   - Hậu quả nếu không có QR: bị loại khỏi chuỗi cung ứng XK
   - Cơ hội: đại lý có QR sẽ thu hút HTX/trang trại lớn

6. Chiến lược tích hợp cho phần mềm POS:
   - API kết nối với Verigoods.vn
   - Tự động sinh mã QR khi nhập kho (batch → QR)
   - In tem truy xuất tích hợp vào hóa đơn bán hàng
   - Dashboard: % hàng hóa đã có QR vs chưa có
```

---

## Prompt 7 → File: `07-phan-tich-doi-thu-compliance-features.md`

```
Báo Cáo Phân Tích Cạnh Tranh Chuyên Sâu: Đánh Giá Năng Lực Tuân Thủ Pháp Lý (Compliance Features) Của Các Nền Tảng POS Phổ Biến Tại Việt Nam — KiotViet, Sapo, MISA eShop — Và Khoảng Trống Chiến Lược Cho Nodi POS Trong Phân Khúc Vật Tư Nông Nghiệp (2026)

Trong bối cảnh chiến lược "Compliance-as-Feature" được xác định là USP cốt lõi của Nodi POS, việc phân tích chính xác năng lực compliance thực tế (không phải marketing) của các đối thủ lớn nhất là điều kiện tiên quyết để xác lập vị thế cạnh tranh. Báo cáo cần phân tích dựa trên bằng chứng thực tế: tính năng có trên giao diện, API documentation, changelog, user reviews, và phản hồi từ cộng đồng người dùng.

Yêu cầu nghiên cứu chi tiết:

1. KiotViet — Phân tích Compliance Features:
   - Hóa đơn điện tử: Tích hợp NCC nào? API hay manual? Offline mode?
   - Quản lý thuế: Có hỗ trợ PIT cho HKD? VAT auto hay manual?
   - Quản lý hạn dùng: Có batch tracking? FEFO/FIFO? Cảnh báo?
   - Thuốc cấm: Có danh mục cấm? Auto-lock? 
   - Truy xuất nguồn gốc: QR code? Verigoods.vn?
   - Báo cáo thuế: Có xuất format chuẩn Sở NN&PTNT?
   - Ngành dọc nông nghiệp: Có module chuyên biệt VTNN?
   - Pricing: Bao nhiêu/tháng? Gói nào có compliance?
   - Số lượng merchant: Bao nhiêu đại lý VTNN đang dùng?

2. Sapo — Phân tích Compliance Features:
   - (Cùng danh mục câu hỏi như KiotViet)
   - Đặc biệt: Sapo có module nông nghiệp không? Sapo X vs Sapo POS?
   - Sapo Web + Sapo Go: Hỗ trợ offline tại nông thôn?

3. MISA eShop — Phân tích Compliance Features:
   - (Cùng danh mục câu hỏi)
   - Đặc biệt: MISA meInvoice tích hợp sâu thế nào với eShop?
   - MISA có module VTNN riêng? Kế toán 4 sổ HKD?
   - MISA Accounting vs MISA eShop: Đâu là compliance hub?

4. Các POS nông nghiệp chuyên biệt (nếu có):
   - Có PM POS nào ở VN chuyên cho ngành VTNN ngoài Nodi?
   - Các giải pháp ERP nông nghiệp: Agri Pro, FPT Agri, v.v.?
   - PM quản lý kho thuốc BVTV chuyên dụng (nếu có)?

5. Bảng so sánh 5/5 lớp Compliance Cocktail:
   Lập bảng chi tiết so sánh KiotViet vs Sapo vs MISA vs Nodi POS trên 5 lớp:
   - Lớp 1: Thuế (VAT + PIT)
   - Lớp 2: HĐĐT
   - Lớp 3: Quản lý thuốc cấm
   - Lớp 4: Hạn sử dụng (FEFO)
   - Lớp 5: Truy xuất nguồn gốc (QR)
   Đánh giá: ✅ Có / ⚠️ Cơ bản / ❌ Không có

6. Khoảng trống chiến lược (Strategic Gap):
   - Tính năng nào đối thủ CÓ mà Nodi CHƯA có?
   - Tính năng nào Nodi ĐỘC QUYỀN so với tất cả?
   - Rủi ro: đối thủ có thể copy compliance features trong bao lâu?
   - Moat (hào phòng thủ): Nodi cần xây dựng gì để không bị bắt kịp?
```

---

## Prompt 8 → File: `08-hanh-vi-so-hoa-dai-ly-dbscl.md`

```
Báo Cáo Nghiên Cứu Thị Trường: Hành Vi Số Hóa, Thói Quen Công Nghệ Và Chân Dung Người Dùng (User Persona) Của Chủ Đại Lý Vật Tư Nông Nghiệp Tại Đồng Bằng Sông Cửu Long — Dữ Liệu Thực Tiễn Phục Vụ Chiến Lược Onboarding Và UX Design Cho Nodi POS (2026)

Khảo sát VCCI tháng 6/2025 cho thấy 73% hộ kinh doanh thiếu kỹ năng công nghệ, 68% mơ hồ về pháp lý HĐĐT. Tuy nhiên, để thiết kế trải nghiệm người dùng (UX) và chiến lược onboarding phù hợp cho Nodi POS, cần dữ liệu chi tiết hơn về hành vi số hóa thực tế của nhóm khách hàng mục tiêu: chủ đại lý VTNN 40-60 tuổi tại ĐBSCL.

Yêu cầu nghiên cứu chi tiết:

1. Nhân khẩu học & tâm lý học (Demographics & Psychographics):
   - Phân bổ độ tuổi chủ đại lý VTNN tại ĐBSCL (% theo nhóm tuổi)
   - Trình độ học vấn phổ biến
   - Cấu trúc gia đình trong kinh doanh (vợ chồng, con cái, thế hệ kế thừa)
   - Tâm lý đối với công nghệ: e ngại, tò mò, sẵn sàng?
   - Ngưỡng chấp nhận chi phí công nghệ (bao nhiêu VNĐ/tháng là "chấp nhận được"?)

2. Thiết bị công nghệ đang sử dụng:
   - Tỷ lệ sở hữu smartphone vs feature phone
   - Hệ điều hành phổ biến: Android (Samsung, Oppo, Xiaomi?) hay iPhone?
   - Kích thước màn hình trung bình (quan trọng cho UX mobile)
   - Có máy tính (PC/laptop) tại cửa hàng không? Tỷ lệ?
   - Máy in (A4, in nhiệt bill?) — tỷ lệ sở hữu?
   - Máy quét mã vạch: có ai dùng chưa?
   - Internet: Wifi, 4G, hay còn dùng 3G? Tốc độ trung bình?

3. Thói quen sử dụng ứng dụng di động:
   - Zalo: Tỷ lệ sử dụng? Dùng cho mục đích gì (chat, nhóm KD, Zalo Pay)?
   - Facebook: Cá nhân hay có fanpage bán hàng?
   - TikTok: Có xem không? Có ai bán hàng qua TikTok?
   - YouTube: Xem nội dung gì (tin tức, nông nghiệp, giải trí)?
   - App ngân hàng: Tỷ lệ có app? MB Bank, Vietcombank, Agribank?
   - Thanh toán QR: Tỷ lệ chấp nhận QR pay tại quầy?

4. Thói quen quản lý kinh doanh hiện tại:
   - Ghi chép sổ tay: Bao nhiêu % vẫn dùng sổ tay?
   - Excel: Có ai dùng Excel trên máy tính?
   - Phần mềm POS: % đã dùng PM nào (KiotViet, Sapo, khác)?
   - Lý do KHÔNG dùng PM: giá đắt, khó dùng, không cần, sợ?
   - Ai là người quyết định mua PM: chủ đại lý hay con cái?

5. Kênh tiếp nhận thông tin:
   - Tin tức nông nghiệp: TV (VTV, HTV), báo giấy, Zalo, Facebook?
   - Thông tin pháp lý: Loa phường, cán bộ thuế, người quen, online?
   - Quảng cáo PM: Đã thấy QC PM bán hàng chưa? Ở đâu?
   - Người ảnh hưởng: Ai có sức ảnh hưởng lớn nhất (NCC, HTX, cán bộ xã)?

6. Rào cản chuyển đổi số (Digital Barriers):
   - Thị lực: % gặp khó khăn đọc chữ nhỏ trên điện thoại
   - Ngón tay: Khó thao tác màn hình cảm ứng (ngón to, chai sạn)?
   - Ngôn ngữ: Có cần giao diện thuần Việt không dấu / tiếng địa phương?
   - Thời gian: Bận rộn mùa vụ, lúc nào rảnh để học PM mới?
   - Sợ mất dữ liệu: Lo ngại "máy hư mất hết sổ sách"?

7. Ứng dụng cho UX Design của Nodi POS:
   - Font size tối thiểu cho nhóm 50+ tuổi
   - Contrast ratio cho môi trường sáng (cửa hàng ngoài trời)
   - Touch target size cho ngón tay lao động
   - Onboarding flow: bao nhiêu bước là tối đa?
   - Ngôn ngữ giao diện: thuần Việt, icon trực quan, voice input?
```

---

## 📁 Mapping File Output

| Prompt | File output | Folder |
|:------:|------------|:------:|
| 5 | `05-danh-muc-hoat-chat-cam-tt75-2025.md` | nghien-cuu/ |
| 6 | `06-truy-xuat-nguon-goc-qd25-verigoods.md` | nghien-cuu/ |
| 7 | `07-phan-tich-doi-thu-compliance-features.md` | nghien-cuu/ |
| 8 | `08-hanh-vi-so-hoa-dai-ly-dbscl.md` | nghien-cuu/ |

## Thứ tự ưu tiên chạy

1. 🔴 **Prompt 5** (TT 75) — Cần cho code Auto-lock database
2. 🟡 **Prompt 7** (Đối thủ) — Cần cho competitive positioning
3. 🟡 **Prompt 6** (QR/Verigoods) — Deadline 01/07/2026
4. 🟢 **Prompt 8** (User persona) — Cần cho UX design
