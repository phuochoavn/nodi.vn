# 🔬 Prompts Google DeepSearch — Nghiên Cứu Mô Hình Kinh Doanh Nodi POS

> **Ngày tạo**: 13/03/2026
> **Mục đích**: Thu thập tài liệu nghiên cứu chuyên sâu trước khi GO LIVE
> **Cách dùng**: Copy từng prompt vào Google DeepSearch (Gemini) → lưu kết quả vào file tương ứng

---

## ĐÃ CÓ (4 file)

| # | File | Chủ đề |
|---|------|--------|
| 01 | `01-dinh-gia.md` | Freemium & Pricing |
| 02 | `02-da-dang-hoa-nguon-thu.md` | Revenue Diversification |
| 03 | `03-tang-truong-dan-dat-san-pham.md` | Product-Led Growth |
| 04 | `04-mo-hinh-agritech-toan-cau.md` | AgriTech Case Studies |

---

## CẦN NGHIÊN CỨU THÊM (6 prompts)

---

### 📌 Prompt 5: Chiến lược Go-to-Market cho B2B SaaS thị trường nông thôn
**Lưu vào**: `05-go-to-market-nong-thon.md`

```
Tôi đang xây dựng một phần mềm POS (Point of Sale) SaaS cho đại lý vật tư nông nghiệp tại Việt Nam. Sản phẩm là ứng dụng desktop + mobile offline-first, giá Freemium (Dùng thử 30 ngày → Free 20 đơn/ngày → 299K/tháng hoặc 1.990K/năm). Đối tượng khách hàng: chủ đại lý thuốc bảo vệ thực vật, phân bón, giống cây trồng ở nông thôn — thường 40-60 tuổi, ít quen công nghệ, quen dùng sổ tay giấy.

Hãy nghiên cứu chuyên sâu về chiến lược Go-to-Market (GTM) cho B2B SaaS nhắm vào tiểu thương/SME ở thị trường nông thôn Đông Nam Á và Việt Nam. Cụ thể:

1. **Case study GTM thành công tại nông thôn ĐNA**: Sổ Bán Hàng, KiotViet, Grab Merchant, GoBiz (Gojek), eFishery (trước scandal), M-Pesa (Kenya), Jio (Ấn Độ) — họ tìm 1,000 khách đầu tiên như thế nào? Kênh nào? Chi phí bao nhiêu? Timeline bao lâu?

2. **Chiến lược "100 khách đầu tiên" cho solopreneur**: Khi KHÔNG có sales team, KHÔNG có budget quảng cáo lớn — làm sao onboard 100 đại lý đầu tiên? Đi field sales 1 mình? Partner với hợp tác xã? Nhờ nhà cung cấp phân bón giới thiệu? Zalo group đại lý?

3. **Channel strategy cho nông thôn VN**: Phân tích hiệu quả từng kênh: Zalo OA, Facebook Group đại lý VTNN, TikTok nông nghiệp, YouTube hướng dẫn, đi trực tiếp cửa hàng, hội chợ nông nghiệp, hợp tác xã, chuỗi phân phối (Lộc Trời, Bình Điền, Đạm Cà Mau). Chi phí vs hiệu quả?

4. **Land-and-expand trong B2B nông nghiệp**: Bắt đầu từ 1 tỉnh (vùng trọng điểm nông nghiệp) → lan ra cả nước? Hay rải đều từ đầu? Case study: DeHaat (Ấn Độ) bắt đầu từ Bihar, AgriAku từ Java — so sánh chiến lược.

5. **Timing & seasonality**: Thời điểm nào trong năm tốt nhất để launch POS cho đại lý VTNN? Đầu vụ (khi mua nhiều) hay giữa vụ (khi rảnh)? Lịch vụ mùa VN ảnh hưởng thế nào đến adoption?

6. **Metrics & milestones cho Year 1**: Benchmark cho B2B SaaS nông thôn — tháng đầu nên có bao nhiêu user? 3 tháng? 6 tháng? Conversion rate từ Free → Paid kỳ vọng bao nhiêu? NPS target?

Cho tôi data thực tế, số liệu cụ thể, case study có nguồn trích dẫn. Tránh lý thuyết chung chung.
```

---

### 📌 Prompt 6: Chiến lược giữ chân khách hàng & chống Churn cho tiểu thương
**Lưu vào**: `06-chong-churn-tieu-thuong.md`

