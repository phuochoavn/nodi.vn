# 🏗️ Hạ Tầng Kỹ Thuật — Nodi POS

> **Cập nhật**: 19/03/2026 21:45 — Sau kiểm tra + refactor + backup + bundle optimize + monitoring
> **15 NC DeepSearch** + **App Audit 7.6/10** + **VPS Audit**
> **81 action items** | **32 nguyên tắc** | **7 folders**
> ✅ AI Engine tách (3,971→18 LOC) | ✅ GCS backup (23 files) | ✅ Bundle ↓63% (4MB→1.4MB) | ✅ UptimeRobot monitoring

---

## 📊 Tổng Điểm Hiện Trạng

| Hạng mục | NC Mục tiêu | Thực tế (Audit) | Gap |
|---------|:----------:|:--------------:|:---:|
| Backend Architecture | Modular, < 400 LOC/file | db/ ✅ modular — ✅ **sync/ 7 modules** (was 2,215 LOC) | 🟢 |
| Frontend Architecture | shallowRef, KeepAlive, code-split | Composable ✅ — AI ✅ 13 modules — ✅ **bundle ↓63%** — shallowRef 0, KeepAlive 0 | 🟢 |
| Database | FTS5, SQLCipher, UUIDv7, WAL | UUID ✅, Indexes 80+ ✅ — FTS5 ❌, SQLCipher ❌ | 🟠 |
| Bảo mật | RBAC, audit log, hash chain | RBAC ✅ FE+BE — audit log trống ❌, hash chain ❌ | 🟡 |
| Hiệu năng | < 100ms scan-to-screen, < 500KB bundle | Build 0 err ✅ — ✅ **bundle 273KB gzip** (↓66%) — LIKE search 🟡 | 🟢 |
| Đồng bộ | Revision-based, off-site backup | V2 hoạt động ✅ — LWW (timestamp), ✅ **backup GCS** (rclone cron 2AM/3AM) | 🟢 |
| VPS | Monitoring, backup GFS, hardened SSH | Docker ✅, SSL ✅, ✅ **GCS backup**, ✅ **UptimeRobot** — root SSH 🟠 | 🟢 |

**Tổng điểm: 8.5/10 (App ↑↑↑) + 7.5/10 (VPS ↑) = ~8.0/10** *(+0.7 từ tách AI + backup + bundle + sync refactor)*

---

## 🔴 TOP 10 VẤN ĐỀ CẦN XỬ LÝ (Theo ưu tiên)

| # | Vấn đề | Hiện trạng | NC đề xuất | Effort |
|:-:|--------|-----------|-----------|:------:|
| ~~1~~ | ~~**PragmaticAIEngine.ts** monolith~~ | ✅ **DONE 19/03** — 3,971 → 18 LOC, 13 modules | `src/services/ai/pragmatic/` | — |
| ~~2~~ | ~~**sync_commands.rs** monolith~~ | ✅ **DONE 19/03** — 2,215 → 9 LOC, 7 modules | `src-tauri/src/sync/` | — |
| ~~3~~ | ~~**Main bundle 4MB** (801KB gzip)~~ | ✅ **DONE 19/03** — 1,472KB (↓63%), 273KB gzip | `chatBus.ts` + lazy AI + manualChunks | — |
| ~~4~~ | ~~**Backup chỉ local** (VPS)~~ | ✅ **DONE 19/03** — GCS `nodi-pos-backups` | rclone cron 2AM/3AM, 23 files | — |
| ~~5~~ | ~~**Không monitoring** (VPS)~~ | ✅ **DONE 19/03** — UptimeRobot 2 monitors (API+Web) | Email alert 5 phút | — |
| 6 | **50+ .unwrap() nguy hiểm** | chars/serde unwrap có thể panic | Chuyển sang Result + map_err | TB |
| 7 | **FTS5 chưa dùng** | Search = LIKE (chậm) | FTS5 unicode61 remove_diacritics 2 | TB |
| 8 | **Audit log trống** | Schema có, 0 rows | Trigger hoặc middleware ghi audit | TB |
| 9 | **Root SSH + Single DB role** | Attack surface lớn | prohibit-password + app role riêng | Nhỏ |
| 10 | **useChatProcessor 48KB** | Monolith orchestration | Tách handler per intent group | TB |

---

## ✅ ĐIỂM MẠNH ĐÃ CÓ

