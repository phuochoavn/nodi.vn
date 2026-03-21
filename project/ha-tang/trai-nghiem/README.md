# 🧑‍💼 Trải Nghiệm Người Dùng — UX Research

> User journey, pain points, flow optimization, feedback thực tế

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ 8 Nghiên Cứu

### 🔴 Ưu tiên Cao nhất (Ảnh hưởng trực tiếp đến adoption)

| # | Việc cần làm | Nguồn NC | Effort |
|:-:|-------------|:--------:|:------:|
| 1 | **"Nhắc nợ một chạm" qua Zalo ZNS** — Hook Feature sát thủ. POS "đóng vai ác" đòi nợ tự động → đại lý thấy tiền quay về nhanh → trung thành | #07 | Lớn |
| 2 | **Font ≥ 16px, Touch target ≥ 48dp, Spacing ≥ 8dp** — WCAG 2.2 bắt buộc cho người 45-65 tuổi tay chai sần | #06 | Nhỏ |
| 3 | **Icon LUÔN kèm label text** — Nông dân không hiểu hamburger menu, biểu tượng đĩa mềm | #06 | Nhỏ |
| 4 | **Transaction Batching** — Gom INSERT vào 1 BEGIN...COMMIT thay vì commit từng dòng | #08 | Nhỏ |
| 5 | **`busy_timeout = 5000`** — Chống SQLITE_BUSY khi peak load | #08 | Nhỏ |

### 🟡 Ưu tiên Trung bình (Nâng cấp UX đáng kể)

| # | Việc cần làm | Nguồn NC | Effort |
|:-:|-------------|:--------:|:------:|
| 6 | **Simplified Mode (RBAC)** — Thu ngân chỉ thấy: Tìm SP → SL → Thanh toán. Ẩn hết báo cáo, cài đặt | #08 | TB |
| 7 | **Training Mode** — Feature Flag → training.db + tắt Sync → thực hành 0 rủi ro | #08 | TB |
| 8 | **Autocomplete 5 quy tắc vàng** — Gợi ý ngay focus, in đậm match, ghost text, max 7 kết quả, phím mũi tên | #08 | TB |
| 9 | **Hiển thị trạng thái "Đã lưu"** cố định — Icon đám mây + dấu tích xanh, KHÔNG dùng toast biến mất | #06 | Nhỏ |
| 10 | **Nút "Hoàn tác" to, rõ, 15 giây** — Thay thế Confirm/Cancel dialog gây rối trí | #06 | TB |
| 11 | **Intent-Based mutations cho tồn kho** — Gửi delta `{decrement: 10}` thay vì số tuyệt đối | #08 | TB |
| 12 | **ESC/POS raw printing** — Rust Worker gửi lệnh trực tiếp máy in, bỏ window.print() | #08 | TB |

### 🟢 Ưu tiên Dài hạn (Chiến lược tăng trưởng)

| # | Việc cần làm | Nguồn NC | Effort |
|:-:|-------------|:--------:|:------:|
| 13 | **Zalo Mini App White-label** — Nông dân quét QR → xem nợ, đặt hàng, nhận hóa đơn | #07 | Lớn |
| 14 | **Local Hub qua LAN** — Máy quầy ↔ tablet kho khi mất internet (như Toast POS) | #08 | Lớn |
| 15 | **Voice Input + AI phương ngữ** — Nút microphone khổng lồ, nói "2 chai thuốc rầy xanh" | #06 | Lớn |
| 16 | **Mô phỏng sổ cái (Skeuomorphism)** — Giao diện dòng kẻ ngang, nền ngà, giống sổ tay | #06 | TB |
| 17 | **QR trên bao bì → O2O loop** — Offline mua → quét QR nhận HĐ → online đặt thêm | #07 | Lớn |

---

### 💎 5 Nguyên tắc Vàng (xuyên suốt mọi thiết kế)

