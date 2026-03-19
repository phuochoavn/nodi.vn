#!/bin/bash
# Nodi POS — Daily PostgreSQL Backup
# Runs via cron: 0 2 * * * /opt/nodi/scripts/backup_db.sh

set -euo pipefail

BACKUP_DIR="/opt/nodi/backups/db"
DAYS_KEEP=14
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/nodi_db_$DATE.sql.gz"

# Create backup directory if missing
mkdir -p "$BACKUP_DIR"

# Dump database (compressed)
docker exec nodi-postgres pg_dump -U nodi_admin -d nodi | gzip > "$BACKUP_FILE"

# Check if backup was created successfully
if [ -f "$BACKUP_FILE" ] && [ -s "$BACKUP_FILE" ]; then
    SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
    echo "[$(date)] ✅ Backup created: $BACKUP_FILE ($SIZE)"
else
    echo "[$(date)] ❌ Backup FAILED!" >&2
    exit 1
fi

# Cleanup old backups (older than $DAYS_KEEP days)
find "$BACKUP_DIR" -name "nodi_db_*.sql.gz" -mtime +$DAYS_KEEP -delete
echo "[$(date)] 🧹 Cleaned backups older than $DAYS_KEEP days"

# === Off-site backup to Google Cloud Storage ===
if command -v rclone &> /dev/null; then
    rclone copy "$BACKUP_DIR/" gcs-nodi:nodi-pos-backups/db/ \
        --log-file=/opt/nodi/logs/rclone.log \
        --log-level=NOTICE
    echo "[$(date)] ☁️ Off-site backup synced to GCS (nodi-pos-backups/db/)"
else
    echo "[$(date)] ⚠️ rclone not found, skipping off-site backup"
fi
