# 📋 NODI POS — Spec-Driven Development

> **Tạo:** 2026-02-28  
> **Mục đích:** Tài liệu hóa spec hiện tại, gap analysis, và định hướng phát triển

---

## 1. Kiến Trúc Tổng Quan

```
┌──────────────────────────────────────────────────────────┐
│                    NODI POS ECOSYSTEM                    │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─────────────┐    ┌──────────────┐    ┌────────────┐  │
│  │  PC App      │    │  Web App      │    │  Admin      │  │
│  │  (Tauri/Rust)│    │  (Nuxt.js)    │    │  Panel      │  │
│  │              │    │               │    │  (Nuxt.js)  │  │
│  │  • Bán hàng  │    │  • Dashboard  │    │  • Quản trị │  │
│  │  • POS       │    │  • Báo cáo    │    │  • License  │  │
│  │  • Offline   │    │  • Theo dõi   │    │  • Thống kê │  │
│  └──────┬───────┘    └──────┬────────┘    └─────┬──────┘  │
│         │ Sync (POST)       │ Read (GET)        │          │
│         ▼                   ▼                   ▼          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              VPS API (Rust / Axum)                   │  │
│  │  • JWT Auth + HWID fallback                         │  │
│  │  • Sync endpoints (identify_store)                  │  │
│  │  • Dashboard APIs                                   │  │
│  │  • Multi-shop management                            │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                         │                                  │
│  ┌──────────────────────▼───────────────────────────────┐  │
│  │           PostgreSQL 16  (Docker)                    │  │
│  │  accounts → account_stores → synced_* tables         │  │
│  └──────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Multi-Shop Data Model

### Database Schema

```
accounts (1 user = 1 account)
  ├── id, username, password_hash, phone, display_name, store_name
  └── hwid (for sync migration)

account_stores (1 account → N stores)
  ├── account_id → accounts.id
  ├── store_id   (varchar, e.g. "STORE-XUCFZVM8")
  ├── store_name
  ├── role       (owner / manager / staff)
  └── is_default

synced_* tables (data per store)
  └── store_id (integer) = account_id + 1,000,000
```

### Identity Flow

| Scenario | Header | store_id |
|----------|--------|----------|
| User có account | `Authorization: Bearer <JWT>` | `claims.store_id` (= account_id + 1M) |
| Anonymous (HWID) | `X-HWID: ABC123` | Lookup/auto-create in `stores` table |
| Register/Login + HWID | JWT + X-HWID | Auto-migrate HWID data → account store |

---

## 3. Dashboard Spec — Hiện Tại vs Cần Có

### ✅ Đã có (Working)

| Feature | Status | Files |
|---------|--------|-------|
| Shop Switcher dropdown | ✅ | `layouts/dashboard.vue` |
| Store list API | ✅ | `GET /api/stores` |
| Switch store + new JWT | ✅ | `POST /api/stores/switch` |
| Overview dashboard | ✅ | `pages/dashboard/index.vue` |
| Đơn hàng list + detail | ✅ | `pages/dashboard/don-hang.vue` |
| Tồn kho | ✅ | `pages/dashboard/ton-kho.vue` |
| Công nợ | ✅ | `pages/dashboard/cong-no.vue` |
| Báo cáo doanh thu | ✅ | `pages/dashboard/bao-cao.vue` |
| Cài đặt + đổi mật khẩu | ✅ | `pages/dashboard/cai-dat.vue` |
| Backup list | ✅ | `pages/dashboard/backup.vue` |
| Dark mode | ✅ | `useColorMode()` |

### ⚠️ Gaps — Multi-Shop

| Gap | Mức độ | Mô tả |
|-----|--------|-------|
| **Switch store JWT logic** | 🔴 Critical | `switch_store` luôn tạo JWT với `store_id = user_id + 1M` — chưa thực sự chuyển data sang store khác |
| **Store management page** | 🟡 Medium | Không có trang quản lý cửa hàng (đổi tên, xem thông tin, thêm nhân viên) |
| **Create store from dashboard** | 🟡 Medium | API `POST /api/stores/create` chưa có, chỉ có list + switch |
| **Cross-store reports** | 🟢 Low | Chưa có báo cáo tổng hợp tất cả stores |
| **Per-store settings** | 🟡 Medium | Trang cài đặt hiện tại chỉ hiện info 1 store |
| **Invite / share store** | 🟢 Low | Chưa có flow mời người khác quản lý cùng store |
| **Notification per store** | 🟢 Low | Chưa có notification center |

### ⚠️ Gaps — Chung

| Gap | Mức độ | Mô tả |
|-----|--------|-------|
| **Charts/biểu đồ** | 🟡 Medium | Báo cáo chỉ hiện bảng số, chưa có chart |
| **Export data** | 🟡 Medium | Chưa có export Excel/CSV |
| **Search global** | 🟢 Low | Chưa có search bar trên header |
| **Mobile responsive** | 🟡 Medium | Sidebar responsive có, nhưng tables chưa tối ưu mobile |
| **Loading states** | 🟢 Low | Các trang chưa có skeleton loading |
| **Error boundary** | 🟢 Low | Chưa có error page riêng cho dashboard |

---

## 4. API Endpoints — Hiện Tại

### Auth & Account
| Method | Endpoint | Mô tả |
|--------|----------|-------|
| POST | `/api/register` | Đăng ký account mới |
| POST | `/api/login` | Đăng nhập (account-based) |
| POST | `/api/auth/login` | Đăng nhập (license-based, legacy) |

### Store Management
| Method | Endpoint | Mô tả |
|--------|----------|-------|
| GET | `/api/stores` | List stores của account |
| POST | `/api/stores/switch` | Chuyển active store → new JWT |

### Dashboard (JWT required)
| Method | Endpoint | Mô tả |
|--------|----------|-------|
| GET | `/api/dashboard/overview` | Tổng quan |
| GET | `/api/dashboard/orders` | Danh sách đơn hàng |
| GET | `/api/dashboard/orders/{id}` | Chi tiết đơn |
| GET | `/api/dashboard/inventory` | Tồn kho |
| GET | `/api/dashboard/debts` | Công nợ |
| GET | `/api/dashboard/reports/revenue` | Doanh thu |
| GET | `/api/dashboard/reports/top-products` | Top sản phẩm |
| GET | `/api/dashboard/settings` | Cài đặt |
| PUT | `/api/dashboard/settings/password` | Đổi mật khẩu |

### Sync (PC App → VPS)
| Method | Endpoint | Mô tả |
|--------|----------|-------|
| POST | `/api/sync` | Sync all data types |
| GET | `/api/health` | Health check |

### Backup
| Method | Endpoint | Mô tả |
|--------|----------|-------|
| POST | `/api/backup/upload` | Upload backup file |
| GET | `/api/backup/list` | List backups |
| GET | `/api/backup/download` | Download backup |

---

## 5. Tech Stack

| Layer | Technology |
|-------|-----------|
| PC App | Rust + Tauri + HTML/CSS/JS |
| Web App | Nuxt 3 (Vue 3) + TailwindCSS |
| API | Rust + Axum + sqlx |
| Database | PostgreSQL 16 |
| Infra | Docker Compose + Nginx |
| Hosting | VPS (Ubuntu 24) |

---

## 6. Sprint Conventions

- **Sprint naming:** Sprint N (sequential number)
- **Duration:** ~1 week per sprint
- **Branching:** Direct push to main (single developer)
- **Deploy:** `docker compose build axum-api && docker compose up -d`
- **Monitoring:** Docker logs + health endpoint
