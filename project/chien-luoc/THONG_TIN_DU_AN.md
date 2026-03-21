# 📋 THÔNG TIN DỰ ÁN — NODI POS

> **Cập nhật**: 13/03/2026 | **Version**: 2.0.0 | **Sprint**: 113
> **Founder**: Hoa | **Mô hình**: Solopreneur (Bus Factor = 1)
> **Website**: [nodi.vn](https://nodi.vn)

---

## 1. Tổng Quan

**Nodi POS** là phần mềm quản lý bán hàng (Point of Sale) dạng SaaS, chuyên biệt cho **đại lý vật tư nông nghiệp** tại Việt Nam. Kiến trúc **offline-first** cho phép hoạt động không cần Internet — dữ liệu lưu local trước, đồng bộ cloud khi có mạng.

| Thông tin | Chi tiết |
|-----------|---------|
| **Sản phẩm** | Phần mềm quản lý bán hàng POS SaaS |
| **Thị trường mục tiêu** | Đại lý vật tư nông nghiệp VN (40-60 tuổi, nông thôn) |
| **Bắt đầu phát triển** | 31/01/2026 |
| **Thời gian phát triển** | ~41 ngày → 113 sprints |
| **LOC** | ~85,000+ dòng code |
| **Test cases** | 1,238 (100% pass) |
| **Giấy phép kinh doanh** | Đang trong quá trình đăng ký |

---

## 2. Tech Stack

### Kiến trúc tổng thể

```
         Cloudflare (SSL + CDN + DDoS Protection)
                    │
               nodi.vn:443
                    │
             ┌──────┴──────┐
             │    Nginx    │ (Docker: nginx:alpine)
             └──────┬──────┘
             ┌──────┴──────┐
             │             │
       nodi.vn/*     /api/* (internal)
             │             │
      ┌──────┴────┐  ┌─────┴─────┐
      │  Nuxt 3   │  │   Axum    │ ◄── sync ── Tauri App (Desktop/Mobile)
      │  (SSR+SPA)│  │  (Rust)   │              │
      └───────────┘  └─────┬─────┘          SQLite (local)
                           │
                    ┌──────┴──────┐
                    │ PostgreSQL  │ (Docker: postgres:16-alpine)
                    │  16-alpine  │
                    └─────────────┘
```

### Stack chi tiết

| Layer | Công nghệ | Phiên bản |
|-------|-----------|:---------:|
| **Desktop App** | Tauri v2 + Vue 3 (Composition API) + TypeScript | v2.0.0 |
| **Mobile App** | Tauri v2 Mobile + Vue 3 | Full Parity (Sprint 113) |
| **Backend API** | Rust Axum | Custom build |
| **Database (Cloud)** | PostgreSQL | 16-alpine |
| **Database (Local)** | SQLite | Embedded |
| **Website** | Nuxt 3 (SSR + SPA) | Live |
| **AI Chatbot** | 100% Offline NLP Engine | Level 8 |
| **Containers** | Docker Compose (4 services) | v29.2.1 |
| **Reverse Proxy** | Nginx | alpine |
| **CDN/SSL/WAF** | Cloudflare (Origin CA) | N/A |
| **CI/CD** | GitHub Actions | N/A |

### Ngôn ngữ lập trình

| Ngôn ngữ | Sử dụng cho |
|----------|------------|
| **Rust** | Backend Axum, Tauri core, SQLite operations, IPC commands |
| **TypeScript** | Vue 3 frontend, AI/NLP engine, services, composables |
| **Vue 3** | UI components (Composition API, `<script setup>`) |
| **SQL** | SQLite migrations (44 files), PostgreSQL queries |
| **CSS** | Styling (Vanilla CSS, Glassmorphism, Dark mode ready) |

---

## 3. Cấu Hình VPS (Production)

> Báo cáo từ VPS Agent — 13/03/2026

| Thông số | Giá trị |
|----------|:-------:|
| **Provider** | Contabo GmbH (Đức) |
| **OS** | Ubuntu 24.04.4 LTS (Noble Numbat) |
| **CPU** | AMD EPYC 9354P — 2 vCPU |
| **RAM** | 7.8 GiB (6.1 GiB available) |
| **Swap** | 2.0 GiB |
| **Disk** | 96 GB SSD (52 GB free — 53%) |
| **Hypervisor** | KVM (QEMU) |
| **IPv6** | 2a02:4780:5e:e673::1 |
| **Docker** | v29.2.1 |

### Docker Containers

| Container | Image | Ports | Uptime |
|-----------|-------|:-----:|:------:|
| `nodi-nginx` | nginx:alpine | 80, 443 | 9 ngày |
| `nodi-postgres` | postgres:16-alpine | 127.0.0.1:5432 | 13 ngày |
| `nodi-api` | nodi-axum-api (custom) | 3000 (internal) | 8 ngày |
| `nodi-web` | nodi-nuxt-web (custom) | 3001 (internal) | 8 ngày |

### PostgreSQL Config (Trong Docker)

| Tham số | Hiện tại | Khuyến nghị (8GB RAM) |
|---------|:--------:|:---------------------:|
| `shared_buffers` | **128MB** ⚠️ | 2GB |
| `work_mem` | **4MB** ⚠️ | 16-32MB |
| `max_connections` | 100 | 100 ✅ |
| `effective_cache_size` | 4GB | 4-5GB |
| `maintenance_work_mem` | 64MB | 256MB |
| Database size | **12 MB** | Mới bắt đầu |

### Nginx Config

| Tính năng | Trạng thái |
|-----------|:----------:|
| SSL (Cloudflare Origin CA) | ✅ |
| Gzip | ✅ |
| Rate Limiting (api/auth/sync) | ✅ |
| WebSocket (/api/support/ws) | ✅ |
| HTTP→HTTPS redirect | ✅ |
| www→non-www redirect | ✅ |
| Security Headers | ⚠️ Chưa có |

### Bảo Mật VPS

| Hạng mục | Trạng thái |
|----------|:----------:|
| UFW Firewall | ✅ Active (22, 80, 443) |
| SSH Key-Only | ⚠️ Chưa cấu hình |
| PermitRootLogin | 🔴 **yes** (cần sửa) |
| SSH port | 🔴 **22** mặc định |
| Fail2ban | ❌ Chưa cài |
| Monitoring (Netdata) | ❌ Chưa cài |
| Auto security updates | ❌ Chưa cài |

### Backup

| Hạng mục | Trạng thái |
|----------|:----------:|
| pg_dump hàng ngày | ✅ 2h + 3h sáng |
| Retention | ✅ 14 ngày (gzip) + 7 files (SQL) |
| Off-site (Cloud R2) | ❌ Chỉ local |
| Mã hóa backup | ❌ Chưa có |
| Verification tự động | ❌ Chưa có |

---

## 4. Ứng Dụng Desktop (Tauri v2)

| Thông số | Giá trị |
|----------|:-------:|
| **Framework** | Tauri v2 + Vue 3 + TypeScript |
| **Backend** | Rust (IPC commands → SQLite) |
| **Database** | SQLite (44 migrations) |
| **Bundle size** | ~10 MB |
| **RAM usage** | ~50 MB |
| **Startup time** | <1 giây |
| **OS hỗ trợ** | Windows 10/11 (WebView2) |
| **Installer** | NSIS (per-user, bỏ qua UAC) |
| **Auto-update** | Tauri plugin-updater (passive mode) |
| **Offline** | ✅ 100% — đồng bộ khi có mạng |
| **Files mã nguồn** | ~280+ files |
| **Components** | ~50+ Vue components |
| **Views** | 12 routes |

### Cấu trúc thư mục

```
src/
├── components/          # ~50+ Vue components
│   └── mobile/          # 20+ mobile-specific components
├── composables/         # Vue composables (useAuth, useSimpleMode...)
├── layouts/             # MainLayout.vue
├── router/              # Vue Router
├── services/            # AI, QR, Print, Ban Check...
│   └── ai/              # NLP modules (~25 files)
├── stores/              # Pinia stores (posStore, etc.)
├── types/               # TypeScript definitions
└── views/               # 12 view pages

src-tauri/
├── src/
│   ├── db/              # Rust DB modules (products, invoices, customers...)
│   ├── lib.rs           # Tauri command registration
│   └── *_commands.rs    # IPC command handlers
└── migrations/          # 44 SQL migration files
```

---

## 5. Ứng Dụng Mobile

| Thông số | Giá trị |
|----------|:-------:|
| **Framework** | Tauri v2 Mobile (cùng codebase Desktop) |
| **Android** | ✅ APK signed release |
| **iOS** | 📋 Kế hoạch (Codemagic cloud build) |
| **Components** | 20+ mobile-specific |
| **Routes** | 8 routes (`/m/*`) |
| **Parity với Desktop** | ✅ 15/15 (Full Parity — Sprint 113) |

### Features Mobile

- Dashboard, POS, Cart, Checkout, Inventory, Orders
- Customers, Customer Detail (4-tab full page)
- Settings, Batch Trace + QR code
- Touch-optimized: bottom nav, floating FAB, sheet cart
- Glassmorphism header, dark mode ready
- Ban Check: POS guard + badges + stock alerts

---

## 6. Tính Năng Sản Phẩm (21 Nhóm)

| # | Nhóm | Mô tả |
|:-:|------|-------|
| 1 | 🛒 **Bán Hàng POS** | Quét barcode, giỏ hàng, thanh toán (tiền mặt/CK/kết hợp), VietQR, ghi nợ, in hóa đơn nhiệt |
| 2 | 📦 **Quản Lý Kho** | Danh mục SP, nhập/xuất, cảnh báo tồn kho/hạn dùng, thao tác hàng loạt, badges thuốc cấm |
| 3 | 🚛 **Nhập Hàng & NCC** | Phiếu nhập, quản lý NCC, công nợ NCC |
| 4 | 👥 **Khách Hàng** | Sổ nợ, công nợ vụ mùa, tách nợ 3 loại, filter GD, hồ sơ 4 tabs |
| 5 | 💰 **Quỹ Tiền Mặt** | Thu chi, trả nợ NCC, số dư quỹ |
| 6 | 📊 **Báo Cáo** | Dashboard, doanh thu ngày/tuần/tháng, top SP, so sánh kỳ |
| 7 | 📋 **Chốt Sổ** | Tóm tắt ngày, đóng sổ, lịch sử |
| 8 | 🧾 **Thuế & Kế Toán** | GTGT (0/5/10%), TNCN hộ KD (TT 40/2021), thuế khoán |
| 9 | 📄 **Hóa Đơn Điện Tử** | VNPT / Viettel / MISA, xuất tự động, theo dõi trạng thái |
| 10 | 🤖 **AI Chatbot** | 46+ intents, 100% offline, chẩn đoán bệnh (315+ entries), tra SP, doanh thu |
| 11 | ☁️ **Cloud & Sync** | Offline-first, Smart-Sync, cloud backup, khôi phục |
| 12 | 🔒 **Bảo Mật** | Multi-device (1 license = 10 devices), HWID, auto-recovery |
| 13 | 👨‍💼 **Nhân Viên** | Chủ/NV, 9 permission flags, PIN riêng |
| 14 | 🔄 **Auto-Update** | OTA kiểm tra + tải + cài trong app |
| 15 | 🆘 **Hỗ Trợ** | Chat WebSocket + Zalo |
| 16 | 🖨️ **In Ấn** | Receipt printer, barcode/nhãn SP (Code 128, 80mm) |
| 17 | 📱 **Quét Từ Xa** | Điện thoại → quét barcode qua QR |
| 18 | 🎁 **Khuyến Mãi** | Giảm giá %, cố định, mua X tặng Y, voucher code |
| 19 | 🔒 **Thuốc Cấm** | 3 tầng bảo vệ (Inventory → POS → Backend), HARD/SOFT ban |
| 20 | 📦 **Truy Xuất Nguồn Gốc** | Mã lô tự động, QR code, forward trace |
| 21 | 📱 **Mobile App** | 20+ components, 8 routes, Full Parity |

---

## 7. AI Chatbot — Module Đặc Biệt

| Metric | Giá trị |
|--------|:-------:|
| **Intents** | 46+ |
| **Disease Expert DB** | 315+ entries (8 loại cây) |
| **Product DB** | 5,656 SP (11 files) |
| **Knowledge bases** | 5 files (farming, fertilizer, tax, FAQ, templates) |
| **Test cases** | 1,238 (100% pass) |
| **ML model** | MiniLM ONNX int8 (~30MB) offline |
| **NLP modules** | ~25 files |
| **Processing** | 100% offline cho core |

### Pipeline

```
Input → Sanitize → CONFIRM/REJECT → VietNormalizer → VietTokenizer
     → SynonymEngine → PhoneticMatcher → normalize()
     → 48 Intent Patterns → NgramScorer fallback
     → ContextBooster → QuerySplitter → ReasoningGate → Response
```

---

## 8. Mô Hình Kinh Doanh

### Pricing (Freemium)

| Gói | Giá | Giới hạn |
|-----|:---:|----------|
| **Dùng thử** | **Miễn phí** | 30 ngày PRO, sau đó 20 đơn/ngày |
| **Theo tháng** | **299.000₫** | Không giới hạn |
| **Theo năm** | **1.990.000₫** | Không giới hạn (tiết kiệm 44%) |

> **Chiến lược**: Tất cả tính năng mở cho TẤT CẢ users. Khác biệt duy nhất: Free = 20 đơn/ngày.

### Thanh toán
- **Desktop/Android**: VietQR (chuyển khoản) → Web Portal nodi.vn → **0% phí**
- **iOS (dự kiến)**: Multiplatform Service model → **0% Apple Tax**

---

## 9. Nền Tảng Phát Hành

| Nền tảng | Trạng thái | Chi phí khởi tạo |
|----------|:----------:|:-----------------:|
| 🖥️ **Desktop Windows** | ✅ Sẵn sàng | Sectigo OV ~$226 |
| 🤖 **Android** | ✅ APK signed | $25 (Play Store) |
| 🍎 **iOS** | 📋 Kế hoạch | ~$150-200 (Apple Dev + Codemagic) |
| 🌐 **Website** | ✅ Live (nodi.vn) | Đã triển khai |

### Chi phí vận hành

| Hạng mục | Chi phí/năm |
|----------|:-----------:|
| VPS Contabo | ~$100 |
| Domain nodi.vn | ~$15 |
| Cloudflare | $0 (Free plan) |
| Sectigo OV (code signing) | ~$226 |
| Apple Developer | $99 (khi triển khai iOS) |
| Google Play | $25 (1 lần) |
| Monitoring | $0 (Netdata + BetterStack Free) |

---

## 10. Database Schema (SQLite — 44 Migrations)

### Bảng chính

| Bảng | Mô tả |
|------|-------|
| `products` | Sản phẩm (5,656+ BVTV) |
| `product_batches` | Lô hàng, mã lô, HSD |
| `customers` | Khách hàng, CCCD |
| `suppliers` | Nhà cung cấp |
| `invoices` | Hóa đơn bán hàng |
| `invoice_items` | Chi tiết hóa đơn |
| `debts` | Công nợ KH/NCC |
| `cash_fund_transactions` | Thu chi quỹ |
| `crop_seasons` | Vụ mùa (HT2025, XH2026...) |
| `banned_ingredients` | Hoạt chất cấm/hạn chế |
| `promotions` | Chương trình khuyến mãi |
| `vouchers` | Mã voucher |
| `staff_members` | Nhân viên + PIN + permissions |
| `daily_closings` | Chốt sổ cuối ngày |
| `devices` | Thiết bị đã đăng ký (max 10) |
| `schema_migrations` | Version control migrations |

---

## 11. Cấu Trúc Dự Án

```
agri-pos-tauri/
├── src/                          # Frontend Vue 3 + TypeScript
│   ├── components/               # ~50+ Vue components
│   ├── composables/              # Reusable logic
│   ├── services/                 # AI engine, Print, QR, Ban Check
│   ├── stores/                   # Pinia state management
│   ├── views/                    # 12 page views
│   └── router/                   # Vue Router
│
├── src-tauri/                    # Tauri v2 + Rust backend
│   ├── src/                      # Rust source
│   │   ├── db/                   # Database modules
│   │   └── *_commands.rs         # IPC command handlers
│   └── migrations/               # 44 SQL migrations
│
├── project/                      # Tài liệu dự án
│   ├── chien-luoc/               # Chiến lược (7 folders)
│   │   ├── ke-hoach-ra-mat/      # 5 tài liệu launch strategy
│   │   ├── mo-hinh-kinh-doanh/   # Mô hình kinh doanh
│   │   ├── doi-thu/              # Phân tích đối thủ
│   │   ├── tiep-thi/             # Marketing (chưa làm)
│   │   ├── nghien-cuu/           # Nghiên cứu thị trường
│   │   └── phan-tich-thieu-hut/  # Gap analysis
│   ├── legal/                    # Pháp lý
│   ├── CHANGELOG.md
│   ├── DEVELOPMENT_JOURNAL.md    # Nhật ký 113 sprints
│   ├── NODI_PLATFORM_ROADMAP.md  # Roadmap sản phẩm
│   └── THONG_TIN_DU_AN.md       # ← File này
│
├── ai_training_center/           # Test framework AI
├── roadmap/                      # Dev journal, research
├── briefs/                       # Agent task briefs
└── .agent/                       # Agent workflows & skills
```

---

## 12. Con Số Tổng Kết

| Metric | Giá trị |
|--------|:-------:|
| Sprints hoàn thành | **113** |
| Files mã nguồn | **280+** |
| LOC | **85,000+** |
| Test cases | **1,238** (100% pass) |
| Disease entries | **315+** |
| Products DB | **5,656** |
| AI Intents | **46+** |
| NLP modules | **~25 files** |
| SQLite migrations | **44** |
| Vue components | **50+** (Desktop) + **20+** (Mobile) |
| Docker containers | **4** |
| VPS API endpoints | **20+** |
| Nuxt pages | **~25** |
| Mobile parity | **15/15** (100%) |
| VTNN killer features | **3** (Thuốc cấm, Vụ mùa, Truy xuất) |
| Thời gian phát triển | **~41 ngày** (31/01 → 13/03/2026) |

---

## 13. Tính Năng So Với Đối Thủ

### So Với KiotViet

| Tính năng | Nodi POS | KiotViet |
|-----------|:--------:|:--------:|
| Offline-first | ✅ | ❌ |
| AI Chatbot chẩn đoán bệnh | ✅ | ❌ |
| Chuyên biệt VTNN | ✅ | ❌ (đa ngành) |
| Thuốc cấm 3 tầng bảo vệ | ✅ | ❌ |
| Truy xuất nguồn gốc lô | ✅ | ✅ |
| Công nợ vụ mùa | ✅ | ❌ |
| Desktop app nhẹ (10MB) | ✅ | ❌ (web-only) |
| Kết nối sàn TMĐT | ❌ | ✅ |
| Kết nối vận chuyển | ❌ | ✅ |
| HĐĐT lifecycle đầy đủ | ⚠️ Thiếu hủy/điều chỉnh | ✅ |

### Tính năng CHƯA CÓ (ưu tiên)

| # | Tính năng | Mức ưu tiên |
|:-:|-----------|:----------:|
| 1 | Kết nối sàn TMĐT (Shopee, Lazada) | 🟡 Chưa cần (VTNN ít bán online) |
| 2 | HĐĐT: Hủy / Điều chỉnh / Thay thế | 🟡 Cần làm |
| 3 | Gửi HĐ qua email/SMS | 🟡 Cần làm |
| 4 | Bluetooth print mobile | 🟡 Tương lai |
| 5 | Video đào tạo | 🔴 Cần gấp (Marketing) |
| 6 | Crop seasons CRUD UI | 🟡 Cần làm |

---

## 14. Liên Hệ & Tài Nguyên

| Tài nguyên | Link |
|-----------|------|
| Website | [nodi.vn](https://nodi.vn) |
| Tải Desktop | nodi.vn/tai-ve |
| GitHub | Private repo |
| Zalo OA | (Đang thiết lập) |
| Domain | Cloudflare DNS |

---

*Tài liệu nội bộ Nodi POS — Cập nhật 13/03/2026*
*Nguồn: NODI_FEATURE_LIST.md, SPEC_DRIVEN_OVERVIEW.md, VPS Agent Report, Knowledge Items*
