#!/bin/bash
# PostgreSQL daily backup script for Nodi
# Runs via cron at 3:00 AM daily

BACKUP_DIR="/opt/nodi/pg_backups"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p "$BACKUP_DIR"

# Backup via docker exec
docker exec nodi-postgres pg_dump -U nodi_admin nodi > "$BACKUP_DIR/nodi_$DATE.sql"

# Check if backup succeeded
if [ $? -eq 0 ]; then
    SIZE=$(du -sh "$BACKUP_DIR/nodi_$DATE.sql" | cut -f1)
    echo "[$(date)] ✅ Backup done: nodi_$DATE.sql ($SIZE)"
else
    echo "[$(date)] ❌ Backup FAILED!"
    exit 1
fi

# Rotation: keep only 7 most recent backups
ls -t "$BACKUP_DIR"/nodi_*.sql | tail -n +8 | xargs -r rm
echo "[$(date)] 🗑️ Old backups cleaned. Current count: $(ls "$BACKUP_DIR"/nodi_*.sql | wc -l)"
