# 🛡️ Giám Sát, Phục Hồi Thảm Họa & Bảo Mật — Micro-SaaS 1 VPS

> **Nguồn**: Google DeepSearch (Gemini) | **Ngày**: 13/03/2026
> **Mục đích**: Monitoring stack $0, Alerting, Auto-healing, Backup/DR, Rollback, Incident Response, Security Hardening
> **Bối cảnh**: 1 VPS (2 vCPU, 8GB RAM, Ubuntu) — Rust Axum + PostgreSQL — Bus Factor = 1

---

## 1. Monitoring Stack ($0)

### 1.1. Giám Sát Ngoại Vi (Uptime)

| Công cụ | Free Tier | Đặc biệt | Đánh giá |
|---------|:---------:|-----------|:--------:|
| UptimeRobot | 50 monitors | Cắt tính năng gần đây | 🟡 |
| **BetterStack** | HTTP/Ping/TCP + Incident Mgmt | **Gọi điện thoại khẩn cấp** trong gói Free | ✅ **CHỌN** |

> **Quan trọng**: KHÔNG self-host uptime monitor trên cùng VPS. VPS sập = monitor cũng sập = không cảnh báo.

### 1.2. Giám Sát Tài Nguyên VPS

| Công cụ | RAM tiêu thụ | Cấu hình | ML Anomaly | Đánh giá |
|---------|:----------:|:--------:|:----------:|:--------:|
| Prometheus + Grafana | 500MB-1GB | YAML phức tạp + PromQL | ❌ | ❌ Quá nặng cho 8GB |
| **Netdata** | **~vài chục MB** | **Zero-config** | ✅ **Tích hợp sẵn** | ✅ **CHỌN** |

**Netdata ưu điểm:**
- Per-second visibility, ~1% CPU
- ML Anomaly Detection: tự học hành vi bình thường → cảnh báo bất thường
- Đọc trực tiếp systemd/Nginx logs → không cần ELK stack
- Scrape `/metrics` OpenMetrics từ Rust Axum

### 1.3. Giám Sát Rust Axum

```
GET /health → SELECT 1 từ PostgreSQL → 200 OK (BetterStack gọi liên tục)
GET /metrics → prometheus-axum-middleware → Netdata scrape
```

### 1.4. Giám Sát PostgreSQL

```ini
# postgresql.conf
shared_preload_libraries = 'pg_stat_statements, auto_explain'
pg_stat_statements.track = all
log_min_duration_statement = 500      # Log query > 500ms
auto_explain.log_min_duration = '1s'  # Auto EXPLAIN query > 1s
```

- **pgbouncer** phía trước PostgreSQL → ổn định connections (mỗi conn ~10MB RAM)
- Netdata có plugin giám sát pgbouncer mặc định

### 1.5. Giám Sát Disk & SSL

- **Disk**: Cảnh báo khi **≥85%** (không chờ 99%)
- **SSL Let's Encrypt** (90 ngày): Cron script hàng ngày kiểm tra `openssl x509 -enddate` → cảnh báo trước 14 ngày

---

## 2. Alerting — Kênh Cảnh Báo

### 2.1. So Sánh Kênh

| Kênh | Miễn phí | Tin cậy | Xuyên DND | Đánh giá |
|------|:--------:|:-------:|:---------:|:--------:|
| Email (Gmail) | ✅ | 🟡 Trễ, spam | ❌ | Chỉ P2 |
| Zalo ZNS | ❌ Trả phí/msg | ✅ | ❌ | ⚠️ Bot scraping = khóa acc |
| **Telegram Bot** | ✅ **100% free** | ✅ Tức thời | ✅ Custom sound + bypass DND | ✅ **CHỌN** |
| **BetterStack** | ✅ Free tier | ✅ | ✅ **Gọi điện thoại** | ✅ **P0 only** |

### 2.2. Ma Trận Leo Thang (Escalation Policy)

| Mức | Đặc điểm | Kênh | Hành động |
|:---:|----------|------|-----------|
| **P0** (Critical) | VPS mất kết nối, PostgreSQL sập, crash loop | **BetterStack gọi điện** + Telegram báo động 30s (bypass DND) | Xử lý ngay lập tức |
| **P1** (Warning) | Disk >85%, CPU >90% 15 phút, HTTP 5xx tăng | Telegram push (âm thanh thường, không bypass DND) | Xử lý trong vài giờ |
| **P2** (Info) | Backup OK, SSL renewed, traffic daily | Telegram silent / Email tóm tắt | Xem trong giờ làm việc |

> **Alert Fatigue**: Chỉ P0 đánh thức. P1/P2 chờ sáng mai.

---

## 3. Auto-Healing Scripts

### 3.1. Systemd — Tự Khởi Động Lại

