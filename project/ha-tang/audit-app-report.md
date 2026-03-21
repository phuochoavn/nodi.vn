# 🔍 BÁO CÁO KIỂM TRA TOÀN DIỆN — Nodi POS

> **Ngày kiểm tra**: 19/03/2026  
> **Agent**: Opus 4.6 | Mode: Chỉ ĐỌC + BÁO CÁO  
> **Phiên bản app**: agri-pos-tauri@1.0.0  
> **Stack**: Tauri v2 + Vue 3 + Rust + Tailwind CSS v4

---

## PHẦN 1: KIẾN TRÚC BACKEND (Rust/Tauri)

### 1.1. Cấu trúc `lib.rs`

| Metric | Giá trị |
|--------|---------|
| **Dung lượng** | 19KB, 603 dòng |
| **Số `#[tauri::command]` đăng ký** | 134+ commands trong `tauri::generate_handler![]` |
| **Số modules declare** | 40+ module declarations |

#### Danh sách files `.rs` và files > 400 dòng (⚠️ vi phạm NC hiệu năng)

| File | Dòng | KB | Trạng thái |
|------|------|----|------------|
| `sync_commands.rs` | **2,215** | 89 | 🔴 CẦN TÁCH |
| `db/migrations.rs` | **1,266** | 55 | 🟡 Chấp nhận (tính chất đặc biệt) |
| `security/online_guardian.rs` | **889** | 32 | 🟡 Biên giới |
| `commands.rs` | **790** | 27 | 🔴 CẦN TÁCH |
| `lib.rs` | **603** | 19 | 🟡 Biên giới |
| `sync_v2_pull.rs` | ~523+ | ~20 | 🟡 Biên giới |
| `sync_v2_push.rs` | ~400+ | ~15 | ⚠️ Cần kiểm tra |

> **Tổng**: 82 files `.rs` trong `src-tauri/src/`

**Đánh giá**: `db/mod.rs` đã được refactor tốt thành 12 sub-modules (products, invoices, customers, reports, suppliers, inventory, returns, alerts, daily_closing, promotions, loyalty, banned_ingredients, crop_seasons, tax, migrations). Tuy nhiên `sync_commands.rs` (2,215 dòng) và `commands.rs` (790 dòng) **cần tách** theo chiến lược tương tự.

---

### 1.2. IPC Commands

| Metric | Giá trị |
|--------|---------|
| **Tổng số IPC commands** | **134+** |
| **Format trả về** | 100% `Result<T, String>` (structured data) |
| **Serialization** | Tauri auto-serialize qua `serde` → JSON |

**✅ Tốt**: Không có command nào trả raw JSON string. Tất cả dùng typed Rust structs.

#### `.unwrap()` Analysis

| Loại | Số lượng | Đánh giá |
|------|----------|----------|
| `state.db.lock().unwrap()` | ~200+ | 🟡 Pattern chuẩn Mutex, chấp nhận được |
| `db.conn.lock().unwrap()` | ~30+ | 🟡 Nested lock, cần cẩn thận deadlock |
| Logic `.unwrap()` (chars, serde) | ~50+ | 🔴 **NGUY HIỂM** — có thể panic |
| **Tổng `.unwrap()`** | **287+** | - |

**Các vị trí `.unwrap()` nguy hiểm cần fix**:
- `staff_invite_commands.rs:20` — `.chars().nth(idx).unwrap()` → panic nếu out-of-bounds
- `sync_commands.rs:705` — `pin.as_ref().unwrap().is_empty()` → panic nếu None
- `online_guardian.rs:884` — `serde_json::to_string().unwrap()` → panic nếu serialize fail
- `system_fingerprint.rs:95` — `.unwrap()` trên operation có thể fail

**Error handling pattern**:
```rust
// ✅ Chuẩn — dùng ở hầu hết commands
pub fn get_all_products(state: State<AppState>) -> Result<Vec<ProductWithUnits>, String> {
    let db = state.db.lock().unwrap(); // Mutex unwrap — acceptable
    db.get_all_products()
        .map_err(|e| format!("Lỗi: {}", e))  // Structured error
}
```

---

### 1.3. Database

| Metric | Giá trị |
|--------|---------|
| **Migration files** | 43 trong `src-tauri/migrations/` + inline |
| **Migration runner** | `db/migrations.rs` — versioned, idempotent, transactional |
| **PRAGMA usage** | `table_info`, `foreign_keys ON/OFF`, `database_list` |
| **Triggers** | `sync_journal` triggers trên **23 tables** (INSERT/UPDATE/DELETE) |
| **Indexes** | 80+ indexes (schema.sql + migrations) |
| **FTS5** | ❌ **KHÔNG SỬ DỤNG** — search bằng LIKE |
| **SQLCipher** | ❌ **KHÔNG TÍCH HỢP** |
| **UUID** | ✅ Có trên tất cả sync tables (uuid TEXT UNIQUE) |
| `foreign_key_check` after migration | ❌ Không có explicit check |

