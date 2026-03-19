# 🔧 UPGRADE Brief — Market Intelligence Dashboard

> **Giao cho**: Agent VPS (Claude Opus 4)
> **Ngày giao**: 2026-02-19
> **Ưu tiên**: HIGH — tính năng cốt lõi admin
> **Phạm vi**: Backend API (Rust) + Frontend (Nuxt/Vue)

---

## Mục tiêu

Nâng cấp trang `/admin/thi-truong` (hiện chỉ có 3 card trống) thành **Market Intelligence Dashboard** đầy đủ, giúp admin nắm bắt:
1. Thị trường sản phẩm VTNN
2. Chuỗi cung ứng (NCC → Đại lý)
3. Tín dụng nông dân (công nợ KH)
4. Dòng tiền toàn hệ thống

---

## Nguồn dữ liệu (đã có sẵn trong DB từ sync)

Khi mỗi cửa hàng sync data lên, server đã nhận đủ 14 bảng:

| Bảng | Data chính |
|------|-----------|
| `stores` + `store_settings` | Tên, địa chỉ cửa hàng |
| `products` | Tên, category (8 nhóm), manufacturer, giá vốn/bán, tồn kho |
| `product_units` | Đơn vị quy đổi + giá |
| `product_batches` | Lô hàng, HSD, SL còn |
| `product_transactions` | Thẻ kho (IN/OUT/ADJUST) |
| `suppliers` | Tên, type (COMPANY/AGENCY/FREELANCE/OTHER), nợ |
| `supplier_transactions` | Lịch sử nợ NCC (IMPORT/PAYMENT/REFUND) |
| `purchase_orders` + `purchase_items` | Phiếu nhập chi tiết |
| `payment_vouchers` | Phiếu chi trả NCC |
| `customers` | Tên, nợ, credit_limit, địa chỉ |
| `customer_transactions` | Giao dịch nợ KH (DEBIT/CREDIT + **season**) |
| `invoices` + `invoice_items` | Hóa đơn bán + chi tiết SP |
| `invoice_payments` | Thanh toán (tiền mặt/CK/nợ) |
| `cash_transactions` + `store_funds` | Thu/chi + số dư quỹ |
| `returns` + `return_items` | Trả hàng (lý do, SP, hoàn tiền) |

---

## Thiết kế Dashboard — 6 Sections

### Bố cục trang `/admin/thi-truong`

```
┌──────────────────────────────────────────────────────────┐
│  📊 Market Intelligence                    [Lọc: Tháng] │
├──────────────────────────────────────────────────────────┤
│  [Tổng quan] [Sản phẩm] [Cung ứng] [Tín dụng] [Tiền]  │ ← Tabs
├──────────────────────────────────────────────────────────┤
│                                                          │
│  << NỘI DUNG TAB >>                                     │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

### TAB 1: Tổng quan (Overview)

**4 stat cards hàng đầu:**
```
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│ Tổng DT  │ │ Tổng nợ  │ │ Tổng SP  │ │ Cửa hàng │
│ toàn HT  │ │ KH+NCC   │ │ đang bán │ │ hoạt động│
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

**Biểu đồ doanh thu theo tháng** (line chart, 12 tháng gần nhất)
- Query: `SELECT strftime('%Y-%m', created_at), SUM(final_amount) FROM orders GROUP BY 1`

---

### TAB 2: Sản phẩm (Product Intelligence)

**2.1. Top 10 SP bán chạy** (bar chart ngang)
- Query: `SELECT oi.product_name, SUM(oi.quantity) as qty, SUM(oi.subtotal) as revenue FROM order_items oi GROUP BY oi.product_name ORDER BY revenue DESC LIMIT 10`

**2.2. Doanh thu theo nhóm hàng** (pie chart)
- 8 nhóm: Trừ bệnh, Trừ sâu, Trừ cỏ, Trừ ốc, Phân bón, Giống, Dưỡng, Khác
- Query: `SELECT p.category, SUM(oi.subtotal) FROM order_items oi JOIN products p ON oi.product_id = p.id GROUP BY p.category`

**2.3. Top nhà sản xuất** (bar chart)
- Query: `SELECT p.manufacturer, SUM(oi.subtotal) as revenue FROM order_items oi JOIN products p ON oi.product_id = p.id WHERE p.manufacturer IS NOT NULL GROUP BY p.manufacturer ORDER BY revenue DESC LIMIT 10`

**2.4. Biên lợi nhuận trung bình** (stat card)
- Query: `SELECT AVG((oi.unit_price - p.cost_price) / NULLIF(oi.unit_price, 0) * 100) FROM order_items oi JOIN products p ON oi.product_id = p.id WHERE p.cost_price > 0`

---

### TAB 3: Cung ứng (Supply Chain)

**3.1. Tỷ lệ kênh phân phối** (donut chart)
- Bao nhiêu % mua từ COMPANY (trực tiếp CTy SX) vs AGENCY (Đại lý C1) vs FREELANCE (Sale lẻ)
- Query: `SELECT s.type, SUM(po.total_amount) FROM purchase_orders po JOIN suppliers s ON po.supplier_id = s.id GROUP BY s.type`

**3.2. Top NCC theo doanh số nhập** (table)
- Cột: Tên NCC | Loại | Tổng nhập | Số phiếu | Nợ hiện tại
- Query: `SELECT s.name, s.type, SUM(po.total_amount), COUNT(po.id), s.current_debt FROM suppliers s LEFT JOIN purchase_orders po ON s.id = po.supplier_id GROUP BY s.id ORDER BY SUM(po.total_amount) DESC LIMIT 10`

