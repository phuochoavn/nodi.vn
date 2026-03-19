# 🔧 UPGRADE Brief — Admin Lifecycle & Billing

> **Giao cho**: Agent VPS (Claude Opus 4)
> **Ngày giao**: 2026-02-19
> **Ưu tiên**: 🔴 CRITICAL — Phải có trước GO LIVE
> **Phạm vi**: Backend API (Rust) + Frontend (Nuxt/Vue)

---

## Mục tiêu

Nâng cấp Admin Panel để **quản lý vòng đời license + thu tiền + giám sát cửa hàng**. Hiện admin chỉ tạo key được, không gia hạn/thu hồi/theo dõi thanh toán.

---

## PHẦN 1: License Lifecycle — Gia hạn, Thu hồi, Hết hạn

### 1.1 Database Changes

Bảng `licenses` cần thêm columns (nếu chưa có):

```sql
ALTER TABLE licenses ADD COLUMN expires_at TIMESTAMP;        -- Ngày hết hạn
ALTER TABLE licenses ADD COLUMN activated_at TIMESTAMP;       -- Ngày kích hoạt
ALTER TABLE licenses ADD COLUMN revoked_at TIMESTAMP;         -- Ngày thu hồi (NULL = chưa)
ALTER TABLE licenses ADD COLUMN duration_days INTEGER DEFAULT 30;  -- Thời hạn gói
```

### 1.2 API Endpoints

```
PUT  /api/admin/licenses/:id/extend    — Gia hạn thêm N ngày
PUT  /api/admin/licenses/:id/revoke    — Thu hồi (vô hiệu hóa)
PUT  /api/admin/licenses/:id/activate  — Kích hoạt lại key đã revoke
GET  /api/admin/licenses/expiring      — Danh sách key sắp hết hạn (< 7 ngày)
```

**PUT /extend** body:
```json
{ "days": 30 }  // Gia hạn thêm 30 ngày từ expires_at hiện tại
```
Logic: `expires_at = MAX(expires_at, NOW()) + days`

**PUT /revoke** → set `revoked_at = NOW()`, status = REVOKED

**GET /expiring** → WHERE `expires_at < NOW() + 7 days AND revoked_at IS NULL`

### 1.3 Frontend — License Page Updates

Bảng license hiện tại thêm cột:

| License Key | Gói | Trạng thái | Hết hạn | Còn lại | Actions |
|-------------|-----|-----------|---------|---------|---------|
| NODI-XXXX | Tháng | ✅ ACTIVE | 2026-03-19 | 28 ngày | [Gia hạn] [Thu hồi] [Reset HWID] |
| NODI-YYYY | Năm | ⚠️ SẮP HẾT | 2026-02-25 | 6 ngày | [Gia hạn] [Thu hồi] [Reset HWID] |
| NODI-ZZZZ | Trial | ❌ HẾT HẠN | 2026-02-10 | -9 ngày | [Gia hạn] [Xóa] |
| NODI-WWWW | Tháng | 🚫 REVOKED | -- | -- | [Kích hoạt lại] |

**Status logic:**
- `revoked_at IS NOT NULL` → 🚫 REVOKED
- `hwid IS NULL` → ⏳ PENDING (chưa kích hoạt)
- `expires_at < NOW()` → ❌ HẾT HẠN
- `expires_at < NOW() + 7 days` → ⚠️ SẮP HẾT (highlight vàng)
- else → ✅ ACTIVE

**Nút Gia hạn** → modal:
```
Gia hạn License: NODI-XXXX
Thêm: [30 ngày ▼]  (options: 30 / 90 / 365 ngày)
Hết hạn mới: 2026-04-18
[Gia hạn]  [Hủy]
```

**Nút Thu hồi** → confirm: "Thu hồi key NODI-XXXX? Cửa hàng sẽ không thể sử dụng."

---

## PHẦN 2: Billing / Thanh toán

### 2.1 Database

