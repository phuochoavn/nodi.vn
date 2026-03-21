# 🎨 Giao Diện — UI Design System & Component Architecture

> Design tokens, component specs, responsive layout, PC vs Mobile

---

## 🏆 Tinh Hoa Cần Làm — Tổng hợp từ Nghiên Cứu

### 🔴 Ưu tiên Cao nhất (Kiến trúc nền tảng)

| # | Việc cần làm | Nguồn | Effort |
|:-:|-------------|:-----:|:------:|
| 1 | **Tạo `design-tokens.css`** — CSS Custom Properties: color, spacing, typography, shadow | NC #01 | TB |
| 2 | **Color Blind Safe** — Đổi `#FF0000` → `#D41159` (Vermillion), `#00FF00` → `#40B0A6` (Teal) | NC #01 | Nhỏ |
| 3 | **Be Vietnam Pro** làm font chính + Roboto cho cột số | NC #01 | Nhỏ |
| 4 | **`line-height: 1.5`** cho body text — chống cắt dấu tiếng Việt | NC #01 | Nhỏ |
| 5 | **Trích xuất Composables** — MobileCustomerDetail 37KB → useCustomerDetail.ts + UI shell | NC #02 | TB |
| 6 | **`shallowRef()`** cho mọi danh sách lớn | NC #02 | Nhỏ |
| 7 | **Pinia persist → IndexedDB** — `localforage` + `pinia-plugin-persistedstate` | NC #02 | TB |

### 🟡 Ưu tiên Trung bình (Tái cấu trúc tiệm tiến)

| # | Việc cần làm | Nguồn | Effort |
|:-:|-------------|:-----:|:------:|
| 5 | **Feature-Sliced Design** — Tổ chức thư mục: app/pages/widgets/features/entities/shared | NC #02 | Lớn |
| 6 | **Atomic UI Library** — Trích Atoms (BaseButton, BaseInput) + Molecules vào `/shared/ui/` | NC #02 | TB |
| 7 | **`useVirtualList`** (VueUse) — Mọi danh sách > 50 items | NC #02 | TB |
| 8 | **Headless Components** — CartLogic → PC: DataGrid, Mobile: SwipeableList | NC #02 | TB |
| 9 | **Render Adapter** — Build-time split: Vite desktop vs mobile target | NC #02 | TB |

### 🟢 Ưu tiên Polish (Micro-interactions)

| # | Việc cần làm | Nguồn | Effort |
|:-:|-------------|:-----:|:------:|
| 10 | **Skeleton Screens** — Thay spinner bằng khung xám + shimmer cho mọi loading state | NC #02 | Nhỏ |
| 11 | **CSS GPU-only animation** — Bỏ animate width/margin → chỉ transform + opacity | NC #02 | Nhỏ |
| 12 | **Optimistic UI** — Thêm SP mờ → confirm sau, không block UI | NC #02 | Nhỏ |
| 13 | **Haptic feedback** — Rung nhẹ khi tap thanh toán (Mobile, nếu Tauri hỗ trợ) | NC #02 | Nhỏ |

---

### 💎 5 Nguyên tắc Kiến trúc Frontend

1. **"Composable = ViewModel"** — Logic 100% trong TypeScript, Component chỉ render _(Shopify MVVM)_
2. **"Feature cô lập, Shared thuần khiết"** — FSD phụ thuộc một chiều, shared KHÔNG chứa business logic
3. **"Build-time, không Runtime"** — Tách PC/Mobile lúc build, không dùng `v-if="isMobile"` trong code
4. **"Shallow trước, Deep khi cần"** — `shallowRef()` mặc định cho list, `ref()` chỉ khi cần reactivity sâu
5. **"IndexedDB > LocalStorage"** — Bất đồng bộ, GB+, không block UI thread
6. **"#D41159 + #40B0A6, không bao giờ #FF0000 + #00FF00"** — Color Blind Safe bắt buộc
7. **"Be Vietnam Pro + Roboto dual font"** — UI tiếng Việt + Data table

---

## Nghiên Cứu Đã Hoàn Thành (Google DeepSearch — 18/03/2026)

| # | File | Nội dung | Điểm |
|:-:|------|----------|:----:|
| 01 | `nghien-cuu-01-design-system-pos-nong-nghiep.md` | Design System: Tokens 3 lớp, Color Blind Safe, Typography TV, Dark Mode | **93** |
| 02 | `nghien-cuu-02-component-architecture-vue3-tauri.md` | Component Architecture: FSD, Composables, Virtual Scrolling, Pinia | **94** |

---

## Tài Liệu Cần Viết (tổng hợp từ NC)

| File | Nội dung | Nguồn NC | Trạng thái |
|------|----------|:--------:|:----------:|
| `design-system.md` | Color palette, typography, spacing, shadows | NC #01 | ⬜ |
| `component-taxonomy.md` | Bảng phân loại: Atoms/Molecules/Features/Widgets/Pages | NC #02 | ⬜ |
| `fsd-migration-guide.md` | Hướng dẫn migrate từ cấu trúc hiện tại → FSD | NC #02 | ⬜ |
| `responsive-specs.md` | Breakpoints, PC (1366-1920) vs Mobile (360-768) | NC #01 | ⬜ |
| `pc-vs-mobile.md` | Render Adapter strategy, build-time split | NC #02 | ⬜ |

## Hiện Trạng

- CSS thuần (không Tailwind) + custom design tokens
- 35 desktop components + 35 mobile components (nhiều duplicate)
- 2 layouts: MainLayout (desktop), MobileLayout (mobile)
- Base components: BaseButton, BaseInput, BaseModal
- **Vấn đề**: MobileCustomerDetail 37KB, logic nhồi trong .vue, chưa có FSD