**3.3. Tổng nợ NCC toàn hệ thống** (stat card + trend)
- Query: `SELECT SUM(current_debt) FROM suppliers`

**3.4. Chi trả NCC theo tháng** (line chart)
- Query: `SELECT strftime('%Y-%m', created_at), SUM(amount) FROM payment_vouchers GROUP BY 1`

---

### TAB 4: Tín dụng nông dân (Credit/Debt)

**4.1. Tổng nợ khách hàng** (big number + so sánh tháng trước)
- Query: `SELECT SUM(current_debt) FROM customers`

**4.2. Nợ theo mùa vụ** (grouped bar chart)
- Đông Xuân vs Hè Thu vs Vụ Mùa vs Khác
- Query: `SELECT season, SUM(CASE WHEN transaction_type='DEBIT' THEN amount ELSE 0 END) as ghi_no, SUM(CASE WHEN transaction_type='CREDIT' THEN amount ELSE 0 END) as thu_no FROM customer_transactions WHERE season IS NOT NULL GROUP BY season`

**4.3. Tỷ lệ thu hồi nợ** (gauge hoặc stat card, %)
- Query: `SELECT SUM(CASE WHEN transaction_type='CREDIT' THEN amount ELSE 0 END) * 100.0 / NULLIF(SUM(CASE WHEN transaction_type='DEBIT' THEN amount ELSE 0 END), 0) FROM customer_transactions`

**4.4. Top KH nợ nhiều nhất** (table)
- Cột: Tên | SĐT | Nợ hiện tại | Hạn mức | % sử dụng
- Query: `SELECT name, phone, current_debt, credit_limit FROM customers WHERE current_debt > 0 ORDER BY current_debt DESC LIMIT 10`

**4.5. Phương thức thanh toán** (donut chart)
- Tiền mặt vs Chuyển khoản vs Ghi nợ
- Query: `SELECT payment_method, COUNT(*), SUM(final_amount) FROM orders GROUP BY payment_method`

---

### TAB 5: Dòng tiền (Cash Flow)

**5.1. Tổng thu / Tổng chi / Lãi ròng** (3 stat cards)
- Query: `SELECT flow_type, SUM(amount) FROM cash_transactions GROUP BY flow_type`

**5.2. Dòng tiền theo tháng** (stacked bar chart: Thu vs Chi)
- Query: `SELECT strftime('%Y-%m', created_at), flow_type, SUM(amount) FROM cash_transactions GROUP BY 1, 2`

**5.3. Cơ cấu chi** (pie chart)
- 6 loại: SALES, IMPORT, SUPPLIER_PAYMENT, EXPENSE, REFUND, OTHER
- Query: `SELECT category, SUM(amount) FROM cash_transactions WHERE flow_type='OUT' GROUP BY category`

**5.4. Số dư quỹ các cửa hàng** (table)
- Cột: Cửa hàng | Số dư | Cập nhật lần cuối
- Query: `SELECT store_name, current_balance, updated_at FROM store_funds`

---

## API Endpoints cần tạo

Tất cả đặt dưới `/api/admin/market/*`:

```
GET /api/admin/market/overview
  → { total_revenue, total_debt_customer, total_debt_supplier, total_products, total_stores, revenue_by_month[] }

GET /api/admin/market/products
  → { top_products[], revenue_by_category[], top_manufacturers[], avg_margin }

GET /api/admin/market/supply-chain
  → { channel_distribution[], top_suppliers[], total_supplier_debt, supplier_payments_by_month[] }

GET /api/admin/market/credit
  → { total_customer_debt, debt_by_season[], collection_rate, top_debtors[], payment_methods[] }

GET /api/admin/market/cashflow
  → { total_in, total_out, net_profit, cashflow_by_month[], expense_breakdown[], store_balances[] }
```

Tất cả endpoint cần middleware `admin_auth` (JWT cookie check).

Query param chung: `?months=12` (lọc theo khoảng thời gian).

---

## Lưu ý quan trọng

1. **Data aggregate cross-store**: Mỗi store sync data riêng. API cần JOIN qua `store_id` (mỗi store có store_id riêng trong DB server)
2. **Empty state**: Khi chưa có data → hiện "Chưa có dữ liệu. Các cửa hàng cần sync để hiển thị phân tích." + icon minh họa
3. **Charts**: Dùng `chart.js` hoặc `apexcharts` (cái nào đã có trong project thì dùng lại)
4. **Responsive**: Tabs nên collapse thành dropdown trên mobile
5. **Performance**: Nếu data lớn, dùng SQL `strftime('%Y-%m', created_at)` cho group by tháng — đã có index
6. **Không sửa Tauri app** — chỉ sửa code Nuxt (frontend) + Rust API (backend) trên VPS

---

## Tham khảo — Giao diện hiện tại

Hiện tại `/admin/thi-truong` chỉ có 3 card trống:
- Top sản phẩm bán chạy toàn hệ thống → "Chưa có dữ liệu"
- Top Nhà sản xuất → "Chưa có dữ liệu"
- Doanh thu theo tháng → "Chưa có dữ liệu"

→ Thay thế bằng 5 tabs trên.

---

## Verification

Sau khi deploy, kiểm tra:
1. Mỗi API endpoint trả 200 + JSON đúng format
2. Trang `/admin/thi-truong` hiện 5 tabs, chuyển tab mượt
3. Có empty state khi chưa có data
4. Admin auth hoạt động (redirect khi chưa login)
