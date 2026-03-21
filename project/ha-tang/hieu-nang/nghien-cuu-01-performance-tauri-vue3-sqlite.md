# 📚 NC Hiệu Năng #01: Tối ưu Tauri v2 + Vue 3 + SQLite cho POS Cường Độ Cao

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-19
> **Prompt**: Performance Engineer perspective — Tauri v2/Vue 3/SQLite
> **Liên quan**: IPC MessagePack, SQLite Index, Memory 12h, Bundle Splitting

## Tóm tắt

Blueprint hiệu năng 4 giai đoạn: (1) IPC JSON→MessagePack + lib.rs modular, (2) SQLite Partial/Covering Index + FTS5 tiếng Việt, (3) Vue 3 memory 12h (KeepAlive LRU, WeakRef, onUnmounted cleanup), (4) Vite manualChunks + lazy routes. Case study Shopify POS (JSI bypass), Square Register (HW tuning), Lightspeed (WAL concurrent).

---

## 1. Metrics → Tool → Target

| Metric | Tool | ✅ Target | ❌ Critical |
|--------|------|:---------:|:----------:|
| Scan-to-Screen | Tauri Tracing / Performance API | **< 100ms** | > 300ms |
| Cold Start | `hyperfine` / tauri-plugin-profiling | **< 800ms** | > 2000ms |
| Memory (12h) | Chrome Memory / mprof (USS) | **< 150MB** | > 300MB |
| IPC 10K rows | `Date.now()` / tokio-console | **< 50ms** | > 200ms |
| JS Bundle (gzip) | rollup-plugin-visualizer | **< 500KB** | > 1MB |
| Component Unmount | Vue DevTools Timeline | **< 15ms** | > 50ms |

---

## 2. Backend Rust — Modular + Lazy Init

### Cấu trúc đề xuất

```
src-tauri/src/
├── commands/
│   ├── mod.rs          ← gom handlers
│   ├── invoices.rs
│   ├── products.rs
│   ├── customers.rs
│   └── sync.rs
├── db/
│   └── sqlite.rs
└── lib.rs              ← chỉ setup + import modules
```

### Lazy Initialization
- `OnceCell` / `lazy_static` cho DB connections, hardware buffers
- Commands khai báo compile-time (Tauri bảo mật), nhưng logic chạy lazy
- **Giảm hàng trăm ms cold start**

### Cargo.toml Release Profile
```toml
[profile.release]
codegen-units = 1    # LLVM cross-analysis tốt hơn
lto = true           # Link-time optimization
strip = true         # Bỏ debug symbols
```

---

## 3. IPC — JSON → MessagePack

| So sánh | JSON | **MessagePack** ✅ | Bincode |
|---------|:----:|:------------------:|:-------:|
| Tốc độ | 1x | **~3x nhanh hơn** | ~5x |
| Payload size | 100% | **63-82.5%** | ~50% |
| Self-describing | ✅ | ✅ | ❌ |
| JS decode | `JSON.parse` (blocking) | `@msgpack/msgpack` | Cần Wasm |
| u64 safe | ❌ (IEEE 754 sai) | ✅ | ✅ |
| **Kết luận** | Loại bỏ cho payload lớn | **Sweet Spot** | Quá phức tạp |

### Rust side
```rust
Response::new(rmp_serde::to_vec(&products).unwrap())
```

### JS side
```typescript
import { decode } from '@msgpack/msgpack';
const products = decode(new Uint8Array(rawBytes));
```

---

## 4. SQLite Index Strategy

| Bảng | Cột | Loại Index | SQL | Tác dụng |
|------|-----|:----------:|-----|---------|
| products | barcode | **Unique** | `CREATE UNIQUE INDEX idx_products_barcode ON products(barcode)` | Tra cứu < 10ms |
| invoices | status, created_at | **Partial** | `...ON invoices(created_at) WHERE status='PENDING'` | B-Tree nhỏ, không tốn chi phí UPDATE |
| invoices | customer_id, total | **Covering** | `...ON invoices(customer_id, total)` | Index-Only Scan, giảm 50% I/O |
| cart_items | invoice_id, product_id | **Composite** | `...ON cart_items(invoice_id, product_id)` | Phục hồi giỏ hàng nhanh |

### FTS5 tiếng Việt (không dấu)

```sql
CREATE VIRTUAL TABLE products_fts USING fts5(
  name, sku, description,
  tokenize = "unicode61 remove_diacritics 2",
  content = "products"        -- External content, không sao chép dữ liệu
);

-- Tìm: "phan bon" → match "phân bón" ✅
SELECT * FROM products_fts WHERE products_fts MATCH 'phan bon'
ORDER BY bm25(products_fts);  -- Ranking tự động
```

