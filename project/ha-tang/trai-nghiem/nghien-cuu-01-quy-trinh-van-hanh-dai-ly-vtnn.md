# 📚 Nghiên cứu 01: Quy trình vận hành đại lý vật tư nông nghiệp Việt Nam

> **Nguồn**: Google DeepSearch (Prompt 1)
> **Ngày**: 2026-03-18
> **Liên quan**: Toàn bộ luồng bán hàng, kho, công nợ trong Nodi POS

## Tóm tắt

Báo cáo phân tích toàn diện quy trình vận hành hàng ngày của đại lý VTNN — từ mở cửa kiểm tra an toàn hóa chất, chẩn đoán bệnh cây, tư vấn phối trộn thuốc, quản lý đơn vị quy đổi, đến công nợ gối đầu theo mùa vụ. Đại lý VTNN **hoàn toàn khác** bán lẻ FMCG thông thường.

---

## 1. Quy trình ngày làm việc

```
Mở cửa (5-6h sáng)
  → Thông gió, kiểm tra rò rỉ hóa chất
  → Sắp xếp kệ theo mùa vụ (đầu vụ: diệt ốc, trừ cỏ; giữa vụ: phân bón lá; cuối vụ: trị đạo ôn)
  → Rà soát tồn kho, chuẩn bị tiền mặt

Cao điểm (8h-16h)
  → Nông dân mang mẫu bệnh → Chẩn đoán → "Kê đơn" thuốc
  → Tư vấn phối trộn (tank-mixing): SC → OD → EC → SL
  → Chốt đơn, quy đổi đơn vị (thùng → chai), ghi nợ
  → Xuất hóa đơn điện tử (HĐĐT)

Cuối ngày (17h-19h)
  → Chốt sổ doanh thu, đối soát tiền mặt
  → Cập nhật công nợ, kiểm tra kho
  → Liên hệ nhà cung cấp đặt hàng bổ sung
```

## 2. Vai trò đặc biệt: "Bác sĩ cây trồng"

Đại lý VTNN **không phải người bán hàng thông thường**:
- Nông dân mang lá úa, rễ sưng, trái thối → đại lý **chẩn đoán** bệnh
- Phân biệt: nấm (cháy rõ ranh giới) vs vi khuẩn (dịch rỉ) vs thiếu vi lượng
- "Kê đơn" thuốc: ưu tiên thuốc sinh học, chú ý thời gian cách ly trước thu hoạch
- Tư vấn **phối trộn tank-mixing** — kỹ năng cao nhất:
  - Thứ tự: Bột (WG) → Huyền phù (SC) → Dầu (OD) → Nhũ dầu (EC) → Dung dịch (SL)
  - Kiêng kỵ: Đồng (Cu) kỵ Chlorpyrifos + kháng sinh + phân bón lá
  - Cây đang bệnh nặng → **KHÔNG** thêm phân NPK (đạm = "thức ăn" cho nấm)

## 3. Quy đổi đơn vị — Nút thắt quan trọng nhất

Đại lý **nhập theo thùng**, **bán lẻ theo chai**:
- 1 Thùng = 40 Chai 100ml → bán 1 chai = trừ 1/40 thùng
- Sai quy đổi → tồn kho lý thuyết ≠ thực tế → thất thoát + cháy hàng
- **POS phải có** tính năng quy đổi đa tầng tự động

> ✅ **Nodi POS đã có**: `exchange_value` trong `product_units` — quy đổi tự động khi bán

## 4. Công nợ gối đầu — "Trái tim" mô hình kinh doanh

Đại lý VTNN hoạt động như **tổ chức tín dụng vi mô phi chính thức**:
- Nông dân **mua chịu 100%** từ đầu vụ → nợ cộng dồn suốt vụ → trả khi thu hoạch
- Chu kỳ: 3-6 tháng tùy vụ (Đông Xuân, Hè Thu, Thu Đông)
- Đại lý phải **đánh giá tín dụng** từng hộ: diện tích, năng suất cũ, uy tín
- Hạn mức công nợ (Credit Limit) cho mỗi khách → cảnh báo khi vượt
- **Đối soát cuối vụ** = thời điểm sống còn: sổ tay → tranh cãi → mất khách

