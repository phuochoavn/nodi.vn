# 🚀 NODI PLATFORM — Roadmap tổng thể

> **Ngày tạo**: 2026-02-15
> **Cập nhật**: 2026-03-12 22:30
> **Trạng thái**: Sprint 0→113 ✅ | Build: 0 errors | Tests: 1238/1238 (100%)
> **Chiến lược**: `project/chien-luoc/` — 6 báo cáo đối thủ + 4 nghiên cứu mô hình kinh doanh ✅
> **VPS**: Hostinger KVM 2 — Ubuntu 24.04 LTS, 2 CPU, 8GB RAM, 100GB SSD
> **Domain**: nodi.vn (Cloudflare DNS)
> **IP**: 76.13.189.151 (Malaysia)

---

## 🎯 Tầm nhìn

Nodi POS không chỉ là app bán hàng — mà là **nền tảng dữ liệu thị trường Bảo vệ Thực vật** (BVTV).
- **Tauri Desktop App**: POS offline tại cửa hàng (✅ ĐÃ CÓ v1.0.0)
- **Nodi.vn Web Platform**: Website + User Dashboard + Admin + Market Intelligence

---

## 🏗️ Kiến trúc đã thống nhất

### Tech Stack

| Layer | Công nghệ | Lý do |
|-------|-----------|-------|
| **Backend API** | **Rust Axum** | Nhanh, nhẹ (~30 MB RAM), tái sử dụng code từ Tauri |
| **Frontend** | **Nuxt 3** (Vue SSR + SPA hybrid) | SSR cho SEO (landing), SPA cho dashboard/admin |
| **Database** | **PostgreSQL 16** | Multi-tenant, aggregate queries cho market intelligence |
| **Containerization** | **Docker Compose** | Dễ deploy, reproducible |
| **Reverse Proxy** | **Nginx** | SSL termination, routing |
| **SSL/CDN** | **Cloudflare** (đã setup) | Free SSL, cache, WAF |
| **VPS Agent** | **Claude Opus 4** (CLI trên VPS) | Setup infra, deploy, debug |

### Kiến trúc tổng thể

```
          Cloudflare (SSL + CDN + WAF)
                     │
                nodi.vn:443
                     │
              ┌──────┴──────┐
              │    Nginx    │
              └──────┬──────┘
              ┌──────┴──────┐
              │             │
        nodi.vn/*     api (internal)
              │             │
       ┌──────┴────┐  ┌─────┴─────┐
       │  Nuxt 3   │  │   Axum    │ ◄── sync ── Tauri App
       │  (SSR+SPA)│  │  (Rust)   │
       │           │  │           │
       │ Landing   │  │ Auth      │
       │ Dashboard │  │ Sync      │
       │ Admin     │  │ Backup    │
       │ Blog      │  │ License   │
       │ Docs      │  │ Dashboard │
       └───────────┘  └─────┬─────┘
                            │
                     ┌──────┴──────┐
                     │ PostgreSQL  │
                     └─────────────┘
```

### Routing (1 domain)

```
nodi.vn/                    → Nuxt SSR — Trang chủ (SEO)
nodi.vn/tinh-nang           → Nuxt SSR — Tính năng (SEO)
nodi.vn/bang-gia            → Nuxt SSR — Bảng giá (SEO)
nodi.vn/tai-ung-dung        → Nuxt SSR — Download app (SEO)
nodi.vn/ve-chung-toi        → Nuxt SSR — Giới thiệu (SEO)
nodi.vn/lien-he             → Nuxt SSR — Liên hệ (SEO)
nodi.vn/blog/*              → Nuxt SSR — SEO content
nodi.vn/huong-dan/*         → Nuxt SSR — Tài liệu hướng dẫn
nodi.vn/login               → Nuxt SPA — Đăng nhập
nodi.vn/dashboard/*         → Nuxt SPA — User dashboard (chủ cửa hàng)
nodi.vn/admin/*             → Nuxt SPA — Admin (chỉ anh Hoa)
nodi.vn/api/*               → Proxy → Axum container
```

---

## 👥 Hệ thống người dùng

### Phân quyền

| Role | Ai | Quyền |
|------|-----|-------|
| `store_owner` | Chủ cửa hàng (mua license) | Xem dashboard CH mình, backup/restore |
| `admin` | Anh Hoa | Tất cả: tạo key, xem tất cả CH, market intelligence |

### Flow đăng ký tài khoản web

```
1. Chủ CH mua license key (qua Zalo/SĐT)
2. Cài Tauri app → nhập key + SĐT + tạo mật khẩu
3. App gửi lên api.nodi.vn → server tạo account tự động
4. Chủ CH vào nodi.vn/login → đăng nhập (SĐT + mật khẩu)
5. Redirect → nodi.vn/dashboard → xem số liệu cửa hàng
```

→ **Chỉ người mua license mới có tài khoản web. Không đăng ký tự do.**

---

## 🌐 Sitemap chi tiết

### Trang công khai (SSR — SEO)

| Route | Nội dung |
|-------|---------|
| `/` | Hero, USP, tính năng tóm tắt, testimonials, bảng giá, CTA |
| `/tinh-nang` | POS, nhập hàng, công nợ, HĐĐT, AI chatbot, báo cáo, backup cloud |
| `/bang-gia` | Gói: Dùng thử / Cơ bản / Chuyên nghiệp / Trọn đời |
| `/tai-ung-dung` | Download .exe + hướng dẫn cài đặt |
| `/ve-chung-toi` | Giới thiệu |
| `/lien-he` | Zalo, SĐT, form liên hệ |
| `/blog/*` | Bài SEO: "phần mềm quản lý cửa hàng vật tư", "HĐĐT nông nghiệp"... |
| `/huong-dan/*` | Tài liệu: cài đặt, bán hàng, công nợ, HĐĐT, backup... |
| `/chinh-sach-bao-mat` | Privacy policy |
| `/dieu-khoan-su-dung` | Terms of service |

### User Dashboard (SPA — sau login)

| Route | Nội dung | Quyền |
|-------|---------|-------|
| `/dashboard` | Tổng quan: doanh thu, đơn hàng, lợi nhuận, tồn kho | 👁️ |
| `/dashboard/don-hang` | Lịch sử bán hàng, chi tiết | 👁️ |
| `/dashboard/ton-kho` | Sản phẩm, số lượng, sắp hết, sắp hết hạn | 👁️ |
| `/dashboard/cong-no` | Danh sách nợ KH, NCC, lịch sử thanh toán | 👁️ |
| `/dashboard/bao-cao` | Doanh thu ngày/tháng, lãi lỗ, top sản phẩm | 👁️ |
| `/dashboard/hoa-don` | Trạng thái HĐĐT | 👁️ |
| `/dashboard/backup` | Tải backup / Khôi phục | ✅ THAO TÁC |
| `/dashboard/cai-dat` | Đổi mật khẩu, thông tin cửa hàng | ✏️ |

### Admin (SPA — chỉ admin)

| Route | Nội dung |
|-------|---------|
| `/admin` | Admin overview: tổng CH, tổng doanh thu, system health |
| `/admin/license` | Tạo key, gia hạn, thu hồi, xem trạng thái |
| `/admin/cua-hang` | Danh sách tất cả cửa hàng, trạng thái sync |
| `/admin/thi-truong` | 🏆 Market Intelligence (xem bên dưới) |
| `/admin/backup` | Quản lý backup tất cả cửa hàng |
| `/admin/he-thong` | System monitoring, logs |

---

## 🧠 Market Intelligence (Admin)

> Tận dụng data từ tất cả đại lý để phân tích thị trường BVTV.

### Insights có thể khai thác:

| Insight | Data source | Giá trị |
|---------|------------|---------|
| Top thuốc BVTV bán chạy | invoices + products | Biết xu hướng thị trường |
| Top nhà sản xuất theo doanh số | products + invoices | Biết doanh nghiệp nào mạnh |
| Xu hướng theo mùa vụ | invoices theo tháng | Dự đoán nhu cầu |
| Thuốc phổ biến theo khu vực | invoices + store location | Phân tích vùng miền |
| Hành vi nông dân | customers + invoices | Chi tiêu TB, mua nợ % |
| Nhà cung cấp hiệu quả | suppliers + purchase_orders | Ai cung ứng tốt |
| Giá bán thị trường | products từ nhiều CH | So sánh giá giữa các đại lý |

---

## 🔄 Data Sync Protocol

### Từ Tauri App → Server

```
Tauri App bán hàng → Lưu SQLite → Thêm sync_queue
→ Background job (mỗi 5 phút, khi có mạng)
→ POST /api/sync/push (batch JSON)
→ Server lưu PostgreSQL
→ Web dashboard tự refresh
```

### Bảng sync_queue (thêm vào Tauri app):

```sql
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    action TEXT NOT NULL,        -- INSERT, UPDATE, DELETE
    local_id INTEGER NOT NULL,
    data TEXT NOT NULL,           -- JSON snapshot
    synced INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now','localtime')),
    synced_at TEXT
);
```

---

## 💾 Backup System

### Cơ chế: Upload file .db

```
Tauri App (mỗi đêm hoặc thủ công)
→ Compress SQLite → .db.gz (~5-20 MB)
→ POST /api/backup/upload
→ Server lưu: /data/backups/store_{id}/YYYY-MM-DD.db.gz
→ Rotation: giữ 14 bản (7 ngày + 4 tuần + 3 tháng)
```

### Khi máy hư:

