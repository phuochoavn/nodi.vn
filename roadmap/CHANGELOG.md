# 📝 NODI POS - Nhật Ký Phát Triển (Development Changelog)

> Ghi chép mọi thay đổi, cập nhật, và sự kiện quan trọng trong quá trình phát triển dự án.  
> **Format:** `[YYYY-MM-DD] [Loại] Mô tả chi tiết`

---

## Các ký hiệu

| Ký hiệu | Ý nghĩa |
|----------|---------|
| ✨ FEAT | Tính năng mới |
| 🐛 FIX | Sửa lỗi |
| 🔧 REFACTOR | Tái cấu trúc code |
| 📦 INFRA | Hạ tầng, DevOps |
| 🎨 UI | Giao diện, thiết kế |
| 📚 DOCS | Tài liệu |
| 🔒 SECURITY | Bảo mật |
| ⚡ PERF | Tối ưu hiệu năng |
| 🧪 TEST | Testing |
| 🗃️ DB | Database migration |

---

## 2026-03-02 (Sprint 53 — Feature Gap Hardening)

> **Mục tiêu:** Đánh giá toàn diện hệ thống dashboard và sửa 11 gaps (P0 → P2)  
> **Kết quả:** 11/11 hoàn thành, score ↑ từ 5.2/10

### 🔒 SECURITY — P0 Critical Fixes

- **401 Auto-redirect:** Khi token hết hạn → clear cookie → toast → redirect `/login`
  - Files: `web/composables/useAuth.ts`
- **Rate Limiting:** `tower-governor` 0.8 với Axum 0.8 (50 req/sec/IP, burst 100)
  - Files: `api/src/main.rs`, `api/Cargo.toml`
- **Automated DB Backup:** Script `pg_dump` chạy cron 2AM daily, giữ 14 ngày
  - Files: `scripts/backup_db.sh`

### 🔧 REFACTOR — CSS Cleanup

- Loại bỏ ~40 dòng CSS trùng lặp trong `ke-toan.vue` (dùng global classes)
  - Files: `web/pages/dashboard/ke-toan.vue`

### ✨ FEAT — P1 Major Features

- **Chart.js Reports:** Viết lại trang Báo cáo với line chart (doanh thu) + bar chart (top 10 SP)
  - Files: `web/pages/dashboard/bao-cao.vue`
- **Purchase Orders Page:** Trang Nhập hàng mới: bảng dữ liệu, search, pagination, status badges
  - Files: `web/pages/dashboard/nhap-hang.vue`, `api/src/routes/dashboard.rs`
  - API: `GET /api/dashboard/purchase-orders`
- **Employees Page:** Trang Nhân viên: stats summary, avatar, role badges, trạng thái
  - Files: `web/pages/dashboard/nhan-vien.vue`, `api/src/routes/dashboard.rs`
  - API: `GET /api/dashboard/employees`
- **Sidebar Navigation:** Thêm 📦 Nhập hàng + 👥 Nhân viên vào sidebar
  - Files: `web/layouts/dashboard.vue`

### ✨ FEAT — P2 Nice-to-Have

- **Toast Notifications:** Hệ thống toast toàn cục (4 types, auto-dismiss, dark mode)
  - Files: `web/composables/useToast.ts`, `web/components/ToastContainer.vue`, `web/app.vue`
  - Tự động hiện toast khi API lỗi (tích hợp vào `fetchApi`)
- **Product Excel Export:** Nút "📥 Xuất Excel" trên trang Tồn kho
  - Files: `web/pages/dashboard/ton-kho.vue`, `api/src/routes/dashboard.rs`
  - API: `GET /api/dashboard/inventory/export` (rust_xlsxwriter, formatted xlsx)
- **Notification Bell:** 🔔 Icon chuông trên header với badge đỏ + dropdown panel
  - Thông báo tự động: sản phẩm sắp hết, sắp hết hạn, công nợ KH/NCC
  - Files: `web/layouts/dashboard.vue`, `api/src/routes/dashboard.rs`
  - API: `GET /api/dashboard/notifications`

### 📊 Tổng kết Sprint 53

| Mức độ | Items | Status |
|--------|-------|--------|
| P0 Critical | 4 | ✅ |
| P1 Major | 4 | ✅ |
| P2 Nice | 3 | ✅ |
| **Tổng** | **11** | **✅ 100%** |

**New APIs:** 4 endpoints | **New Files:** 5 | **Modified Files:** 6

---

## 2026-02-24

### 🎨 UI — Redesign Logo SVG
- Vẽ lại `logo.svg`: từ icon bông lúa → **ngôi nhà + lá cây** (khớp với logo thật trong app)
- Cập nhật `favicon.svg` cho nhất quán
- Tông màu: xanh dương `#0ea5e9` (nhà) + xanh lá `#22c55e` (lá)
- Files: `web/public/logo.svg`, `web/public/favicon.svg`

### 🎨 UI — Hero Section: Ảnh chụp thật thay mockup trừu tượng
- Xóa toàn bộ CSS mockup giả (fake sidebar, fake chart bars)
- Thay bằng **3 ảnh chụp thật** xếp chồng (stacked floating mockup):
  - Ảnh POS (bán hàng) — nổi bật nhất, ở trên cùng
  - Ảnh Thu Chi (Cashflow) — xếp phía sau
  - Ảnh Thuế & Kế toán — xếp dưới cùng
