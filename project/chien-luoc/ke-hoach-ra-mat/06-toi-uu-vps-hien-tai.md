# ⚙️ Tối Ưu VPS Hiện Tại — Contabo 2vCPU / 8GB RAM

> **Nguồn**: Google DeepSearch (Gemini) + VPS Agent Report | **Ngày**: 13/03/2026
> **VPS**: Contabo GmbH (Đức) — Ubuntu 24.04.4 LTS — Docker Compose 4 services
> **Mục đích**: PostgreSQL tuning, Nginx hardening, Security, Backup R2, Monitoring, DR Playbook

---

## 1. Đánh Giá Hiện Trạng — Contabo Đức Có Phù Hợp?

### So Sánh Chi Phí

| Provider | Vị trí | Cấu hình | Chi phí/tháng |
|----------|--------|:--------:|:-------------:|
| **Contabo** | 🇩🇪 Đức | 2 vCPU / 8GB RAM | **~$5.50** |
| Vultr | 🇸🇬 Singapore | 2 vCPU / 8GB RAM | ~$24 |
| Bizfly | 🇻🇳 Việt Nam | 2-6 vCPU / 8GB RAM | ~$45 |

### Kết Luận

> **Giữ Contabo**. Latency 150-170ms tới VN nhưng **không ảnh hưởng** nhờ offline-first:
> - Bán hàng, quét barcode, in hóa đơn → **100% local** (Tauri + SQLite)
> - Sync nền → bất đồng bộ, 160ms vô hình với user
> - Website nodi.vn → Cloudflare CDN cache ở VN/SG
> - Chi phí thấp hơn **4-8x** so với migrate sang Châu Á

---

## 2. PostgreSQL Tuning (Docker)

### Hiện tại vs Khuyến nghị

| Tham số | Hiện tại | Khuyến nghị | Lý do |
|---------|:--------:|:-----------:|-------|
| `shared_buffers` | **128MB** ⚠️ | **2048MB** | 25% RAM. Toàn bộ DB 12MB vào RAM |
| `work_mem` | **4MB** ⚠️ | **20MB** | (RAM×0.25)/max_connections. Tránh spill to disk |
| `effective_cache_size` | 4GB | **6144MB** | 75% RAM. Hint cho Query Planner dùng index |
| `maintenance_work_mem` | 64MB | **512MB** | VACUUM, CREATE INDEX chạy nhanh hơn |
| `random_page_cost` | 4.0 (default) | **1.1** | SSD → random ≈ sequential |
| `wal_buffers` | auto | **16MB** | Giảm I/O calls khi ghi WAL |
| `checkpoint_completion_target` | 0.5 | **0.9** | Làm phẳng I/O spikes |
| `max_connections` | 100 | **100** ✅ | Đủ cho Axum pool |

### File cấu hình: `/opt/nodi/postgres/postgresql.conf`

```ini
# PostgreSQL 16 - Nodi POS Production
# Hardware: 2 vCPU AMD EPYC, 8GB RAM, SSD/NVMe

listen_addresses = '*'
max_connections = 100

# Memory
shared_buffers = 2048MB
huge_pages = off
temp_buffers = 16MB
work_mem = 20MB
maintenance_work_mem = 512MB
autovacuum_work_mem = -1
logical_decoding_work_mem = 64MB

# WAL & Checkpoints
wal_level = replica
wal_buffers = 16MB
checkpoint_timeout = 15min
checkpoint_completion_target = 0.9
max_wal_size = 4GB
min_wal_size = 1GB

# Query Planner (SSD optimized)
random_page_cost = 1.1
effective_io_concurrency = 200
effective_cache_size = 6144MB
default_statistics_target = 100

# Parallel Processing (2 vCPU)
max_worker_processes = 2
max_parallel_workers_per_gather = 1
max_parallel_workers = 2
max_parallel_maintenance_workers = 1
```

### Mount vào Docker

```yaml
# docker-compose.yml — service postgres
services:
  nodi-postgres:
    image: postgres:16-alpine
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
      - /opt/nodi/postgres/postgresql.conf:/etc/postgresql/postgresql.conf:ro
    command: postgres -c config_file=/etc/postgresql/postgresql.conf
```

---

## 3. Nginx Production Hardening

### Security Headers (CHƯA CÓ → CẦN THÊM)