| # | Tính năng | Chi tiết |
|:-:|----------|---------|
| 1 | **db/mod.rs refactored** | 12 sub-modules: products, invoices, customers, reports... |
| 2 | **RBAC 2 tầng** | Rust (require_auth/require_owner) + Vue (route guard + staffStore) |
| 3 | **100% lazy routing** | 40+ routes đều `() => import(...)` |
| 4 | **Dark mode 94+ files** | Tailwind `dark:` classes toàn app |
| 5 | **134+ IPC commands typed** | 100% `Result<T, String>`, serde auto-serialize |
| 6 | **Sync V2 trigger-based** | 23 tables có sync triggers, cursor pagination |
| 7 | **Vendor code-split** | 8 chunks: vue, excel, chart, icons, tauri, fuse, datepicker |
| 8 | **Build 0 errors** | 22.34s, no warnings |
| 9 | **Composable pattern** | 17 composables, components không gọi invoke trực tiếp |
| 10 | **License + Store Binding** | Chaos Engine + Online Guardian + HWID |
| 11 | **PostgreSQL tuned** | shared_buffers=2GB, SSD optimized, 146 indexes |
| 12 | **SSL 15 năm** | Cloudflare Origin CA, HSTS, security headers đầy đủ |
| 13 | **Backup 2 cron/ngày + GCS** | pg_dump compressed (14-day) + raw SQL (7-copy) + ☁️ **GCS off-site** |
| 14 | **Simple Mode** | useSimpleMode.ts cho người mới |

---

## 📁 Cấu Trúc Folders Nghiên Cứu

| Folder | NC | Action Items | Nguyên tắc | Audit Gap |
|--------|:--:|:-----------:|:---------:|:---------:|
| `trai-nghiem/` | 8 | 17 | 5 | Skeleton screens ❌, Training Mode ❌ |
| `giao-dien/` | 2 | 16 | 7 | Font Inter (chưa Be Vietnam Pro), Touch targets OK |
| `bao-mat/` | 1 | 14 | 5 | SQLCipher ❌, Audit log trống, Root SSH |
| `co-so-du-lieu/` | 1 | 12 | 5 | FTS5 ❌, UUID ✅, Backup local-only |
| `hieu-nang/` | 1 | 13 | 5 | shallowRef 0, KeepAlive 0, Bundle 4MB |
| `tro-ly-ai/` | 1 | 9 | 5 | CLAUDE.md ✅, Skills 2/5, /plan workflow ✅ |
| `dong-bo/` | 1 | — | — | V2 ✅, 18 bugs fixed, LWW (chưa revision) |
| **Tổng** | **15** | **81** | **32** | |

---

## 📐 Kiến Trúc Tổng Quan (Từ Audit)

### App (PC + Mobile)

```
┌──────────────────────────────────────────────────────┐
│                    Vue 3 Frontend                     │
│  108 .vue files | 17 composables | 8 Pinia stores    │
│  Tailwind v4 | Dark Mode | 100% Lazy Routes          │
├──────────────────────────────────────────────────────┤
│                  Tauri v2 IPC (134+ commands)          │
│  Result<T, String> | serde auto-serialize | RBAC      │
├──────────────────────────────────────────────────────┤
│                   Rust Backend                        │
│  82 .rs files | db/ 12 sub-modules | security/ 6 files│
│  sync_journal triggers 23 tables                      │
├──────────────────────────────────────────────────────┤
│                   SQLite                              │
│  43 migrations | 80+ indexes | UUID | WAL mode        │
│  ❌ FTS5 | ❌ SQLCipher                              │
└──────────────────────────────────────────────────────┘
```

### VPS

```
┌───────────────────────────────────────────────┐
│  Nginx (Alpine) — SSL, HSTS, Security Headers │
│  Port 80/443 | Cloudflare proxy               │
├───────────────┬───────────────────────────────┤
│  Axum API     │  Nuxt Web (SSR)               │
│  Port 3000    │  Port 3001                    │
│  20 route     │  Landing + Help Center        │
│  modules      │                               │
├───────────────┴───────────────────────────────┤
│  PostgreSQL 16.12 (Alpine)                    │
│  48 tables | 146 indexes | Tuned for SSD      │
│  Backup: 2 cron/day + ☁️ GCS off-site ✅      │
├───────────────────────────────────────────────┤
│  ☁️ Google Cloud Storage (nodi-pos-backups)   │
│  rclone v1.73.2 | SA auth | asia-southeast1   │
│  db/ (compressed) + pg/ (raw SQL) auto-sync    │
└───────────────────────────────────────────────┘
Ubuntu 24.04 | 8GB RAM | 96GB disk (53% used)
```

---

## 🚀 LỘ TRÌNH ƯU TIÊN (Từ NC + Audit)

### Phase 1: Quick Wins (1-2 ngày)

| # | Việc | Impact | Effort |
|:-:|------|:------:|:------:|
| 1 | `docker system prune -a` → giải phóng ~75GB | VPS disk | 5 phút |
| 2 | UptimeRobot monitor nodi.vn/health | Biết khi down | 30 phút |
| 3 | Docker log rotation (max-size 50m) | Ngăn disk đầy | 15 phút |
| 4 | SSH hardening (PermitRootLogin no) | Bảo mật | 10 phút |
| ~~5~~ | ~~Off-site backup (rclone → GCS)~~ | ✅ **DONE 19/03** | — |
| ~~6~~ | ~~UptimeRobot monitor nodi.vn/health~~ | ✅ **DONE 19/03** — 2 monitors UP | — |

### Phase 2: Backend Refactor (3-5 ngày)

