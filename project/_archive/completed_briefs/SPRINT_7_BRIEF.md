# 🚀 Sprint 7 Brief — Polish + Go Live

> **Giao cho**: Agent VPS (Claude Opus 4)
> **Ngày giao**: 2026-02-18
> **Trạng thái**: Sprint 6 ✅ → Sprint 7 bắt đầu
> **Deadline**: Hoàn thành trong 1 session

---

## 🎯 Mục tiêu

Hoàn thiện website nodi.vn với Blog, Hướng dẫn sử dụng, SEO optimization, và Performance tuning. Đây là sprint cuối cùng trước GO LIVE.

---

## 📋 Task 1: Blog System (Nuxt Content)

### Yêu cầu:
- Cài `@nuxt/content` module vào Nuxt 3 project
- Tạo layout blog: listing page `/blog` + detail page `/blog/:slug`
- Blog listing: card grid, thumbnail, tiêu đề, mô tả ngắn, ngày đăng
- Blog detail: markdown render, TOC sidebar, breadcrumb
- Design: cùng theme với website hiện tại (xanh lá/nông nghiệp)

### Viết 5 bài blog SEO mẫu:

| # | Slug | Tiêu đề | Từ khóa target |
|---|------|---------|-----------------|
| 1 | `phan-mem-quan-ly-cua-hang-vat-tu-nong-nghiep` | Phần mềm quản lý cửa hàng vật tư nông nghiệp 2026 | phần mềm quản lý, vật tư nông nghiệp |
| 2 | `hoa-don-dien-tu-dai-ly-phan-bon` | Hướng dẫn xuất hóa đơn điện tử cho đại lý phân bón | hóa đơn điện tử, đại lý phân bón |
| 3 | `quan-ly-cong-no-khach-hang-nong-dan` | Cách quản lý công nợ khách hàng nông dân hiệu quả | công nợ, khách hàng nông dân |
| 4 | `quan-ly-ton-kho-thuoc-bvtv` | Quản lý tồn kho thuốc BVTV — tránh hết hạn, thất thoát | tồn kho, thuốc BVTV |
| 5 | `chuyen-doi-so-dai-ly-nong-nghiep` | Chuyển đổi số cho đại lý nông nghiệp — bắt đầu từ đâu? | chuyển đổi số, đại lý nông nghiệp |

### Nội dung mỗi bài:
- Dài 800-1200 từ tiếng Việt
- Có heading structure (H2, H3)
- Kết thúc bằng CTA dẫn về `/tai-ung-dung` hoặc `/bang-gia`
- Thêm meta description cho SEO

---

## 📋 Task 2: Hướng Dẫn Sử Dụng (`/huong-dan`)

### Yêu cầu:
- Trang listing `/huong-dan` — danh sách tất cả hướng dẫn, chia theo nhóm
- Detail page `/huong-dan/:slug` — nội dung markdown
- Sidebar navigation (TOC)
- Design: clean, documentation-style

### Viết 6 bài hướng dẫn:

| # | Slug | Tiêu đề | Nội dung |
|---|------|---------|----------|
| 1 | `cai-dat` | Cài đặt Nodi POS | Download, cài đặt .exe, nhập license key, cấu hình cửa hàng |
| 2 | `ban-hang` | Hướng dẫn bán hàng | Tìm SP, thêm giỏ, thanh toán, in hóa đơn, bán nợ |
| 3 | `nhap-hang` | Hướng dẫn nhập hàng | Tạo phiếu nhập, chọn NCC, nhập số lượng/giá, xác nhận |
| 4 | `cong-no` | Quản lý công nợ | Xem nợ KH/NCC, thu tiền, lịch sử thanh toán |
| 5 | `backup` | Sao lưu & khôi phục | Backup tự động, backup thủ công, khôi phục khi máy hỏng |
| 6 | `hoa-don-dien-tu` | Hóa đơn điện tử | Cấu hình VNPT, xuất HĐĐT từ đơn hàng, tra cứu |

### Nội dung mỗi bài:
- 400-800 từ, kèm danh sách bước (step-by-step)
- Nếu có thể, thêm placeholder cho screenshot (dùng emoji mô tả)
- Tone: đơn giản, dễ hiểu, cho người dùng không rành công nghệ

---