| Header | Giá trị | Bảo vệ chống |
|--------|---------|--------------|
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains; preload` | Downgrade attacks, MITM |
| `X-Frame-Options` | `SAMEORIGIN` | Clickjacking |
| `X-Content-Type-Options` | `nosniff` | MIME-sniffing |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Token leak qua Referrer |
| `Permissions-Policy` | `geolocation=(), microphone=(), camera=()` | Hardware API abuse |

### Connection Pooling (Mới)

```nginx
upstream axum_api {
    server nodi-api:3000;
    keepalive 64;  # Duy trì 64 kết nối idle → giảm TCP handshake
}

upstream nuxt_web {
    server nodi-web:3001;
    keepalive 32;
}
```

### Sync Endpoint (Payload lớn từ Tauri)

```nginx
location /api/sync/ {
    client_max_body_size 50M;    # Mặc định 1MB → reject sync payload
    proxy_buffer_size 32k;
    proxy_buffers 16 32k;
    proxy_busy_buffers_size 64k;
}
```

> **Config hoàn chỉnh**: Xem phần "Bản cấu hình Nginx toàn diện" trong báo cáo gốc.

---

## 4. Security Hardening Script

### Hiện trạng → Cần sửa

| Hạng mục | Hiện tại | Sau hardening |
|----------|:--------:|:-------------:|
| SSH Port | 22 (default) | **22222** (custom) |
| PermitRootLogin | **yes** 🔴 | **no** |
| PasswordAuthentication | default | **no** (key-only) |
| Fail2ban | ❌ Chưa cài | ✅ SSH + Nginx jails |
| Auto security updates | ❌ | ✅ unattended-upgrades |

### Script: `harden_nodi.sh`

```bash
#!/bin/bash
set -euo pipefail

NEW_SSH_PORT=22222
ADMIN_USER="$SUDO_USER"

# Safety check 1: Không chạy trực tiếp từ root
if [ -z "$ADMIN_USER" ] || [ "$ADMIN_USER" = "root" ]; then
    echo "LỖI: Chạy bằng 'sudo bash harden_nodi.sh', không chạy từ root"
    exit 1
fi

# Safety check 2: Kiểm tra SSH key tồn tại TRƯỚC khi tắt password
AUTH_KEYS="/home/$ADMIN_USER/.ssh/authorized_keys"
if [ ! -s "$AUTH_KEYS" ]; then
    echo "LỖI: SSH key chưa có! Chạy ssh-copy-id trước"
    exit 1
fi

# Backup sshd_config
cp /etc/ssh/sshd_config /etc/ssh/sshd_config.backup_$(date +%F)

# SSH hardening
sed -i "s/^#\?Port.*/Port $NEW_SSH_PORT/" /etc/ssh/sshd_config
sed -i "s/^#\?PermitRootLogin.*/PermitRootLogin no/" /etc/ssh/sshd_config
sed -i "s/^#\?PasswordAuthentication.*/PasswordAuthentication no/" /etc/ssh/sshd_config

# UFW
ufw allow $NEW_SSH_PORT/tcp
ufw reload

# Fail2ban
apt-get update -y && apt-get install -y fail2ban
cat <<EOF > /etc/fail2ban/jail.local
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = $NEW_SSH_PORT

[nginx-http-auth]
enabled = true
port = http,https
logpath = /var/log/nginx/error.log
EOF
systemctl enable fail2ban && systemctl restart fail2ban

# Auto security updates
apt-get install -y unattended-upgrades apt-listchanges
cat <<EOF > /etc/apt/apt.conf.d/20auto-upgrades
APT::Periodic::Update-Package-Lists "1";
APT::Periodic::Unattended-Upgrade "1";
EOF
systemctl enable unattended-upgrades

systemctl restart sshd
echo "✅ Hardening hoàn tất. KHÔNG ĐÓNG terminal này!"
echo "→ Mở terminal MỚI, SSH qua port $NEW_SSH_PORT để verify"
```

> ⚠️ **QUAN TRỌNG**: Không đóng terminal hiện tại cho đến khi verify SSH qua port mới thành công!

---

## 5. Backup Off-site → Cloudflare R2

### Hiện trạng

- ✅ pg_dump hàng ngày (2h + 3h sáng)
- ❌ **Chỉ local** → VPS chết = mất hết
- ❌ Không mã hóa
- ❌ Không verification

### Giải pháp: GPG Encrypt → Cloudflare R2 (0 egress fee)

#### Setup GPG (1 lần)

```bash
# Trên máy CÁ NHÂN (không phải VPS):
gpg --full-generate-key  # RSA, admin@nodi.vn

