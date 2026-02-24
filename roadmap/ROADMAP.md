# 🗺️ NODI POS - Roadmap (Lộ Trình Phát Triển)

> **Cập nhật:** 2026-02-24  
> **Nguyên tắc:** Ưu tiên ổn định → tính năng → mở rộng

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

### 🔄 Đang làm

- [ ] UI/UX Polish — Hoàn thiện giao diện website
- [ ] Testing & Bug fixes
- [ ] Mobile App ↔ API integration testing
- [ ] Documentation (project info, roadmap, changelog)

---

## Phase 2: Hoàn Thiện Sản Phẩm 📦

**Thời gian:** Q2 2026 (Apr — Jun)  
**Trạng thái:** ⬜ Chưa bắt đầu

### Tính năng mới

- [ ] **Báo cáo nâng cao** — Biểu đồ doanh thu, lợi nhuận, tồn kho theo thời gian
- [ ] **Quản lý nhân viên nâng cao** — Chấm công, phân ca, hoa hồng
- [ ] **Thông báo push** — Thông báo hết hàng, công nợ quá hạn, license sắp hết
- [ ] **Import/Export data** — Excel, CSV cho sản phẩm, khách hàng
- [ ] **Email notifications** — Xác nhận đơn, nhắc nợ, thông báo hệ thống
- [ ] **Blog & SEO** — Hệ thống blog, bài hướng dẫn, SEO
- [ ] **Payment Gateway** — Tích hợp thanh toán online (VNPay, Momo)

### Cải thiện

- [ ] **Performance tuning** — Database query optimization, caching (Redis)
- [ ] **Error monitoring** — Sentry / tương đương
- [ ] **Logging nâng cao** — Structured logging, log rotation
- [ ] **Rate limiting** — Bảo vệ API khỏi abuse
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
- [ ] **Tích hợp kế toán** — Xuất dữ liệu cho phần mềm kế toán (Misa, Fast...)
- [ ] **Quản lý vùng miền** — Analytics theo tỉnh/huyện, mùa vụ nông nghiệp
- [ ] **Chương trình khuyến mãi** — Discount, loyalty, tích điểm
- [ ] **In hóa đơn** — Hỗ trợ máy in nhiệt, template hóa đơn tùy chỉnh
- [ ] **Quản lý vận chuyển** — Giao hàng, theo dõi đơn

### Hạ tầng

- [ ] **Database replication** — Read replicas
- [ ] **Auto-scaling** — Container orchestration (Kubernetes / Docker Swarm)
- [ ] **CDN** — Static assets & images
- [ ] **Monitoring** — Grafana + Prometheus dashboard
- [ ] **Backup tự động** — Scheduled PostgreSQL backups

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
| Sprint 11 | Feb-Mar 2026 | UI Polish, Testing, Documentation | 🔄 Đang làm |

---

## KPIs & Metrics Theo Dõi

| Metric | Mục tiêu Q1 2026 | Mục tiêu Q2 2026 |
|--------|-------------------|-------------------|
| Cửa hàng đăng ký | 50 | 200 |
| Cửa hàng active (sync data) | 20 | 100 |
| Uptime | 95% | 99% |
| API Response Time (avg) | < 500ms | < 200ms |
| Bug count (critical) | < 5 | 0 |
