#!/bin/bash
# Sprint 167: Zero-downtime deploy with docker-rollout
# Usage: ./scripts/deploy.sh [--skip-pull]
set -euo pipefail

DEPLOY_DIR="/opt/nodi"
HEALTH_URL="http://localhost:3000/api/health"
HEALTH_RETRIES=5
HEALTH_DELAY=3
LOG_FILE="/opt/nodi/logs/deploy-$(date +%Y%m%d-%H%M%S).log"

log() { echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG_FILE"; }

cd "$DEPLOY_DIR"
mkdir -p logs

log "🚀 Deploy started"

# 1. Pull latest code (skip with --skip-pull)
if [[ "${1:-}" != "--skip-pull" ]]; then
    log "📥 Pulling latest code..."
    git pull origin main 2>&1 | tee -a "$LOG_FILE"
fi

# 2. Build new image
log "🔨 Building axum-api image..."
docker compose build axum-api 2>&1 | tee -a "$LOG_FILE"

# 3. Zero-downtime rollout via docker-rollout
log "🔄 Rolling out axum-api (zero-downtime)..."
docker rollout -f docker-compose.yml axum-api 2>&1 | tee -a "$LOG_FILE"

# 4. Health check verification
log "🏥 Verifying health..."
HEALTHY=false
for i in $(seq 1 $HEALTH_RETRIES); do
    sleep $HEALTH_DELAY
    if curl -sf "$HEALTH_URL" > /dev/null 2>&1; then
        HEALTHY=true
        log "✅ Health check passed (attempt $i/$HEALTH_RETRIES)"
        break
    fi
    log "⏳ Health check attempt $i/$HEALTH_RETRIES failed, retrying..."
done

# 5. Rollback if unhealthy
if [ "$HEALTHY" = false ]; then
    log "❌ Health check failed after $HEALTH_RETRIES attempts"
    log "🔙 Rolling back..."
    docker compose up -d --force-recreate axum-api 2>&1 | tee -a "$LOG_FILE"
    
    # Verify rollback
    sleep 5
    if curl -sf "$HEALTH_URL" > /dev/null 2>&1; then
        log "↩️ Rollback successful"
    else
        log "🆘 CRITICAL: Rollback also failed!"
    fi
    exit 1
fi

# 6. Cleanup old images
docker image prune -f 2>&1 | tee -a "$LOG_FILE" || true

log "✅ Deploy completed successfully"
log "📊 Container status:"
docker compose ps axum-api 2>&1 | tee -a "$LOG_FILE"