```
Tôi đang xây dựng phần mềm POS SaaS cho đại lý vật tư nông nghiệp (VTNN) tại Việt Nam. Giá: Freemium → 299K/tháng hoặc 1.990K/năm. Đặc thù ngành: kinh doanh có mùa vụ rõ ràng (3-4 tháng bận rộn, 2-3 tháng nhàn rỗi), chủ đại lý lớn tuổi, có thể quên dùng app khi không bận.

Hãy nghiên cứu chuyên sâu về Customer Retention và Churn Prevention cho B2B SaaS micro-SME tại thị trường mới nổi:

1. **Nguyên nhân churn của tiểu thương**: Nghiên cứu tại sao tiểu thương ĐNA bỏ phần mềm POS/kế toán — data từ KiotViet churn, Sổ Bán Hàng, Moka (Indonesia), Grab Merchant. Top 5 lý do phổ biến nhất? "Mùa nhàn rỗi" ảnh hưởng thế nào?

2. **Cohort analysis cho seasonal business**: Cách tính churn cho business có mùa vụ? "Ngủ đông" (dormant) vs "churn thật"? Cách phân biệt? Metrics phù hợp (MAU vs WAU vs "monthly active invoices")?

3. **Retention tactics cho low-tech users**: 
   - Push notification vs SMS vs Zalo OA — cái nào hiệu quả nhất với đại lý 50 tuổi?
   - Re-engagement campaign: khi user không mở app 14 ngày → làm gì?
   - "Giá trị thụ động": app TỰ ĐỘNG tạo giá trị mà user không cần mở (nhắc nợ, cảnh báo hết hạn thuốc, báo cáo tháng gửi qua Zalo)
   - Gamification có phù hợp cho tiểu thương lớn tuổi không? (leaderboard, streak, badges)

4. **Lock-in strategy (chốt chặn)**: Data lock-in — khi đã nhập 6 tháng data nợ/khách hàng/kho hàng, chi phí chuyển sang app khác = bao nhiêu? So sánh: Excel export mitigation vs hard lock-in.

5. **Lifecycle marketing cho POS SaaS**:
   - Ngày 1 → Ngày 7 → Ngày 30 → Ngày 90: mỗi mốc nên làm gì?
   - "Health Score" cho từng cửa hàng (tần suất dùng, số đơn, số KH) → trigger can thiệp khi nào?
   - Winback campaign: user đã churn 60 ngày → chiến lược nào hiệu quả?

6. **Case study retention thành công**:
   - Shopify: 70%+ retention rate cho merchant nhỏ — bí quyết?
   - Square: churn < 10% cho micro-merchant — vì sao?
   - MISA: 250K doanh nghiệp VN — retention strategy cụ thể?
   - Toast: 110% net revenue retention — expansion revenue model?

7. **Community-driven retention**: Zalo group "Hội Đại Lý VTNN Dùng Nodi" — peer support thay customer success team. Case study: Notion Ambassador, Figma Community, Canva Creators.

Cho tôi số liệu benchmark, framework cụ thể, playbook có thể thực thi bởi 1 người (solopreneur).
```

---

### 📌 Prompt 7: Unit Economics & Financial Modeling cho Micro-SaaS nông nghiệp
**Lưu vào**: `07-unit-economics-micro-saas.md`