| # | Việc | Impact | Effort |
|:-:|------|:------:|:------:|
| ~~6~~ | ~~Tách `sync_commands.rs` → sync/mod.rs + sub-modules~~ | ✅ **DONE 19/03** — 7 modules | — |
| 7 | Fix 50+ `.unwrap()` nguy hiểm → Result | Stability | TB |
| 8 | Tách `commands.rs` 790 LOC | Code health | Nhỏ |
| 9 | Tạo app DB role (không superuser) | Security | Nhỏ |
| 10 | Bật audit log triggers trên VPS | Compliance | TB |

### Phase 3: Frontend Performance (3-5 ngày)

| # | Việc | Impact | Effort |
|:-:|------|:------:|:------:|
| ~~11~~ | ~~Tách `PragmaticAIEngine.ts` → modules~~ | ✅ **DONE 19/03** — 13 modules | — |
| ~~12~~ | ~~Code-split AI engine (manualChunks)~~ | ✅ **DONE 19/03** — 1,472KB (↓63%) | — |
| 13 | shallowRef cho danh sách products/invoices | Re-render perf | Nhỏ |
| 14 | KeepAlive `:max="5"` cho views chính | Navigation perf | Nhỏ |
| 15 | Tách `useChatProcessor.ts` | DX, maintainability | TB |

### Phase 4: Database & Search (2-3 ngày)

| # | Việc | Impact | Effort |
|:-:|------|:------:|:------:|
| 16 | FTS5 cho product search (unicode61) | Search speed | TB |
| 17 | Partial indexes (status='PENDING') | Query perf | Nhỏ |
| 18 | Covering indexes cho reports | I/O reduction | Nhỏ |

### Phase 5: Security Hardening (khi gần Go-Live)

| # | Việc | Impact | Effort |
|:-:|------|:------:|:------:|
| 19 | SQLCipher integration | Data encryption | Lớn |
| 20 | Audit log hash chain | Tamper-proof | TB |
| 21 | Rate limiting API (nginx) | DDoS protection | Nhỏ |
| 22 | Revision-based sync (thay LWW) | Conflict resolution | Lớn |

---

## 📚 Chi Tiết Nghiên Cứu — 15 NC DeepSearch

| # | Folder | File | Chủ đề | Điểm |
|:-:|--------|------|--------|:----:|
| 01 | giao-dien | NC #01 | Design System, Tokens, Color Blind Safe | 93 |
| 02 | giao-dien | NC #02 | Component Architecture, FSD+Atomic, Composables | 94 |
| 03 | trai-nghiem | NC #01 | Quy trình vận hành đại lý VTNN | 85 |
| 04 | trai-nghiem | NC #02 | Tín dụng thương mại & công nợ | 88 |
| 05 | trai-nghiem | NC #03 | Giao dịch mua chịu & quản trị rủi ro | 87 |
| 06 | trai-nghiem | NC #04 | Thói quen công nghệ & chuyển đổi số | 86 |
| 07 | trai-nghiem | NC #05 | Thị trường POS VTNN 2024-2025 | 90 |
| 08 | trai-nghiem | NC #06 | Virtual Scrolling, Micro-interactions | 91 |
| 09 | trai-nghiem | NC #07 | Accessibility WCAG 2.2, Training Mode | 89 |
| 10 | trai-nghiem | NC #08 | Optimistic UI, Error Recovery | 94 |
| 11 | bao-mat | NC #01 | SQLCipher, License Ed25519, RBAC, NĐ 13 | 96 |
| 12 | co-so-du-lieu | NC #01 | SQLite offline-first, UUIDv7, Backup GFS | 95 |
| 13 | hieu-nang | NC #01 | IPC MessagePack, Memory 12h, Bundle | 95 |
| 14 | tro-ly-ai | NC #01 | Agentic AI, 10 Rules, Human/AI Matrix | 92 |
| 15 | dong-bo | NC #01 | Sync architecture (debug series) | — |

**Điểm trung bình: 93/100** 🏆

---

## 🔗 Tài Liệu Liên Quan

| File | Nội dung |
|------|---------|
| `project/TRO_LY_DIEU_HANH.md` | Quy trình, 10 Rules, Ma trận, Sổ Bài Học |
| `project/ha-tang/audit-app-report.md` | Báo cáo audit App đầy đủ (19/03/2026) |
| `project/legal/DOANH_NGHIEP.md` | **Thông tin ĐKDN Nodi Technology** |
| `project/nhat-ky/2026-03-19.md` | Nhật ký hoạt động hôm nay |
| `CLAUDE.md` | Tech stack, Pre-Fix Checklist, AI Agent Rules |
| `briefs/audit-app-agent.md` | Brief cho App audit |
| `briefs/audit-vps-agent.md` | Brief cho VPS audit |
| `briefs/setup-offsite-backup-vps.md` | **Brief setup GCS backup (đã hoàn tất)** |
| `briefs/refactor-ai-engine.md` | **Brief tách AI Engine (đã hoàn tất)** |