- Thêm 2 **floating badges**: "Bán hàng POS nhanh 5s" và "Báo cáo Realtime"
- Hiệu ứng: entrance animation staggered, float animation, green glow
- Files: `web/components/HeroSection.vue`, `web/public/screenshots/`

### 📚 DOCS — Tạo tài liệu dự án
- Tạo folder `roadmap/` chứa tài liệu dự án
- `PROJECT_INFO.md` — Thông tin toàn diện về dự án: kiến trúc, tech stack, database, cấu trúc thư mục
- `ROADMAP.md` — Lộ trình phát triển 4 phases từ MVP đến Scale & AI
- `CHANGELOG.md` — Nhật ký phát triển (file này)

---

## 2026-02-21

### 🔒 SECURITY — Hệ thống Authentication
- Triển khai JWT authentication cho API
- Bcrypt password hashing cho user accounts
- Auth middleware bảo vệ các protected routes
- Phân quyền: `store_owner` vs `admin` roles

### 🎨 UI — Sửa lỗi Header & Bảng Giá
- Cải thiện visual clarity của pricing cards trên trang Bảng Giá
- Fix dark mode toggle bị mất khi navigate
- Thêm nút Đăng Ký / Đăng Nhập vào header

### 🎨 UI — Redesign Website nodi.vn
- Redesign toàn bộ website nodi.vn theo phong cách SaaS premium
- 5 trang chính: HOME, Tính Năng, Bảng Giá, Tải Ứng Dụng, Liên Hệ
- Dark mode ready, smooth animations, responsive
- Color palette & typography hiện đại

---

## 2026-02-20

### ✨ FEAT — WebSocket Chat Support
- Triển khai WebSocket chat realtime
- Customer ↔ Admin chat theo ticket
- Typing indicators, read receipts
- Xử lý reconnection, `ticket_closed` event
- Messages persist vào database

---

## 2026-02 (Sprint 10)

### 🗃️ DB — Employees & Store Groups Migration
- Tạo bảng `employees` — quản lý nhân viên cửa hàng (cashier, manager)
- Hỗ trợ PIN code cho nhân viên
- Phân quyền nhân viên qua JSONB `permissions`
- Tạo bảng `store_groups` — quản lý chuỗi cửa hàng
- Thêm `group_id`, `branch_name` vào bảng `stores`

---

## 2026-01 — 2026-02 (Sprint 3)

### 🗃️ DB — Mở rộng Sync Data
- Thêm nhiều bảng sync mới:
  - `synced_purchase_orders` + `synced_purchase_items` — Đơn nhập hàng
  - `synced_customer_transactions` — Giao dịch công nợ khách hàng
  - `synced_supplier_transactions` — Giao dịch nhà cung cấp
  - `synced_product_units` — Đơn vị tính sản phẩm
  - `synced_product_batches` — Lô hàng (hạn sử dụng)
  - `synced_product_transactions` — Biến động kho
  - `synced_store_funds` — Quỹ tiền cửa hàng
  - `synced_cash_transactions` — Thu chi tiền mặt
  - `synced_payment_vouchers` — Phiếu chi
  - `synced_store_settings` — Cài đặt cửa hàng
  - `synced_invoice_payments` — Thanh toán hóa đơn
- Bổ sung các columns mới cho `synced_customers`, `synced_products`, `synced_invoices`
- Tạo indexes cho performance

---

## 2026-01 (Sprint 1-2)

### 📦 INFRA — Setup Hạ Tầng Ban Đầu
- Docker Compose: 4 services (Nginx, Axum API, Nuxt Web, PostgreSQL)
- Nginx reverse proxy với SSL
- PostgreSQL 16 Alpine
- Bridge network nội bộ
- Volume mounts cho data persistence

### ✨ FEAT — API Backend Core
- Rust/Axum web framework setup
- Database connection pool (SQLx)
- Health check endpoint
- CORS configuration
- Tracing & structured logging

### 🗃️ DB — Schema Ban Đầu
- Bảng `stores` — thông tin cửa hàng, license
- Bảng `users` — tài khoản đăng nhập
- Bảng `backup_files` — metadata backup
- Các bảng sync cơ bản: products, customers, invoices, invoice_items, suppliers
- Indexes cho Market Intelligence queries

### ✨ FEAT — Frontend Setup
- Nuxt 3 project initialization
- TailwindCSS integration
- Vue Router cấu hình
- Layout system (default, admin)
- Component library setup (Lucide icons)

---

## Template ghi chú hàng ngày

<!--
Sử dụng template dưới đây để ghi chú mỗi ngày:

## YYYY-MM-DD

### [Ký hiệu] [Loại] — Tiêu đề ngắn
- Chi tiết thay đổi 1
- Chi tiết thay đổi 2
- Files thay đổi: `file1.rs`, `file2.vue`
- Ghi chú đặc biệt (nếu có)

### Vấn đề gặp phải
- Mô tả vấn đề
- Giải pháp áp dụng

### Ghi chú
- Ghi chú bất kỳ
-->
