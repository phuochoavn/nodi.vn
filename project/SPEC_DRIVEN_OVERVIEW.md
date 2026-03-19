# 🔭 NODI POS — Spec-Driven Overview & Gap Analysis

> **Ngày tạo**: 2026-02-27 | **Cập nhật**: 2026-03-14 08:11
> **Dựa trên**: 3 file roadmap (~200KB), 120 sprints
> **Mục đích**: Định hình spec-driven development cho giai đoạn tiếp theo

---

## 1. Kiến Trúc Tổng Thể — ĐÃ RÕ RÀNG ✅

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
     └───────────┘  └─────┬─────┘
                          │
                   ┌──────┴──────┐
                   │ PostgreSQL  │
                   └─────────────┘
```

| Layer       | Stack                       |     Status     |
| ----------- | --------------------------- | :-------------: |
| Desktop App | Tauri v2 + Vue 3 + SQLite   |    ✅ v2.0.0    |
| **Mobile App** | **Tauri v2 Mobile + Vue 3** | **✅ Full Parity (Sprint 113)** |
| Backend API | Rust Axum + PostgreSQL 16   |  ✅ Sprint 2-8  |
| Website     | Nuxt 3 SSR + SPA            |     ✅ Live     |
| Infra       | Docker + Nginx + Cloudflare | ✅ 4 containers |
| AI Chatbot  | 100% Offline NLP Engine     |   ✅ Level 8   |

---

## 2. AI Chatbot — Module Lớn Nhất

### Pipeline (6 stages → 48 intents)

```
Input → Sanitize → CONFIRM/REJECT → VietNormalizer(VietTokenizer)
      → SynonymEngine → PhoneticMatcher → normalize()
      → 48 Intent Patterns → NgramScorer fallback
      → ContextBooster → QuerySplitter → ReasoningGate → Response