> ✅ **Nodi POS đã có**: `credit_limit`, `current_debt`, `crop_seasons`, sao kê chi tiết

## 5. Quản lý kho — FEFO + Lô + Truy xuất

- Thuốc BVTV có **hạn sử dụng** → bán quá hạn = kiện + tước giấy phép
- Nguyên tắc **FEFO** (First Expired, First Out) bắt buộc
- Theo dõi **số lô** phục vụ: cảnh báo hết hạn, truy xuất nguồn gốc, thanh tra
- Kiểm kê định kỳ: đối chiếu vật lý vs số liệu phần mềm

> ✅ **Nodi POS đã có**: `product_batches` (lô), `expiry_date`, FEFO auto-deduct, `stocktake`

## 6. Phân loại đại lý

| | Đại lý nhỏ (cấp 2) | Nhà phân phối (cấp 1) |
|--|---------------------|----------------------|
| **Quy mô** | 1-2 người, hộ gia đình | Công ty, nhiều phòng ban |
| **Nguồn hàng** | Mua từ NPC cấp 1 | Nhập trực tiếp nhà máy |
| **Sức mạnh** | Quan hệ cá nhân, "vốn xã hội" | Vốn lớn, hệ thống DMS |
| **Công cụ** | Sổ tay / Excel / POS đơn giản | ERP / DMS / GPS |
| **Khách** | Nông dân trực tiếp | Hàng trăm đại lý cấp 2 |

> 🎯 **Nodi POS target**: Đại lý nhỏ-vừa (cấp 2) — đúng phân khúc!

## 7. Chuyển đổi số: 3 kỷ nguyên

| Giai đoạn | Công cụ | Hạn chế |
|-----------|---------|---------|
| **1.0** Sổ tay | Chi phí 0đ | Mất sổ = mất nợ, tranh cãi liên miên |
| **2.0** Excel | Tính toán tốt hơn | Không scan barcode, dễ xóa công thức |
| **3.0** POS/ERP | Tự động hóa toàn diện | Chi phí subscription |

## 8. Đối thủ POS cho VTNN

| Phần mềm | Thế mạnh | Phân khúc |
|-----------|----------|-----------|
| **Sapo** | Công nợ gối đầu, quy đổi đơn vị, lô/hạn | Đại lý vừa-nhỏ |
| **KiotViet** | Phổ biến nhất VN, dễ dùng | Đại lý nhỏ |
| **MekongSoft** | Giao diện giống Excel | Chủ lớn tuổi |
| **ARITO/Ecount** | ERP chuyên sâu | Công ty/tổng kho |
| **MobiWork DMS** | GPS nhân viên, route, anti-fraud | Nhà phân phối lớn |

---

## Áp dụng cho Nodi POS

### ✅ Đã có (xác nhận từ nghiên cứu)
- Quy đổi đơn vị đa tầng (`exchange_value`)
- Công nợ vụ mùa (`crop_seasons`, `credit_limit`, sao kê)
- Lô + hạn sử dụng + FEFO (`product_batches`)
- Thuốc cấm 3 tầng (`banned_ingredients`)
- Kiểm kê (`stocktake`, `adjust_stock`)
- Truy xuất nguồn gốc + QR

### 🟡 Cần cải thiện
- **AI chatbot**: Thêm tư vấn phối trộn tank-mixing (SC→OD→EC)
- **Dashboard đại lý**: Biểu đồ dòng tiền vs công nợ theo vụ mùa
- **Cảnh báo FEFO**: Hiện trên trang chủ mobile (hiện chỉ có "SP sắp hết hàng")

### ⬜ Chưa có (cân nhắc tương lai)
- HĐĐT tích hợp (e-invoice: ký số, gửi thuế)
- Multi-branch (đa chi nhánh)
- DMS route planning (cho NPC cấp 1)

---

## Nguồn tham khảo

- Google DeepSearch — Prompt: "Quy trình làm việc hàng ngày đại lý VTNN VN"
- Luật BVTV 2013, Thông tư 21/2015/TT-BNNPTNT
- TCVN 5507:2002 (An toàn hóa chất)
- Nghị định 123/2020/NĐ-CP (HĐĐT)
- Data sources: Sapo, ARITO, KiotViet, MobiWork DMS
