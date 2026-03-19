# 💬 Live Support Chat — Brief cho Agent VPS

## Mục tiêu
Xây dựng hệ thống Live Chat 2 chiều giữa khách hàng (Tauri POS app) và admin panel.
Khách gõ "muốn gặp kỹ thuật viên" trong chatbot → tạo ticket → admin thấy → chat qua lại.

---

## 1. Database Schema

```sql
-- Bảng ticket hỗ trợ
CREATE TABLE support_tickets (
    id SERIAL PRIMARY KEY,
    store_id INTEGER REFERENCES stores(id),
    license_key TEXT NOT NULL,
    store_name TEXT,
    phone TEXT,
    subject TEXT NOT NULL,
    status TEXT DEFAULT 'open' CHECK(status IN ('open','in_progress','resolved','closed')),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    resolved_at TIMESTAMP
);

-- Bảng tin nhắn
CREATE TABLE support_messages (
    id SERIAL PRIMARY KEY,
    ticket_id INTEGER REFERENCES support_tickets(id) ON DELETE CASCADE,
    sender_type TEXT NOT NULL CHECK(sender_type IN ('customer','admin')),
    sender_name TEXT,
    message TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_tickets_status ON support_tickets(status);
CREATE INDEX idx_tickets_license ON support_tickets(license_key);
CREATE INDEX idx_messages_ticket ON support_messages(ticket_id);
```

---

## 2. API Endpoints

### App-facing (Tauri POS gọi — auth bằng license_key trong body/header)

| Endpoint | Method | Request Body | Response |
|----------|--------|-------------|----------|
| `/api/support/ticket` | POST | `{ license_key, store_name, phone, subject, message }` | `{ ticket_id, status }` |
| `/api/support/my-tickets` | GET | query: `?license_key=NODI-XXXX` | `[{ id, subject, status, last_message, unread_count, created_at }]` |
| `/api/support/ticket/:id/messages` | GET | query: `?license_key=NODI-XXXX&since=2026-02-19T00:00:00` | `[{ id, sender_type, sender_name, message, created_at }]` |
| `/api/support/ticket/:id/reply` | POST | `{ license_key, message }` | `{ message_id }` |

> **Auth logic**: Verify `license_key` exists in `stores` table AND matches `ticket.license_key`

### Admin-facing (JWT auth — giống các admin API hiện tại)

| Endpoint | Method | Request Body | Response |
|----------|--------|-------------|----------|
| `/api/admin/support/tickets` | GET | query: `?status=open` | `[{ id, store_name, phone, subject, status, unread_count, last_message, created_at }]` |
| `/api/admin/support/tickets/:id` | GET | — | `{ ticket details + all messages }` |
| `/api/admin/support/tickets/:id/reply` | POST | `{ message }` | `{ message_id }` |
| `/api/admin/support/tickets/:id/status` | PUT | `{ status }` | `{ ok }` |
| `/api/admin/support/unread` | GET | — | `{ count }` |

---

## 3. Frontend Admin — Trang `/admin/support`

### Layout
```
┌──────────────────────────────────────────────┐
│  Admin > Hỗ trợ Kỹ thuật                    │
├─────────────────┬────────────────────────────┤
│ 📋 Ticket List  │  💬 Chat Thread            │
│                 │                            │
│ ● Nguyễn Văn A │  Nguyễn Văn A              │
│   "Máy in lỗi" │  Cửa hàng Bình Dương       │
│   5 phút trước  │  0909-xxx-xxx | NODI-ABCD  │
│                 │                            │
│ ○ Trần Thị B   │  ┌───────────────────────┐  │
│   "Không sync"  │  │ Máy in không in được, │  │
│   1 giờ trước   │  │ bị lỗi kẹt giấy      │  │
│                 │  └───────────────────────┘  │
│                 │           👤 Khách - 14:20  │
│                 │                            │
│                 │  ┌───────────────────────┐  │
│                 │  │ Anh thử tắt máy in    │  │
│                 │  │ và bật lại nhé         │  │
│                 │  └───────────────────────┘  │
│                 │  🛡️ Admin - 14:22          │
│                 │                            │
│                 │  [____Nhập tin nhắn____] 📤 │
├─────────────────┴────────────────────────────┤
│  Trạng thái: [Open ▼] [Đánh dấu đã xử lý]  │
└──────────────────────────────────────────────┘
```

### Yêu cầu
- **Sidebar badge**: Thêm menu "Hỗ trợ" vào admin sidebar, hiện badge số ticket `open`
- **Ticket list**: Sắp theo updated_at DESC, badge đỏ cho unread
- **Chat thread**: Messenger-style bubbles (khách = trái/xanh, admin = phải/tím)
- **Store info card**: Hiện tên cửa hàng, SĐT, license key, gói
- **Status dropdown**: Open → In Progress → Resolved → Closed
- **Auto-refresh**: Polling mỗi 10 giây cho ticket list + chat thread
- **Sound/notification**: Optional — beep khi có tin mới

### Sidebar update (layout hiện tại)
```
Tổng quan     ← hiện có
Licenses      ← hiện có
Cửa hàng      ← hiện có
Thị trường    ← hiện có
Hỗ trợ (3)    ← MỚI (badge = số ticket open)
```

---

## 4. Verification

### API Tests
```bash
# 1. Create ticket
curl -X POST https://api.nodi.vn/api/support/ticket \
  -H "Content-Type: application/json" \
  -d '{"license_key":"NODI-TEST-KEY","store_name":"Cửa hàng Test","phone":"0909123456","subject":"Máy in không hoạt động","message":"Máy in bị kẹt giấy, không in được hóa đơn"}'
# Expected: 200 { ticket_id: 1, status: "open" }

# 2. Admin list tickets
curl -H "Authorization: Bearer $TOKEN" \
  "https://api.nodi.vn/api/admin/support/tickets"
# Expected: 200 [{ id:1, store_name:"Cửa hàng Test", subject:..., unread_count:1 }]

# 3. Admin reply
curl -X POST -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  "https://api.nodi.vn/api/admin/support/tickets/1/reply" \
  -d '{"message":"Anh thử tắt máy in và bật lại nhé"}'
# Expected: 200 { message_id: 2 }

# 4. Customer check messages
curl "https://api.nodi.vn/api/support/ticket/1/messages?license_key=NODI-TEST-KEY"
# Expected: 200 [{ sender_type:"customer", message:"Máy in bị..." }, { sender_type:"admin", message:"Anh thử..." }]

# 5. Unread count
curl -H "Authorization: Bearer $TOKEN" \
  "https://api.nodi.vn/api/admin/support/unread"
# Expected: 200 { count: 0 }
```

### Browser Tests
- Navigate to `/admin/support` → ticket list visible
- Click ticket → chat thread loads
- Type reply → message appears in thread
- Sidebar shows "Hỗ trợ" with badge count