```
Tôi đang xây dựng phần mềm POS SaaS cho đại lý vật tư nông nghiệp tại Việt Nam. Mô hình hiện tại:
- Stack: Tauri (Rust) desktop + mobile app, offline-first, cloud sync
- Server: VPS $10/tháng (2 CPU, 8GB RAM, 100GB SSD) — chạy Rust Axum + PostgreSQL + Nginx
- Giá: Free (20 đơn/ngày) → 99,000đ/tháng → 199,000đ/tháng  
- Team: 1 người (founder/developer)
- Chi phí cố định hàng tháng hiện tại: ~$10 VPS + domain

Hãy nghiên cứu chuyên sâu về Unit Economics và Financial Modeling cho Micro-SaaS (bootstrap, 1 người) nhắm vào thị trường nông thôn mới nổi:

1. **Server economics cho SaaS offline-first**: 
   - 1 VPS $10/tháng chịu được bao nhiêu concurrent users? (Rust Axum rất nhẹ, sync mỗi 5 phút)
   - Khi nào cần nâng cấp? 100 user? 500? 1000? 5000?
   - So sánh: VPS cố định vs Serverless (AWS Lambda) vs Managed DB (Supabase) cho micro-SaaS
   - Bandwidth cost cho sync payload trung bình 50KB/lần × 288 lần/ngày/user?

2. **Unit economics benchmark cho micro-SaaS ĐNA**:
   - ARPU thực tế (sau discount, churn) cho SaaS giá $4-8/tháng tại VN/Indonesia/Philippines
   - CAC cho B2B micro-merchant khi dùng PLG (0đ ads) vs paid ads vs field sales
   - Payback period thực tế
   - Gross margin cho SaaS offline-first (gần 100% vì chi phí biến đổi cực thấp?)

3. **Financial model cho 3 năm**: Giả định:
   - Năm 1: 500 user Free, 50 user Paid (10% conversion)
   - Năm 2: 2000 user Free, 300 user Paid (15% conversion)
   - Năm 3: 5000 user Free, 1000 user Paid (20% conversion)
   - Churn: 5%/tháng Year 1, 4% Year 2, 3% Year 3
   - Tính: MRR, ARR, revenue, profit, break-even point

4. **Bootstrap vs Fundraise**: 
   - Ở quy mô nào thì cần gọi vốn? 1000 user? 5000? 
   - Pre-seed/Seed cho B2B SaaS nông nghiệp VN — mức định giá kỳ vọng? Metrics VC muốn thấy?
   - Case study: Sổ Bán Hàng ($4M vốn), Sapo (nhiều vòng), KiotViet (bootstrapped lâu) — timeline nào?

5. **Cost structure khi scale**: 
   - Support cost per user (khi chỉ có 1 người)
   - Infrastructure cost curve (log scale hay linear?)
   - Tại milestone nào CẦN thuê người đầu tiên? (support? sales? dev?)

6. **Pricing experiment framework**: A/B test giá cho tiểu thương — methodology? Willingness-to-pay survey cho người lớn tuổi ít quen tech? Van Westendorp Price Sensitivity? Conjoint analysis đơn giản?

Cho tôi bảng tính mẫu (spreadsheet format), số liệu benchmark thực tế, và framework ra quyết định.
```

---

### 📌 Prompt 8: Hệ thống thanh toán & billing cho SaaS nông thôn Việt Nam
**Lưu vào**: `08-thanh-toan-billing-nong-thon.md`

```
Tôi đang xây dựng phần mềm POS SaaS cho đại lý vật tư nông nghiệp tại Việt Nam, giá 299K/tháng hoặc 1.990K/năm. Khách hàng: tiểu thương ở nông thôn, 40-60 tuổi, ít dùng thẻ tín dụng, quen chuyển khoản và tiền mặt.

Hãy nghiên cứu chuyên sâu về payment collection và billing system cho SaaS nhắm vào tiểu thương nông thôn Việt Nam:

1. **Payment gateway cho micro-subscription VN ($4-8/tháng)**:
   - So sánh chi tiết: VNPay, MoMo Business, ZaloPay Merchant, Stripe (VN), PayOS, Casso — phí giao dịch, phí cố định, hỗ trợ recurring, minimum payout, thời gian setup
   - Auto-recurring (tự trừ tiền hàng tháng) có khả thi ở VN không? Tỷ lệ chấp nhận? Legal requirements?
   - VietQR manual transfer + webhook tự reconcile (model Casso) — pros/cons vs payment gateway?

2. **Billing psychology cho tiểu thương**:
   - Thanh toán tháng vs quý vs năm — tỷ lệ chọn? Data từ KiotViet, MISA, Sapo
   - "Mua 1 năm tặng 3 tháng" vs "Giảm 25% khi mua năm" — framing nào hiệu quả hơn?
   - Grace period bao lâu trước khi khóa tính năng? 3 ngày? 7 ngày? 30 ngày?
   - Dunning strategy (nhắc thanh toán): bao nhiêu lần? Qua kênh nào? Tone of voice?

3. **Collection challenge ở nông thôn**:
   - Tỷ lệ involuntary churn (thẻ hết hạn, quên gia hạn) ở VN?
   - Xử lý khi user không trả tiền: khóa mềm (chỉ view, không tạo đơn mới) vs khóa cứng (block hoàn toàn) vs downgrade về Free?
   - Thu tiền qua đại lý/đối tác địa phương (agent model) — có hiệu quả không?

4. **Case study billing cho micro-merchant ĐNA**:
   - M-Pesa (Kenya): thu phí SaaS qua mobile money — tỷ lệ thu thành công?
   - GoBiz/GrabMerchant: billing model cho small merchant Indonesia/VN
   - Sổ Bán Hàng: họ thu tiền thế nào? (app store IAP? direct? agent?)

5. **In-App Purchase vs Direct billing**:
   - Google Play / App Store IAP (30% commission) vs direct billing (VietQR/MoMo)
   - Legal risk khi bypass app store? Apple/Google enforcement tại VN?
   - Tauri desktop app có cần qua store không? → direct billing = 0% commission?

6. **Accounting & tax cho SaaS revenue VN**:
   - Bán phần mềm online có phải xuất HĐĐT cho từng giao dịch 299K không?
   - Thuế GTGT cho SaaS: 0% (phần mềm) hay 10% (dịch vụ)? Nghị định nào?
   - Cần đăng ký kinh doanh gì để bán SaaS hợp pháp? Hộ kinh doanh đủ hay cần công ty?

Cho tôi so sánh chi tiết dạng bảng, workflow billing cụ thể, và playbook thu tiền thực tế cho 1-person team.
```

