# 📚 NC Giao Diện #01: Design System cho Nodi POS — Color, Typography, Responsive

> **Nguồn**: Google DeepSearch
> **Ngày**: 2026-03-18
> **Prompt**: Cải tiến 4 trụ cột (Design System Architect perspective)
> **Liên quan**: CSS Custom Properties, WCAG 2.2, Be Vietnam Pro, Dark Mode

## Tóm tắt

Kiến trúc Design System toàn diện cho POS nông nghiệp: Design Tokens 3 lớp (Primitive → Semantic → Component), Color Blind Safe palette, Typography tiếng Việt (Be Vietnam Pro + Roboto), Responsive 360px-1920px, Dark Mode, case study Shopify Polaris / Square / Toast.

---

## 1. Case Study: Design Systems POS

| Hệ thống | Bài học | Áp dụng Nodi |
|----------|--------|-------------|
| **Shopify Polaris** | CSS Variables thay SCSS, Container Queries, middle-out scale | CSS thuần + `--nodi-` namespace |
| **Square Design** | Design Tokens 3 lớp: Primitive → Semantic → Component | Tách giá trị thô ↔ ngữ nghĩa ↔ component |
| **Toast POS** | Feedback components nghiêm ngặt: Toast chỉ cho xác nhận, Modal cho lỗi/rủi ro cao | Bảo vệ người dùng lớn tuổi khỏi quá tải nhận thức |

---

## 2. Design Tokens — 3 Lớp (CSS Custom Properties)

```
Primitive (giá trị thô)     →  Semantic (ngữ nghĩa)      →  Component (áp dụng)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Blue-500: #1A73E8            →  color-action-primary       →  Button-Bg
4px                          →  space-4                    →  Input-Padding
Neutral-200: #E0E0E0         →  color-border-default       →  Card-Border
```

### Bảng Tokens Chính

| Nhóm | Token CSS | Giá trị | Mục đích |
|------|----------|---------|---------|
| **Nền** | `--nodi-color-bg-light` | `#F8F9FA` | Light Mode, dịu mắt |
| **Nền** | `--nodi-color-bg-dark` | `#121212` | Dark Mode, tiết kiệm OLED |
| **Text** | `--nodi-color-text-primary` | `#111827` | Contrast > 4.5:1 WCAG |
| **Brand** | `--nodi-color-brand` | `#1A73E8` | Nút Thanh toán, Xác nhận |
| **Nợ** | `--nodi-color-critical` | `#D41159` | Vermillion - Color Blind Safe |
| **Đã TT** | `--nodi-color-success` | `#40B0A6` | Teal - Color Blind Safe |
| **Space** | `--nodi-space-xs/sm/md/lg/xl` | `4/8/16/24/32px` | Modular Scale 4px-8px |
| **Font UI** | `--nodi-font-family-ui` | `Be Vietnam Pro` | Dấu tiếng Việt tối ưu |
| **Font Data** | `--nodi-font-family-data` | `Roboto` | Cột số, tabular alignment |
| **Font Size** | `--nodi-font-size-base` | `16px` | 1rem baseline |
| **Line Height** | `--nodi-line-height-base` | `1.5` | Chống cắt dấu tiếng Việt |

---

## 3. Màu sắc — Color Blind Safe

### Cặp trạng thái truyền thống vs Nodi POS

| Trạng thái | ❌ Truyền thống | ✅ Nodi POS | Lý do |
|-----------|:-----------:|:--------:|-------|
| **Nợ** | `#FF0000` (Pure Red) | `#D41159` (Vermillion) | 8% nam giới mù đỏ-xanh |
| **Đã TT** | `#00FF00` (Pure Green) | `#40B0A6` (Teal) | Deuteranopia không phân biệt được |

### Quy tắc bắt buộc

- ✅ Màu sắc **KHÔNG BAO GIỜ** là tín hiệu duy nhất (WCAG)
- ✅ Trạng thái = `[Màu] + [Icon] + [Text]`
- ✅ Contrast Ratio ≥ 4.5:1 cho text thường, ≥ 3:1 cho text lớn
- ❌ Cấm text xám nhạt trên nền trắng (< 4.5:1)

---

## 4. Typography Tiếng Việt

### So sánh Font

| Tiêu chí | Roboto | Inter | **Be Vietnam Pro** ✅ |
|---------|:------:|:-----:|:------------------:|
| X-Height | 52.8% (tốt) | Cao nhất | Cân bằng |
| Dấu TV | Xếp chồng tuyến tính | Tốt | **Adaptive forms** (né va chạm) |
| Chiều rộng số | Hẹp, nén tốt | Rộng, tốn chỗ | Cân bằng |
| Vai trò | **Data Tables** | — | **UI chính** |

### Chiến lược Split Typography

