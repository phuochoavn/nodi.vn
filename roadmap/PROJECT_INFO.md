# 📋 NODI POS - Thông Tin Dự Án (Project Information)

> **Cập nhật lần cuối:** 2026-02-24  
> **Phiên bản hiện tại:** 0.1.0 (MVP)  
> **Trạng thái:** Đang phát triển (In Development)

---

## 1. Tổng Quan Dự Án

**Nodi POS** là nền tảng quản lý cửa hàng vật tư nông nghiệp (Point of Sale), bao gồm:

- **Ứng dụng di động** (Mobile App) dành cho chủ cửa hàng — quản lý bán hàng, tồn kho, công nợ tại chỗ
- **Hệ thống Web** — dashboard quản trị, phân tích dữ liệu, sync dữ liệu từ app
- **Nền tảng quản trị Admin** — quản lý cửa hàng, cấp license, hỗ trợ khách hàng
- **Website giới thiệu** (nodi.vn) — landing page, bảng giá, tính năng, liên hệ

### Mục Tiêu

| Mục tiêu | Mô tả |
|-----------|-------|
| **Số hóa quản lý** | Chuyển đổi các cửa hàng vật tư nông nghiệp từ quản lý giấy sang số |
| **Quản lý đa chi nhánh** | Hỗ trợ chuỗi cửa hàng với store groups |
| **Phân tích thị trường** | Market Intelligence — phân tích xu hướng bán hàng theo vùng miền |
| **Hỗ trợ trực tuyến** | Chat realtime giữa khách hàng và admin qua WebSocket |
| **Backup cloud** | Sao lưu dữ liệu cửa hàng lên cloud tự động |

---

## 2. Kiến Trúc Hệ Thống

```
┌──────────────────────────────────────────────────────────┐
│                    NGINX (Reverse Proxy)                 │
│                 Port 80 / 443 (SSL)                      │
├───────────────┬──────────────────┬───────────────────────┤
│               │                  │                       │
│  nodi.vn      │  /api/*          │  Web App              │
│  (Static)     │  Axum API        │  Nuxt 3 SSR           │
│               │  (Rust)          │  (Vue 3)              │
└───────────────┴────────┬─────────┴───────────────────────┘
                         │
                ┌────────┴────────┐
                │  PostgreSQL 16  │
                │   (Database)    │
                └─────────────────┘
```

### Kiến trúc triển khai

- **Containerized** trên Docker Compose
- **4 services**: Nginx, Axum API, Nuxt Web, PostgreSQL
- **Network**: Bridge network nội bộ (`nodi-network`)
- **SSL**: Termination tại Nginx

---

## 3. Công Nghệ & Hạ Tầng

### 3.1 Backend API

| Thành phần | Công nghệ | Phiên bản |
|------------|-----------|-----------|
| **Ngôn ngữ** | Rust | Edition 2021 |
| **Web Framework** | Axum | 0.8 |
| **Async Runtime** | Tokio | 1.x (full features) |
| **Database Driver** | SQLx | 0.8 (PostgreSQL + Chrono) |
| **Authentication** | jsonwebtoken + bcrypt | JWT 9 / bcrypt 0.16 |
| **WebSocket** | Axum WS | Tích hợp trong Axum |
| **CORS** | tower-http | 0.6 |
| **Logging** | tracing + tracing-subscriber | 0.1 / 0.3 |
| **UUID** | uuid v4 | 1.x |
| **Serialization** | serde + serde_json | 1.x |

#### API Route Modules (11 modules)

| Module | Chức năng |
|--------|-----------|
| `auth.rs` | Đăng ký, đăng nhập, JWT authentication |
| `admin.rs` | Quản trị cửa hàng, users, thống kê |
| `dashboard.rs` | Dashboard cho store owners |
| `sync.rs` | Đồng bộ dữ liệu từ mobile app |
| `backup.rs` | Quản lý backup files |
| `license.rs` | Quản lý license keys |
| `market.rs` | Market Intelligence analytics |
| `support.rs` | Hệ thống hỗ trợ/ticket |
| `ws_support.rs` | WebSocket chat realtime |
| `health.rs` | Health check endpoint |

### 3.2 Frontend Web