---

### 📌 Prompt 9: Phân tích rủi ro & kịch bản xấu cho SaaS startup nông nghiệp
**Lưu vào**: `09-phan-tich-rui-ro.md`

```
Tôi đang xây dựng phần mềm POS SaaS cho đại lý vật tư nông nghiệp tại Việt Nam, hoạt động như solopreneur (1 người làm tất cả: dev, support, marketing). Sản phẩm gần hoàn chỉnh (113 sprints, 85K+ LOC), chuẩn bị GO LIVE.

Hãy phân tích chuyên sâu các rủi ro và kịch bản xấu cho micro-SaaS startup nông nghiệp tại VN:

1. **Rủi ro vận hành (solopreneur)**:
   - "Bus factor = 1": nếu founder ốm/bận 2 tuần → app crash → mất toàn bộ KH. Mitigation?
   - Support overload: khi đạt 200+ user, 1 người support nổi không? Tính giờ support/user/tháng?
   - Burnout timeline: solopreneur SaaS trung bình burn out sau bao lâu? Prevention strategies?

2. **Rủi ro kỹ thuật**:
   - VPS sập (single point of failure) → offline app vẫn chạy, nhưng sync/backup mất → hậu quả?
   - Data corruption: SQLite-to-PostgreSQL sync conflict → mất data khách hàng → trách nhiệm?
   - Security breach: leak data công nợ khách hàng → legal liability tại VN?
   - Tauri framework risk: nếu Tauri ngừng phát triển hoặc breaking change lớn?

3. **Rủi ro thị trường**:
   - KiotViet/Sapo launch "Gói Nông Nghiệp" cạnh tranh trực tiếp → response strategy?
   - Sổ Bán Hàng thêm tính năng công nợ vụ mùa → mất USP?
   - Thị trường đại lý VTNN co hẹp (consolidation, đại lý nhỏ bị thay thế bởi chuỗi lớn)?
   - Chính sách nông nghiệp thay đổi (cấm thêm thuốc BVTV, siết quản lý → đại lý giảm)?

4. **Rủi ro pháp lý**:
   - AI chatbot gợi ý thuốc BVTV sai → nông dân dùng → thiệt hại mùa vụ → kiện? Trách nhiệm pháp lý?
   - Disclaimer cần có: "Chỉ mang tính tham khảo" có đủ bảo vệ không?
   - GDPR/PDPA Việt Nam (Nghị định 13/2023): thu thập data bán hàng đại lý có cần consent không? Data thuốc BVTV = sensitive?
   - Nếu bán Credit Score cho ngân hàng (tương lai) → quy định dữ liệu tín dụng?

5. **Rủi ro tài chính**:
   - Cash runway: với $10/tháng server + 0 revenue → bao lâu chịu được?
   - Revenue lag: từ khi GO LIVE → đến khi có $100/tháng revenue mất bao lâu (benchmark)?
   - Pricing: 299K/tháng có đủ cover opportunity cost (thời gian founder)?

6. **Contingency plan cho từng rủi ro**:
   - Risk matrix: Probability × Impact cho mỗi rủi ro
   - Top 5 rủi ro cao nhất → action plan cụ thể
   - "Kill criteria": nếu sau X tháng vẫn < Y user → pivot hay bỏ?

7. **Case study thất bại SaaS nông nghiệp**: 
   - AgroStar (Ấn Độ): layoff 40% — nguyên nhân?
   - Crofarm → layoff, gộp với Otipy — bài học?
   - AppHarvest: IPO → phá sản — vertical farming trap
   - Bài học chung từ startup VN thất bại: Base.vn, Bework, Teamcrop

Cho tôi risk matrix dạng bảng, probability scoring, và playbook ứng phó cụ thể.
```

---

### 📌 Prompt 10: Chiến lược Partnership & Hệ sinh thái cho POS nông nghiệp
**Lưu vào**: `10-partnership-he-sinh-thai.md`

