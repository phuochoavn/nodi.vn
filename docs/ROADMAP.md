# 🗺️ NODI POS - Roadmap (Lộ Trình Phát Triển)

> **Cập nhật:** 2026-03-03  
> **Nguyên tắc:** Ưu tiên ổn định → tính năng → mở rộng  
> **Spec chi tiết:** [SPEC_DRIVEN.md](./SPEC_DRIVEN.md)

---

## Tổng Quan Phases

```
Phase 1 (MVP)          Phase 2               Phase 3               Phase 4
Q1 2026                Q2 2026               Q3-Q4 2026            2027+
─────────────────────  ──────────────────    ──────────────────    ──────────────
Nền tảng cơ bản        Hoàn thiện sản phẩm   Mở rộng thị trường    Scale & AI
```

---

## Phase 1: MVP — Nền Tảng Cơ Bản ⏳ (Đang thực hiện)

**Thời gian:** Q1 2026 (Jan — Mar)  
**Trạng thái:** 🟡 Đang phát triển

### ✅ Đã hoàn thành

- [x] **Hạ tầng**: Docker Compose, Nginx, PostgreSQL 16
- [x] **API Backend**: Rust/Axum framework setup
- [x] **Authentication**: JWT + bcrypt, middleware bảo vệ routes
- [x] **Database schema**: 20+ tables cho toàn bộ nghiệp vụ
- [x] **Data Sync**: API đồng bộ từ mobile app (products, customers, invoices, suppliers...)
- [x] **License Management**: Tạo, kích hoạt, quản lý license keys
- [x] **Backup System**: Upload, download, quản lý file backup
- [x] **WebSocket Chat**: Real-time support chat (customer ↔ admin)
- [x] **Admin Panel**: Dashboard quản trị, quản lý cửa hàng, license, hỗ trợ
- [x] **Store Dashboard**: Dashboard cho store owners (đơn hàng, tồn kho, công nợ, báo cáo)
- [x] **Market Intelligence**: Phân tích xu hướng thị trường
- [x] **Website nodi.vn**: Landing page, tính năng, bảng giá, liên hệ
- [x] **User Registration**: Đăng ký, đăng nhập, quản lý tài khoản
- [x] **Dark Mode**: Hỗ trợ chế độ tối
- [x] **Sprint 3 Migration**: Sync mở rộng (purchase orders, payments, batches, transactions...)
- [x] **Sprint 10 Migration**: Employees + Store Groups (multi-branch)

### 🔄 Đã hoàn thành (tiếp)

- [x] Logo SVG — Vẽ lại logo (house + leaf) khớp với app thật
- [x] Hero Section — Thay mockup trừu tượng bằng ảnh chụp thật + floating badges
- [x] Documentation — Tạo folder roadmap (project info, roadmap, changelog)
- [x] Sprint 48 — Unified Sync Identity (JWT ưu tiên, HWID auto-create, auto-migrate)
- [x] Dashboard API fix — Sửa 500 errors (NUMERIC cast, column names, backup/list)
- [x] Sprint 53 — Feature Gap Hardening (11 items, P0-P2)
  - [x] 401 auto-redirect + session clearing
  - [x] Rate limiting (tower-governor 0.8)
  - [x] Automated DB backups (cron daily)
  - [x] CSS refactoring
  - [x] Chart.js reports (line + bar charts)
  - [x] Purchase Orders page + API
  - [x] Employees page + API
  - [x] Sidebar navigation updates
  - [x] Global toast notification system
  - [x] Product Excel export
  - [x] Notification bell + dropdown panel
- [x] Sprint 89 — Staff Permission Management (sync + web dashboard)
  - [x] Tạo bảng `sync_staff_members` (PostgreSQL migration)
  - [x] Sync handler collection #15 (`staff_members` desktop → VPS)
  - [x] `GET /api/dashboard/staff` (danh sách, không lộ PIN)
  - [x] `PUT /api/dashboard/staff/:id/permissions` (9 cờ quyền)
  - [x] `PUT /api/dashboard/staff/:id/toggle-active` (bật/tắt)
  - [x] Viết lại trang `/dashboard/nhan-vien` (bảng, modal phân quyền, dark mode)
- [x] Sprint 90 — Update Management & Download System
  - [x] Nginx `/download/` location block serve file `.exe`
  - [x] Dynamic version config (`update_config.json`) — thay thế hardcoded constants
  - [x] API endpoints: `GET/PUT /api/admin/update`, `POST upload`, `DELETE files`
  - [x] Admin dashboard `/admin/cap-nhat` — drag-drop upload, version management
  - [x] Trang `/tai-ung-dung` — fetch version từ API, hiển thị download button thật
  - [x] Docker volume mount cho `/opt/nodi/downloads` (API + Nginx)