```ini
# /etc/systemd/system/axum-app.service
[Unit]
Description=Rust Axum POS Backend
After=network.target postgresql.service
OnFailure=telegram-alert@%n.service

[Service]
ExecStart=/opt/myapp/backend
Restart=always
RestartSec=5
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

> `LimitNOFILE=65535` → tránh "Too many open files" gây crash.

### 3.2. OOM Killer Protection — PostgreSQL

```ini
# /etc/systemd/system/postgresql.service.d/oom.conf
[Service]
OOMScoreAdjust=-1000
```

**Cấu hình RAM PostgreSQL cho VPS 8GB:**

| Tham số | Giá trị |
|---------|:-------:|
| `shared_buffers` | 2GB |
| `work_mem` | 16MB |
| `max_connections` | 100 |

> Công thức: shared_buffers + (max_connections × work_mem × 2) + (autovacuum_workers × maintenance_work_mem) < 8GB

### 3.3. Nginx Failover (1 VPS)

```nginx
upstream backend_api {
    server 127.0.0.1:8081 max_fails=3 fail_timeout=30s;
    server 127.0.0.1:8080 backup;  # Trang tĩnh "đang bảo trì"
}
```

### 3.4. Auto-Cleanup Script

```bash
#!/bin/bash
# /usr/local/bin/auto-cleanup.sh — chạy mỗi giờ (crontab: 0 * * * *)
set -euo pipefail

USAGE=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')
if [ "$USAGE" -ge 85 ]; then
    find /var/log/nginx /opt/myapp/logs -name "*.log" -mtime +7 -exec gzip {} \;
    find /var/log/nginx /opt/myapp/logs -name "*.log.gz" -mtime +30 -exec rm -f {} \;
    find /var/backups/sqlite -name "*.db" -mtime +14 -exec rm -f {} \;
    /usr/local/bin/telegram-alert.sh "Auto-cleanup: đĩa vượt 85%"
fi
```

### 3.5. Certbot Auto-Renew (Đúng Cách)

```bash
certbot renew --post-hook "systemctl reload nginx"
```

> **Cạm bẫy**: Certbot renew thành công nhưng Nginx vẫn dùng cert cũ (load cert lúc khởi động). `--post-hook reload` bắt buộc.

---

## 4. Backup & Disaster Recovery

### 4.1. Lợi Thế Offline-First

| Tham số | Web App thuần | **Offline-First (Nodi POS)** |
|---------|:------------:|:----------------------------:|
| **RPO** (mất dữ liệu max) | <1 phút (PITR) | **6-12 giờ** (pg_dump đủ) |
| **RTO** (downtime max) | <5 phút (Cluster) | **1-4 giờ** (VPS mới) |
| **Tác động khách hàng** | Mất giao dịch đang xảy ra | ✅ Vẫn bán hàng offline bình thường |

> Client SQLite giữ tất cả dữ liệu → VPS sập = khách hàng KHÔNG bị ảnh hưởng → sync lại khi VPS phục hồi.

### 4.2. pg_dump + Cloudflare R2

```bash
#!/bin/bash
# pg_backup_r2.sh — chạy mỗi 6 giờ
set -euo pipefail

DB_NAME="pos_production"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="/tmp/${DB_NAME}_${TIMESTAMP}.dump"

# Dump
sudo -u postgres pg_dump -Fc $DB_NAME -f "$BACKUP_FILE"

# Mã hóa GPG
gpg --batch --yes --passphrase "$GPG_SECRET" -c "$BACKUP_FILE"

# Push lên Cloudflare R2 (0 egress fee)
aws s3 cp "${BACKUP_FILE}.gpg" "s3://pos-backups/" \
    --endpoint-url "https://$CF_ACCOUNT_ID.r2.cloudflarestorage.com" \
    --profile cloudflare

rm "$BACKUP_FILE" "${BACKUP_FILE}.gpg"
```

> **Cloudflare R2** > AWS S3: API tương thích S3 nhưng **miễn phí hoàn toàn egress**.

### 4.3. Backup Verification (Hàng Tuần)

```
Tải backup mới nhất từ R2 → Giải mã GPG → pg_restore vào test DB
→ SELECT COUNT(*) FROM users → Kết quả hợp lệ?
    ├─ ✅ → Xóa test DB, báo Telegram "Backup verified"
    └─ ❌ → P0 ALERT! Kiểm tra quy trình backup ngay
```

> **"Backup chưa test = Không có backup"**

---

## 5. Rollback Strategy

### 5.1. Blue-Green Deployment (1 VPS)

```
Blue (v1.0) → port 8081 (đang chạy)
Green (v1.1) → port 8082 (mới deploy)

Nginx → proxy_pass 8081 (Blue)
    ↓ Test Green qua SSH tunnel
    ↓ OK? → nginx.conf: proxy_pass 8082 → nginx -s reload (zero-downtime)
    ↓ Lỗi? → giữ nguyên 8081 hoặc rollback 8081 ngay lập tức
```

**Canary Release**: Nginx `split_clients` → 90% Blue + 10% Green → dần tăng lên 100%.

### 5.2. Tauri Client Rollback (Dynamic Update Server)

```
Client v1.0.1 (bị lỗi crash on startup)
    ↓ Gọi Dynamic Update Server
    ↓ Server trả v1.0.0 (bản ổn định cũ) với HTTP 200
    ↓ Tauri tự downgrade → client hoạt động lại
