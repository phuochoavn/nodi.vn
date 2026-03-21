# 📚 NC Giao Diện #02: Kiến Trúc Frontend — Tái Cấu Trúc Nodi POS (Vue 3 + Tauri v2)

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-18
> **Prompt**: Cải tiến 4 trụ cột (Frontend Architect perspective)
> **Liên quan**: Vue 3 Composition API, Feature-Sliced Design, Pinia, Virtual Scrolling

## Tóm tắt

Blueprint tái cấu trúc 35 components Nodi POS thành hệ thống module hóa. Kết hợp Feature-Sliced Design (FSD) + Atomic Design, chia sẻ logic PC↔Mobile qua Composables/Headless Components, virtual scrolling 10K+ SKU, Pinia offline-first với IndexedDB.

---

## 1. Case Study: Shopify, Square, Lightspeed

| POS | Bài học cốt lõi | Áp dụng Nodi |
|-----|-----------------|-------------|
| **Shopify POS** | Mixed Architecture: native cho checkout, WebView cho admin. MVVM "Dumb Views" + ViewModel | Composables = ViewModel, Component = Dumb View |
| **Square Register** | "Single System" — một codebase cho mọi chức năng | Tauri v2 = đúng hướng. Chia sẻ logic, tách UI |
| **Lightspeed** | Data Layer toàn cục cho 10K+ SKU, không để logic trong component | Pinia Stores modular, logic tách khỏi .vue |

---

## 2. Kiến trúc Lai: FSD + Atomic Design

### Feature-Sliced Design (FSD) — Cấp vĩ mô

```
src/
├── app/          ← Cấu hình, providers, router
├── pages/        ← Tổ hợp widgets → trang
├── widgets/      ← Khối UI phức hợp (CartSidebar, CartBottomSheet)
├── features/     ← Tính năng cốt lõi (cart, customer, auth)
│   └── cart/
│       ├── ui/       ← Components chỉ cho cart
│       ├── model/    ← Pinia store + composables
│       └── api/      ← Gọi API/IPC
├── entities/     ← Đối tượng nghiệp vụ (product, user, session)
└── shared/       ← UI Toolkit + utils (KHÔNG có logic nghiệp vụ)
    └── ui/
        ├── atoms/      ← BaseButton, BaseInput, Icon, Typography
        ├── molecules/  ← SearchInput, QuantityAdjuster, LabeledToggle
        └── organisms/  ← AppHeader, NotificationTray
```

### Quy tắc vàng FSD
- ✅ Phụ thuộc **một chiều**: lớp trên chỉ import lớp dưới
- ✅ Mỗi feature = **container cô lập**, giao tiếp qua `index.ts` (Public API)
- ❌ Feature A **KHÔNG được** import trực tiếp từ Feature B
- ❌ Shared **KHÔNG được** chứa logic nghiệp vụ

---

## 3. Chia sẻ Logic PC ↔ Mobile

### 3 Pattern chính

| Pattern | Dùng khi | Ví dụ |
|---------|---------|-------|
| **Composables** | Tái sử dụng logic thuần (data, computed, API) | `useCustomerDetail()` → cả PC và Mobile import |
| **Headless Components** | Logic gắn với UI lifecycle (focus, keyboard) | `CartLogic` → PC: DataGrid, Mobile: SwipeableList |
| **Render Adapter** | Layout hoàn toàn khác nhau | `<component :is="PCLayout/MobileLayout">` |

### Composables — Giải quyết MobileCustomerDetail 37KB

```
❌ Hiện tại: MobileCustomerDetail.vue = 37KB (logic + UI + API + CSS)

✅ Sau refactor:
   useCustomerDetail.ts (logic thuần) → 15KB
   CustomerDetailMobile.vue (UI only) → 8KB
   CustomerDetailDesktop.vue (UI only) → 10KB
   → Tiết kiệm: 37KB → 33KB tổng, nhưng 0 duplicate
```

### Build-time Platform Split (Tauri v2)

```
Vite build --target desktop → Chỉ bundle DesktopLayout.vue
Vite build --target mobile  → Chỉ bundle MobileLayout.vue
→ Mobile bundle KHÔNG chứa code PC (và ngược lại)
```

---

## 4. Component Taxonomy — Bảng phân loại

| Cấp | Component | Thư mục | Logic | Nền tảng |
|-----|----------|---------|:-----:|:--------:|
| Atoms | BaseButton, Typography, Badge | `/shared/ui/atoms/` | ❌ | Shared |
| Atoms | TouchArea, KeyboardShortcut | `/shared/ui/atoms/` | Event listeners | Forked |
| Molecules | QuantityAdjuster | `/shared/ui/molecules/` | Local state nhỏ | Shared |
| Features | CustomerSearchLogic | `/features/customer/` | Headless/Composable | Shared logic |
| Widgets | CartSidebar | `/widgets/cart/ui/` | Store binding | **PC only** |
| Widgets | CartBottomSheet | `/widgets/cart/ui/` | Store binding | **Mobile only** |
| Pages | POSDashboard | `/pages/pos/` | Render Adapter | Dynamic import |

---

## 5. Virtual Scrolling cho 10K+ SKU

### useVirtualList (VueUse) vs vue-virtual-scroller

| Tiêu chí | vue-virtual-scroller | **useVirtualList (VueUse)** ✅ |
|---------|:---:|:---:|
| Kiến trúc | Component đóng gói | Composable (Headless) |
| Variable height | ✅ Mạnh | 🟡 Khó hơn |
| HTML control | ❌ Inject wrappers | ✅ 100% kiểm soát |
| Tải nặng | 🟡 Giật khi vuốt nhanh | ✅ Nhẹ, ổn định |
| Phù hợp Nodi | 🟡 | ✅ Nhất quán với Composition API |

