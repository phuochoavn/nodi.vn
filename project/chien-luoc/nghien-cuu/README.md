# 📚 Kho Nghiên Cứu Chuyên Sâu — Nodi POS × Ngành VTNN Việt Nam 2026

> **Tổng số báo cáo**: 8 | **Nguồn**: Google DeepSearch (Gemini)
> **Giai đoạn**: 03/2026 | **Mục đích**: Xây dựng chiến lược "Compliance-as-Feature"

---

## 📋 Danh Mục Nghiên Cứu

### Đợt 1 — Thuế, HĐĐT & Pháp Lý (Prompts 1-4)

| # | Báo cáo | Nội dung chính | Ứng dụng Nodi |
|:-:|---------|---------------|--------------|
| 01 | [Phân tích thị trường VTNN](01-PHAN_TICH_THI_TRUONG_VTNN_2026.md) | Quy mô thị trường, chuỗi cung ứng, xu hướng 2026 | Định vị sản phẩm |
| 02 | [Thuế HKD VTNN ĐBSCL](02-thue-ho-kinh-doanh-vtnn-2026.md) | Xóa thuế khoán → kê khai, ngưỡng 200M, VAT 5% phân bón, sổ 4 tài khoản | Module tính thuế tự động |
| 03 | [Hóa đơn điện tử VTNN](03-hoa-don-dien-tu-vtnn-2026.md) | NĐ 123/2020, HĐĐT từ máy tính tiền, xử lý sai sót, phân loại HĐ | Module HĐĐT |
| 04 | [Thanh tra & rủi ro pháp lý](04-thanh-tra-rui-ro-phap-ly-vtnn.md) | 4 cơ quan thanh tra, khung phạt, Big Data quản lý thị trường | Compliance Shield |

### Đợt 2 — Hoạt Chất Cấm, Truy Xuất, Đối Thủ & UX (Prompts 5-8)

| # | Báo cáo | Nội dung chính | Ứng dụng Nodi |
|:-:|---------|---------------|--------------|
| 05 | [Danh mục hoạt chất cấm TT75](05-danh-muc-hoat-chat-cam-tt75-2025.md) | 31 chất Phụ lục II, Carbosulfan/Benfuracarb countdown, DB schema, UX 3 trạng thái | **Auto-Lock Database** |
| 06 | [Truy xuất nguồn gốc QĐ25](06-truy-xuat-nguon-goc-qd25-verigoods.md) | Verigoods.vn, GS1 Digital Link, QR, nhật ký canh tác | Module TXNG |
| 07 | [Phân tích đối thủ compliance](07-phan-tich-doi-thu-compliance-features.md) | KiotViet 0/5, Sapo 2/5, MISA 2/5 → Nodi POS = DUY NHẤT 5/5 | Chiến lược cạnh tranh |
| 08 | [Hành vi số hóa đại lý ĐBSCL](08-hanh-vi-so-hoa-dai-ly-dbscl.md) | 73% thiếu IT, Android <5M chiếm 70%, Zalo=OS, ngưỡng 199K/tháng | **UX Design & Onboarding** |

### Phụ lục — Prompts gốc

| File | Mô tả |
|------|-------|
| [PROMPTS_THUE_HDDT.md](PROMPTS_THUE_HDDT.md) | 4 prompts DeepSearch đợt 1 (Thuế, HĐĐT, Compliance, Thanh tra) |
| [PROMPTS_BO_SUNG.md](PROMPTS_BO_SUNG.md) | 4 prompts DeepSearch đợt 2 (Hoạt chất cấm, TXNG, Đối thủ, UX) |

---

## 🎯 Findings Chiến Lược — Top 10

| # | Phát hiện | Nguồn | Tác động |
|:-:|----------|:-----:|---------|
| 1 | KiotViet/Sapo/MISA **0 tính năng** kiểm soát thuốc cấm | #07 | **Moat độc quyền Nodi** |
| 2 | **31+2 hoạt chất cấm**, Benfuracarb hết hạn 10/02/2027 | #05 | Auto-Lock P0 |
| 3 | **73% HKD** thiếu kỹ năng công nghệ | #08 | UX đặc biệt cho 40-60 tuổi |
| 4 | Phạt tối đa **100M VNĐ + tước giấy phép** | #04, #05 | Fear-based marketing |
| 5 | Android <5M VNĐ chiếm **70%** thị trường | #08 | Tối ưu RAM 3-4GB |
| 6 | Zalo **100%** sử dụng trong kinh doanh | #08 | UI tương tự Zalo |
| 7 | HĐĐT bắt buộc từ **01/06/2025** (doanh thu >1 tỷ) | #03 | Pain point #1 |
| 8 | Ngưỡng giá chấp nhận **150-250K/tháng** | #08 | Freemium → 199K |
| 9 | MISA có thể copy trong **6-9 tháng** | #07 | Cửa sổ vàng 12-18 tháng |
| 10 | Verigoods.vn deadline **01/07/2026** | #06 | Tích hợp TXNG Q2/2026 |

---

## 🏗️ Chuyển Hóa Nghiên Cứu → Code

```
Nghiên cứu #05 (Hoạt chất cấm)
  └── src/services/BanCheckService.ts        ← Auto-Lock logic
  └── src-tauri/src/db/banned_ingredients.rs  ← Rust backend
  └── migrations/xxx_banned_substances.sql    ← Seed data 31+2 chất

Nghiên cứu #02-03 (Thuế + HĐĐT)
  └── src/services/tax/                      ← Module thuế tự động
  └── src/services/einvoice/                 ← E-invoice integration

Nghiên cứu #06 (Truy xuất)
  └── src/services/QRCodeService.ts          ← GS1 Digital Link
  └── Tích hợp Verigoods.vn API             ← Roadmap Q2/2026

Nghiên cứu #08 (UX)
  └── Simple Mode toggle                    ← Đã implement
  └── Touch target audit ≥48px              ← Roadmap
  └── Voice-to-text search                  ← Roadmap
```

---

## 📅 Lộ Trình Áp Dụng

| Thời gian | Hành động | Nguồn |
|:---------:|----------|:-----:|
| **Ngay** | Seed data 31+2 hoạt chất → Auto-Lock DB | #05 |
| **Sprint hiện tại** | Countdown timer Benfuracarb/Carbosulfan | #05 |
| **Q2/2026** | Tích hợp Verigoods.vn API | #06 |
| **Q2/2026** | Audit UX touch target ≥48px | #08 |
| **Q3/2026** | Smart cross-selling thuốc sinh học | #05 |
| **Q3/2026** | Voice-to-text tìm kiếm sản phẩm | #08 |
| **Q4/2026** | Onboarding tương tác 3 bước | #08 |

---

*Cập nhật lần cuối: 13/03/2026 — 8/8 nghiên cứu hoàn tất*