**Sync triggers pattern** (từ `db/mod.rs:repair_sync_triggers`):
- 7 **mutable** tables: INSERT + UPDATE + DELETE triggers
- 16 **append-only** tables: chỉ INSERT trigger
- Triggers tự động ghi vào `sync_journal` với `record_id`, `record_uuid`, `action`, `device_id`, `changed_at`
- `repair_sync_triggers()` chạy mỗi lần startup để đảm bảo trigger integrity

---

### 1.4. Bảo mật

| Metric | Giá trị |
|--------|---------|
| **SQLCipher** | ❌ Không có |
| **Tauri Capabilities** | `default.json` — HTTP whitelist: `googleapis.com`, `api.groq.com`, `api.nodi.vn` |
| **RBAC** | ✅ Cả Rust (`require_auth`, `require_owner`) VÀ Frontend (route guard) |
| **Security module** | 6 files: `chaos.rs`, `feature_guard.rs`, `license.rs`, `mod.rs`, `online_guardian.rs`, `system_fingerprint.rs` |
| **Audit triggers** | ✅ `sync_journal` trên 23 tables |
| **License protection** | ✅ Chaos Engine (soft fail) + Online Guardian (kill switch) |
| **Store Binding** | ✅ HWID-based hardware binding |

**RBAC Implementation**:
- **Backend**: `require_auth()` cho mutating commands, `require_owner()` cho reports/sensitive data
- **Frontend**: `ROUTE_PERMISSION_MAP` trong router guard, `staffStore.hasPermission()`
- **Capabilities**: HTTP chỉ cho phép 3 domains (Google AI, Groq, Nodi API)

---

## PHẦN 2: KIẾN TRÚC FRONTEND (Vue 3/TypeScript)

### 2.1. Component Architecture

| Metric | Giá trị |
|--------|---------|
| **Tổng `.vue` files** | **108** |
| **Desktop views** | 19 files trong `src/views/` |
| **Mobile components** | 30 files trong `src/components/mobile/` |
| **Settings tabs** | 13 files trong `src/components/settings/` |
| **Shared components** | 33+ files trong `src/components/` |
| **Layouts** | 2 files: `MainLayout.vue`, `MobileLayout.vue` |
| **UI components** | `BaseModal.vue` (reusable modal) |

#### Files > 400 dòng (Frontend — từ view_file total lines)

| File | Dòng | KB | Đánh giá |
|------|------|----|----------|
| `services/PragmaticAIEngine.ts` | **3,971** | 222 | 🔴 **MONOLITH** — cần tách patterns/matchers |
| `composables/useChatProcessor.ts` | ~1,400+ | 48 | 🔴 CẦN TÁCH |
| `router/index.ts` | 494 | 15 | 🟡 Chấp nhận (route config) |

---

### 2.2. Reactive Performance

| Kiểm tra | Kết quả | Đánh giá |
|----------|---------|----------|
| `shallowRef()` usage | **0 files** | 🟡 Toàn bộ dùng deep `ref()` — có thể gây re-render thừa cho arrays lớn |
| `<KeepAlive>` usage | **0 files** | 🟡 Views không được cache — mỗi lần navigate sẽ re-mount |
| `onUnmounted` cleanup | ~19 files | ✅ Một số composables có cleanup |
| `setInterval` usage | ~7 files | ⚠️ Cần verify cleanup on unmount |
| `addEventListener` | 9 files | ⚠️ Cần verify `removeEventListener` paired |
| `watch()` calls | **55+** instances | 🟡 Không thấy `watchEffect` — tất cả dùng explicit `watch()` |
| `watch()` stop handles | ❌ Không được lưu | 🔴 Watchers không bao giờ bị stop manually |

---

### 2.3. Bundle Analysis (Production Build)

```
Build time: 22.34s
Build errors: 0
Build warnings: 0 (suppressed by vite.config.ts)
```

| Chunk | Size | Gzip | Đánh giá |
|-------|------|------|----------|
| `index.js` (main app) | **4,000 KB** | 801 KB | 🔴 **CỰC LỚN** — toàn bộ app code + AI engine |
| `vendor-excel` (xlsx/exceljs) | 1,224 KB | 366 KB | 🟡 Expected — library nặng |
| `vendor-vue` | 353 KB | 117 KB | ✅ OK |
| `vendor-chart` | 207 KB | 71 KB | ✅ OK |
| `vendor-icons` (lucide) | 49 KB | 9 KB | ✅ OK |
| `vendor-tauri` | 23 KB | 5 KB | ✅ OK |
| `vendor-fuse` | 18 KB | 7 KB | ✅ OK |