```

> **Không dùng static JSON** → dùng Dynamic Update Server để có thể force rollback.

### 5.3. Database Migration — Expand & Contract

- Mọi migration phải **tương thích ngược** với code cũ
- **KHÔNG** `DROP COLUMN`, `RENAME` trong up migration khi app đang chạy
- Xóa vật lý: chỉ ở đợt bảo trì có lịch trình
- **Feature flags**: Tắt/bật logic mới qua config DB, không cần redeploy

---

## 6. Incident Response Playbook

### 6.1. P0 Response Plan (Lúc 3 Giờ Sáng)

```
📞 BetterStack gọi điện → Thức dậy
    ↓
⏸️ DỪNG 60 GIÂY — Hít thở. KHÔNG reboot ngay.
    ↓
🔍 TRIAGE: SSH vào VPS
    → htop / Netdata: CPU? RAM? Disk?
    → journalctl -u axum-app -n 50: Lỗi gì?
    ↓
🔧 MITIGATE (Khôi phục trước, tìm Root Cause sau):
    → Nếu sau update: Rollback Blue-Green về port cũ
    → Nếu OOM: systemctl restart postgresql
    → Nếu crash loop: systemd đã tự restart 3 lần?
    ↓
📢 TRUYỀN THÔNG:
    → Instatus/Cachet: "Hệ thống đang gián đoạn"
    → Zalo OA: "Dữ liệu bán hàng AN TOÀN trên thiết bị (offline).
       Sẽ tự đồng bộ khi máy chủ ổn định."
```

### 6.2. Post-Mortem Template (Blameless)

```markdown
## Sự cố [ngày/giờ]
- **Thời gian**: Bắt đầu HH:MM → Khôi phục HH:MM (X phút downtime)
- **Tác động**: [Mô tả]
- **Phát hiện qua**: [Kênh nào? BetterStack/Telegram/Khách báo?]

## 5 Whys
1. VPS sập → Tại sao?
2. Hết RAM → Tại sao?
3. OOM Killer chọn PostgreSQL → Tại sao?
4. Không cấu hình oom_score_adj → Tại sao?
5. Chưa có checklist hardening → Root Cause

## Action Items
- [ ] Cấu hình OOMScoreAdjust=-1000
- [ ] Thêm cảnh báo RAM >80% vào Netdata
```

---

## 7. Security Hardening

### 7.1. SSH

```bash
# /etc/ssh/sshd_config
Port 22849                    # Đổi cổng mặc định
PasswordAuthentication no     # Key-Only
PermitRootLogin no
MaxAuthTries 3
```

### 7.2. Firewall (UFW)

```bash
ufw default deny incoming
ufw default allow outgoing
ufw allow 80/tcp      # HTTP
ufw allow 443/tcp     # HTTPS
ufw allow 22849/tcp   # SSH custom port
ufw enable
```

> **PostgreSQL**: `listen_addresses = 'localhost'` → KHÔNG expose ra Internet.

### 7.3. Fail2ban

```
SSH brute-force: Đọc /var/log/auth.log → Ban IP 24h
Nginx API abuse: Đọc access.log → Ban IP khi quá nhiều 401/403
+ Nginx limit_req → Rate limiting L7 DDoS
```

### 7.4. Auto Security Updates

```bash
# unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
# Chỉ install security patches tự động
# Reboot lúc 02:00 nếu kernel update yêu cầu
```

---

## Tổng Kết

```
┌──────────────────────────────────────────────────────────┐
│  NODI POS — MONITORING & DISASTER RECOVERY              │
│                                                          │
│  📊 MONITORING ($0)                                     │
│     BetterStack (external uptime + gọi điện P0)         │
│     Netdata (internal metrics + ML anomaly)              │
│     Telegram Bot (alerting chính)                        │
│                                                          │
│  🔄 AUTO-HEALING                                        │
│     systemd Restart=always + OnFailure=telegram          │
│     OOM Killer protection cho PostgreSQL                 │
│     Nginx passive failover + auto-cleanup script         │
│     Certbot --post-hook reload nginx                     │
│                                                          │
│  💾 BACKUP                                              │
│     pg_dump mỗi 6h → GPG encrypt → Cloudflare R2       │
│     Backup verification hàng tuần (tự động)             │
│     Offline-first = RPO 6-12h đủ tốt                    │
│                                                          │
│  🔁 ROLLBACK                                            │
│     Blue-Green trên 1 VPS (2 ports + nginx reload)      │
│     Tauri Dynamic Update Server (force downgrade)        │
│     DB migration: Expand & Contract only                 │
│                                                          │
│  🚨 INCIDENT RESPONSE                                   │
│     P0: BetterStack gọi điện → 60s thở → SSH triage    │
│     Mitigate trước, Root Cause sau                      │
│     Post-mortem Blameless + 5 Whys                      │
│                                                          │
│  🔒 SECURITY                                            │
│     SSH key-only + custom port + Fail2ban               │
│     UFW deny all + PostgreSQL localhost only             │
│     unattended-upgrades (auto security patches)          │
└──────────────────────────────────────────────────────────┘
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Lưu trữ bởi Nodi POS Strategy Team*