### 🔄 Đang làm

- [ ] Multi-shop Dashboard Fix — `switch_store` JWT logic chưa chuyển data thật
- [ ] UI/UX Polish — Hoàn thiện giao diện các trang còn lại
- [ ] Testing & Bug fixes
- [ ] Mobile App ↔ API integration testing

### 📌 Multi-Shop Gaps (ưu tiên Q1-Q2)

- [ ] **Store management page** — Đổi tên shop, xem thông tin, quản lý nhân viên
- [ ] **Create store từ dashboard** — API + UI tạo shop mới
- [ ] **Per-store settings** — Trang cài đặt riêng cho từng store
- [x] **Charts/biểu đồ** — Thêm chart vào trang báo cáo ✅ (Sprint 53)

---

## Phase 2: Hoàn Thiện Sản Phẩm 📦

**Thời gian:** Q2 2026 (Apr — Jun)  
**Trạng thái:** ⬜ Chưa bắt đầu

### Tính năng mới

- [x] **Báo cáo nâng cao** — Biểu đồ doanh thu, lợi nhuận, tồn kho theo thời gian ✅ (Sprint 53 — Chart.js)
- [ ] **Quản lý nhân viên nâng cao** — Chấm công, phân ca, hoa hồng (cơ bản ✅ Sprint 53)
- [ ] **Hóa đơn điện tử (E-Invoice)** — Tích hợp API xuất HĐĐT (VNPT/Viettel)
- [ ] **Kế toán Thuế HKD** — Xuất 7 tự động sổ kế toán (TT88) & Tờ khai thuế (TT40)
- [x] **Thông báo push** — Thông báo hết hàng, công nợ quá hạn ✅ (Sprint 53 — Bell + Toast)
- [x] **Import/Export data** — Excel export sản phẩm ✅ (Sprint 53)
- [ ] **Email notifications** — Xác nhận đơn, nhắc nợ, thông báo hệ thống
- [ ] **Blog & SEO** — Hệ thống blog, bài hướng dẫn, SEO
- [ ] **Payment Gateway** — Tích hợp thanh toán online (VNPay, Momo)

### Cải thiện

- [ ] **Performance tuning** — Database query optimization, caching (Redis)
- [ ] **Error monitoring** — Sentry / tương đương
- [ ] **Logging nâng cao** — Structured logging, log rotation
- [x] **Rate limiting** — Bảo vệ API khỏi abuse ✅ (Sprint 53 — tower-governor 0.8)
- [ ] **CI/CD Pipeline** — GitHub Actions, auto-deploy
- [ ] **Unit & Integration Tests** — Backend + Frontend test coverage

---

## Phase 3: Mở Rộng Thị Trường 🚀

**Thời gian:** Q3-Q4 2026 (Jul — Dec)  
**Trạng thái:** ⬜ Chưa bắt đầu

### Tính năng mới

- [ ] **Mobile App v2** — Cải tiến UX, tính năng offline enhanced
- [ ] **Quản lý đa chi nhánh hoàn chỉnh** — Tổng hợp báo cáo, phân quyền chi nhánh
- [ ] **API cho bên thứ 3** — RESTful public API, API keys
- [ ] **Tích hợp phần mềm kế toán** — Xuất dữ liệu cho MISA, Fast (cho DN lớn)
- [ ] **Quản lý vùng miền** — Analytics theo tỉnh/huyện, mùa vụ nông nghiệp
- [ ] **Chương trình khuyến mãi** — Discount, loyalty, tích điểm
- [ ] **In hóa đơn** — Hỗ trợ máy in nhiệt, template hóa đơn tùy chỉnh
- [ ] **Quản lý vận chuyển** — Giao hàng, theo dõi đơn

### Hạ tầng

- [ ] **Database replication** — Read replicas
- [ ] **Auto-scaling** — Container orchestration (Kubernetes / Docker Swarm)
- [ ] **CDN** — Static assets & images
- [ ] **Monitoring** — Grafana + Prometheus dashboard
- [x] **Backup tự động** — Scheduled PostgreSQL backups ✅ (Sprint 53 — cron + pg_dump)

---

## Phase 4: Scale & AI 🤖

**Thời gian:** 2027+  
**Trạng thái:** ⬜ Tầm nhìn tương lai

### Tầm nhìn

- [ ] **AI Forecasting** — Dự đoán nhu cầu hàng hóa theo mùa vụ
- [ ] **Smart Inventory** — Gợi ý tự động đặt hàng khi sắp hết
- [ ] **Chatbot AI** — Hỗ trợ khách hàng tự động
- [ ] **Data Marketplace** — Chia sẻ dữ liệu thị trường (ẩn danh)
- [ ] **Multi-tenant SaaS** — Tách biệt hoàn toàn dữ liệu các tenant
- [ ] **Mobile App iOS** — Hỗ trợ iOS ngoài Android
- [ ] **Regional Expansion** — Mở rộng sang các nước Đông Nam Á