1. **"Sổ tay thông minh, không phải ERP"** — Phần mềm phải hạ mình xuống, không ép người dùng lên _(NC #06)_
2. **"Zalo = Frontend, Nodi = Backend SSOT"** — Không bao giờ lưu dữ liệu trên Zalo Cloud _(NC #07)_
3. **"Offline-First = đặc quyền, không phải dự phòng"** — Mọi thao tác → SQLite local trước, cloud = background _(NC #08)_
4. **"Simplicity over Features"** — MoMo Merchant chỉ 2 chức năng → 33 triệu người dùng _(NC #06)_
5. **"Mùa vụ quyết định sống còn"** — 200-300 HĐ/ngày × 10x = phải chạy mượt 100% _(NC #01, #08)_

---

## Nghiên Cứu Đã Hoàn Thành (Google DeepSearch — 18/03/2026)

### Đợt 1: Nền tảng kiến thức ngành (Prompt tiêu chuẩn)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-quy-trinh-van-hanh-dai-ly-vtnn.md` | Quy trình vận hành hàng ngày đại lý VTNN | ~65 |
| 02 | `nghien-cuu-02-tin-dung-thuong-mai-cong-no-vtnn.md` | Tín dụng thương mại & quản trị công nợ chuỗi cung ứng | ~65 |
| 03 | `nghien-cuu-03-giao-dich-mua-chiu-quan-tri-rui-ro.md` | Giao dịch mua chịu & quản trị rủi ro tín dụng | ~65 |
| 04 | `nghien-cuu-04-thoi-quen-cong-nghe-chuyen-doi-so.md` | Thói quen công nghệ & chuyển đổi số đại lý VTNN | ~65 |
| 05 | `nghien-cuu-05-thi-truong-pos-vtnn-2024-2025.md` | Thị trường phần mềm POS cho VTNN 2024-2025 | ~65 |

### Đợt 2: Nghiên cứu chuyên sâu (Prompt cải tiến — 4 trụ cột)

> Áp dụng phương pháp từ `.agent/nghien-cuu-deepsearch-prompting.md`

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 06 | `nghien-cuu-06-ux-nguoi-dung-nong-thon-trung-nien.md` | UX cho người dùng trung niên nông thôn (WCAG 2.2, case study) | **90** |
| 07 | `nghien-cuu-07-zalo-ecosystem-vtnn.md` | Hệ sinh thái Zalo (OA/ZNS/Mini App) tích hợp POS | **92** |
| 08 | `nghien-cuu-08-peak-load-offline-first-pos.md` | Peak Load mùa vụ & kiến trúc Offline-First (SQLite WAL, Sync) | **95** |

---

## Tài Liệu Cần Viết (tổng hợp từ NC → thiết kế sản phẩm)

| File | Nội dung | Nguồn NC | Trạng thái |
|------|----------|:--------:|:----------:|
| `user-personas.md` | Chân dung 3 personas (Chủ, NV bán, NV kho) | 01, 04, 06 | ⬜ |
| `user-journey-ban-hang.md` | Flow bán hàng tối ưu (keyboard-first, autocomplete) | 01, 06, 08 | ⬜ |
| `user-journey-nhap-hang.md` | Flow nhập hàng (PO → nhận hàng → kiểm lô) | 01 | ⬜ |
| `user-journey-cong-no.md` | Flow công nợ (bán nợ → ZNS nhắc nợ → thu nợ) | 02, 03, 07 | ⬜ |
| `pain-points.md` | Tổng hợp pain points từ 8 NC | 01-08 | ⬜ |
| `feedback-thuc-te.md` | Feedback từ user test thực tế | Thực địa | ⬜ |

## User Personas Chính

1. **Chủ cửa hàng VTNN (45-65 tuổi)** — Xem báo cáo, quản lý nợ, ra quyết định. Tay chai sần, ngại công nghệ, cần sổ tay thông minh _(NC #06)_
2. **Nhân viên bán hàng (thời vụ)** — Bán nhanh, tra SP, tạo hóa đơn. Cần sẵn sàng trong 5 phút _(NC #08)_
3. **Nhân viên kho** — Nhập hàng, kiểm kê. Dùng mobile ngoài kho bãi, cần offline + touch lớn _(NC #06, #08)_