```
1. Cài máy mới → cài Nodi POS → nhập license
2. App tự tải backup mới nhất HOẶC
3. Chủ CH vào nodi.vn/dashboard/backup → download thủ công
4. Import → mọi thứ phục hồi: sản phẩm, khách hàng, công nợ, lịch sử
```

### Storage budget (VPS 100 GB):

| Thành phần | GB |
|-----------|:--:|
| Ubuntu + Docker | 5 |
| PostgreSQL (live data) | 10 |
| Backup .db files | 60 |
| Logs, temp | 5 |
| Dự phòng | 20 |
| **Tổng** | **100** |

→ Đủ cho **100-150 cửa hàng** với rotation 14 bản.

---

## 📅 Sprint Plan

### Sprint 0: App v1.1.0 — Nâng cấp tính năng ✅ (2026-02-15)
- [x] Trả hàng / Hoàn tiền (migration 027 + ReturnModal + InvoiceDetail integration)
- [x] Cảnh báo tồn kho thấp + sắp hết hạn (migration 028 + alert commands + min_stock field)
- [x] Chốt sổ cuối ngày (migration 029 + DailyClosingView + sidebar nav)
- [x] **Bonus**: Import sản phẩm từ Excel (calamine 0.26 + ImportProductsModal + auto-detect Vietnamese headers)

### Sprint 1: Hạ tầng VPS ✅ (2026-02-15)
- [x] Agent VPS (Claude Opus 4) setup Ubuntu
- [x] Docker + Docker Compose (2 containers: nodi-nginx, nodi-postgres)
- [x] Nginx + SSL (Cloudflare Origin CA, hết hạn 2041)
- [x] PostgreSQL 16 container (8 tables + 5 indexes)
- [x] Firewall UFW (22, 80, 443)
- [x] https://nodi.vn → HTTP/2 200 ✅
- [x] https://nodi.vn/health → OK ✅
- [x] Coming Soon page live

### Sprint 2: Axum API Skeleton ✅ (2026-02-15)
- [x] Axum project structure (`/opt/nodi/api/` — Rust, multi-stage Dockerfile)
- [x] Auth module (JWT + bcrypt, login-with-license)
- [x] License API (verify-license, check-activation, auto-recover HWID)
- [x] Health check endpoint (`/api/health` → OK)
- [x] CORS config (tower-http)
- [x] 6 endpoints deployed: health, verify-license, check-activation, login-with-license, check-update, sync(501)
- [x] 3 containers running: `nodi-api`, `nodi-nginx`, `nodi-postgres`
- [x] Admin user seeded (phone: 0000000000, role: admin)

### Sprint 3: Sync + Backup API ✅ (2026-02-15)
- [x] POST /api/sync (14 data collections, atomic transaction, upsert)
- [x] Identity: `X-License-Key` hoặc `X-HWID` header
- [x] POST /api/backup/upload (multipart, persistent volume)
- [x] GET /api/backup/download + /api/backup/list (JWT required)
- [x] Backup rotation (max 10 files/store)
- [x] DB migration: 12 bảng mới + 22 ALTER columns + 7 indexes
- [x] Body limit 50MB, incremental rebuild ~14s