| Thành phần | Công nghệ | Phiên bản |
|------------|-----------|-----------|
| **Framework** | Nuxt 3 | ^3.16.0 |
| **UI Framework** | Vue 3 | ^3.5.0 |
| **Styling** | TailwindCSS | ^6.14.0 (Nuxt module) |
| **Router** | Vue Router | ^4.5.0 |
| **Icons** | Lucide Vue Next | ^0.575.0 |
| **Charts** | Chart.js + vue-chartjs | ^4.5.1 / ^5.3.3 |
| **Dark Mode** | @nuxtjs/color-mode | ^4.0.0 |

#### Cấu trúc Pages

**Trang công khai (Public):**
| Trang | File | Mô tả |
|-------|------|-------|
| Trang chủ | `index.vue` | Landing page giới thiệu |
| Tính năng | `tinh-nang.vue` | Chi tiết tính năng sản phẩm |
| Bảng giá | `bang-gia.vue` | Gói Basic, Pro, Lifetime |
| Tải ứng dụng | `tai-ung-dung.vue` | Download app |
| Liên hệ | `lien-he.vue` | Form liên hệ |
| Đăng nhập | `login.vue` | Đăng nhập user |
| Đăng ký | `register.vue` | Đăng ký tài khoản |
| Chính sách bảo mật | `chinh-sach-bao-mat.vue` | Privacy policy |
| Điều khoản sử dụng | `dieu-khoan-su-dung.vue` | Terms of service |
| Blog | `blog/` | Bài viết, hướng dẫn |

**Dashboard (Store Owner):**
| Trang | File | Mô tả |
|-------|------|-------|
| Tổng quan | `dashboard/index.vue` | Overview dashboard |
| Đơn hàng | `dashboard/don-hang.vue` | Quản lý đơn hàng |
| Tồn kho | `dashboard/ton-kho.vue` | Quản lý tồn kho |
| Công nợ | `dashboard/cong-no.vue` | Quản lý công nợ |
| Báo cáo | `dashboard/bao-cao.vue` | Báo cáo & thống kê |
| Sao lưu | `dashboard/backup.vue` | Quản lý backup |
| Cài đặt | `dashboard/cai-dat.vue` | Cài đặt cửa hàng |

**Admin Panel:**
| Trang | File | Mô tả |
|-------|------|-------|
| Tổng quan | `admin/index.vue` | Admin dashboard |
| Cửa hàng | `admin/cua-hang.vue` | Quản lý cửa hàng |
| License | `admin/license.vue` | Quản lý license |
| Thị trường | `admin/thi-truong.vue` | Market Intelligence |
| Hỗ trợ | `admin/support.vue` | Chat support |
| Backup | `admin/backup.vue` | Backup management |
| Hệ thống | `admin/he-thong.vue` | System settings |

### 3.3 Database

| Thành phần | Công nghệ |
|------------|-----------|
| **DBMS** | PostgreSQL 16 Alpine |
| **Database** | `nodi` |
| **User** | `nodi_admin` |
| **Port** | 5432 (chỉ localhost) |

#### Database Schema — 20+ Tables

**Core Tables:**
- `stores` — Thông tin cửa hàng (license, địa chỉ, loại gói)
- `users` — Tài khoản đăng nhập (store_owner, admin)
- `employees` — Nhân viên cửa hàng (cashier, manager)
- `store_groups` — Nhóm chuỗi cửa hàng (multi-branch)
- `backup_files` — Metadata file backup

**Synced Data Tables (từ mobile app):**
- `synced_products` — Sản phẩm
- `synced_customers` — Khách hàng
- `synced_invoices` — Hóa đơn bán hàng
- `synced_invoice_items` — Chi tiết hóa đơn
- `synced_invoice_payments` — Thanh toán hóa đơn
- `synced_suppliers` — Nhà cung cấp
- `synced_purchase_orders` — Đơn nhập hàng
- `synced_purchase_items` — Chi tiết đơn nhập
- `synced_customer_transactions` — Giao dịch công nợ khách hàng
- `synced_supplier_transactions` — Giao dịch nhà cung cấp
- `synced_product_units` — Đơn vị tính sản phẩm
- `synced_product_batches` — Lô hàng (hạn sử dụng)
- `synced_product_transactions` — Biến động kho
- `synced_store_funds` — Quỹ tiền cửa hàng
- `synced_cash_transactions` — Thu chi tiền mặt
- `synced_payment_vouchers` — Phiếu chi
- `synced_store_settings` — Cài đặt cửa hàng

