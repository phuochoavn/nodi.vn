-- Sprint 89 Migration: Staff Permission Management (Sync from Desktop)
-- Date: 2026-03-03
-- Run: docker cp migration_sprint89.sql nodi-postgres:/tmp/ && docker exec nodi-postgres psql -U nodi_admin -d nodi -f /tmp/migration_sprint89.sql

-- ============================================================
-- 1. sync_staff_members — synced from desktop app users table
-- ============================================================
CREATE TABLE IF NOT EXISTS sync_staff_members (
    id BIGINT NOT NULL,
    store_id INTEGER NOT NULL,                   -- data_store_id from sync (NOT FK to stores)
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'staff',   -- 'owner' or 'staff'
    pin TEXT,                             -- PIN 4 digits (private, never exposed via web API)
    permissions JSONB DEFAULT '{}',       -- 9 permission flags
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    synced_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (store_id, id)
);

CREATE INDEX IF NOT EXISTS idx_sync_staff_store ON sync_staff_members(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_staff_role ON sync_staff_members(store_id, role);
