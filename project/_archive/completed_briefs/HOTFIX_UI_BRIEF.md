# 🔧 HOTFIX Brief — Sửa giao diện nodi.vn

> **Giao cho**: Agent VPS (Claude Opus 4)
> **Ngày giao**: 2026-02-18
> **Ưu tiên**: KHẨN — sửa trước khi GO LIVE
> **Phạm vi**: Frontend Nuxt only — KHÔNG sửa API/DB

---

## 🚨 Fix 1: Nút Hero bị trắng trơn (CRITICAL)

**Trang**: `/` (Homepage)
**Vấn đề**: Nút thứ 2 trong Hero section (bên phải nút "Tải miễn phí") bị trắng, không hiện text.
**Fix**: Nút phải hiện text "Xem tính năng →" với style outline (viền trắng, chữ trắng, hover đổi nền).

---

## 🚨 Fix 2: Bảng giá — Sửa toàn bộ (CRITICAL)

**Trang**: `/bang-gia` + section bảng giá trên Homepage `/`

### 2.1 Bỏ gói "Trọn đời" — chỉ giữ 3 gói:

| Gói | Giá | Mô tả |
|-----|-----|-------|
| **Dùng thử** | Miễn phí | 30 ngày đầy đủ tính năng. Sau 30 ngày → Free (20 đơn/ngày) |
| **Theo tháng** | **299.000đ**/tháng | Linh hoạt, hủy bất cứ lúc nào |
| **Theo năm** ⭐ | **1.990.000đ**/năm | ≈ 166k/tháng — Tiết kiệm 44% |

### 2.2 Tính năng chi tiết — thay thế list chung chung hiện tại:

**Gói Dùng thử / Free:**
- ✅ Bán hàng POS (giới hạn 20 đơn/ngày sau 30 ngày)
- ✅ Quản lý sản phẩm, khách hàng, NCC
- ✅ Nhập hàng, tồn kho
- ✅ Báo cáo cơ bản
- ✅ Xuất Excel
- ❌ AI Chatbot
- ❌ Hóa đơn điện tử
- ❌ Cloud backup & khôi phục
- ❌ Hỗ trợ kỹ thuật

**Gói Theo tháng (PRO):**
- ✅ Bán hàng **không giới hạn**
- ✅ Quản lý sản phẩm, khách hàng, NCC
- ✅ Nhập hàng, tồn kho, lô hàng
- ✅ Báo cáo chi tiết (doanh thu, lãi/lỗ, top SP)
- ✅ Xuất Excel
- ✅ 🤖 AI Chatbot thông minh
- ✅ 🧾 Hóa đơn điện tử (VNPT)
- ✅ ☁️ Cloud backup tự động
- ✅ 🔄 Khôi phục data khi đổi máy
- ✅ Cập nhật phiên bản mới
- ✅ Hỗ trợ kỹ thuật qua Zalo

**Gói Theo năm (PRO):** — giống gói tháng +
- ✅ Tất cả tính năng gói tháng
- ✅ **Tiết kiệm 44%** so với gói tháng
- ✅ Ưu tiên hỗ trợ kỹ thuật

### 2.3 Fix text bị blur/mờ
Kiểm tra CSS của pricing cards — text trong feature list bị mờ. Đảm bảo:
- `opacity: 1` cho tất cả text
- Contrast ratio đủ (text trắng trên nền tối hoặc text tối trên nền trắng)
- Không sử dụng `color` quá nhạt

---

## 🚨 Fix 3: SĐT + Email thật — thay toàn bộ placeholder (CRITICAL)

**Ảnh hưởng**: Footer (mọi trang) + trang Liên hệ + Chính sách bảo mật

**Thay thế:**
- `0XXX.XXX.XXX` → `0374.222.326`
- `[email protected]` → `hoavn12345@gmail.com`
- Link Zalo: `https://zalo.me/0374222326`

**Lưu ý**: Tìm và thay **tất cả** nơi xuất hiện placeholder trong toàn bộ project. Không được bỏ sót.

---

## 🚨 Fix 4: Trang Login — Thêm form đăng nhập (CRITICAL)

**Trang**: `/login`
**Vấn đề**: Hiện tại chỉ có text, không có form input.

**Fix**: Thêm form đăng nhập hoàn chỉnh:
- Input: Số điện thoại (placeholder: "Nhập số điện thoại")
- Input: Mật khẩu (placeholder: "Nhập mật khẩu", type=password, toggle show/hide)
- Button: "Đăng nhập" (style primary, full width)
- Link: "Quên mật khẩu?" (hiện alert "Liên hệ Zalo 0374.222.326 để reset")
- Phía dưới: "Chưa có tài khoản? Tải app & kích hoạt license →"
- Submit → POST `/api/login-with-license` (đã có sẵn API)
- Thành công → lưu JWT cookie → redirect `/dashboard`
- Lỗi → hiện thông báo "Sai số điện thoại hoặc mật khẩu"

