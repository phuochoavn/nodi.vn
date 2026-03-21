# ⚡ Hiệu Năng — Performance Optimization

> Benchmarks, profiling, IPC optimization, memory management, bundle size

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ Nghiên Cứu

### 🔴 Ưu tiên Cao nhất (Scan-to-Screen < 100ms)

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 1 | **IPC → MessagePack** | Loại bỏ JSON cho payload lớn, dùng `rmp_serde` + `@msgpack/msgpack` | TB |
| 2 | **lib.rs modular** | 18KB → commands/ folder, mỗi domain 1 file | TB |
| 3 | **Lazy Init** | `OnceCell`/`lazy_static` cho DB/hardware, giảm cold start | Nhỏ |
| 4 | **Partial Index** | `WHERE status='PENDING'` — B-Tree nhỏ, không tốn UPDATE | Nhỏ |
| 5 | **Covering Index** | `(customer_id, total)` — Index-Only Scan, giảm 50% I/O | Nhỏ |

### 🟠 Ưu tiên Cao (Memory < 150MB sau 12h)

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 6 | **onUnmounted cleanup** | Stop watchers, remove listeners, clear timers | Nhỏ |
| 7 | **KeepAlive LRU** | `<KeepAlive :max="5">` cho tab hóa đơn | Nhỏ |
| 8 | **shallowRef** cho lists | 10K products → `shallowRef()` thay `ref()` | Nhỏ |
| 9 | **FTS5 tiếng Việt** | `unicode61 remove_diacritics 2`, external content | TB |
| 10 | **WeakRef cho AI Engine** | 222KB PragmaticAIEngine → GC tự giải phóng khi cần | Nhỏ |

### 🟡 Ưu tiên Trung bình (Bundle < 500KB gzip)

| # | Việc cần làm | Chi tiết | Effort |
|:-:|-------------|---------|:------:|
| 11 | **manualChunks** | vendor-vue, vendor-ai-engine, vendor-excel tách riêng | Nhỏ |
| 12 | **Lazy routes** | Reports, Settings, Staff → `() => import(...)` | Nhỏ |
| 13 | **Cargo release profile** | `codegen-units=1`, `lto=true`, `strip=true` | Nhỏ |

---

### 💎 5 Nguyên tắc Hiệu năng

1. **"< 100ms hoặc thất bại"** — Scan-to-Screen phải dưới 100ms, mọi thao tác POS
2. **"MessagePack, không JSON"** — IPC payload lớn = binary, JSON chỉ cho config nhỏ
3. **"shallowRef mặc định"** — Danh sách lớn KHÔNG BAO GIỜ dùng `ref()` sâu
4. **"Mọi thứ đều dọn dẹp"** — Watcher, listener, timer → `onUnmounted` bắt buộc
5. **"Tách để trị"** — lib.rs modular, Vite chunks riêng, lazy routes

---

## Nghiên Cứu (Google DeepSearch — 19/03/2026)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-performance-tauri-vue3-sqlite.md` | Benchmark, IPC MessagePack, SQLite Index, Memory 12h, Bundle Splitting | **95** |

---

## Metrics Dashboard

| Metric | Hiện tại | Mục tiêu |
|--------|:-------:|:-------:|
| Scan-to-Screen | ~200ms? | **< 100ms** |
| Cold Start | ~1500ms? | **< 800ms** |
| Memory 12h | Chưa đo | **< 150MB** |
| IPC 10K rows | JSON (~150ms?) | **< 50ms** (MessagePack) |
| JS Bundle | > 500KB | **< 500KB** gzip |
| lib.rs | 18KB monolithic | **< 2KB** + commands/ |