**`manualChunks` configuration** (8 vendor splits):
- `vendor-vue`: vue, pinia, vue-router
- `vendor-icons`: lucide-vue-next
- `vendor-excel`: xlsx, exceljs
- `vendor-chart`: chart.js, vue-chartjs
- `vendor-datepicker`: @vuepic, vue-datepicker
- `vendor-fuse`: fuse.js
- `vendor-tauri`: @tauri-apps

> **GHI CHÚ**: `chunkSizeWarningLimit` được set `Number.MAX_SAFE_INTEGER` vì Tauri desktop load assets cục bộ — chunk size không ảnh hưởng performance. Tuy nhiên main `index.js` 4MB cho thấy `PragmaticAIEngine.ts` (222KB) KHÔNG được code-split.

**Lazy loading**: ✅ 100% routes dùng `() => import(...)` — tất cả 40+ routes

---

### 2.4. Styling & UX

| Metric | Giá trị |
|--------|---------|
| **CSS Framework** | Tailwind CSS v4 |
| **Font** | Inter (Google Font) — fallback: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto |
| **Dark Mode** | ✅ Implemented — `dark:` Tailwind classes trong **94+ files** |
| **Skeleton screens** | ❌ **KHÔNG CÓ** — loading states dùng text/spinner |
| **Simple Mode** | ✅ `useSimpleMode.ts` — hạn chế routes cho người mới |
| **Toast notifications** | ✅ `useToast.ts` composable — sử dụng trong **58+ files** |
| **Error handling (UI)** | Toast-based — ✅ Có hệ thống |
| **Touch targets** | Mobile components sử dụng Tailwind responsive classes |
| **Product search** | `LIKE` query (SQLite) — ❌ **KHÔNG dùng FTS5** |

---

## PHẦN 3: ĐỒNG BỘ DỮ LIỆU (Sync)

### 3.1. Kiến trúc Sync

| Metric | Giá trị |
|--------|---------|
| **Sync versions** | V1 (full push) + V2 (incremental push/pull) — **cùng tồn tại** |
| **V1 strategy** | Full table dump → POST to VPS |
| **V2 strategy** | `sync_journal` trigger-based + cursor pagination |
| **Conflict resolution** | **Last-Write-Wins** (timestamp `updated_at`) |
| **Real-time sync** | WebSocket `sync_v2_update` event |
| **Identity** | JWT (account) → HWID fallback (anonymous) |
| **API endpoint** | `https://api.nodi.vn/api/sync` |

### 3.2. Sync Data Coverage (V1 Push)

Sync payload bao gồm **18 data types**:
1. `customers` + `customer_transactions`
2. `products` + `product_units` + `product_batches` + `product_transactions`
3. `invoices` (orders) + `invoice_items` + `invoice_payments`
4. `suppliers` + `supplier_transactions`
5. `purchase_orders` + `purchase_items`
6. `cash_transactions` + `payment_vouchers`
7. `store_settings` + `store_funds`
8. `staff_members`
9. `promotions` + `vouchers`
10. `daily_closings`
11. `loyalty_transactions` + `loyalty_settings`
12. `returns` + `return_items`

### 3.3. Sync V2 Pull

- `sync_v2_pull.rs` xử lý cursor-based pull từ VPS
- `PRAGMA foreign_keys = OFF` trong quá trình pull → bật lại sau
- Column mapping tự động qua `PRAGMA table_info()`
- Conflict resolution: `INSERT OR REPLACE` based on `uuid`

### 3.4. Backup

| Metric | Giá trị |
|--------|---------|
| **Cloud backup table** | `cloud_backups` (migration 32) |
| **DB file backup** | VPS sync doubles as backup |
| **Manual export** | ✅ Excel export cho customers, inventory, orders, cashflow |

---

## PHẦN 4: TỔ CHỨC CODE

### 4.1. Pinia Stores (8 stores)

| Store | KB | Mô tả |
|-------|------|-------|
| `userStore.ts` | 12 | Auth, JWT, cloud user |
| `posStore.ts` | 11 | Cart, checkout flow |
| `syncStore.ts` | 11 | Sync status, timestamps |
| `staffStore.ts` | 7 | Staff management, RBAC |
| `cashflow.ts` | 3 | Cash transactions |
| `trialStore.ts` | 2 | Trial tracking |
| `settings.ts` | 2 | App settings (localStorage persist) |
| `authStore.ts` | 1 | Auth state |

**Persistence**: Không dùng `pinia-plugin-persistedstate`. `settings.ts` tự persist vào `localStorage`. `userStore.ts` persist JWT token.