### Sprint 4: Nuxt 3 Website ✅ (2026-02-15)
- [x] Nuxt 3 project setup + Dockerfile + docker-compose
- [x] Trang chủ (Hero, USP, Features, Pricing, CTA)
- [x] Trang tính năng chi tiết (6 sections)
- [x] Trang bảng giá (4 gói + FAQ)
- [x] Trang tải app + hướng dẫn
- [x] Trang liên hệ + Chính sách bảo mật
- [x] Nginx routing: /* → Nuxt, /api/* → Axum
- [x] 4 containers: nodi-web, nodi-api, nodi-nginx, nodi-postgres
- [x] SSR + SEO meta tags (og:title, lang=vi)

### Sprint 5: User Dashboard ✅ (2026-02-15)
- [x] Login page `/login` (SĐT + mật khẩu, JWT cookie)
- [x] Auth middleware: redirect về `/login` nếu chưa đăng nhập
- [x] 9 Axum API endpoints JWT-protected (`/api/dashboard/*`)
- [x] Dashboard layout: sidebar + header
- [x] Dashboard overview: 8 stat cards
- [x] Đơn hàng: paginated table + detail modal
- [x] Tồn kho: search/filter + low stock/expiry alerts
- [x] Công nợ: KH + NCC tabs
- [x] Báo cáo: revenue chart + top products
- [x] Backup: list + download
- [x] Cài đặt: store info + đổi mật khẩu

### Sprint 6: Admin + Intelligence ✅ (2026-02-18)
- [x] Admin dashboard (6 trang: overview, license, cửa hàng, thị trường, backup, hệ thống)
- [x] License manager (CRUD: tạo NODI-XXXX / thu hồi / gia hạn / reset HWID)
- [x] Danh sách cửa hàng (cards + sync status badges)
- [x] Market Intelligence v1 (top sản phẩm, revenue trend, top NCC)
- [x] Admin guard middleware (check role=admin)
- [x] 11 Axum admin API endpoints

### Sprint 7: Polish + Go Live ✅ (2026-02-18) — VPS tasks done, chờ switch Tauri app
- [x] Blog system (5 bài SEO, TOC sidebar, JSON-LD)
- [x] Hướng dẫn sử dụng (6 bài step-by-step, prev/next nav)
- [x] SEO (sitemap.xml 20 URLs, robots.txt, structured data)
- [x] Performance (Nginx gzip, /_nuxt/ cache 1yr immutable)
- [x] Trang pháp lý (điều khoản sử dụng)
- [x] Chuyển Tauri app → api.nodi.vn (thay quanly.hoadigital.com) ✅ 2026-02-19
- [ ] 🚀 GO LIVE

### Sprint 8: Admin Lifecycle & Billing (2026-02-19) ✅
- [x] Market Intelligence v2 (5 tabs: Tổng quan, Sản phẩm, Cung ứng, Tín dụng, Dòng tiền)
- [x] License lifecycle (5-state status, gia hạn GREATEST logic, thu hồi, kích hoạt lại)
- [x] Billing (ghi nhận thanh toán, lịch sử, tổng doanh thu theo phương thức)
- [x] Cửa hàng chi tiết (6 stat cards: SP, KH, ĐH, DT, nợ KH, nợ NCC)
- [x] Cảnh báo (key sắp hết hạn, hết hạn, cửa hàng inactive)
- [x] Build final Tauri app
- [ ] 🚀 GO LIVE

### Sprint 9: AI Offline-First Chatbot 🔄 (2026-02-21)

> **Tầm nhìn:** Chatbot thiên về offline, dần dần hoàn thiện để KHÔNG cần API bên ngoài (Groq/Gemini).
> Data bệnh hại do chuyên gia tạo, không phụ thuộc mô tả sản phẩm.

**Phase 1: Disease Knowledge DB ✅** (Agent Data — 21/02)
- [x] Tạo `src/data/disease_expert_db.ts` — 51 entries (29 bệnh, 20 sâu hại, 2 cỏ dại)
- [x] Schema: bệnh → triệu chứng → hoạt chất khuyên dùng → lời khuyên → liều lượng
- [x] Bao phủ: lúa (15), cây ăn trái (10), rau màu (10), cây công nghiệp (10), chung (6)
- [x] Cross-reference: 88.9% hoạt chất có trong `final_merged_db.ts` (4500 SP)

**Phase 2: Engine Upgrade ✅** (Agent Leader — 21/02)
- [x] Nâng cấp `DiseaseDiagnosisEngine.ts`: crop-aware scoring thay vì Array.find()
- [x] Flow: Bệnh → Hoạt chất → Tìm trong kho → Bổ sung từ danh mục
- [x] Thêm lời khuyên sử dụng, liều lượng cho mỗi chẩn đoán
- [x] `useChatProcessor.ts`: expert engine primary, Tauri IPC fallback

**Phase 3: Test & Verify ✅** (Agent Test — 21/02)
- [x] Test suite: `disease_db_tests.ts` (68 tests)
- [x] Baseline: 73.5% → Final: **89.7%** (61/68) ✅ Target ≥85%
- [x] Data Integrity 100%, Cross-Reference 100%, Engine Integration 100%
- [x] 7 edge cases còn lại = ambiguity tự nhiên ("héo vàng" match nhiều bệnh)

**Phase 4: Full Chatbot Audit ✅** (Agent Test — 21/02)
- [x] Audit 260 tests across 26 intent groups: **255/260 (98%)** ✅
- [x] 22/26 groups = 100%, chỉ 5 failures (pattern ordering)
- [x] Response quality: 31/37 "good", 6 "partial", 0 "bad"
- [x] **100% OFFLINE** — không intent nào bắt buộc cần internet 🎉
- [x] Report: `ai_training_center/reports/chatbot_audit_phase4.md`

**Phase 5: Polish & Enrich ✅** (Agent Intent + Agent Test — 22/02)
- [x] Fix 5 intent routing failures → 260/260 (100%)
- [x] Fix `hi` greeting + LICENSE_INFO detection
- [x] Improve PRODUCT_INQUIRY detection
- [ ] Improve CUSTOMER_CHURN: add `last_purchase_date` tracking
- [ ] Improve CUSTOMER_SEGMENT: purchase frequency data
- [ ] Build DEBT_REMINDER scheduling backend
- [ ] Bổ sung data: ĐHST, phân bón từ giáo trình (khi cần)

### Sprint 10: Security + Free User Access ✅ (2026-02-21 — Agent VPS)

**Phase 1: Security ✅** (`26cbaa2`)
- [x] CORS restrict → chỉ `nodi.vn`, `api.nodi.vn`, `localhost`
- [x] Rate limiting Nginx: auth 3r/m, sync 1r/s, API 10r/s
- [x] PostgreSQL backup cron: 3 AM daily, 7-day rotation
- [x] Swap 2GB active

**Phase 2: Free User Access ✅** (`5e7a512`)
- [x] `POST /api/auth/register` — tạo user + store + license `FREE-XXXX`
- [x] Refresh token flow (30 ngày)
- [x] Backup quota: Free 3 files/50MB, Pro 10 files/200MB
- [x] `register.vue` — trang đăng ký web, real-time validation
- [x] Login page → link "Đăng ký miễn phí"

**Phase 3: DB Schema ✅** (`1191abb`)
- [x] `employees` table (PIN, role: cashier/manager/owner, permissions JSONB)
- [x] `store_groups` table (multi-branch support)
- [x] `stores` → thêm `group_id`, `branch_name`
- [x] Tổng database: 25 tables

### Sprint 11: Nodi Brain Engine + Product DB Audit ✅ (2026-02-22)

**Phase 1: Product DB Restructure ✅**
- [x] Tách `final_merged_db.ts` (5,656 SP) → 11 files theo nhóm hàng
- [x] Thêm 2 nhóm mới: 🐀 Diệt chuột, 🔧 Dụng cụ
- [x] Barrel export `products/index.ts` — backward compatible

**Phase 2: Product Data Audit ✅** (4 rounds)
- [x] Audit `khac.ts`: 146 → 3 SP (143 reclassified)
- [x] Audit `thuoc_tru_benh.ts`: 104 doi_tuong filled, 1026 typos
- [x] Audit `thuoc_tru_sau.ts`: 90 doi_tuong filled, 1059 typos
- [x] Audit 7 files còn lại: 977 filled, 41 "ốc giả" caught
- [x] Tổng: ~1,171 doi_tuong filled, ~2,531 typos fixed, ~200 SP moved

**Phase 3: Knowledge Base ✅**
- [x] `farming_calendar_db.ts` — 16 entries (lịch mùa vụ)
- [x] `fertilizer_db.ts` — 27 entries (phân bón + ĐHST)
- [x] `tax_knowledge_db.ts` — 17 entries (thuế nông nghiệp)
- [x] `faq_db.ts` — 35 entries (FAQ app + nông nghiệp)
- [x] `response_templates_db.ts` — 40 entries (26 intents)

**Phase 4: Nodi Brain Engine ✅** (bỏ Cloud AI)
- [x] `NodiBrain.ts` — Orchestrator singleton
- [x] `ConversationMemory.ts` — nhớ 10 câu gần nhất
- [x] `NodiBrainEntityExtractor.ts` — tách tên KH, SP, số tiền
- [x] `FollowUpResolver.ts` — hiểu "nó/đó/thêm nữa"
- [x] `KnowledgeRouter.ts` — route đến tax/farming/FAQ KB
- [x] `SmartResponseGenerator.ts` — response tự nhiên từ templates
- [x] Bỏ tab "Cấu hình AI" trong Settings

### Sprint 12: Chatbot Intelligence V2 + Disease DB Expansion ✅ (2026-02-22)

> **6 agents xử lý song song, 8 modules hoàn thành trong 1 ngày**

**Module 1: Seasonal Awareness ✅** (Agent Seasonal)
- [x] `SeasonalAdvisor.ts` — lời chào + gợi ý theo mùa vụ

**Module 2: Multi-Symptom Scoring ✅** (Agent Scoring)
- [x] `detectDiseaseExpert()` trả top 3 thay vì 1 (`ScoredDiseaseEntry[]`)
- [x] `formatMultiDiagnosis()` hiển thị khi điểm gần nhau

**Module 3: Guided Diagnostic Flow ✅** (Agent Diagnostic)
- [x] `GuidedDiagnostic.ts` — hỏi step-by-step khi query vague

**Module 4: Cross-Sell Intelligence ✅** (Agent CrossSell)
- [x] `CrossSellEngine.ts` — gợi ý bán chéo sau chẩn đoán

**Module 5: Auto-Sync Disease Patterns ✅** (Agent Intent)
- [x] `DiseasePatternGenerator.ts` — Single Source of Truth
- [x] Thêm bệnh mới vào DB = tự động nhận diện (không cần sửa 3 file)

**Module 6+6b: Expand Disease Expert DB ✅** (Agent Data)
- [x] General expansion: 58 → 76 entries (+18)
- [x] Rice-specific expansion: 76 → 86 entries (+10 lúa: tungro, tiêm lửa, thối rễ, rầy lưng trắng, rầy xanh, sâu phao, sâu năn, sâu keo lúa, cỏ đồng tiền, lúa cỏ)
- [x] Tổng: 42 disease, 36 pest, 6 weed, 1 rodent, 1 snail

**Verification ✅** (Agent Test)
- [x] Regression: 260/260 (100%) 🎉
- [x] Expansion: 41/43 (95.3%)
- [x] V2 Intelligence: 39/41 (95.1%)
- [x] 86-Entry Intent: 85/86 (98.8%)
- [x] **TOTAL: 425/430 (98.8%)**

### Sprint 13: Chatbot Intelligence V3 — "Hiểu Nông Dân" ✅ (2026-02-22)

> **4 agents mới, 6 modules, 3 intents mới, 8 issues giải quyết**

**Module 1: CROP_ACTION Intent ✅** (Agent CropAction)
- [x] Intent mới: "kích đọt sầu riêng", "trước trổ phun gì", "bón gì giai đoạn mạ"
- [x] Wire `fertilizer_db.ts` (27 entries) + `farming_calendar_db.ts` vào chatbot
- [x] 30+ patterns: kích đọt, kích hoa, kích rễ, đón đòng, bón lót, bón thúc...

**Module 2: INGREDIENT_SEARCH ✅** (Agent Ingredient)
- [x] Intent mới: "Jasmonic acid", "có Abamectin không", "hoạt chất Hexaconazole"
- [x] `IngredientMatcher.ts` — auto-extract ingredients từ disease_expert_db + 50+ phổ biến
- [x] Tìm SP trong kho theo `hoat_chat` field (5,656 SP)

**Module 3: HERBICIDE_TYPE ✅** (Agent CropAction)
- [x] Intent mới: phân biệt "tiền nảy mầm" vs "hậu nảy mầm"
- [x] Pre: Pretilachlor, Butachlor, Pendimethalin
- [x] Post: 2,4-D, Glyphosate, Paraquat, Propanil

**Module 4: Intent Conflict Fix ✅** (Agent Fix)
- [x] "vi khuẩn" → DISEASE_DIAGNOSIS (was PRODUCT_SEARCH)
- [x] "cỏ đồng tiền" → DISEASE_DIAGNOSIS (was CASHFLOW)
- [x] "vỏ gạo" → DISEASE_DIAGNOSIS (was UNKNOWN, added alias)

**Module 5: Crop Name Guard ✅** (Agent Fix)
- [x] "Mít" → CROP_ACTION (was matching product "Mitop")
- [x] Blocklist 30+ crop names trong `searchWithKnowledge`

**Module 6: Short Query Guard + Polish ✅** (Agent Fix)
- [x] Min 3 chars trong product search → "JS" no longer false positive
- [x] Multi-diagnosis header: 🩺 + count thay vì 🔬

**Verification ✅**
- [x] Build: 12.85s, 0 errors
- [x] Regression: 260/260 (100%) 🎉

### Sprint 14: Nodi AI v3.5 — "Tư Duy Nhỏ" + UX Polish ✅ (2026-02-23)

> **3 agents mới, 5 modules, context awareness + personality + UI**

**Module 1: Entity Memory Chain ✅** (Agent Reasoning)
- [x] `lastProductObjects[]` + `lastCrop` trong ConversationMemory
- [x] Ordinal resolve: "cái đầu tiên" → product[0]
- [x] Implicit product: "lấy 2 chai" (no name) → resolve from memory

**Module 2: Smart Disambiguation ✅** (Agent Reasoning)
- [x] Symptom without crop → "Cây gì bị héo?"
- [x] Crop without intent → "Muốn biết gì về mít?"
- [x] `extractQueryHints()` in PragmaticAIEngine

**Module 3: FAQ Expansion ✅** (Agent Data)
- [x] 35 → 87 entries (+52 mới)
- [x] Agriculture: +22 (sinh học, vi lượng, IPM, đất, bẫy...)
- [x] App: +11 (scanner, backup, phân quyền...)
- [x] Business: +19 (marketing, NCC, cash flow...)

**Module 4: Response Personality ✅** (Agent Polish)
- [x] ~30 string replacements: "Tôi" → "Em", "Bạn" → "Anh"
- [x] Time-of-day greeting (☀️ sáng, 🌤️ trưa, 🌅 chiều, 🌙 tối, 😴 khuya)
- [x] Emoji + friendly tone throughout

**Module 5: Collapsible Messages ✅** (Agent Polish)
- [x] Messages >300 chars → truncated + gradient fade + "Xem thêm ▼"
- [x] Toggle expand/collapse + smooth CSS transition

**Verification ✅**
- [x] Build: 12.86s, 0 errors
- [x] Regression: 260/260 (100%) 🎉

### Sprint 15: Code Refactoring — Clean Architecture ✅ (2026-02-23)

> **Codebase hygiene: tách module, giảm coupling, chuẩn bị cho NLP expansion**

- [x] Extract `normalize.ts` — shared utility (was duplicated in 3 files)
- [x] Extract `intent-types.ts` — canonical IntentName type + FuzzyIntentResult
- [x] Extract `agri-constants.ts` — shared agricultural constants
- [x] Refactor `detectFuzzyIntent()` → intent-registry pattern (46 descriptors)
- [x] Extract `intent-patterns.ts` — all pattern arrays (single source of truth)
- [x] Extract `intent-helpers.ts` — helper functions
- [x] Refactor `processMessage()` → Handler Map (12 handler files)
- [x] Chat UI: Markdown rendering for bot messages
- [x] ConversationMemory: TTL mechanism (5-minute entity expiry)
- [x] New `logger.ts` utility — replace console.log in handlers

### Sprint 16: NLP Pipeline Core ✅ (2026-02-23)

> **Xây nền tảng NLP: 5 modules mới cho xử lý ngôn ngữ tự nhiên**

- [x] `SynonymEngine.ts` — Vietnamese agricultural synonym resolution (disease aliases, ingredient variants)
- [x] `PhoneticMatcher.ts` — 100+ known misspelling corrections
- [x] `NgramScorer.ts` — weighted n-gram scoring for intent matching
- [x] `ContextBooster.ts` — topic-aware intent boosting with time decay
- [x] NLP Pipeline barrel export `nlp/index.ts`
- [x] Input sanitization (garbage/empty rejection, min length guard)
- [x] Audit baseline: **56% (50/90)**

### Sprint 17: P0 Audit Fixes ✅ (2026-02-23)

> **Fix critical issues từ audit lần 1**

- [x] VietNormalizer word-boundary collisions (regex `\b` breaks Vietnamese)
- [x] Input length guard: allow single-char Vietnamese greetings ("ê", "ơi")
- [x] Audit score: **56% → 68% (61/90)**

### Sprint 18: "Trí Tuệ Nông Dân" — Intelligence Upgrade ✅ (2026-02-23)

> **5 modules nâng cấp trí tuệ chatbot, 2 audits gắt gao, 260/260 tests**

**Module 1: VietTokenizer ✅**
- [x] `VietTokenizer.ts` — word-level tokenizer, `replaceTokens()` greedy n-gram
- [x] Fix VietNormalizer: replace `\b` regex → VietTokenizer.replaceTokens()
- [x] Re-enable dialect: 'coi'→'xem', 'hổng'→'không', 'hồi sáng'→'sáng nay'

**Module 2: Confirmation & Rejection ✅**
- [x] 2 intents mới: CONFIRMATION ("ok", "ừ", "lấy") + REJECTION ("không", "thôi bỏ")
- [x] Guards: ≤6 words, no product context, ORDER_COMPLETE guard
- [x] `confirmHandler.ts` — pendingAction + globalLastSuggestedProducts
- [x] ConversationMemory: pendingAction slot with TTL

**Module 3: QuerySplitter ✅**
- [x] `QuerySplitter.ts` — tách compound queries ("A, còn B?" → 2 responses)
- [x] Anti-split guards: so sánh, còn hàng, với giá
- [x] useChatProcessor: `isSubQuery` parameter, recursion-safe

**Module 4: SynonymEngine Fix ✅**
- [x] Rewrite `resolve()` → VietTokenizer.replaceTokens() (was indexOf substring)
- [x] RESOLVE_BLOCKLIST ~40 entries (prevent common words from being replaced)
- [x] Re-enable in NLP pipeline: VietNormalizer → SynonymEngine → PhoneticMatcher

**Module 5: ContextBooster + NgramScorer Activation ✅**
- [x] NgramScorer smart fallback: score 7 key intents, threshold > 0.6
- [x] ContextBooster: boost low-confidence (0.5-0.8), topic-aware, 5-min TTL
- [x] ConversationMemory: getLastIntent(), getLastTurnTimeMs()

**Intent Confusion Audit ✅** (70 test cases)
- [x] 6 zones tested: CONFIRMATION collision, SynonymEngine, QuerySplitter, NgramScorer, ContextBooster, VietNormalizer
- [x] Score: **50/70 → 70/70 (100%)** — 0 collisions
- [x] 11 code fixes applied (P1: 5, P2: 4, P3: 2)

**Response Accuracy Audit ✅** (60 test cases)
- [x] 6 zones: Disease accuracy 4.2/5, Business 80%, Customer 80%, Multi-turn 70%, Fallback 87.5%, Quality 4.3/5
- [x] P0 fixed: Customer disambiguation (find→filter), FAQ_DB Layer 3.5 search
- [x] P1 fixed: Empty revenue ₫0→"không có dữ liệu", 5 nutritional entries, context-aware compare
- [x] P2 fixed: "mục"/"viền nâu" keywords, pendingAction, suggestedActions, TemporalParser tuần
- [x] Final: **260/260 tests**, disease DB 86→91+ entries (5 dinh dưỡng)

**Stats:**
- Intent count: 46 → **48** (+CONFIRMATION, +REJECTION)
- NLP pipeline: 1 stage → **6 stages**
- Disease DB: 86 → **91+** entries
- Audit: 56% → **100%** intent accuracy, 80%+ response accuracy

### Sprint 19: Advanced Features ✅ (2026-02-23)

> **GuidedDiagnostic + CrossSell + SeasonalAdvisor + Test Expansion + UI Chips**

**Agent 1: GuidedDiagnostic ✅**
- [x] Full rewrite: `DiagnosticSession` interface, 4-step flow (Crop → Part → Symptom → Detail)
- [x] `processAnswer()` + `scoreCandidates()` — score 95+ DB entries by matched keywords
- [x] Auto-skip: "lúa lá vàng mép lá" → all 4 steps filled → instant diagnosis
- [x] 10-min TTL in ConversationMemory + backward compat exports
- [x] Updated `diseaseHandler.ts` + `useChatProcessor.ts` LAYER -0.5

**Agent 2: CrossSell + SeasonalAdvisor ✅** (already implemented Sprint 12)
- [x] CrossSell: 9 category + 9 disease combo rules in `CrossSellEngine.ts` (385 lines)
- [x] SeasonalAdvisor: season mapping + greeting + advice in `SeasonalAdvisor.ts` (218 lines)
- [x] Wired: `diseaseHandler.ts`, `sellingHandler.ts`, `DiseaseDiagnosisEngine.ts` (+15 seasonal score)
- [x] Verified: 260/260 tests ✔️

**Agent 3: Test Expansion + UI Chips ✅**
- [x] Test suite: 260 → **390 tests** (+70 audit + 30 response + 30 NLP)
- [x] UI suggestion chips in `ChatAssistant.vue` (dark mode compatible)
- [x] `suggestedActions` wired: `HandlerContext` → `ChatMessage` → chips render
- [x] 390/390 ALL PASS 🎉

### Sprint 20: Level 7 — Sentence Transformer (MiniLM) ✅ (2026-02-23)

> **Offline AI model `multilingual-e5-small` (ONNX int8, ~30MB) — hiểu NGHĨA câu, không chỉ match từ**

**Agent 1: TF-IDF Baseline (TypeScript) ✅**
- [x] `TFIDFClassifier.ts` — TF-IDF vector + cosine similarity (~150 lines)
- [x] `build_tfidf_centroids.ts` — 41 intents, 931 vocab tokens, 60.4KB
- [x] Wired as secondary scorer in PragmaticAIEngine (confidence < 0.85 + fallback < 0.70)

**Agent 2: ONNX Runtime Setup (Rust) ✅**
- [x] Added `ort` 2.0.0-rc.11 + `tokenizers` 0.21 to Cargo.toml
- [x] `embedding.rs` — OnceLock<Mutex<Session>>, mean pooling, L2 normalize
- [x] `embedding_commands.rs` — `embed_text_cmd` + `is_model_loaded` IPC
- [x] `tauri.conf.json` — resources bundle config

**Agent 3: Centroid Builder + Classifier (Rust) ✅**
- [x] `export_training_cases.ts` — 382 unique cases → JSON
- [x] `build_centroids.rs` — Rust binary: embed all cases → per-intent centroids
- [x] `intent_classifier.rs` — `IntentClassifier` struct + cosine similarity + gap detection
- [x] `classify_intent_ml` IPC command (lazy-loads classifier)

**Agent 4: Frontend Hybrid Wiring + Disambiguation ✅**
- [x] `MLClassifierBridge.ts` — Tauri IPC bridge with cached `isMLModelAvailable()`
- [x] `disambiguationHandler.ts` — intent labels → Vietnamese, 2-choice prompt
- [x] Hybrid pipeline: Pattern (<0.85) → ML vote/override/boost → disambiguation
- [x] `confirmHandler.ts` — "1"/"2" disambiguation response handling

**Agent 5: Testing + Performance ✅**
- [x] 55 ML test cases (novel, ambiguous, synonym, colloquial, edge)
- [x] Benchmark: Pattern 100% / TF-IDF 62% / Hybrid 100% @ 2.57ms avg
- [x] `ML_CONFIG` thresholds extracted + documented
- [x] Total tests: 390 → **445** | Performance: 2.57ms < 15ms target ✅

### Sprint 21: Multi-Agent Chatbot Audit & Enhancement ✅ (2026-02-24)

> **5 agents chuyên biệt, 150 test cases audit, 11 entries mới, 100% pass rate**

**Agent 1: Audit Agent ✅** — 150 test cases, 7 zones (bacteria, weed, nutrition, dialect, collision...)
- Kết quả audit: 99/150 (66%) — phát hiện 51 failures

**Agent 2: Fix Agent ✅** — 5 bugs + 4 pattern fixes
- BUG 1-5: "vàng lá" CONFIRMATION→DISEASE, "nó bị cháy" DEBT→DISEASE, etc.
- PF1-4: ADD_TO_CART guard, AGRI_SINGLE_WORD_INTENTS, 14 dialect entries

**Agent 3: Data Agent ✅** — +11 entries (5 dinh dưỡng + 4 cỏ + 2 updated)
- SynonymEngine: +12 groups | PhoneticMatcher: +5 scientific name mappings

**Agent 4: Triage Agent ✅** — 32 failures → 14 ACCEPT, 3 WONT_FIX, 15 code fix

**Agent 5: Fix Agent R2 ✅** — 15 fixes → **595/595 (100%)** 🏆

### Sprint 22: Deep Agriculture Intelligence ✅ (2026-02-25)

> **4 agents, 18 new disease entries, Level 2 reasoning, Smart UX chips**

**Agent 1: Data Agent ✅** — +18 entries
| Category | Entries |
|----------|---------|
| Vi khuẩn lúa (4) | Sọc trong VK, Đốm sọc VK, Thối bẹ VK, Héo xanh VK |
| Cỏ dại (6) | Cỏ đuôi chồn, Cỏ bợ, Cỏ mần trầu, Lúa cỏ, Bèo tấm, Cỏ cú |
| Sầu riêng (8) | Nấm hồng, Thối rễ, Cháy lá, Sâu đục trái, Nhện đỏ, Rệp sáp, Thối hoa, Đốm rong |

- NLP collision fixes: "thối"↔"thôi", "nấm hồng sầu riêng" triple collision
- +26 test cases

**Agent 2: Brain Agent ✅** — Tư Duy Level 2
- `detectCategoryQuery()` — "vi khuẩn" → liệt kê nhóm bệnh
- 4-level confidence: L1 Sure → L2 Multi-diagnosis → L3 GuidedDiag → L4 Don't know
- Graceful fallback: "Em chưa tìm thấy..." + gợi ý cách hỏi
- Vietnamese word boundary fix: `(?:^|\s)` thay `\b`
- +15 test cases, +5 IT fixes

**Agent 3: Pipeline Agent ✅** — Smart Product Chips UX
- ✕ dismiss button, auto-clear mỗi message mới
- Stock icons ✅⚠️❌ + số lượng trên chips
- Click feedback + remove selected chip
- REJECTION "không"/"thôi" → clear chips

**Agent 4: Test Agent ✅** — Final Verification
- +20 regression tests (collision, category, boundary, new entries)
- **656/656 (100%)** — 0 bugs, 7/7 integration checks
- Build: 0 errors — TypeScript: 0 problems

### Sprint 23: Product Intelligence Enhancement ✅ (2026-02-25)

> **Product name recognition + rich product info handler**

- [x] `ProductNameIndex.ts` — fuzzy matching product names in user queries
- [x] `productInfoHandler.ts` — rich product info formatting (stock, price, active ingredient)
- [x] Wired into `useChatProcessor.ts` pipeline
- [x] **691/691 (100%)**

### Sprint 24: Symptom Chain Engine — Level 3 Reasoning ✅ (2026-02-25)

> **Multi-symptom combination rules for precise diagnosis**

- [x] `SymptomChainEngine.ts` — 40 symptom chain rules
- [x] 5 nhóm: Lúa bệnh (10), Lúa sâu/thiếu chất (10), Sầu riêng (5), Cà chua/ớt (5), Cà phê/tiêu (5), Chung (5)
- [x] Integrated into `diagnoseExpert()` scoring with scoreBoost + confidence
- [x] **691+ tests pass**

### Sprint 25: Direct Add-to-Cart ✅ (2026-02-25)

- [x] `suggestedActions` chips + `pendingAction` ADD_TO_CART in `useChatProcessor.ts`
- [x] `confirmHandler.ts` processes ADD_TO_CART pending action
- [x] "Add to Cart" chips only shown when product is in stock
- [x] +10 test cases → **691+ pass**

### Sprint 26: Multi-Step Parser ✅ (2026-02-25)

- [x] `MultiStepParser.ts` — parse compound queries into ordered action steps
- [x] `executeMultiSteps` integration in `useChatProcessor.ts` (Layer 0.9)
- [x] +12 test cases → **713 pass**

### Sprint 27: Context-Aware Follow-Up ✅ (2026-02-25)

- [x] `FollowUpResolver.ts` wired into `useChatProcessor.ts` pipeline
- [x] Conversation memory updated after each turn
- [x] Follow-up questions resolved using context
- [x] **713+ pass**

### Sprint 28: Hotfix — Pipeline Bug Fixes ✅ (2026-02-25)

- [x] Cart keyword guard for Layer 0.8
- [x] Chain reasoning rule fix for "bọ trĩ" vs "thiếu kali"
- [x] +30 response quality tests
- [x] **828 pass**

### Sprint 29: Smart Fallback ✅ (2026-02-25)

- [x] Partial intent detection in fallback
- [x] Suggested chips in `FallbackResult`
- [x] Context-aware fallback responses
- [x] **828+ pass**

### Sprint 30: Query Modifier Extraction ✅ (2026-02-25)

- [x] `QueryModifierExtractor.ts` — identify modifiers and superlatives
- [x] `modifierHandler.ts` — filter and sort products by modifier
- [x] Layer 0.7 in `useChatProcessor.ts`
- [x] +15 test cases

### Sprint 31: Response Variety ✅ (2026-02-25)

- [x] `ResponseVariety.ts` — randomized prefixes for different intents
- [x] Greeting closer variety in `SeasonalAdvisor.ts`
- [x] +10 test cases

### Sprint 32: Confidence-Aware Responses ✅ (2026-02-25)

- [x] `ConfidenceGate.ts` — HIGH/MEDIUM/LOW response levels
- [x] Confidence badge in responses
- [x] Applied to disease and product handlers
- [x] +15 test cases

### Sprint 33: User Feedback Feature ✅ (2026-02-25)

- [x] Feedback service (thumbs up/down) to local storage
- [x] UI elements for feedback on bot messages
- [x] Metadata wiring (query, intent, confidence)

### Sprint 34: Self-Verification Engine ✅ (2026-02-25)

- [x] `SelfVerifier.ts` — 3 types: DISEASE, PRODUCT, CROSS-CHECK
- [x] Integrated into `useChatProcessor.ts` pipeline after response variation
- [x] Disease data verification wired in `diseaseHandler.ts`
- [x] +15 test cases

### Sprint 35: Deep Audit — 4 Audit Suites ✅ (2026-02-25)

> **Sales Flow, Diagnosis Flow, Business Intelligence, UX audit**

- [x] `audit_sales_flow.ts` — sales funnel tests
- [x] `audit_diagnosis_flow.ts` — diagnosis accuracy tests
- [x] `audit_intent_tests.ts` — normalizer + dialect tests
- [x] Stress test: `stress_test_brutal.ts` — 115 edge cases
- [x] Total: **1123 tests pass**

### Sprint 36: Hotfix + Data Upgrade ✅ (2026-02-25)

> **4 engine bugs fixed, 46 new data entries, 100% test pass rate**

**Hotfix: Intent Classification** ✅
- [x] HELP guard fix — "Help 400SC" no longer triggers HELP
- [x] Standalone product name detection → PRODUCT_INQUIRY at 92% confidence
- [x] Product name regex tightened: `[A-Z][a-z]+\s*\d+[A-Z]+` (excludes "Lan 200k")
- [x] Intent-registry regex synced
- [x] +15 Sprint 36 test cases

**Data Upgrade: Disease + Fertilizer + Tonic** ✅
- [x] +12 bệnh lúa mới (lúa von, hoa cúc, lùn sọc đen, sâu gai, sâu năn...)
- [x] +9 sầu riêng mới (đốm mắt cua, rầy phấn, sâu ăn bông, bọ xít muỗi...)
- [x] +15 phân bón (category mới `phan_bon`: URê, NPK, DAP, hữu cơ vi sinh...)
- [x] +10 thuốc dưỡng (category mới `duong_cay`: kích rễ, ra hoa, nuôi trái...)
- [x] +8 symptom chain rules (R046–R053)
- [x] `DiseasePatternGenerator.ts` — skip phan_bon/duong_cay from disease keywords
- [x] 7 regression fixes (EM/lân/đạm alias collisions)
- [x] Disease Expert DB: 100 → **146 entries**

**Final Stats:**
- [x] Tests: **1183/1183 (100%)** — Sprint 36 Level 2 (test upgrade +60)
- [x] Build: 0 errors
- [x] Disease DB: 146 entries (42 disease, 36 pest, 12 weed, 1 rodent, 1 snail, 10 dinh_duong, 15 phan_bon, 10 duong_cay)
- [x] Known issues fixed: DU-045 DEBT_COLLECTION → ADD_TO_CART (agri guard), DU-004 → DISEASE_DIAGNOSIS (enhanced keywords)
- [x] Added 16 tonic/growth regulator patterns to CROP_ACTION
- [x] Added 2 agri keyword guards to DEBT_COLLECTION checks

### Sprint 43H: Category Data Separation — "Phân bón lá" vs "Phân bón" ✅ (2026-02-26)

> **Tách "Phân bón lá" thành canonical category riêng biệt, xuyên suốt toàn bộ data pipeline**

- [x] `AddProductModal.vue` — `NHOM_HANG_MAP` tách "phân bón lá" → `'Phân bón lá'`
- [x] `ChatbotTestDashboard.vue` — Seed data: "Đầu Trâu 502" → `category: 'Phân bón lá'`
- [x] `categories.ts` — Thêm tab `{ label: 'Phân bón lá', color: 'lime', icon: '🍃' }`
- [x] `InventoryView.vue` — Filter: `includes()` → `===` exact match
- [x] `productHandler.ts` — `CATEGORY_MAP` + exact canonical match
- [x] `useChatProcessor.ts` — BUG-H3 Layer 0.8 `isCategoryTerm` guard
- [x] Build: 0 errors ✅ | Chatbot: exact category match ✅

### Sprint 43J: Rename "Phân bón" → "Phân bón gốc" + Parent-Child ✅ (2026-02-26)

> **Tách hoàn toàn: "Phân bón gốc" (Urê/DAP/NPK) vs "Phân bón lá" (Đầu Trâu/ADC-Phos). Query "phân bón" → hiện cả 2.**

- [x] `AddProductModal.vue` — NHOM_HANG_MAP → canonical `'Phân bón gốc'` + thêm hữu cơ/vi sinh
- [x] `categories.ts` — Filter tab: `'Phân bón'` → `'Phân bón gốc'`
- [x] `productHandler.ts` — `PARENT_CATEGORIES` parent-child + icon tag 🌾/🍃
- [x] `ChatbotTestDashboard.vue` — Seed: NPK/DAP/Urê → `'Phân bón gốc'`
- [x] Build: 0 errors ✅

### Sprint 43L: Mở rộng Data Phân bón ✅ (2026-02-26)

> **+93 sản phẩm phân bón mới — coverage ~80% brands VN market**

**Phân bón gốc (+55 SP)** — `phan_bon.ts`
- [x] Lâm Thao (10 SP), Văn Điển/Ninh Bình (5), Đức Giang (4), JVF Việt Nhật (5)
- [x] Đạm Hà Bắc (2), Kali/DAP nhập khẩu (6), Việt Hàn (2)
- [x] Bửu Nông/Năm Sao/Sao Vàng (6), Yara (3), Hữu cơ vi sinh (5), Coromandel/IPL (2)

**Phân bón lá (+38 SP)** — `phan_bon_la.ts`
- [x] Growmore (5), YaraVita (4), Combi/BASF (3), Đầu Trâu bổ sung (5)
- [x] Syngenta (1), Map Pacific (2), Thiên Nông (2), Phú Điền (2)
- [x] Kích thích rễ (2), Humic acid (2), Canxi Bo (2), Vi lượng Chelate (5)

**Verification:** Build 0 errors ✅

### Sprint 43M+N: Mở rộng Data Lúa ✅ (2026-02-26)

> **Rice coverage: 29 → 47 entries — delegation model thành công**

**Sprint 43M (Agent chính):**
- [x] +14 entries lúa mới (nấm, vi khuẩn, sâu, cỏ)
- [x] -11 entries duplicate xóa
- [x] -3 entries thiếu ingredients fix

**Sprint 43N (Delegated agent):**
- [x] +18 entries: nấm/VK (3), sinh lý (5), vi lượng (3), sâu (2), kích thích (2), bệnh khác (3)
- [x] -4 entries duplicate xóa
- [x] Rice total: 29 → **47 entries**

### Sprint 43O: Fix Disease DB Tests → 100% ✅ (2026-02-27)

> **Delegated agent fix scoring + data → 50/68 → 68/68 (100%)**

**Scoring Function:**
- [x] Min 3-char length cho name/alias substring matching
- [x] Crop-specificity bonus (+150) cho entries đặc thù
- [x] Word-boundary crop matching (ngăn nhầm 'nhãn'→'nhanh')

**Database Entry Fixes (10 entries):**
- [x] `lua_co` — xóa alias 'lúa cỏ' gây false match
- [x] `oc_buou_vang` — xóa alias 'ốc' quá ngắn
- [x] `rep_sap` / `rep_sap_sr` — đổi alias bớt generic
- [x] `phan_trang_dua` / `phan_trang` — tách crop specificity
- [x] + 5 entries khác: alias, keyword fixes

**Test Results:**

| Suite | Result |
|-------|--------|
| Data Integrity | 13/13 ✅ |
| Symptom Matching | 44/44 ✅ |
| Cross-Reference | 1/1 ✅ |
| Engine Integration | 10/10 ✅ |
| **Total** | **68/68 (100%)** 🏆 |

- [x] Build: 0 errors (20.60s) ✅

### Sprint 44: Phase 1 Core Reliability ✅ (2026-02-27)

> **3 agents song hành — Cloud Backup + Sync Reliability + UX Polish**
> **Trợ lý điều hành**: phân tích spec-driven → viết 3 briefs → giao 3 agents → verify

**Sprint 44A: Cloud Backup System ✅** (Agent 1)
- [x] `cloud_backup` — VACUUM INTO → gzip (flate2) → multipart upload → record history
- [x] `get_backup_history` — Query recent backup records
- [x] `schedule_auto_backup` — Toggle nightly backup (AtomicBool + tokio::spawn 24h)
- [x] `restore_from_cloud` — Download → gunzip → overwrite database
- [x] Migration #32: `cloud_backups` table
- [x] UI: Cloud backup section + history table + auto-backup toggle in DatabaseSettingsTab
- [x] Build: 0 errors ✅

**Sprint 44B: Sync Reliability ✅** (Agent 2)
- [x] `syncStore.ts` (NEW) — Pinia store sync state management + toast notifications
- [x] `SyncStatusIndicator.vue` (NEW) — Header icon: synced/syncing/error/offline + badge
- [x] `sync_queue` table — Migration #33 (collection, record_id, action, payload, status)
- [x] 4 new commands: `queue_sync_item`, `get_pending_sync_count`, `process_sync_queue`, `clear_synced_items`
- [x] `POSView.vue` — Direct invoke → syncStore.triggerSync()
- [x] Build: 0 errors ✅

**Sprint 44C: UX Polish ✅** (Agent 3)
- [x] `UpgradePromptModal.vue` (NEW) — Premium modal (gradient blue→purple) + feature list
- [x] `useLicenseGuard.ts` — canUseCloudBackup, canUseSync, canUseEInvoice, shouldShowUpgrade
- [x] `POSView.vue` — Free user quota gate (20 đơn) → UpgradePromptModal
- [x] `MainLayout.vue` — FREE/TRIAL/PRO tier badges + quota countdown
- [x] Build: 0 errors ✅

**Final: `npm run build` → 0 errors (13.88s)** ✅

### Sprint 44D: VPS API Audit ✅ (2026-02-27)

> **Agent VPS audit — 10 endpoints checked, 3 fixed, 0 lỗi 404**

- [x] `/api/upload` — **NEW** multipart handler (backup + product_image)
- [x] `/api/update/check` — **Fixed** POST handler (was 405)
- [x] `/api/scanner/*` — **NEW** stub routes
- [x] `docker-compose.yml` — `./uploads` volume cho nginx + axum-api
- [x] `nodi.conf` — Static file serving `/uploads/`
- [x] Docker rebuild + deploy thành công

### Sprint 48: Unified Sync Identity ✅ (2026-02-28)

> **Fix sync data mismatch — JWT primary auth, HWID fallback, data migration**

**PC App Changes:**
- [x] `sync_commands.rs` — JWT `Authorization: Bearer` primary, `X-HWID` fallback
- [x] Removed `X-License-Key` + `X-Store-Id` from sync headers
- [x] License Key = local only (Pro features), NOT for cloud identity

**VPS Changes:**
- [x] `identify_store()` — JWT ưu tiên → HWID fallback → auto-create cho HWID mới
- [x] Data migration: `store_id=59` → `store_id=1000004`
- [x] Auto-migrate HWID data khi register/login
- [x] Fixed 5 dashboard API 500 errors (SQL casting, column names, JWT store_id)
- [x] Cleanup: bỏ X-License-Key support, xóa orphan data

**Verification:** Dashboard 100% — 180K doanh thu, 3 đơn hàng, tồn kho, báo cáo, công nợ ✅

### Sprint 73B: UI Enhancements — Maximize + Chat Popup ✅ (2026-03-01)

> **App mở maximize mặc định + tách chat ra cửa sổ riêng (Tauri Multi-Window)**

- [x] `tauri.conf.json` — `"maximized": true` (app mở full màn hình khi khởi động)
- [x] `ChatPopupView.vue` (NEW) — Standalone chat UI cho popup window + nút "Thu lại"
- [x] `ChatAssistant.vue` — Nút "Tách ra" ↗ + `WebviewWindow` + event-based state sync
- [x] `router/index.ts` — Route `/chat-popup` ngoài MainLayout
- [x] `default.json` — `chat-popup` window + `core:webview:allow-create-webview-window` + `core:window:allow-destroy`
- [x] Hotfix: Thêm `core:window:allow-destroy` — fix lỗi bấm nút X popup không hoạt động
- [x] Build: 0 errors ✅ | Tests: 1213/1213 (100%) ✅
- [x] Git push: `3c82c1d` + `e236d2e` (hotfix)

### Sprint 74: IntentScorer — Competitive Scoring Engine ✅ (2026-03-01)

> **Multi-intent conflict resolution: pattern scoring engine + V2 pipeline wrapper**

- [x] `IntentScorer.ts` (NEW) — 40+ intent patterns, weighted TF-IDF-style scoring
- [x] `detectFuzzyIntentV2()` — wraps V1 + scorer: guards preserved → V1 first → scorer override only when gap > 0.20
- [x] `GUARDED_INTENTS` set (10 intents) prevents scorer from overriding complex guards
- [x] Feature flag `USE_INTENT_SCORER_V2 = true`
- [x] Tests: 1236/1238 (99.8%) ✅ — 0 new regressions
- [x] Brief: `briefs/AGENT_INTENT_SCORER_SPRINT74_BRIEF.md`

### Sprint 75: Self-Learning Loop ✅ (2026-03-01)

> **Feedback 👎 → auto intent adjustment → improved accuracy over time**

- [x] `SelfLearningEngine.ts` (NEW) — analyze feedback, generate boost/penalty map
- [x] `BoostCache.ts` (NEW) — standalone boost cache, breaks circular dep, in-memory caching
- [x] `FeedbackLogger.ts` — triggers re-analysis every 5 negative feedbacks
- [x] `IntentScorer.ts` — `getBoost()` from BoostCache applied in `scoreAll()`
- [x] `useChatProcessor.ts` — `SelfLearningEngine.analyze()` on app init
- [x] `AITrainingTab.vue` — "🧠 Self-Learning" tab with stats + reset
- [x] Partial match fix: word overlap ≥50% (prevents "nợ" matching "công nợ khách hàng")
- [x] Tests: 1236/1238 (99.8%) ✅
- [x] Brief: `briefs/AGENT_SELF_LEARNING_SPRINT75_BRIEF.md`

### Sprint 76: Pipeline Simplification ✅ (2026-03-02)

> **`useChatProcessor.ts`: 1207 → 1097 dòng (-110 LOC, -9.1%), 0 regressions**

- [x] Phase 1: Xóa Layer 2 legacy regex (6 blocks) — -85 LOC
- [x] Phase 2: Gộp 7 guards → `shouldSkipProductNameLayer()` — -34 +32 LOC
- [x] Phase 3: GREETING + UNKNOWN → HANDLER_MAP — -17 LOC
- [x] Phase 4: Modifier boost inline removed — -6 LOC
- [x] Tests: 1236/1238 (99.8%) ✅
- [x] Brief: `briefs/AGENT_PIPELINE_SPRINT76_BRIEF.md`

### Sprint 77: Fix RQ-019 + SF-005 → 100% Baseline ✅ (2026-03-02)

> **1238/1238 (100%) — 0 failures 🎉**

- [x] SF-005 ("thuốc gì"): VAGUE_STANDALONE confidence 0.30 → 0.88 (V2 Step 3 return early)
- [x] RQ-019 ("giá bao nhiêu"): VAGUE_PRICE_RE guard → PRODUCT_SEARCH at 0.88
- [x] Both fixes anchored `^...$` — no regression on product-specific queries
- [x] Git push: `0f39168` (Sprint 74-77 combined)
- [x] Brief: `briefs/AGENT_SPRINT77_100PCT_BRIEF.md`

### Sprint 78: VPS Qwen2.5-3B Deploy ✅ + Response Polish ❌ (2026-03-02)

> **Ollama + Qwen2.5-3B self-hosted trên VPS. Response Polish cancelled do latency.**

- [x] VPS: Ollama v0.17.4 installed (CPU-only)
- [x] Model: `qwen2.5:3b` (1.9 GB, Q4) — RAM 4.3/7.8 GB
- [x] Endpoints: `POST api.nodi.vn/llm/api/generate` & `/llm/api/chat`
- [x] Rate limit: 10 req/min, management APIs blocked (404)
- [x] Security: chỉ `api.nodi.vn` access, nodi.vn blocked
- [x] Latency: 8.2-11.2s (CPU-only)
- [x] Nginx reverse proxy + Docker `host.docker.internal`
- [x] `QwenProvider.ts` — progressive enhancement (tạo → test → cancel → revert)
- [x] Quyết định: template đủ tốt, Qwen polish không worth latency 8-11s
- [x] Brief: `briefs/AGENT_VPS_QWEN_SPRINT78_BRIEF.md`

### Sprint 43K: Knowledge DB Category Suggestion (TODO)

> **Khi kho ít/không có SP trong 1 category → gợi ý SP từ Knowledge DB (5,656 SP)**

- [ ] `productHandler.ts` — fallback search Knowledge DB khi `categoryProducts.length < 3`
- [ ] Hiển thị gợi ý: "💡 Có thể nhập thêm: [SP từ KB]..."
- [ ] Parent-child logic: "phân bón" → hiện cả "Phân bón gốc" + "Phân bón lá"

| Agent | Model | Nằm ở | Nhiệm vụ |
|-------|-------|-------|---------|
| **Giám sát** (Gemini + Anh Hoa) | Gemini | Windows local | Kiểm soát toàn bộ, review báo cáo, quyết định chiến lược, viết brief |
| **Agent Leader** (Opus 4.6) | Claude Opus 4.6 | Windows local | Điều phối agents, code Tauri app, tích hợp data, sửa engine |
| **Agent Data** (Opus 4.6) | Claude Opus 4.6 | Windows local | Tạo disease knowledge DB, mở rộng DB, research nông nghiệp |
| **Agent Test** (Opus 4.6) | Claude Opus 4.6 | Windows local | Viết test suites, chạy benchmark, verify chất lượng |
| **Agent Product** (Opus 4.6) | Claude Opus 4.6 | Windows local | Audit chất lượng data sản phẩm, verify hoạt chất, fix nhóm hàng |
| **Agent Intent** (Opus 4.6) | Claude Opus 4.6 | Windows local | Pattern sync, intent routing fixes, engine optimization |
| **Agent Seasonal** (Opus 4.6) | Claude Opus 4.6 | Windows local | Seasonal awareness, mùa vụ intelligence |
| **Agent Diagnostic** (Opus 4.6) | Claude Opus 4.6 | Windows local | Guided diagnostic flow, step-by-step Q&A |
| **Agent CrossSell** (Opus 4.6) | Claude Opus 4.6 | Windows local | Cross-sell intelligence, gợi ý bán chéo |
| **Agent VPS** (Opus 4.6) | Claude Opus 4.6 | Ubuntu VPS | Setup infra, deploy, debug, website nodi.vn |

### Flow giao việc:

```
Giám sát (Gemini + Anh Hoa)
    │
    ├─── Viết Brief ──→ Agent Data ──→ Tạo disease_expert_db.ts
    │                                        │
    │                                        ▼
    ├─── Viết Brief ──→ Agent Test ──→ Test data + benchmark
    │                                        │
    │                                        ▼
    ├─── Review kết quả ◄────────────────────┘
    │
    ├─── Viết Brief ──→ Agent Leader ──→ Tích hợp vào Engine
    │
    └─── Viết Brief ──→ Agent VPS ──→ Deploy website / API
```

### Nguyên tắc phân công:
- **Gemini + Anh Hoa** không code trực tiếp cho data task — chỉ review + quyết định
- **Mỗi agent 1 branch Git riêng** — không conflict
- **Báo cáo bắt buộc** — mỗi agent xong task phải có summary + diff
- **Opus 4.6 mạnh về research** — tận dụng cho data creation + testing

---

## 📌 Ghi chú

- **quanly.hoadigital.com** → **api.nodi.vn** ✅ đã chuyển xong (2026-02-19)
- Tất cả data khách hàng được ẩn danh khi phân tích market intelligence
- EULA cần cập nhật thêm điều khoản cloud sync + backup

---

## ✅ Sprint 88 — Tax Compliance Hardening (02/03/2026)

> Mục tiêu: Đảm bảo cơ chế khai thuế chính xác 100% theo quy định HĐDT Bộ Tài Chính.

### Đã hoàn thành:
- **Tax DB Fix**: `calculate_quarterly_revenue()` đã lọc `AND has_invoice = 1` — báo cáo thuế quý chính xác
- **UI Fix**: `CheckoutModal.vue` — xóa duplicate checkbox, toggle "Xuất HĐ VAT" hiển thị đúng 1 nút
- **Build Fix**: Vite Exit Code 0 (rewrite `vite.config.ts`, suppress warnings phù hợp Tauri)
- **Invoice Filter**: `OrderHistoryView.vue` — filter "Hóa đơn VAT" (Tất cả / Có HĐ / Bán lẻ)
- **Type System**: `Invoice` interface bổ sung `has_invoice?: boolean`
- **Tax Rate UI**: Xác nhận `AddProductModal.vue` đã có dropdown 0%/5%/10% từ Sprint 83

### Pháp lý đã nghiên cứu (Bộ Tài Chính):
| Ngưỡng DT | Bắt buộc HĐDT | Khai thuế |
|:--|:--|:--|
| ≤ 500 triệu/năm | ❌ Không | 1 lần/năm (31/01) |
| 500M – 1 tỷ | ⚡ Tùy chọn | Theo quý |
| ≥ 1 tỷ | ✅ Bắt buộc | Theo quý |
| > 50 tỷ | ✅ Bắt buộc | Theo tháng |

### Kế hoạch tiếp theo (Sprint 89+):
- [ ] **XML HĐDT Export**: Xuất file XML theo chuẩn TCVN (để nộp miễn phí qua eTax tổng cục thuế)
- [ ] **Khai báo tự động**: Dashboard khai thuế quý với 1 click (tự tính và điền vào mẫu)
- [ ] **Cảnh báo ngưỡng 1 tỷ**: Notification khi doanh thu tích lũy vượt 900M (sắp đến ngưỡng bắt buộc HĐ)
- [ ] **Tích hợp HĐDT bên thứ 3**: Kết nối API với Viettel, VNPT, MISA để xuất HĐDT tự động từ app

---

## 🚀 Feature Gap Sprint — Cạnh tranh KiotViet (03/03/2026)

> **Nguồn**: Phân tích cạnh tranh KiotViet (đối thủ chính ngành VTNN)
> **Mục tiêu**: Bổ sung 9 tính năng còn thiếu so với KiotViet
> **Lưu ý**: Multi-branch ĐÃ CÓ (web dashboard hỗ trợ đa cửa hàng tách biệt)
> **Briefs**: `briefs/FEATURE_GAP_BRIEF.md`, `briefs/MOBILE_APP_BRIEF.md`, `briefs/VPS_MULTI_DEVICE_LICENSE_BRIEF.md`

### ✅ Phase 1 — HOÀN THÀNH (03/03/2026)

| # | Tính năng | Trạng thái | Files chính |
|---|----------|:----------:|------------|
| 1 | **Khuyến mãi / Voucher** | ✅ Full-stack | `promotions.rs`, `PromotionEngine.ts`, `PromotionsView.vue` |
| 2 | **In barcode / nhãn SP** | ✅ Frontend | `BarcodeGenerator.ts`, `LabelPrintModal.vue` |
| 3 | **Thuế TNCN tự động** | ✅ Frontend | `PIT_Calculator.ts` |

### ✅ Phase 2A — Mobile MVP + Multi-Device License — HOÀN THÀNH (03/03/2026)

| # | Tính năng | Trạng thái | Chi tiết |
|---|----------|:----------:|----------|
| 4 | **Mobile App MVP** | ✅ 14 components | 7 routes `/m/*`, `MobileLayout.vue`, Rust feature-gate |
| ⭐ | **Multi-Device License** | ✅ VPS + PC | `devices` table, 5 API endpoints, `device_type: "windows"` |

### 📋 Phase 2B — Tiếp theo (Ưu tiên cao)

| # | Tính năng | Mô tả | Effort | Deadline |
|---|----------|-------|:------:|:--------:|
| 5 | **Tích điểm KH** | Loyalty points, đổi điểm khi checkout | 2-3 ngày | Tuần 2/3 |
| 6 | **Đồng bộ HĐ từ CQT** | Import XML hóa đơn đầu vào | 2-3 tuần | Tuần 3/3 |
| 7 | **Mobile — `tauri android init`** | Android SDK setup, real build, emulator test | 1 tuần | Tuần 2/3 |
| 8 | **Mobile — Camera + Bluetooth** | Replace stubs với Tauri mobile plugins thực | 1 tuần | Tuần 2/3 |
| 9 | **Mobile — Auto-redirect** | `useDeviceDetect.ts` trong `App.vue` → `/m/*` | 1 ngày | Tuần 2/3 |

#### Tích Điểm Khách Hàng
- [ ] Migration: thêm `loyalty_points`, `total_points_earned` vào bảng customers
- [ ] Cấu hình: X điểm / 10.000đ mua hàng (owner tự set)
- [ ] `CheckoutModal.vue`: hiển thị điểm tích, cho phép dùng điểm
- [ ] `CustomerDetailModal.vue`: lịch sử tích/dùng điểm

#### Đồng Bộ Hóa Đơn Đầu Vào
- [ ] Import XML/Excel hóa đơn đầu vào từ file tải về cổng thuế
- [ ] Parser: đọc format XML HĐĐT theo chuẩn Bộ Tài Chính
- [ ] `InputInvoiceImport.vue` — upload + preview + confirm
- [ ] Đối chiếu: so sánh HĐ đầu vào vs nhập hàng trong app

#### Mobile App Phase 2
- [ ] `npx tauri android init` — Initialize Android project
- [ ] Install Android SDK + NDK 25+
- [ ] `npx tauri android build` — First APK
- [ ] Test on emulator + real device
- [ ] Camera barcode scanner (replace stub)
- [ ] Bluetooth ESC/POS printer (replace stub)
- [ ] Biometric login (vân tay / FaceID)
- [ ] Push notifications (đơn mới, SP hết hàng)

### 📋 Phase 3 — P3 Priority (Khi có nhu cầu)

| # | Tính năng | Mô tả | Effort |
|---|----------|-------|:------:|
| 10 | **Sàn TMĐT** | Kết nối Shopee/Lazada/TikTok Shop API | 2-4 tuần |
| 11 | **Vận chuyển** | Kết nối GHN/GHTK/J&T API | 1-2 tuần |
| 12 | **API mở** | REST API cho đối tác tích hợp | 1 tuần |
| 13 | **Video đào tạo** | YouTube series hướng dẫn sử dụng | 2-3 tuần |
| 14 | **iOS Build** | `npx tauri ios init` + TestFlight | 1-2 tuần |
| 15 | **AI Mobile** | Chatbot trên mobile (offline TF-IDF) | 1 tuần |

> Phase 3 chỉ triển khai khi có khách hàng yêu cầu cụ thể.

---

## ✅ Sprint 89 — Staff Permission Management Phase 1 (03/03/2026)

> Mục tiêu: Chủ cửa hàng phân quyền nhân viên — ẩn/hiện sidebar, đổi ca bằng PIN 4 số.

### Đã hoàn thành:
- **DB Migration**: `037_staff_permissions.sql` — thêm `pin` + `permissions` (JSON) vào bảng `users`
- **Backend**: 4 Tauri commands (`list_staff`, `create_staff`, `update_staff`, `verify_staff_pin`)
- **Sidebar Filter**: `MainLayout.vue` — `routePermissionMap` + `staffStore.hasPermission()` (thay hardcoded list)
- **PIN Switching**: `authStore.ts` — thử staff PIN trước, fallback global PIN
- **UI**: `StaffManagementTab.vue` — bảng NV, form tạo/sửa, toggle 9 quyền

### 9 Permissions:
`view_sales`, `view_inventory`, `view_revenue`, `view_reports`, `view_customers`, `view_cashflow`, `delete_order`, `edit_product_price`, `access_settings`

### Build: ✅ Vite 0 errors | Cargo build OK

---

## 🧪 Hệ thống Testing — Regression Suite

> **Quan trọng cho tất cả agents**: Mọi thay đổi code chatbot PHẢI pass regression tests.

### Regression Tests (260 → 1123+ cases, Sprint 36)

Bộ test tự động kiểm tra **intent routing** — mỗi test case là 1 câu hỏi giả lập user:

```typescript
{ input: "doanh thu hôm nay",   expected: "REVENUE" }
{ input: "Minh nợ bao nhiêu",   expected: "CUSTOMER_DEBT" }
{ input: "lấy 2 chai Beam",     expected: "ADD_TO_CART" }
{ input: "đạo ôn lúa",          expected: "DISEASE_DIAGNOSIS" }
{ input: "vi khuẩn lúa",        expected: "DISEASE_DIAGNOSIS" }   // Sprint 22 — category query
{ input: "thối bẹ lúa",         expected: "DISEASE_DIAGNOSIS" }   // Sprint 22 — bacteria
```

### Test Suites

| File | Nội dung | Cases |
|:--|:--|:-:|
| `test_suites/full_intent_coverage.ts` | Core 26 intents × 10 queries | 260 |
| `test_suites/sprint21_audit_tests.ts` | Sprint 21 Audit: 7 zones | 150 |
| `test_suites/sprint22_data_tests.ts` | **Sprint 22 Data: bacteria + weed + durian** | **26** |
| `test_suites/sprint22_brain_tests.ts` | **Sprint 22 Brain: category + confidence** | **15** |
| `test_suites/sprint22_regression_tests.ts` | **Sprint 22 Regression: collision + boundary** | **20** |
| Other suites | NLP, expansion, V2, etc. | ~185 |
| **Tổng** | **48 intents, 27 suite files** | **656** |

### Sprint 21 Audit Zones (150 tests)

| Zone | Tests | Pass Rate |
|:--|:-:|:-:|
| bacteria_rice | 25 | 100% |
| weed_management | 25 | 88% |
| nutrition_deficiency | 20 | 85% |
| keyword_to_product | 20 | 95% |
| farmer_dialect | 20 | 90% |
| conflict_collision | 20 | 75% |
| edge_stress | 20 | 80% |

### Agent Briefs (Sprint 21)

Agent briefs được lưu tại `briefs/`:
- `AGENT_AUDIT_SPRINT21_BRIEF.md` — 150 test cases, 7 zones
- `AGENT_FIX_SPRINT21_BRIEF.md` — 5 bugs + 4 pattern fixes
- `AGENT_DATA_SPRINT21_BRIEF.md` — 11 DB entries + synonyms
- `AGENT_TRIAGE_SPRINT21_BRIEF.md` — Triage 32 remaining failures

### Cách chạy

```bash
npx vite build                              # Build phải 0 errors
npx tsx ai_training_center/test_runner.ts    # 1123/1123 = 100% PASS ✅
```

### Quy tắc bắt buộc

1. **TRƯỚC** khi commit → chạy tests
2. **SAU** mỗi code change → chạy tests
3. Nếu bất kỳ test FAIL → **PHẢI fix trước khi tiếp tục**
4. KHÔNG được disable/skip test — phải fix root cause