## 📋 Task 3: SEO Optimization

### Yêu cầu:

#### 3.1 Sitemap
- Tạo auto-generated `sitemap.xml` (dùng `@nuxtjs/sitemap` hoặc tự build)
- Bao gồm tất cả routes: trang chủ, tính năng, bảng giá, blog/*, huong-dan/*
- Submit URL: `https://nodi.vn/sitemap.xml`

#### 3.2 Robots.txt
```
User-agent: *
Allow: /
Disallow: /dashboard/
Disallow: /admin/
Disallow: /api/
Sitemap: https://nodi.vn/sitemap.xml
```

#### 3.3 Structured Data (JSON-LD)
- Trang chủ: `Organization` + `SoftwareApplication`
- Blog: `Article` schema cho mỗi bài
- Bảng giá: `Product` + `Offer` schema
- FAQ: `FAQPage` schema (trang bảng giá có FAQ)

#### 3.4 Meta Tags
- Đảm bảo mỗi trang có: `<title>`, `<meta description>`, `og:title`, `og:description`, `og:image`
- Canonical URLs cho tất cả trang
- `lang="vi"` trên `<html>`
- Favicon (nếu chưa có)

---

## 📋 Task 4: Performance Optimization

### Yêu cầu:

#### 4.1 Nginx
- Gzip compression cho text/html, css, js, json, xml
- Brotli nếu có thể (optional)
- Static asset caching: `Cache-Control: max-age=31536000` cho /_nuxt/*
- HTML caching: `Cache-Control: no-cache` (SSR pages)

#### 4.2 Nuxt
- Ensure SSR pages are properly cached
- Lazy load images (nếu có)
- Preconnect/preload critical resources

#### 4.3 Verify
- Chạy `curl -sI https://nodi.vn/ | grep -i content-encoding` → phải thấy gzip
- Chạy `curl -sI https://nodi.vn/_nuxt/xxx.js | grep -i cache-control` → phải thấy max-age

---

## 📋 Task 5: Trang pháp lý (nếu chưa có)

Kiểm tra và tạo nếu thiếu:
- `/chinh-sach-bao-mat` — Chính sách bảo mật (privacy policy)
- `/dieu-khoan-su-dung` — Điều khoản sử dụng (terms of service)

Nội dung phải đề cập:
- Thu thập dữ liệu (backup, sync)
- Bảo mật thông tin cửa hàng
- Quyền sở hữu data
- Chính sách hoàn tiền license

---

## ✅ Verification Checklist

Sau khi hoàn thành, verify từng mục:

```
[ ] /blog                     → 200, listing page có 5 bài
[ ] /blog/phan-mem-quan-ly... → 200, bài viết render đẹp
[ ] /huong-dan                → 200, listing page có 6 bài
[ ] /huong-dan/cai-dat        → 200, step-by-step hiển thị OK
[ ] /sitemap.xml              → 200, XML valid, có tất cả URLs
[ ] /robots.txt               → 200, nội dung đúng
[ ] curl -sI / | grep gzip    → content-encoding: gzip
[ ] curl -sI /_nuxt/*.js      → cache-control có max-age
[ ] /chinh-sach-bao-mat       → 200
[ ] /dieu-khoan-su-dung       → 200
[ ] /, /login, /dashboard     → Vẫn hoạt động bình thường (regression)
[ ] 4 containers              → All running
[ ] /api/health               → OK
```

---

## ⚠️ Lưu ý quan trọng

1. **KHÔNG sửa bất kỳ API endpoint nào** — Sprint 7 chỉ là frontend + SEO + performance
2. **KHÔNG thay đổi auth/login flow** — giữ nguyên
3. **KHÔNG thay đổi database schema** — giữ nguyên
4. **Test regression** — sau khi deploy, tất cả trang cũ phải vẫn hoạt động
5. **Commit message** rõ ràng: `sprint-7: blog system`, `sprint-7: seo optimization`, etc.

---

## 📊 Kết quả mong đợi

Sau Sprint 7:
- Website nodi.vn có Blog SEO + Hướng dẫn sử dụng
- Google có thể crawl và index tất cả trang công khai
- Performance tối ưu (gzip, caching)
- Trang pháp lý đầy đủ
- **Sẵn sàng GO LIVE** 🚀