### Tại sao chọn useVirtualList
- POS product grid = **fixed height** → không cần variable height
- Nhất quán với kiến trúc Composable + Headless đã chọn
- Không dependency nặng

---

## 6. shallowRef — Vũ khí bí mật

```typescript
// ❌ Sai: Vue tạo Proxy cho 10,000 x ~20 thuộc tính = 200,000 Proxy
const products = ref<Product[]>(allProducts) // Deep reactivity

// ✅ Đúng: Vue chỉ track mảng gốc, không đào sâu
const products = shallowRef<Product[]>(allProducts) // Shallow reactivity
// → CPU giảm 80%, load gần tức thì
```

**Dùng khi**: Danh sách lớn chỉ đọc (product catalog, invoice history, customer list)

---

## 7. Pinia Store — Modular + Offline-First

### Store phân miền

| Store | Trách nhiệm | Persist |
|-------|------------|:-------:|
| `useAuthStore` | Token, session, RBAC | ✅ |
| `useProductStore` | Catalog 10K+ SKU, cache | ✅ IndexedDB |
| `useCartStore` | Giỏ hàng, thuế, chiết khấu, đa phiên | ✅ |
| `useCustomerStore` | CRM, công nợ | ✅ |
| `useSyncStore` | Queue, trạng thái đồng bộ | ✅ |

### LocalStorage vs IndexedDB

| | LocalStorage | **IndexedDB** ✅ |
|--|:---:|:---:|
| Dung lượng | 5MB max | GB+ |
| Đồng bộ/Bất đồng bộ | ❌ Đồng bộ (block UI) | ✅ Bất đồng bộ |
| Kiểu dữ liệu | String only (JSON parse) | Object + Binary |
| Phù hợp POS | ❌ | ✅ |

### Giải pháp: `localforage` + `pinia-plugin-persistedstate`

```
pinia-plugin-persistedstate
  → storage: localforage (API đơn giản như localStorage)
    → Backend: IndexedDB (bất đồng bộ, GB+)
      → Kết quả: 10K+ SKU load tức thì khi offline
```

---

## 8. Micro-interactions & Perceived Performance

### Quy tắc

| Kỹ thuật | Dùng cho | Hiệu quả |
|---------|---------|----------|
| **Button ripple/color shift** | Mọi nút bấm | Xác nhận tap được ghi nhận |
| **Optimistic UI** | Thêm SP vào giỏ | Hiển thị ngay (mờ) → confirm sau |
| **Skeleton Screens** | Tải danh sách, báo cáo | Giảm perceived wait 40%+ |
| **Haptic feedback** | Mobile tap thanh toán | Xác nhận vật lý |

### CSS Animation — Luật sắt

```css
/* ❌ KHÔNG BAO GIỜ animate: */
width, height, margin, padding, left, top
/* → Gây Layout Thrashing, giật cục */

/* ✅ CHỈ animate: */
transform (translateX, scale, rotate)
opacity
/* → GPU-accelerated, 60fps mọi thiết bị */
```

---

## 9. Lộ trình Refactor 4 Giai đoạn

### GĐ1: Extraction — Chặn phình to
- [ ] Cấm thêm logic mới vào file .vue > 20KB
- [ ] Trích xuất Composables từ MobileCustomerDetail (37KB)
- [ ] Thay `ref()` → `shallowRef()` cho danh sách lớn

### GĐ2: Foundation — Kiến trúc FSD
- [ ] Tạo cấu trúc thư mục FSD: app/pages/widgets/features/entities/shared
- [ ] Trích xuất Atoms/Molecules vào `/shared/ui/`
- [ ] Refactor Pinia Stores → modular + localforage

### GĐ3: Cross-Platform — Tách PC/Mobile
- [ ] Viết Headless Components cho cart, checkout
- [ ] Vite build config: desktop vs mobile target
- [ ] Lazy import PCLayout vs MobileLayout

### GĐ4: Polish — Tối ưu vi mô
- [ ] Tích hợp `useVirtualList` cho mọi danh sách > 50 items
- [ ] Skeleton Screens cho mọi trạng thái loading
- [ ] Rà soát CSS: bỏ animate width/margin → dùng transform/opacity
- [ ] Haptic feedback trên Mobile (nếu Tauri hỗ trợ)

---

## Hiện trạng Nodi POS vs Blueprint

| Tiêu chí | Hiện tại | Mục tiêu |
|---------|:-------:|:-------:|
| Component lớn nhất | 37KB | < 15KB |
| Duplicate PC↔Mobile | Nhiều | 0 (Composables) |
| Virtual Scrolling | ❌ | ✅ useVirtualList |
| shallowRef cho list lớn | ❌ | ✅ |
| Pinia persist | LocalStorage | IndexedDB (localforage) |
| CSS animation | Hỗn hợp | GPU-only |
| FSD architecture | ❌ | ✅ |

---

## Nguồn tham khảo

- Shopify POS Engineering Blog (MVVM migration)
- Square Register Hardware-Software Integration
- Lightspeed POS Architecture
- Feature-Sliced Design (feature-sliced.design)
- Brad Frost: Atomic Design methodology
- VueUse: useVirtualList documentation
- Vue 3 docs: Composables, shallowRef
- localforage + pinia-plugin-persistedstate