### 3.4 Infrastructure & DevOps

| Thành phần | Công nghệ |
|------------|-----------|
| **Containerization** | Docker + Docker Compose |
| **Reverse Proxy** | Nginx Alpine |
| **SSL** | Let's Encrypt (Nginx SSL termination) |
| **Data persistence** | Docker volumes (`./data/postgres`) |
| **Backup storage** | Volume mount (`./backups`, `./pg_backups`) |
| **Logs** | Volume mount (`./logs/nginx`) |
| **Networking** | Docker bridge network |

### 3.5 Bảo Mật

| Cơ chế | Chi tiết |
|--------|----------|
| **Authentication** | JWT tokens (jsonwebtoken) |
| **Password** | bcrypt hashing |
| **License** | Unique license key per store |
| **Database** | Port chỉ expose localhost (127.0.0.1) |
| **Environment** | Secrets qua `.env` file |
| **CORS** | Cấu hình qua tower-http |
| **Employee PIN** | PIN code cho nhân viên cashier |

---

## 4. Mô Hình Kinh Doanh

### Gói License

| Gói | Mô tả |
|-----|-------|
| **Basic** | Gói cơ bản — tính năng quản lý cơ bản |
| **Pro** | Gói nâng cao — đầy đủ tính năng |
| **Lifetime** | Gói trọn đời — thanh toán một lần |

### Tính Năng Chính Của Sản Phẩm

1. **Quản lý bán hàng** — POS, hóa đơn, thanh toán
2. **Quản lý tồn kho** — Sản phẩm, đơn vị tính, lô hàng, hạn sử dụng
3. **Quản lý nhập hàng** — Đơn nhập, nhà cung cấp, phiếu chi
4. **Công nợ** — Công nợ khách hàng, nhà cung cấp, giao dịch theo mùa vụ
5. **Quản lý quỹ** — Thu chi tiền mặt, quỹ cửa hàng
6. **Báo cáo** — Doanh thu, tồn kho, công nợ, biến động kho
7. **Backup cloud** — Sao lưu & khôi phục dữ liệu
8. **Đa chi nhánh** — Store groups, quản lý nhân viên
9. **Market Intelligence** — Phân tích xu hướng thị trường

---

## 5. Đối Tượng Khách Hàng

- Cửa hàng vật tư nông nghiệp (phân bón, thuốc BVTV, hạt giống...)
- Đại lý phân phối nông sản
- Chuỗi cửa hàng vật tư

---

## 6. Liên Hệ & Thông Tin

| Thông tin | Chi tiết |
|-----------|----------|
| **Website** | nodi.vn |
| **Admin Phone** | 0374222326 |
| **Repository** | `/opt/nodi` |

---

## 7. Cấu Trúc Thư Mục Dự Án

```
/opt/nodi/
├── api/                    # Backend API (Rust/Axum)
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src/
│       ├── main.rs         # Entry point, router setup
│       ├── config.rs       # Configuration
│       ├── db.rs           # Database operations
│       ├── error.rs        # Error handling
│       ├── middleware/      # Auth middleware
│       ├── models/         # Data models (store, user)
│       └── routes/         # API route handlers (11 modules)
├── web/                    # Frontend (Nuxt 3/Vue 3)
│   ├── package.json
│   ├── Dockerfile
│   ├── pages/              # Nuxt pages (auto-routing)
│   ├── components/         # Vue components
│   ├── layouts/            # Layout templates
│   ├── composables/        # Vue composables
│   └── assets/             # Static assets
├── nodi.vn/                # Static landing page assets
├── nginx/                  # Nginx configuration
│   ├── nginx.conf
│   ├── conf.d/
│   └── ssl/
├── docker-compose.yml      # Container orchestration
├── init.sql                # Initial database schema
├── migration_sprint3.sql   # Sprint 3 migrations
├── migration_sprint10.sql  # Sprint 10 migrations
├── scripts/                # Utility scripts
├── backups/                # App backup files
├── pg_backups/             # PostgreSQL backups
├── logs/                   # Application logs
└── .env                    # Environment variables
```