---

## Sprint History

| Sprint | Thời gian | Nội dung chính | Trạng thái |
|--------|-----------|----------------|------------|
| Sprint 1-2 | Jan 2026 | Setup hạ tầng, DB schema, API cơ bản | ✅ Hoàn thành |
| Sprint 3 | Jan-Feb 2026 | Sync mở rộng (PO, payments, batches, transactions) | ✅ Hoàn thành |
| Sprint 4-9 | Feb 2026 | Dashboard, Admin Panel, WebSocket, Market Intel | ✅ Hoàn thành |
| Sprint 10 | Feb 2026 | Employees + Store Groups (multi-branch) | ✅ Hoàn thành |
| Sprint 11 | Feb 2026 | Logo redesign, Hero screenshots, Documentation | ✅ Hoàn thành |
| Sprint 47 | Feb 2026 | Multi-shop management (account_stores, shop switcher, JWT) | ✅ Hoàn thành |
| Sprint 48 | Feb 2026 | Unified Sync Identity (JWT priority, HWID auto-create, data migration) | ✅ Hoàn thành |
| Sprint 49 | Feb-Mar 2026 | Dashboard API fixes (NUMERIC casts, column names, backup/list) | ✅ Hoàn thành |
| Sprint 50 | Mar 2026 | Multi-shop polish, store management, charts | ⬜ Todo |
| Sprint 51 | Mar 2026 | Accounting & Tax module (TT88/TT40) | ⬜ Todo |
| Sprint 53 | Mar 2 2026 | Feature Gap Hardening — 11 items (P0 security, P1 features, P2 UX) | ✅ Hoàn thành |
| Sprint 89 | Mar 3 2026 | Staff Permission Management — sync + web dashboard phân quyền nhân viên | ✅ Hoàn thành |
| Sprint 90 | Mar 3 2026 | Update Management — admin upload .exe, dynamic config, download page | ✅ Hoàn thành |

---

## Tính Năng App Desktop (Tóm tắt)

Danh sách 16 nhóm tính năng chính đã có trong Nodi POS:

| # | Tính năng | Mô tả ngắn |
|---|-----------|-------------|
| 1 | **Bán hàng POS** | Quét mã vạch, thanh toán VietQR, ghi nợ, in hóa đơn nhiệt |
| 2 | **Quản lý kho** | Tồn kho, lô hàng, FEFO, cảnh báo hết hạn, xuất Excel |
| 3 | **Nhập hàng & NCC** | Phiếu nhập, công nợ NCC, lịch sử nhập |
| 4 | **Khách hàng & Công nợ** | Ghi nợ/thu nợ, phân khúc KH, lịch sử giao dịch |
| 5 | **Thu chi (Cashflow)** | Sổ thu chi ngoài bán hàng, phân loại, tổng hợp |
| 6 | **Hóa đơn điện tử** | Tích hợp VNPT/Viettel, phát hành HĐĐT tại POS |
| 7 | **Thuế & Kế toán** | Dual-Mode thuế, bảng VAT, cảnh báo ngưỡng |
| 8 | **Báo cáo** | Biểu đồ doanh thu, lợi nhuận, so sánh, xuất Excel |
| 9 | **Trợ lý AI** | 46 lệnh chat, chẩn đoán 315+ bệnh cây, 100% offline |
| 10 | **Nhân viên** | PIN 4 số, 9 quyền RBAC, đổi ca nhanh |
| 11 | **Đơn hàng & Trả hàng** | Lịch sử, filter, trả hàng, chốt sổ |
| 12 | **Cloud & Sao lưu** | Offline-first, sync tự động, backup/restore 1 click |
| 13 | **Bảo mật** | License key, HWID, JWT, mã hóa dữ liệu |
| 14 | **In ấn** | Hóa đơn nhiệt 58/80mm, phiếu nhập, A4 |
| 15 | **Import Excel** | Nhập danh mục SP từ Excel, auto-map cột |
| 16 | **Đa cửa hàng** | 1 tài khoản quản lý tối đa 10 chi nhánh |

---

## KPIs & Metrics Theo Dõi

| Metric | Mục tiêu Q1 2026 | Mục tiêu Q2 2026 |
|--------|-------------------|-------------------|
| Cửa hàng đăng ký | 50 | 200 |
| Cửa hàng active (sync data) | 20 | 100 |
| Uptime | 95% | 99% |
| API Response Time (avg) | < 500ms | < 200ms |
| Bug count (critical) | < 5 | 0 |