```sql
CREATE TABLE license_payments (
    id SERIAL PRIMARY KEY,
    license_id INTEGER REFERENCES licenses(id),
    amount INTEGER NOT NULL,           -- Số tiền (VND)
    payment_method TEXT DEFAULT 'BANK_TRANSFER',  -- CASH, BANK_TRANSFER, MOMO
    period_start DATE,                 -- Giai đoạn thanh toán từ
    period_end DATE,                   -- Giai đoạn thanh toán đến
    note TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### 2.2 API

```
POST /api/admin/licenses/:id/payments     — Ghi nhận thanh toán
GET  /api/admin/licenses/:id/payments     — Lịch sử thanh toán
GET  /api/admin/billing/summary           — Tổng doanh thu license
```

**POST /payments** body:
```json
{
  "amount": 200000,
  "payment_method": "BANK_TRANSFER",
  "note": "Chuyển khoản T2/2026"
}
```

### 2.3 Frontend

**Trên bảng license**, thêm cột "Thanh toán":
- 💰 Đã TT (nếu có payment trong 30 ngày gần nhất)
- ⚠️ Chưa TT (nếu không có)

**Click vào license** → detail panel bên phải hoặc modal:
```
License: NODI-XXXX-YYYY
Gói: Tháng (200,000đ/tháng)
Trạng thái: ✅ ACTIVE
Hết hạn: 2026-03-19 (còn 28 ngày)
Cửa hàng: Đại lý Bình Minh

--- Lịch sử thanh toán ---
| Ngày       | Số tiền    | PT          | Ghi chú           |
|------------|-----------|-------------|-------------------|
| 2026-02-19 | 200,000đ  | Chuyển khoản | CK T2/2026       |
| 2026-01-15 | 200,000đ  | Tiền mặt    | Thu trực tiếp     |

[+ Ghi nhận thanh toán]  [Gia hạn]  [Thu hồi]
```

---

## PHẦN 3: Cửa hàng Chi tiết

### 3.1 API

```
GET /api/admin/stores                     — Danh sách cửa hàng (đã có, fix nếu cần)
GET /api/admin/stores/:id                 — Chi tiết 1 cửa hàng
```

**GET /stores/:id** returns:
```json
{
  "store": { "name", "address", "phone" },
  "license": { "key", "plan", "status", "expires_at" },
  "sync": { "last_synced_at", "total_syncs" },
  "stats": {
    "total_products": 150,
    "total_customers": 45,
    "total_orders": 320,
    "total_revenue": 50000000,
    "total_customer_debt": 12000000,
    "total_supplier_debt": 8000000
  }
}
```

### 3.2 Frontend — `/admin/cua-hang`

**Danh sách** (table):

| Cửa hàng | License | Gói | Trạng thái | Sync cuối | DT tháng |
|-----------|---------|-----|-----------|-----------|----------|
| ĐL Bình Minh | NODI-XXXX | Tháng | ✅ | 2 giờ trước | 15tr |
| ĐL Hưng Phát | NODI-YYYY | Năm | ⚠️ Sắp hết | 3 ngày trước | 8tr |

**Click vào** → `/admin/cua-hang/:id` hoặc modal chi tiết:
- Info: Tên, địa chỉ, SĐT, license
- Stats: 6 cards (SP, KH, ĐH, DT, nợ KH, nợ NCC)
- Sync: Lần cuối, tổng số lần

---

## PHẦN 4: Cảnh báo Dashboard

### 4.1 API

```
GET /api/admin/alerts — Danh sách cảnh báo
```

Returns:
```json
{
  "alerts": [
    { "type": "LICENSE_EXPIRING", "message": "NODI-XXXX hết hạn trong 3 ngày", "severity": "warning" },
    { "type": "STORE_INACTIVE", "message": "ĐL Hưng Phát không sync 7 ngày", "severity": "info" },
    { "type": "LICENSE_EXPIRED", "message": "NODI-ZZZZ đã hết hạn", "severity": "error" }
  ]
}
```

### 4.2 Frontend

Hiển thị trên trang **Tổng quan** (`/admin`), phía trên các stat cards:

```
⚠️ 2 cảnh báo
┌─────────────────────────────────────────────────┐
│ ⚠️ NODI-XXXX hết hạn trong 3 ngày    [Gia hạn] │
│ ℹ️ ĐL Hưng Phát không sync 7 ngày     [Xem]    │
└─────────────────────────────────────────────────┘
```

---

## Thứ tự ưu tiên

1. **License lifecycle** (extend/revoke/expiry) — Quan trọng nhất
2. **Billing** (ghi nhận thanh toán) — Cần để thu tiền
3. **Store detail** — Cần để support khách
4. **Alerts** — Nice-to-have nhưng nên có

## Verification

1. Tạo 1 license mới → gia hạn 30 ngày → kiểm tra expires_at
2. Thu hồi 1 license → kiểm tra status = REVOKED
3. Ghi nhận thanh toán → kiểm tra hiển thị đúng
4. Mở chi tiết cửa hàng → thấy đủ 6 stats
5. Tạo license sắp hết hạn → cảnh báo hiện trên Overview