---

## 5. Memory 12h — Vue 3

### 3 Loại rò rỉ chính

| Loại | Nguyên nhân | Giải pháp |
|------|-----------|---------|
| **Watchers/Closures** | `watch()` không stop khi unmount | Lưu `stop = watch(...)`, gọi `stop()` trong `onUnmounted` |
| **Event/Timer** | `addEventListener` trên window, `setInterval` mồ côi | `removeEventListener` + `clearInterval` trong `onUnmounted` |
| **Reactive bloat** | 10K items trong `ref()` → Proxy mọi property | **`shallowRef()`** — chỉ track reference level |

### KeepAlive LRU

```vue
<KeepAlive :max="5">
  <component :is="activeTab" />
</KeepAlive>
```
→ Tab thứ 6 tự động destroy tab cũ nhất (LRU)

### WeakRef cho AI Engine

```typescript
let engineRef = new WeakRef(pragmaticAIEngine);
// GC tự giải phóng khi RAM căng
if (!engineRef.deref()) {
  // Tạo engine mới khi cần
}
```

### Rust Memory
- `Drop` trait cho hardware connections (máy in, scanner)
- `tokio::select!` + cancellation token cho background sync
- Tránh `Rc`/`Arc` reference cycles

---

## 6. Bundle Splitting — Vite

```typescript
// vite.config.ts
manualChunks(id) {
  if (id.includes('vue') || id.includes('pinia'))
    return 'vendor-vue';          // ~150KB
  if (id.includes('PragmaticAIEngine'))
    return 'vendor-ai-engine';    // ~222KB, lazy load
  if (id.includes('xlsx'))
    return 'vendor-excel';        // Chỉ load khi xuất báo cáo
  return 'vendor-utils';
}
```

### Lazy Routes
```typescript
// router.ts
{ path: '/reports', component: () => import('./views/Reports.vue') }
{ path: '/settings', component: () => import('./views/Settings.vue') }
```
→ Critical path chỉ load: Login → POS → Cart

---

## 7. Case Studies

| POS | Bài học | Áp dụng Nodi |
|-----|--------|-------------|
| **Shopify POS** | RN Bridge JSON → JSI shared memory. Giảm 37% TCO | IPC JSON → MessagePack binary |
| **Square Register** | HW-SW vertical integration, loại bỏ abstraction | SW-only optimization: chunking, lazy init, WAL |
| **Lightspeed** | WAL concurrent read-write, LiteServer offline cache | WAL + PRAGMA cache_size cho peak load |

---

## 8. Performance Audit Checklist

| # | Hạng mục | Kiểm tra | Pass? |
|:-:|---------|---------|:-----:|
| 1 | IPC Protocol | Payload lớn dùng MessagePack, không JSON | ⬜ |
| 2 | Rust Modular | lib.rs < 2KB, commands/ folder | ⬜ |
| 3 | SQLite Index | `EXPLAIN QUERY PLAN` → USING INDEX (không SCAN TABLE) | ⬜ |
| 4 | WAL Mode | `PRAGMA journal_mode` = wal | ⬜ |
| 5 | Memory 30min | Chrome Timeline: no upward trend, no detached DOM | ⬜ |
| 6 | Cleanup | Mọi listener/timer/watcher có onUnmounted | ⬜ |
| 7 | Chunks | Không file > 500KB gzip. AI engine = vendor riêng | ⬜ |

---

## 9. Blueprint 4 Giai Đoạn

| GĐ | Trọng tâm | Việc chính |
|:--:|----------|----------|
| **1** | Backend (Rust + IPC) | MessagePack, modular lib.rs, lazy init, Cargo release profile |
| **2** | Database (SQLite) | WAL + Partial/Covering Index + FTS5 tiếng Việt |
| **3** | Runtime (Vue 3 Memory) | onUnmounted cleanup, KeepAlive LRU, shallowRef, WeakRef |
| **4** | Frontend (Bundle) | manualChunks, lazy routes, tree-shaking AI engine |

---

## Nguồn tham khảo

- Tauri v2 IPC architecture & Custom Protocol
- MessagePack specification & @msgpack/msgpack
- SQLite FTS5, Partial Index, Covering Index documentation
- Vue 3 KeepAlive, shallowRef, WeakRef APIs
- Shopify POS React Native → JSI migration
- Square Register hardware-software integration
- Lightspeed POS WAL & LiteServer architecture
- Vite rollupOptions.manualChunks documentation
- cargo-bloat, hyperfine, rollup-plugin-visualizer tools