**Design**: Card form trung tâm màn hình, logo Nodi POS phía trên, background gradient tối.

---

## ⚠️ Fix 5: Trial 7 ngày → 30 ngày (Tìm và thay toàn bộ)

Tìm tất cả chỗ ghi "7 ngày" liên quan đến dùng thử, thay thành "30 ngày":
- Homepage: "7 ngày đầy đủ tính năng" → "30 ngày đầy đủ tính năng"
- `/bang-gia`: "7 ngày, đầy đủ tính năng" → "30 ngày đầy đủ tính năng. Sau 30 ngày: miễn phí giới hạn 20 đơn/ngày"
- `/tinh-nang`: "dùng thử 7 ngày" → "dùng thử 30 ngày"
- `/tai-ung-dung`: "Chưa có key? Dùng thử miễn phí 7 ngày!" → "Chưa có key? Dùng thử miễn phí 30 ngày!"

---

## ⚠️ Fix 6: Trang Tính năng — Bổ sung thiếu

**Trang**: `/tinh-nang`

Thêm 3 tính năng đang thiếu:

### Trả hàng & Hoàn tiền
Xử lý trả hàng linh hoạt, hoàn tiền nhanh chóng
- ✅ Chọn sản phẩm cần trả từ hóa đơn gốc
- ✅ Nhiều lý do trả: hết hạn, hư hỏng, nhầm sản phẩm
- ✅ Hoàn tiền mặt hoặc ghi có vào công nợ
- ✅ Lịch sử trả hàng đầy đủ

### Chốt sổ cuối ngày
Đối soát tiền mặt, kiểm tra doanh thu cuối ngày
- ✅ Tự động tổng hợp doanh thu trong ngày
- ✅ Đối soát tiền mặt thực tế vs hệ thống
- ✅ Lịch sử chốt sổ theo ngày

### Import từ Excel
Nhập nhanh danh mục sản phẩm từ file Excel
- ✅ Tự động nhận diện cột tiếng Việt
- ✅ Xem trước (preview) trước khi import
- ✅ Hỗ trợ 4500+ sản phẩm BVTV

---

## ⚠️ Fix 7: Tải app — Sửa thông tin

**Trang**: `/tai-ung-dung`

- File size: `~35 MB` → sửa thành `~7 MB`
- Kiểm tra link download `/download/NodiPOS_1.1.0_x64-setup.exe` có hoạt động không. Nếu không, tạm thời đổi nút thành "Liên hệ qua Zalo để nhận link tải" + link `https://zalo.me/0374222326`

---

## ⚠️ Fix 8: Hướng dẫn cài đặt — Sửa size

**Trang**: `/huong-dan/cai-dat`

- "File có dung lượng khoảng 80MB" → sửa thành "khoảng 7MB"

---

## ✅ Verification Checklist

```
[ ] Homepage: nút Hero thứ 2 có text "Xem tính năng →"
[ ] Homepage: bảng giá 3 gói (không có Trọn đời)
[ ] Homepage: text bảng giá đọc được rõ ràng (không blur)
[ ] /bang-gia: 3 gói, giá đúng (0đ, 299k/tháng, 1.99M/năm)
[ ] /bang-gia: tính năng chi tiết (có ❌ cho Free, ✅ cho PRO)
[ ] /login: có form SĐT + mật khẩu + nút Đăng nhập
[ ] /login: submit form → gọi API → redirect hoặc báo lỗi
[ ] Footer: SĐT = 0374.222.326, Email = hoavn12345@gmail.com
[ ] /lien-he: thông tin liên hệ thật
[ ] /chinh-sach-bao-mat: email thật
[ ] Tất cả trang: "30 ngày" (không còn "7 ngày")
[ ] /tinh-nang: có 9 nhóm tính năng (thêm 3 mới)
[ ] /tai-ung-dung: file size = ~7 MB
[ ] /huong-dan/cai-dat: file size = ~7 MB
[ ] Regression: /, /blog, /huong-dan, /dashboard, /admin vẫn OK
[ ] 4 containers running
[ ] /api/health → OK
```

---

## ⚠️ Lưu ý

1. **KHÔNG sửa API** — chỉ sửa Nuxt frontend
2. **KHÔNG sửa database** — giữ nguyên
3. **Login form** gọi API `/api/login-with-license` đã có sẵn
4. Sửa xong **rebuild + redeploy** Nuxt container
5. **Test regression** — tất cả trang cũ phải vẫn hoạt động
