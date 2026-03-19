# 🏗️ Hạ Tầng Kỹ Thuật — Nodi POS

> **Ngày tạo**: 2026-03-16 | **Cập nhật**: 2026-03-16 01:06
> **Mục đích**: Tài liệu kiến trúc & hạ tầng nền toàn hệ thống — từ client đến server đến UX.

---

## 📁 Cấu Trúc Folder

| Folder | Mục đích | Trạng thái |
|--------|----------|:----------:|
| `dong-bo/` | Đồng bộ multi-device (sync architecture) | 📝 Đang nghiên cứu |
| `co-so-du-lieu/` | Database schema, migration, backup | ⬜ Chưa viết |
| `bao-mat/` | License, auth, RBAC, encryption | ⬜ Chưa viết |
| `hieu-nang/` | Benchmarks, load test, optimization | ⬜ Chưa viết |
| `giao-dien/` | Design system, components, responsive | ⬜ Chưa viết |
| `trai-nghiem/` | User journey, pain points, feedback | ⬜ Chưa viết |
| `vps-report.md` | **Báo cáo VPS thực tế** (Agent VPS, 15/03) | ✅ Hoàn thành |

---

## 📊 Đánh Giá Tổng Thể

```
┌────────────────────────────────────────────────────────────────┐
│ TẦNG 1: CLIENT                                                 │
│                                                                 │
│ ┌──── PC (Tauri) ────┐    ┌──── Mobile (Tauri) ────┐          │
│ │ Vue 3 + Rust        │    │ Vue 3 + Rust            │          │
│ │ SQLite 46 migrations│    │ SQLite (shared schema)  │          │
│ │ 16 routes, 35 comps │    │ 22 routes, 35 comps     │          │
│ │ AI: 26 intents      │    │ Full parity ✅           │          │
│ │ Score: 8/10         │    │ Score: 7/10             │          │
│ └─────────┬───────────┘    └──────────┬─────────────┘          │
│           │         Sync (3/10) 🔴     │                       │
│           └────────────┬───────────────┘                       │
│                        ▼                                       │
│ TẦNG 2: VPS — api.nodi.vn (6/10)                               │
│ ┌──────────────────────────────────────────────────────┐       │
│ │ Contabo: 2C AMD EPYC / 8GB RAM / 96GB SSD            │       │
│ │ Stack: Nginx 1.29 → Rust/Axum → PostgreSQL 16.12     │       │
│ │ All Docker — 4 containers (973 MB images)             │       │
│ │ 103 API endpoints | 44 DB tables | ~2.3 MB data       │       │
│ │ Cloudflare CDN + UFW firewall                         │       │
│ └──────────────────────────────────────────────────────┘       │
│                                                                 │
│ TẦNG 3: INFRA OPS (4/10)                                       │
│ ❌ No auto-backup  ❌ No monitoring  ❌ No CI/CD               │
│ ❌ No rate limit   ⚠️ SSL expires 15/05/2026                   │
└────────────────────────────────────────────────────────────────┘
```

### Điểm số chi tiết

| Hạng mục | Điểm | Ghi chú |
|----------|:-----:|---------|
| Frontend PC | **8/10** | 35 components, AI tích hợp, RBAC. Cần tách file lớn (sync_commands 88KB, AI Engine 222KB) |
| Frontend Mobile | **7/10** | 35 components, full parity. Chưa test thiết bị thật |
| Rust Backend | **7/10** | Type-safe, 200+ IPC commands. lib.rs monolithic |
| Database SQLite | **7/10** | 46 migrations idempotent. Thiếu UUID, updated_at |
| **Sync** | **3/10** | 🔴 Full-dump push/pull, ID collision, no conflict resolution |
| VPS Stack | **7/10** | Rust/Axum + PG 16 tuned, Docker. Tốt cho scale |
| VPS Security | **5/10** | Cloudflare ✅, nhưng no rate limit, no fail2ban |
| VPS Ops | **4/10** | No auto-backup, no monitoring, SSL sắp hết |
| AI/NLP | **8/10** | 26 intents, 7-layer architecture, 140+ test cases |

---