```

| Metric            |                        Hiện tại                        |
| ----------------- | :------------------------------------------------------: |
| Intent types      |                       **46+**                       |
| Disease Expert DB |             **315+ entries** (8 loại cây)             |
| Product DB        |              **5,656 SP** (11 files)              |
| Knowledge bases   | 5 files (farming, fertilizer, tax 125+, FAQ 87, templates 40) |
| Test cases        |               **1,238** (100% pass)               |
| ML model          |             MiniLM ONNX int8 (~30MB) offline             |
| NLP modules       |                        ~25 files                        |

---

## 3. Spec Coverage — Đã Có vs Thiếu

### ✅ ĐÃ CÓ SPEC RÕ RÀNG

| Mục                   | Nguồn                                     | Mức độ |
| ---------------------- | ------------------------------------------ | :-------: |
| Kiến trúc hệ thống | `NODI_PLATFORM_ROADMAP.md` §Kiến trúc |  ⭐⭐⭐  |
| User roles & auth flow | §Hệ thống người dùng                 |  ⭐⭐⭐  |
| Data sync protocol     | §Data Sync Protocol                       |  ⭐⭐⭐  |
| Backup system          | §Backup System                            |  ⭐⭐⭐  |
| Sitemap & routing      | §Sitemap chi tiết                        |  ⭐⭐⭐  |
| Market Intelligence    | §Market Intelligence                      |   ⭐⭐   |
| Test framework         | §Hệ thống Testing                       |  ⭐⭐⭐  |
| Agent workflow         | `ANTIGRAVITY_OPTIMIZATION_RESEARCH.md`   |  ⭐⭐⭐  |
| AI chatbot pipeline    | Sprint 9-43 trong cả 2 files              |  ⭐⭐⭐  |

### ⚠️ CÓ NHƯNG CHƯA ĐỦ SPEC

| Mục           | Có gì                          | Thiếu gì                                                 |
| -------------- | -------------------------------- | ---------------------------------------------------------- |
| GO LIVE        | 2 checkbox `[ ] 🚀 GO LIVE`    | Checklist cụ thể: smoke tests, rollback plan, monitoring |
| Multi-branch   | `store_groups` table đã tạo | Feature spec: UI, sync, group management                   |
| CUSTOMER_CHURN | Intent tồn tại                 | `last_purchase_date` tracking spec                       |
| DEBT_REMINDER  | TODO Sprint 9 Phase 5            | Scheduling backend spec                                    |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 44

| Mục                | Trước                     | Sprint 44                                                |
| ------------------- | --------------------------- | -------------------------------------------------------- |
| Free/Pro user gates | Chưa có spec              | ✅ 44C:`UpgradePromptModal` + quota gate + tier badges |
| Sync error handling | Im lặng (console.warn)     | ✅ 44B:`SyncStatusIndicator` + toast + offline queue   |
| Cloud backup        | Chỉ có local backup       | ✅ 44A:`cloud_backup` + auto-schedule + restore        |
| OTA update UI       | Chỉ check, không download | ✅ 44C: Đã có sẵn (UpdateNotificationModal)          |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 49–50

| Mục                    | Trước                         | Sprint 49–50                                                   |
| ----------------------- | ------------------------------- | -------------------------------------------------------------- |
| SQLite migration strategy | 600+ dòng inline checks      | ✅ 49: `migrations.rs` — numbered runner + `schema_migrations` table |
| OTA in-app download     | Mở browser tải               | ✅ 49B: reqwest stream + progress bar + auto-install           |
| Error monitoring        | `println!` mất khi đóng app | ✅ 49B: `app_logger.rs` — daily log file + 7-day rotation      |
| API contract            | Không có docs                 | ✅ 49B: `docs/API_REFERENCE.md` — 15 endpoints documented     |
| Multi-store isolation   | Shared data partition          | ✅ 50 VPS: `data_store_id` column + switch/create endpoints   |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 89-93

| Mục | Trước | Sprint 89-93 |
| --- | ----- | ------------ |
| Staff Permissions | Không có phân quyền NV | ✅ 89: 9 permission flags + PIN + sidebar filter |
| Khuyến mãi / Voucher | Không có | ✅ 91: Full-stack promotions + vouchers |
| In barcode / nhãn SP | Không có | ✅ 91: SVG barcode + 80mm thermal template |
| Thuế TNCN | Chỉ có GTGT | ✅ 91: PIT Calculator theo TT 40/2021 |
| Mobile App | Chỉ Windows | ✅ 92: 14 components MVP, 7 routes |
| Multi-Device License | 1 HWID / license | ✅ 93: 1 license = 10 devices max |
| KiotViet Gap Analysis | Không có | ✅ 93: 14 feature gaps analyzed |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 103-113 (12/03/2026)

| Mục | Trước | Sprint 103-113 |
| --- | ----- | -------------- |
| Hoạt chất cấm trong AI DB | 8 hoạt chất cấm được gợi ý | ✅ 104A: Thay thế + cảnh báo |
| active_ingredient không lưu DB | Bug Rust bỏ qua field | ✅ 105A: Migration + IPC commands |
| Công nợ vụ mùa | Chỉ có nợ chung | ✅ 105B: crop_seasons + auto-season + sao kê |
| Auto-Lock thuốc cấm | Không có bảo vệ | ✅ 106: 3 tầng (Inventory+POS+Backend) |
| Truy xuất nguồn gốc | Không có | ✅ 107: Batch trace + QR code |
| Mutex deadlock checkout | App freeze khi ghi nợ | ✅ 108: drop(conn) fix |
| UX tabs KH quá nhiều | 6 tabs confusing | ✅ 109: Gộp thành 4 tabs + sub-tabs |
| Category filter không hoạt động | 0 kết quả khi filter | ✅ 110: Đồng bộ mapCategory + backfill |
| Nợ không phân loại | Chỉ 1 con số tổng | ✅ 111: 3 cards + filter loại GD |
| Mobile thiếu 11 features | 4/15 parity | ✅ 113A-C: 15/15 parity |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 115 (14/03/2026)

| Mục | Trước | Sprint 115 |
| --- | ----- | ---------- |
| Seed data hoạt chất cấm | Chỉ 6 hoạt chất | ✅ 045: ~45 hoạt chất (31 Phụ lục II + 10 lịch sử) |
| Countdown timer Carbosulfan/Benfuracarb | Không có | ✅ get_transitioning_ingredients + UI countdown |
| Ánh xạ thương phẩm → hoạt chất | Chỉ match active_ingredient | ✅ 046: product_ingredient_mapping (11 patterns) |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 116 (14/03/2026)

| Mục | Trước | Sprint 116 |
| --- | ----- | ---------- |
| Cross-selling thuốc sinh học | Không gợi ý thay thế | ✅ 047: bio_alternatives (10 entries) + POS toast |
| HĐĐT lifecycle | Chỉ có tạo mới | ✅ Hủy/Điều chỉnh/Thay thế (stub + UI) |
| Khóa nhập kho thuốc cấm | Không kiểm tra | ✅ Guard trong create_purchase_order() |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 117 (14/03/2026)

| Mục | Trước | Sprint 117 |
| --- | ----- | ---------- |
| Chọn đơn vị tính mobile | Luôn lấy units[0] | ✅ Unit Picker bottom sheet + đổi trong giỏ |
| Chọn lô hàng mobile | Không có | ✅ Batch Picker FEFO + badge hết hạn |
| Trả hàng mobile | Không có | ✅ Return modal full flow |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 118 (14/03/2026)

| Mục | Trước | Sprint 118 |
| --- | ----- | ---------- |
| Nhập hàng mobile | Không có | ✅ MobileImportView 3-step + History |
| Staff CRUD mobile | Không có | ✅ MobileStaffManager + role presets |
| Kiểm kê tồn kho | Không có (cả PC lẫn Mobile) | ✅ MobileStocktake + adjust_stock cmd |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 119 (14/03/2026)

| Mục | Trước | Sprint 119 |
| --- | ----- | ---------- |
| Thuế mobile | Không có | ✅ MobileTaxView + HĐĐT badges |
| Onboarding | Không có | ✅ 4-step wizard + localStorage |
| Support | Không có | ✅ Hotline/Zalo/FAQ |
| Loyalty CRUD | Không có | ✅ Toggle + config + save |
| Banned CRUD mobile | Readonly | ✅ Thêm/Xóa form |

### ✅ ĐÃ GIẢI QUYẾT TRONG SPRINT 120 (14/03/2026) — FULL PARITY

| Mục | Trước | Sprint 120 |
| --- | ----- | ---------- |
| AI Chatbot mobile | Không có | ✅ FAB + fullscreen chat + quick actions |
| Xuất Excel mobile | Không có | ✅ Reports + Orders → xlsx |
| Mời NV QR | Không có | ✅ QR invite bottom sheet |
| In tem mã vạch | Không có | ✅ Label preview (Bluetooth TBD) |
| DB Backup mobile | Không có | ✅ Push/Pull sync + timestamp |

> **🎯 MOBILE GAP: 25 → 0 — Full Parity đạt được!**

### ❌ CHƯA CÓ SPEC

| Mục                                | Tại sao cần                                                |
| ----------------------------------- | ------------------------------------------------------------ |
| **PRD per feature**           | Mỗi module (POS, Chatbot, Dashboard, Admin) cần doc riêng |
| **Performance targets**       | Response time cho API, chatbot, sync                        |
| **Security audit**            | Penetration test, vulnerability scan                         |
| **Acceptance criteria**       | Given/When/Then cho mỗi feature                             |

---

## 4. Đánh Giá Chiến Lược

### Điểm Mạnh 💪

1. **Offline-first** — Chatbot 100% offline, không phụ thuộc Cloud AI
2. **Test-driven** — 1,213+ tests, regression bắt buộc trước commit
3. **Multi-agent** — Brief → Agent → Verify flow hiệu quả
4. **Domain depth** — 206 disease entries, 5,656 SP BVTV thực tế
5. **Data moat** — Market intelligence từ aggregated đại lý data

### Rủi ro ⚠️

1. **Spec drift** — Roadmap = nhật ký hoàn thành, không phải backlog ưu tiên
2. **Single-person dependency** — Mọi quyết định qua anh Hoa
3. **GO LIVE chưa rõ** — Chiến lược: hoàn thiện trước, tung sau (đã thống nhất)
4. **Chưa test thực tế** — Sprint 44 thêm nhiều code, cần anh Hoa test trước khi tiếp tục

---

## 5. Đề Xuất: 5 Bước Spec-Driven

| # | Hành động                                                        | Output                    | Ưu tiên |
| :-: | ------------------------------------------------------------------- | ------------------------- | :-------: |
| 1 | **Backlog có priority** — P0/P1/P2 thay sprint tuyến tính | `roadmap/BACKLOG.md`    |    P0    |
| 2 | **GO LIVE checklist** — smoke tests, monitoring, rollback    | Thêm vào ROADMAP        |    P0    |
| 3 | **API contract** — OpenAPI YAML cho tất cả endpoints       | `specs/api.yaml`        |    P1    |
| 4 | **PRD per module** — POS, Chatbot, Dashboard, Admin          | `specs/` folder         |    P1    |
| 5 | **Test plan mapping** — spec → test case                    | Liên kết specs ↔ tests |    P2    |

### Template PRD đề xuất

```markdown
# [Tên Feature]
## Bối cảnh & Mục tiêu
## User Stories
## Acceptance Criteria (Given/When/Then)
## Technical Design
## API Endpoints (nếu có)
## Test Cases
## Rollback Plan
```

---

## 6. Tổng Kết Con Số

| Metric                  |              Giá trị              |
| ----------------------- | :----------------------------------: |
| Sprints hoàn thành    |            **120**             |
| Files mã nguồn        |            **280+**            |
| LOC ước tính         |          **85,000+**          |
| Test cases              |           **1,238**            |
| Disease entries         |            **315+**            |
| Products                |           **5,656**           |
| VPS API endpoints       |         **20+ (documented)**   |
| Nuxt pages              |            **~25**            |
| Docker containers       |             **4**             |
| SQLite migrations       |         **47** (numbered runner) |
| Mobile components       |         **32+** (✅ FULL PARITY)  |
| VTNN killer features    |         **3** (Thuốc cấm, Vụ mùa, Truy xuất) |
| Agents đã dùng       |         **15+ loại**         |
| Thời gian phát triển | **~42 ngày** (31/01 → 14/03) |

---

*Cập nhật: 14/03/2026. Dựa trên: `NODI_PLATFORM_ROADMAP.md`, `DEVELOPMENT_JOURNAL.md`, `ANTIGRAVITY_OPTIMIZATION_RESEARCH.md`*