# Export public key:
gpg --export -a "admin@nodi.vn" > nodi_pub.asc

# Upload lên VPS và import:
gpg --import nodi_pub.asc
echo -e "5\ny\n" | gpg --command-fd 0 --edit-key "admin@nodi.vn" trust
```

#### Script backup: `/opt/nodi/scripts/r2_backup.sh`

```bash
#!/bin/bash
set -euo pipefail

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/nodi/backups"
FILENAME="nodi_db_${TIMESTAMP}.sql.gz.gpg"

# Pipeline: pg_dump → gzip → GPG encrypt
docker exec -t nodi-postgres pg_dump -U nodi_admin -d nodi \
    | gzip -9 \
    | gpg --batch --yes --always-trust --encrypt --recipient "admin@nodi.vn" \
    -o "${BACKUP_DIR}/${FILENAME}"

# Push to R2
aws s3 cp "${BACKUP_DIR}/${FILENAME}" s3://nodi-pos-backups/db_backups/ \
    --endpoint-url "https://<ACCOUNT_ID>.r2.cloudflarestorage.com"

# Cleanup local > 14 days
find $BACKUP_DIR -name "*.gpg" -mtime +14 -exec rm {} \;
```

#### Verification hàng tuần: `/opt/nodi/scripts/r2_verify.sh`

```bash
#!/bin/bash
set -euo pipefail