### 4.2. Composables (17 + handlers/)

| Composable | KB | Chức năng |
|------------|------|----------|
| `useChatProcessor.ts` | **48** | 🔴 AI chat processing — CẦN TÁCH |
| `useTour.ts` | 17 | Onboarding tour |
| `useDatabase.ts` | 15 | IPC database calls |
| `useExportHandler.ts` | 15 | Excel/PDF export |
| `useRemoteScanner.ts` | 5 | Barcode scanner |
| `useLicenseGuard.ts` | 5 | License check |
| `useDismissibleNotifications.ts` | 4 | Dismissible alerts |
| `useInventoryAlerts.ts` | 3 | Low stock alerts |
| `useInvoiceAlerts.ts` | 3 | Invoice warnings |
| `useDeviceDetect.ts` | 2 | Mobile/desktop detection |
| `useToast.ts` | 2 | Toast notifications |
| `useSimpleMode.ts` | 1 | Simple mode toggle |
| `useOnboarding.ts` | 1 | Onboarding state |
| `useMobileScanner.ts` | 1 | Mobile camera scanner |
| `useConversationMemory.ts` | 9 | AI conversation context |
| `useDeviceType.ts` | 0.5 | Device type enum |
| `usePrivacy.ts` | 0.4 | Privacy mode |

**IPC Pattern**: Composables (`useDatabase.ts`) wrap Tauri `invoke()` calls — components KHÔNG gọi IPC trực tiếp. ✅ Tốt.

### 4.3. Types

| File | KB | Nội dung |
|------|------|---------|
| `types/index.ts` | 19 | Shared TypeScript interfaces |
| `types/intent-types.ts` | 3 | AI intent type definitions |

**Rust ↔ TypeScript sync**: ❌ **Manual** — không có codegen tự động. Rust structs trong `models.rs` và TypeScript interfaces trong `types/index.ts` phải được sync thủ công.

---

## PHẦN 5: TỔNG KẾT & KHUYẾN NGHỊ

### 🟢 Điểm Mạnh

1. **Architecture**: db/mod.rs refactored tốt (12 sub-modules), lazy routing 100%, vendor code-split
2. **Security**: 4-layer model (Session, Staff PIN, Password, HWID Binding), RBAC cả FE + BE
3. **Sync**: Trigger-based change tracking (23 tables), comprehensive data coverage (18 types)
4. **DX**: `formatCurrency()`, `useToast()`, `useDatabase()` composable pattern
5. **Dark mode**: Implemented across 94+ files
6. **Error handling**: Consistent `Result<T, String>` pattern

### 🔴 Cần Cải Thiện Ngay

| # | Vấn đề | Mức độ | File |
|---|--------|--------|------|
| 1 | `PragmaticAIEngine.ts` **3,971 dòng** | 🔴 Critical | `src/services/` |
| 2 | `sync_commands.rs` **2,215 dòng** | 🔴 Critical | `src-tauri/src/` |
| 3 | `useChatProcessor.ts` **48KB** | 🔴 Critical | `src/composables/` |
| 4 | `.unwrap()` nguy hiểm (non-Mutex) | 🔴 High | Multiple files |
| 5 | Main bundle `index.js` **4MB** | 🔴 High | Build output |
| 6 | FTS5 chưa dùng (search = LIKE) | 🟡 Medium | Backend DB |
| 7 | SQLCipher chưa tích hợp | 🟡 Medium | Backend DB |
| 8 | Skeleton screens không có | 🟡 Medium | Frontend UX |
| 9 | `shallowRef` không dùng | 🟡 Low | Frontend perf |
| 10 | Rust ↔ TS type sync manual | 🟡 Low | Cross-stack |
| 11 | `<KeepAlive>` không dùng | 🟡 Low | Frontend perf |

### 📊 Scoreboard Tổng Hợp

| Hạng mục | Điểm | Ghi chú |
|----------|------|---------|
| Backend Architecture | **7/10** | db modularized tốt, nhưng sync_commands monolith |
| Frontend Architecture | **8/10** | Composable pattern tốt, lazy routing, dark mode |
| Security | **8/10** | Multi-layer, RBAC, nhưng thiếu SQLCipher |
| Sync System | **7/10** | V2 đang phát triển, V1+V2 coexist phức tạp |
| Code Organization | **7/10** | 3 files monolith (AI engine, chat processor, sync) |
| Build Health | **9/10** | 0 errors, 22s build, vendor split tốt |
| UX/Accessibility | **7/10** | Dark mode, Simple Mode, nhưng thiếu skeleton |
| **TỔNG** | **7.6/10** | - |

---

*Báo cáo này chỉ ĐỌC và PHÂN TÍCH, không sửa code.*
