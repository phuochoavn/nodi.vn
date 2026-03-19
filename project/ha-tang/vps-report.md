# 📊 Báo Cáo Hạ Tầng VPS — api.nodi.vn

> **Ngày**: 15/03/2026 | **Uptime**: 29 ngày | **Hostname**: srv1377210

---

## 1. Server Specs

```
┌─────────────────────────────────────────────────┐
│  Provider:  Contabo VPS                         │
│  OS:        Ubuntu 24.04.4 LTS (Noble Numbat)   │
│  CPU:       AMD EPYC 9354P — 2 cores @ 2.0GHz   │
│  RAM:       7.8 GB (used: 3.9 GB, free: 3.8 GB)│
│  Disk:      96 GB SSD (used: 47 GB — 49%)       │
│  Load avg:  0.09 / 0.12 / 0.08                  │
│  Uptime:    29 days, 2h34m                      │
└─────────────────────────────────────────────────┘
```

---

## 2. Software Stack

| Component | Version | Container | Image Size |
|-----------|---------|-----------|------------|
| Nginx | 1.29.5 | `nodi-nginx` | 93 MB |
| Rust/Axum API | 0.1.0 | `nodi-api` | 148 MB |
| PostgreSQL | 16.12 | `nodi-postgres` | 395 MB |
| Nuxt.js Web | — | `nodi-web` | 337 MB |

**SSL**: Google Trust Services (WE1) — Expires **15/05/2026** ⚠️

---

## 3. API Endpoints (103 total)

### Auth & Account (8)
| Method | Path | Mô tả |
|--------|------|-------|
| POST | `/api/login` | Đăng nhập |
| POST | `/api/register` | Đăng ký |
| POST | `/api/login-with-license` | Login bằng license key |
| POST | `/api/auth/register` | Đăng ký store |
| POST | `/api/auth/refresh` | Refresh JWT |
| POST | `/api/unbind-device` | Gỡ liên kết thiết bị |
| POST | `/api/update-phone` | Cập nhật SĐT |
| GET | `/api/verify-license` | Xác thực license |

### Sync (3)
| Method | Path | Mô tả |
|--------|------|-------|
| POST | `/api/sync` | Push data lên cloud |
| GET | `/api/sync/pull` | Pull data từ cloud |
| WS | `/ws/sync` | WebSocket real-time notification |

### Dashboard (16)
- `/api/dashboard/summary`, `/orders`, `/inventory`, `/debts`
- `/api/dashboard/reports/revenue`, `/top-products`
- `/api/dashboard/staff`, `/settings`, `/notifications`
- `/api/dashboard/accounting/*` (4), `/einvoice/*` (2)

### Admin (35)
- `/api/admin/overview`, `/stores`, `/licenses`, `/accounts`
- `/api/admin/intelligence/*`, `/market/*`, `/support/*`
- `/api/admin/billing/*`, `/export/*`, `/update/*`

### Others (41)
- Backup (3), Payment (3), Scanner (3), Staff Invite (3)
- Devices (2), Downloads (1), Upload (1), Health (1)
- Support REST (6) + WS (1), Store Management (3)

---

## 4. Database Schema

**PostgreSQL 16.12** — Total: ~2.3 MB, 44 tables

| Bảng | Rows | Size |
|------|------|------|
| `synced_products` | 164 | 176 KB |
| `synced_product_units` | 168 | 112 KB |
| `synced_invoices` | 22 | 112 KB |
| `synced_product_transactions` | 20 | 96 KB |
| `synced_cash_transactions` | 17 | 96 KB |
| `synced_customers` | 14 | 96 KB |
| `synced_invoice_items` | 13 | 96 KB |
| `sync_staff_members` | 13 | 96 KB |
| `synced_customer_transactions` | 10 | 64 KB |
| `synced_suppliers` | 7 | 80 KB |
| `accounts` | 5 | 64 KB |
| `devices` | 6 | 80 KB |

**Indexes**: Unique on `(store_id, local_id)` cho mỗi `synced_*` table.

**PG Tuning**: shared_buffers=2GB, work_mem=32MB, effective_cache_size=5GB, max_connections=100

---

## 5. Security

| Item | Status |
|------|--------|
| UFW Firewall | ✅ Active (22/80/443) |
| Cloudflare CDN | ✅ DDoS protection |
| Nginx Security Headers | ✅ Full CSP |
| Rate Limiting | ❌ Không (bỏ tower_governor) |
| Fail2ban | ❌ Không chạy |
| JWT | ✅ Access 24h, Refresh 30d |
| DB exposure | ✅ Bind 127.0.0.1 only |

---

## 6. Bottlenecks

### 🔴 Critical
1. **Không có backup tự động** — cần cron pg_dump
2. **SSL hết hạn 15/05/2026** — cần auto-renew certbot
3. **Không có rate limiting** — DDoS/brute-force risk

### 🟡 Warning
4. Nginx buffer warning (sync payload lớn)
5. Fail2ban không chạy
6. Không có uptime monitoring
7. Disk 49% used

### Khả năng chịu tải (ước tính)
- Concurrent connections: ~500-1000
- API latency: <50ms health, ~200ms login, ~100ms sync
- WebSocket: ~100 concurrent
- Memory headroom: ~3.8 GB free

---

*Report generated: 15/03/2026 by Agent VPS (đọc metrics thực tế)*