R2_ENDPOINT="https://<ACCOUNT_ID>.r2.cloudflarestorage.com"
LATEST=$(aws s3 ls s3://nodi-pos-backups/db_backups/ --endpoint-url $R2_ENDPOINT | sort | tail -1 | awk '{print $4}')

aws s3 cp "s3://nodi-pos-backups/db_backups/${LATEST}" /tmp/verify.gpg --endpoint-url $R2_ENDPOINT

if gpg --list-packets /tmp/verify.gpg > /dev/null 2>&1; then
    echo "✅ Backup ${LATEST} valid"
else
    /opt/nodi/scripts/telegram_alert.sh "P1" "Backup ${LATEST} CORRUPTED!"
fi
rm -f /tmp/verify.gpg
```

#### Crontab

```cron
0 3 * * * /opt/nodi/scripts/r2_backup.sh >> /var/log/nodi_backup.log 2>&1
0 4 * * 0 /opt/nodi/scripts/r2_verify.sh >> /var/log/nodi_verify.log 2>&1
```

---

## 6. Monitoring Stack ($0)

### Kiến trúc

```
BetterStack (external) ──ping──→ nodi.vn
    └── Sập? → Gọi điện thoại P0

Netdata (host agent) ──đọc──→ Docker containers + host metrics
    └── Anomaly? → Telegram Bot

Telegram Bot ──nhận──→ Backup OK, SSL check, Disk alert
```

### Telegram Alert Script: `/opt/nodi/scripts/telegram_alert.sh`

```bash
#!/bin/bash
PRIORITY=$1; MESSAGE=$2
BOT_TOKEN="YOUR_BOT_TOKEN"
CHAT_ID="YOUR_CHAT_ID"

EMOJI="🚨"; [ "$PRIORITY" = "P1" ] && EMOJI="⚠️"
TEXT="${EMOJI} <b>NODI POS</b>%0A${MESSAGE}%0A<i>$(hostname)</i>"

curl -s -X POST "https://api.telegram.org/bot${BOT_TOKEN}/sendMessage" \
    -d chat_id="${CHAT_ID}" -d text="${TEXT}" -d parse_mode="HTML" > /dev/null
```

### SSL Cert Check (Cloudflare Origin CA)

```bash
#!/bin/bash
# /opt/nodi/scripts/check_ssl_expiry.sh — chạy hàng tuần
CERT="/etc/nginx/ssl/nodi.vn.pem"
DAYS_LEFT=$(( ($(openssl x509 -enddate -noout -in "$CERT" | cut -d= -f2 | date -f - '+%s') - $(date '+%s')) / 86400 ))

[ "$DAYS_LEFT" -lt 30 ] && /opt/nodi/scripts/telegram_alert.sh "P1" "SSL cert hết hạn trong ${DAYS_LEFT} ngày!"
```

---

## 7. Docker Compose Production

### Cải tiến quan trọng

| Cải tiến | Trước | Sau |
|----------|:-----:|:---:|
| Log rotation | ❌ Vô hạn | ✅ 10MB × 3 files |
| Health check | ❌ | ✅ PostgreSQL `pg_isready` |
| Memory limits | ❌ | ✅ cgroups v2 |
| Depends on condition | ❌ | ✅ `service_healthy` |
| Non-root container | ❌ | ✅ Axum user 1000 |

### Docker Compose tối ưu

```yaml
version: "3.8"

x-logging: &default-logging
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"

services:
  nodi-postgres:
    image: postgres:16-alpine
    container_name: nodi-postgres
    restart: unless-stopped
    command: postgres -c config_file=/etc/postgresql/postgresql.conf
    volumes:
      - ./postgres_data:/var/lib/postgresql/data
      - /opt/nodi/postgres/postgresql.conf:/etc/postgresql/postgresql.conf:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U nodi_admin -d nodi"]
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 30s
    deploy:
      resources:
        limits:
          memory: 3G
    logging: *default-logging

  nodi-api:
    image: nodi-axum-api:latest
    container_name: nodi-api
    restart: unless-stopped
    depends_on:
      nodi-postgres:
        condition: service_healthy
    user: "1000:1000"
    ports:
      - "127.0.0.1:3000:3000"
    deploy:
      resources:
        limits:
          memory: 1G
    logging: *default-logging

  nodi-web:
    image: nodi-nuxt-web:latest
    container_name: nodi-web
    restart: unless-stopped
    deploy:
      resources:
        limits:
          memory: 1G
    logging: *default-logging

  nodi-nginx:
    image: nginx:alpine
    container_name: nodi-nginx
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - /opt/nodi/nginx/nginx.conf:/etc/nginx/nginx.conf:ro
      - /opt/nodi/nginx/ssl:/etc/nginx/certs:ro
    depends_on:
      - nodi-api
      - nodi-web
    logging: *default-logging
```

---

## 8. Disaster Recovery Playbook

### RTO / RPO với Offline-First

| Chỉ số | Giá trị | Lý do |
|--------|:-------:|-------|
| **RTO** | **35-45 phút** | Provision VPS mới + restore + DNS switch |
| **RPO** | 24 giờ (backup 3h sáng) | Nhưng **effective data loss ≈ 0** nhờ Tauri resync |

> VPS chết → Khách vẫn bán hàng offline → VPS mới → Tauri tự sync lại → 0 mất dữ liệu

### 4 Giai Đoạn Phục Hồi

| GĐ | Thao tác | RTO |
|:--:|----------|:---:|
| 1 | Provision VPS mới (Vultr/DO) + cài Docker + AWS CLI | 15 phút |
| 2 | Tải backup từ R2 + GPG decrypt (private key từ máy cá nhân) | 10 phút |
| 3 | `docker-compose up -d` + pg_restore | 10 phút |
| 4 | Cloudflare DNS → IP mới | 5 phút |

---

## Tổng Kết — Checklist Triển Khai

```
TRƯỚC KHI RA MẮT:
  [ ] PostgreSQL tuning (shared_buffers 2GB, work_mem 20MB)
  [ ] Nginx security headers (HSTS, X-Frame, nosniff)
  [ ] SSH hardening (port 22222, key-only, no root)
  [ ] Fail2ban (SSH + Nginx jails)
  [ ] Netdata cài trên host
  [ ] BetterStack monitor nodi.vn
  [ ] Telegram Bot alerting
  [ ] GPG key pair tạo + import
  [ ] R2 backup script + crontab
  [ ] R2 verification script
  [ ] Docker log rotation (10m × 3)
  [ ] Docker health checks + memory limits
  [ ] unattended-upgrades enabled
  [ ] SSL cert check script

SAU KHI RA MẮT:
  [ ] Test DR playbook (restore từ R2 thực tế)
  [ ] Monitor pg_stat_statements cho slow queries
  [ ] Review Fail2ban bans hàng tuần
```

---

*Nghiên cứu bởi Google DeepSearch (Gemini) — 13/03/2026*
*Dựa trên báo cáo thực tế từ VPS Agent — Contabo srv1377210*
