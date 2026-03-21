#!/bin/bash
# Sprint 160: pgBackRest Backup Cron Script
# Install: crontab -e, add:
#   0 2 * * * /opt/nodi/scripts/backup-cron.sh incr >> /opt/nodi/logs/backup.log 2>&1
#   0 3 * * 0 /opt/nodi/scripts/backup-cron.sh full >> /opt/nodi/logs/backup.log 2>&1

set -euo pipefail
BACKUP_TYPE=${1:-incr}
CONTAINER=nodi-postgres
STANZA=nodi_cluster

echo "$(date '+%Y-%m-%d %H:%M:%S') — Starting $BACKUP_TYPE backup..."
docker exec $CONTAINER pgbackrest --stanza=$STANZA --type=$BACKUP_TYPE backup
echo "$(date '+%Y-%m-%d %H:%M:%S') — $BACKUP_TYPE backup complete."
docker exec $CONTAINER pgbackrest --stanza=$STANZA info