```
Tôi đang xây dựng phần mềm POS SaaS cho đại lý vật tư nông nghiệp (VTNN) tại Việt Nam. Sản phẩm offline-first, có AI chẩn đoán bệnh cây trồng, quản lý công nợ vụ mùa, truy xuất nguồn gốc. Tôi là solopreneur, cần tận dụng partnership thay vì tự xây mọi thứ.

Hãy nghiên cứu chuyên sâu về Partnership và Ecosystem Strategy cho B2B SaaS nông nghiệp:

1. **Partnership với nhà sản xuất VTNN tại VN**:
   - Lộc Trời Group (HSX: LTG), Bình Điền (BFC), Đạm Cà Mau (DCM), Syngenta VN, Bayer VN, Corteva, BASF — mô hình hợp tác nào có lợi cho cả hai?
   - "POS đại lý Lộc Trời" white-label vs "Nodi POS + catalog Lộc Trời" partnership — ưu nhược?
   - Revenue share: họ trả phí để đưa SP lên catalog POS (advertising model)?
   - Case study: FBN + Bayer/Corteva, ProAgrica + manufacturers — mô hình partnership cụ thể?

2. **Partnership với hệ thống phân phối**:
   - Chuỗi đại lý cấp 1 (nhà phân phối lớn) → giới thiệu Nodi cho đại lý cấp 2/3 → commission?
   - "Đại lý cấp 1 TẶNG Nodi Pro cho đại lý con mua trên 500M/năm" → win-win?
   - Data sharing: nhà phân phối muốn biết sell-through rate ở đại lý con → Nodi cung cấp dashboard?

3. **Partnership fintech/banking**:
   - Agribank (cho vay nông nghiệp), BIDV, Vietcombank, HDBank — mô hình "POS = kênh phân phối tín dụng"?
   - MoMo Business, ZaloPay — tích hợp thanh toán + lending?
   - Case study: Sổ Bán Hàng + UOB, MFast + banks, Toast + Toast Capital — partnership structure?
   - Revenue: referral fee bao nhiêu %? Per-lead hay per-conversion?

4. **Partnership với hệ thống nông nghiệp nhà nước**:
   - Hợp tác xã nông nghiệp (HTXNN) — có 18,000+ HTX tại VN → kênh phân phối?
   - Sở Nông nghiệp tỉnh → chương trình "chuyển đổi số nông nghiệp" → Nodi = giải pháp?
   - Trung tâm Khuyến nông → kênh tiếp cận nông dân gián tiếp?
   - Trường Đại học Nông Lâm → sinh viên thực tập = support team miễn phí?

5. **Partnership technology**:
   - IoT: Enfarm (bón phân thông minh), MimosaTEK (tưới tiêu) → POS hiện AI recommendation + bán sensor?
   - Traceverified (truy xuất blockchain) → Nodi QR code = front-end cho truy xuất nguồn gốc?
   - Máy in hóa đơn: Sunmi, Xprinter → bundle deal cho đại lý?

6. **API Economy & Ecosystem play**:
   - Open API cho bên thứ 3 tích hợp → mô hình Shopify App Store thu 20% commission?
   - Data API cho nghiên cứu thị trường (ẩn danh) → bán cho tập đoàn BVTV?

7. **Partnership roadmap theo giai đoạn**:
   - 0-500 user: partner nào ưu tiên đầu tiên?
   - 500-2000 user: mở rộng hệ sinh thái?
   - 2000+ user: platform play?

Cho tôi framework đánh giá partnership (effort × value), template MOU/proposal, và playbook tiếp cận từng loại đối tác.
```

---

## HƯỚNG DẪN SỬ DỤNG

1. **Copy prompt** vào Google DeepSearch (Gemini với search grounding)
2. **Review kết quả** — bổ sung context nếu cần
3. **Lưu output** vào file tương ứng trong `project/chien-luoc/mo-hinh-kinh-doanh/`
4. **Đánh dấu** đã hoàn thành ở bảng trên

### Thứ tự ưu tiên nghiên cứu

| Ưu tiên | Prompt | Lý do |
|:-------:|--------|-------|
| 🔴 #1 | **05 — Go-to-Market** | Trả lời "100 khách đầu tiên ở đâu?" — câu hỏi sống còn |
| 🔴 #2 | **08 — Thanh toán billing** | Phải biết THU TIỀN thế nào trước khi launch |
| 🟡 #3 | **07 — Unit Economics** | Tính toán break-even, biết có sustainable không |
| 🟡 #4 | **06 — Chống Churn** | Cần sẵn sàng retention strategy TRƯỚC khi có user |
| 🟢 #5 | **09 — Rủi ro** | Risk matrix để chuẩn bị contingency |
| 🟢 #6 | **10 — Partnership** | Tận dụng leverage, không tự làm mọi thứ |

