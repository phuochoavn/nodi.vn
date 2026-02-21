-- Sprint 10 Migration: Employees + Store Groups
-- Date: 2026-02-21
-- Run: docker exec nodi-postgres psql -U nodi_admin -d nodi -f /tmp/migration.sql

-- ============================================================
-- 1. Employees table
-- ============================================================
CREATE TABLE IF NOT EXISTS employees (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    name VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    pin VARCHAR(10) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'cashier',
    is_active BOOLEAN DEFAULT true,
    permissions JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, pin)
);

CREATE INDEX IF NOT EXISTS idx_employees_store ON employees(store_id);
CREATE INDEX IF NOT EXISTS idx_employees_phone ON employees(store_id, phone);

-- ============================================================
-- 2. Store Groups table (multi-branch management)
-- ============================================================
CREATE TABLE IF NOT EXISTS store_groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    owner_user_id INTEGER REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Add group/branch columns to stores
ALTER TABLE stores ADD COLUMN IF NOT EXISTS group_id INTEGER REFERENCES store_groups(id);
ALTER TABLE stores ADD COLUMN IF NOT EXISTS branch_name VARCHAR(255);

CREATE INDEX IF NOT EXISTS idx_stores_group ON stores(group_id);
