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

## 2026-03-15 (Sprint 113 — Infrastructure Hardening & Sync Debug)

> **Mục tiêu:** Tuning PostgreSQL, bảo mật Nginx, debug lỗi 429 + sync field mismatch + WebSocket real-time sync
> **Kết quả:** 9 items hoàn thành, phối hợp VPS Agent ↔ App Agent debug xuyên suốt

### ✨ FEAT — WebSocket Real-time Sync (`/ws/sync`)
- Endpoint `GET /ws/sync?token=JWT` — upgrade HTTP→WebSocket, join room theo `store_id`
- `SyncRooms` hub: room-based broadcast per `store_id`, heartbeat ping/30s
- Tích hợp `handle_sync()`: sau `tx.commit()` → broadcast `sync_update` event cho all connected clients
- Event format: `{"type":"sync_update","store_id":X,"collections":["products","orders",...],"timestamp":"..."}`
- Nginx: thêm `/ws/sync` location block với WebSocket proxy headers
- Files: `api/src/routes/ws_sync.rs` [NEW], `api/src/main.rs`, `api/src/routes/sync.rs`, `api/src/routes/mod.rs`, `nginx/conf.d/nodi.conf`

### ⚡ PERF — PostgreSQL Tuning (8GB VPS)
- `shared_buffers`: 128MB → **2GB**, `work_mem`: 4MB → **32MB**
- `maintenance_work_mem`: 64MB → **256MB**, `random_page_cost`: 4.0 → **1.1** (SSD)
- `effective_io_concurrency`: 1 → **200**, `wal_buffers`: 4MB → **64MB**
- Thêm slow query logging (>500ms), autovacuum tuning
- Docker `shm_size: 2g` cho shared memory
- Files: `data/postgresql.conf` [NEW], `docker-compose.yml`

### 🔒 SECURITY — Nginx Security Headers + Cloudflare Real IP
- 7 headers: HSTS, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy, Permissions-Policy, CSP
- Cloudflare Real IP: `set_real_ip_from` cho 15 dải IP + `real_ip_header CF-Connecting-IP`
- Gỡ bỏ toàn bộ Nginx rate limit (không phù hợp mô hình 1 store = 10 devices cùng IP)
- Files: `nginx/conf.d/nodi.conf`

### 🐛 FIX — Lỗi 429 Too Many Requests (Root Cause: `tower_governor`)
- **Triệu chứng:** Mobile app (AgriPOS) bị 429 khi login, sync, pull dữ liệu
- **Debug path:** Nginx rate limit → Cloudflare WAF → **Rust API `tower_governor`**
- **Root cause:** `tower_governor` + `SmartIpKeyExtractor` đọc `X-Forwarded-For` từ Cloudflare proxy → tất cả users chung 1 bucket rate limit → 429
- **Fix:** Gỡ `tower_governor` khỏi `main.rs` + `Cargo.toml`, rebuild Docker API container
- Files: `api/src/main.rs`, `api/Cargo.toml`

### 🐛 FIX — Sync Pull thiếu `created_at` cho products, customers, suppliers
- **Nguyên nhân:** `handle_pull()` không SELECT `created_at` cho 3 bảng → mobile SQLite có `NOT NULL` constraint → INSERT fail im lặng → 0 sản phẩm
- **Fix VPS:** Thêm `synced_at::text` làm `created_at` vào pull response (DB không có column `created_at`, dùng `synced_at` auto-fill by PG)
- **Fix App:** Agent app thêm fallback `chrono::Local::now()` khi VPS thiếu `created_at` + error logging thay `.ok()`
- Files: `api/src/routes/sync.rs`

### ✨ FEAT — Xác nhận `GET /api/sync/pull` hoạt động
- Endpoint trả đầy đủ 21 collections (62 products, 64 units cho store_id=1000004)
- Báo cáo field mapping chính xác cho agent app đối chiếu: `notes` (không phải `note`), `transaction_type`, `code` (từ `company`)

### 📚 DOCS — Tổ chức lại tài liệu
- Đổi tên `roadmap/` → `docs/`
- Cập nhật APK mới → `downloads/Nodi-Pos.apk`

---
## 2026-03-03 (Sprint 89 — Staff Permission Management)

> **Mục tiêu:** Đồng bộ nhân viên từ desktop + quản lý quyền từ xa trên web dashboard  
> **Kết quả:** 6/6 items hoàn thành

### 🗃️ DB — PostgreSQL Migration
- `sync_staff_members` table: composite PK `(store_id, id)`, JSONB permissions, 3 indexes

### ✨ FEAT — Sync Handler (Collection #15)
- `SyncStaffMember` struct + `staff_members` field trong `SyncPayload`
- Upsert vào `sync_staff_members` với `ON CONFLICT (store_id, id) DO UPDATE`
- PIN `TEXT` lưu nhưng không bao giờ expose qua web API

### ✨ FEAT — Dashboard Staff API (3 endpoints)
- `GET /api/dashboard/staff` — danh sách nhân viên (trả `pin_set: bool`, không lộ PIN)
- `PUT /api/dashboard/staff/:id/permissions` — cập nhật 9 cờ quyền (chặn sửa owner)
- `PUT /api/dashboard/staff/:id/toggle-active` — bật/tắt nhân viên (chặn tắt owner)

### 🎨 UI — Staff Management Page
- Viết lại `/dashboard/nhan-vien` hoàn toàn
- Bảng nhân viên: avatar, role badge, trạng thái, PIN, số quyền (x/9)
- Modal phân quyền: 9 toggle switches với nhãn tiếng Việt
- Skeleton loading, empty state, dark mode

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