## 🖥️ App PC — Điểm mạnh & Điểm yếu

### ✅ Điểm mạnh
- **Offline-first**: SQLite local, bán hàng không cần internet
- **Type-safe**: TypeScript + Rust = double type checking
- **Build optimized**: Vite chunking (vendor-vue, vendor-icons, vendor-excel)
- **RBAC**: Permission-based route guard (Owner/Manager/Staff)
- **AI tích hợp**: 26 intents NLP offline + Cloud fallback
- **Auto-update**: Check + notify khi có version mới

### ⚠️ Điểm yếu
| Vấn đề | Mức độ |
|--------|--------|
| sync_commands.rs = 88KB (push + pull + 20 fetch functions) | 🔴 |
| PragmaticAIEngine = 222KB (26 intents, regex-heavy) | 🟠 |
| lib.rs = 18KB, ~200 commands monolithic | 🟠 |
| Sync mỗi 60s blind (kể cả không có thay đổi) | 🟡 |
| Integer auto-increment IDs (collision multi-device) | 🔴 |
| Không có unit test cho business logic | 🟠 |

---

## 📱 App Mobile — Điểm mạnh & Điểm yếu

### ✅ Điểm mạnh
- **Full feature parity** — 35/35 components mirror desktop
- **Safe-area handling** — Notch/home indicator support
- **Auto-redirect** — PC routes → `/m/*` trên mobile tự động
- **Native transitions** — slide-fade, iOS-like feel

### ⚠️ Điểm yếu
| Vấn đề | Mức độ |
|--------|--------|
| Components quá lớn (MobileCustomerDetail 37KB) | 🟠 |
| Duplicate logic giữa PC và Mobile UI | 🟠 |
| Chưa test trên thiết bị Android thật | 🔴 |
| Bluetooth printer chưa test máy in thật | 🟡 |

---

## 🌐 VPS — Tóm Tắt Báo Cáo

> Chi tiết đầy đủ: [vps-report.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/vps-report.md)

| Spec | Giá trị |
|------|---------|
| Provider | Contabo VPS |
| CPU/RAM/Disk | 2C AMD EPYC / 8GB / 96GB SSD |
| OS | Ubuntu 24.04 LTS |
| Stack | Nginx → Rust/Axum → PostgreSQL 16 |
| Containers | 4 Docker (nginx, api, web, postgres) |
| Endpoints | 103 (8 auth, 3 sync, 16 dashboard, 35 admin, 41 khác) |
| DB tables | 44 (all `synced_*` prefix, index on `store_id + local_id`) |
| SSL | Expires **15/05/2026** ⚠️ |
| Chịu tải | ~500-1000 concurrent, ~100 WebSocket |

### 🔴 3 việc khẩn cấp trên VPS
1. **Backup tự động** — cron pg_dump hàng ngày (~30 phút)
2. **Auto-renew SSL** — certbot + cron (~15 phút)
3. **Rate limiting** — Nginx layer hoặc Axum middleware (~1 giờ)

---

## 🔴 Vấn Đề Ưu Tiên Cao Nhất: SYNC

> Đang nghiên cứu. Chi tiết: [dong-bo/README.md](file:///d:/App%20pc/New%20folder/agri-pos-tauri/project/ha-tang/dong-bo/README.md)

**Hiện trạng**: Full-dump push/pull, INSERT OR REPLACE, integer auto-increment IDs
**Hướng đi**: UUID + Incremental + Server-side merge (6 phase, 6-9 tuần)
**Trạng thái**: Đang chờ kết quả nghiên cứu Google DeepSearch

---

## 🗺️ Vị Trí Trong Dự Án

```
project/
├── chien-luoc/          ← WHY + WHAT (chiến lược kinh doanh)
├── ha-tang/             ← HOW (hạ tầng kỹ thuật) ← BẠN ĐANG Ở ĐÂY
├── DEVELOPMENT_JOURNAL.md
├── NODI_PLATFORM_ROADMAP.md
└── SPEC_DRIVEN_OVERVIEW.md
```

---

*Duy trì bởi Trợ Lý Điều Hành. Cập nhật khi có thay đổi hạ tầng.*