```
Be Vietnam Pro → Headers, Labels, Toast, Badges, Modals (mọi text UI)
Roboto        → Numpad, Price Column, Stock Qty (chỉ cột số)
```

### Line-height chống cắt dấu

| Loại text | Line-height | Lý do |
|----------|:-----------:|-------|
| Body text | **1.5** | Bảo vệ dấu kép (Ắ, Ằ, Ể) |
| Headings | **1.2-1.3** | Nén nhưng vẫn an toàn |
| ❌ Cấm | `overflow: hidden` trên inline text | Cắt mất dấu |
| ✅ Dùng | `display: flex; align-items: center` | Canh giữa button an toàn |

---

## 5. Dark Mode

### Tại sao cần cho VTNN

- Ca làm 12 tiếng → mỏi mắt, khô giác mạc
- OLED Android → điểm ảnh đen = tắt LED = tiết kiệm pin
- `prefers-color-scheme` → tự chuyển sáng/tối theo OS

### Cách triển khai (CSS thuần)

```css
:root {
  --nodi-color-bg-surface: var(--nodi-color-bg-light);
  --nodi-color-text: var(--nodi-color-text-primary);
}

[data-theme="dark"] {
  --nodi-color-bg-surface: var(--nodi-color-bg-dark);
  --nodi-color-text: #E5E7EB;
  --nodi-shadow-card: 0 4px 6px rgba(0, 0, 0, 0.4);
}
```

### Lưu ý

- ⚠️ Dark mode dưới ánh nắng → quầng sáng (halation) cho người loạn thị
- → Cần Dynamic Theme Toggle (sáng ban ngày, tối ca đêm)

---

## 6. Responsive Layout — 360px → 1920px

| Breakpoint | Thiết bị | Layout | Đặc điểm |
|:----------:|---------|--------|---------|
| **360-480px** | Android kho bãi | Flex column 1 chiều | Scanner + nút lớn, Progressive Disclosure |
| **768-1024px** | Tablet lưu động | Flex row + hamburger | Biểu đồ đơn giản, padding tăng cho touch |
| **1366-1920px** | PC quầy thu ngân | **CSS Grid 3 cột** | `300px | 1fr | 350px` — KH / SP / Numpad |

### Grid PC Layout

```css
@media (min-width: 1366px) {
  .nodi-pos-layout {
    display: grid;
    grid-template-columns: 300px 1fr 350px;
    height: 100vh;
    overflow: hidden; /* Cấm cuộn ngang */
  }
}
```

---

## 7. Touch Target & Fitts's Law

| Chuẩn | Kích thước | Áp dụng |
|-------|:----------:|---------|
| WCAG 2.2 AA | 24x24px min | Quá nhỏ cho nông dân |
| **Nodi POS** | **48px min** | Tay chai sần, thị lực giảm |
| Gap giữa nút rủi ro | **≥16px** | "Hủy đơn" ↔ "Thanh toán" |

---

## 8. Elevation (Đổ bóng)

| Cấp | Token | Dùng cho |
|:---:|-------|---------|
| 0 | `--nodi-shadow-none` | Panel nền cố định |
| 1 | `--nodi-shadow-sm` | Product Cards, hóa đơn |
| 2 | `--nodi-shadow-md` | Dropdowns, Tooltips |
| 3 | `--nodi-shadow-lg` | Modals, Dialog thanh toán |

---

## 9. Component Taxonomy (4 nhóm)

| Nhóm | Components | Đặc điểm |
|------|-----------|---------|
| **Primitives** | Button (Primary/Subdued/Critical), Badge, Typography | Không thể phân rã thêm |
| **Input/Form** | Numpad Controller, Barcode Scanner Field | Thiết kế cho 1000+ lần nhập/ngày |
| **Data Dense** | Editable Data Tables, Customer Insight Cards | Inline editing, pivot views |
| **Feedback** | Toast (tự biến mất 3-5s), Modal (ép xác nhận) | Toast = xác nhận; Modal = rủi ro |

### Quy tắc Feedback (từ Toast POS)

```
✅ Toast/Snackbar: "Đã thêm sản phẩm" → tự mất sau 3-5s, KHÔNG chặn UI
❌ Toast cho lỗi: KHÔNG BAO GIỜ → dùng Modal/Dialog
✅ Modal: "Nợ vượt hạn mức" → backdrop tối, ÉP xác nhận
```

---

## Nguồn tham khảo

- Shopify Polaris Design System (CSS Variables migration)
- Square Design System (3-tier token architecture)
- Toast Design System (Kitchen Display feedback)
- WCAG 2.1 AA, WCAG 2.2 Target Size
- Be Vietnam Pro (Google Fonts)
- Fitts's Law in HCI
- WHO: Color Vision Deficiency statistics
