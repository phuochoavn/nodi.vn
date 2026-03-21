-- ============================================================
-- Sprint 156: pg_partman Setup + Partition Schema
-- Run MANUALLY via psql — NOT auto-run on startup
-- Prerequisite: pg_partman extension available in PG image
-- ============================================================

-- ══════════════════════════════════════════════════════════════
-- Step 0: Install pg_partman
-- ══════════════════════════════════════════════════════════════

CREATE EXTENSION IF NOT EXISTS pg_partman;

-- ══════════════════════════════════════════════════════════════
-- Step 1: Add partition_key column to all 6 tables
-- ══════════════════════════════════════════════════════════════

ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE synced_invoice_items ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE synced_customer_transactions ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE synced_supplier_transactions ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE synced_product_transactions ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE synced_cash_transactions ADD COLUMN IF NOT EXISTS partition_key TIMESTAMP NOT NULL DEFAULT NOW();

-- ══════════════════════════════════════════════════════════════
-- Step 2: Backfill partition_key from created_at / synced_at
-- ══════════════════════════════════════════════════════════════

-- synced_invoices: created_at is TIMESTAMP
UPDATE synced_invoices SET partition_key = COALESCE(created_at, synced_at, NOW());

-- synced_invoice_items: no created_at originally, use synced_at
UPDATE synced_invoice_items SET partition_key = COALESCE(synced_at, NOW());

-- synced_customer_transactions: created_at is TEXT
UPDATE synced_customer_transactions SET partition_key = CASE
  WHEN created_at IS NOT NULL AND created_at != '' THEN created_at::timestamp
  ELSE COALESCE(synced_at, NOW())
END;

-- synced_supplier_transactions: created_at is TEXT
UPDATE synced_supplier_transactions SET partition_key = CASE
  WHEN created_at IS NOT NULL AND created_at != '' THEN created_at::timestamp
  ELSE COALESCE(synced_at, NOW())
END;

-- synced_product_transactions: created_at is TEXT
UPDATE synced_product_transactions SET partition_key = CASE
  WHEN created_at IS NOT NULL AND created_at != '' THEN created_at::timestamp
  ELSE COALESCE(synced_at, NOW())
END;

-- synced_cash_transactions: created_at is TEXT
UPDATE synced_cash_transactions SET partition_key = CASE
  WHEN created_at IS NOT NULL AND created_at != '' THEN created_at::timestamp
  ELSE COALESCE(synced_at, NOW())
END;

-- ══════════════════════════════════════════════════════════════
-- Step 3: Convert each table to Range Partitioned
-- Pattern: RENAME old → CREATE new partitioned → pg_partman
-- ══════════════════════════════════════════════════════════════

-- ──────────────────────────────────────────────────────────────
-- 1/6: synced_invoices
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_invoices RENAME TO synced_invoices_old;
ALTER INDEX IF EXISTS synced_invoices_pkey RENAME TO synced_invoices_old_pkey;
ALTER INDEX IF EXISTS synced_invoices_store_id_local_id_key RENAME TO synced_invoices_old_store_local;
ALTER INDEX IF EXISTS idx_synced_invoices_uuid RENAME TO idx_synced_invoices_old_uuid;

CREATE TABLE synced_invoices (LIKE synced_invoices_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_invoices DROP CONSTRAINT IF EXISTS synced_invoices_old_pkey;
ALTER TABLE synced_invoices ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_invoices_uuid
  ON synced_invoices (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_invoices',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_invoices_store_partkey
  ON synced_invoices (store_id, partition_key DESC);

-- ──────────────────────────────────────────────────────────────
-- 2/6: synced_invoice_items
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_invoice_items RENAME TO synced_invoice_items_old;
ALTER INDEX IF EXISTS synced_invoice_items_pkey RENAME TO synced_invoice_items_old_pkey;
ALTER INDEX IF EXISTS idx_synced_invoice_items_uuid RENAME TO idx_synced_invoice_items_old_uuid;

CREATE TABLE synced_invoice_items (LIKE synced_invoice_items_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_invoice_items DROP CONSTRAINT IF EXISTS synced_invoice_items_old_pkey;
ALTER TABLE synced_invoice_items ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_invoice_items_uuid
  ON synced_invoice_items (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_invoice_items',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_invoice_items_store_partkey
  ON synced_invoice_items (store_id, partition_key DESC);

-- ──────────────────────────────────────────────────────────────
-- 3/6: synced_customer_transactions
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_customer_transactions RENAME TO synced_customer_transactions_old;
ALTER INDEX IF EXISTS synced_customer_transactions_pkey RENAME TO synced_customer_transactions_old_pkey;
ALTER INDEX IF EXISTS synced_customer_transactions_store_id_local_id_key RENAME TO synced_customer_transactions_old_store_local;
ALTER INDEX IF EXISTS idx_synced_customer_transactions_uuid RENAME TO idx_synced_customer_transactions_old_uuid;

CREATE TABLE synced_customer_transactions (LIKE synced_customer_transactions_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_customer_transactions DROP CONSTRAINT IF EXISTS synced_customer_transactions_old_pkey;
ALTER TABLE synced_customer_transactions ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_customer_transactions_uuid
  ON synced_customer_transactions (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_customer_transactions',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_customer_tx_store_partkey
  ON synced_customer_transactions (store_id, partition_key DESC);

-- ──────────────────────────────────────────────────────────────
-- 4/6: synced_supplier_transactions
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_supplier_transactions RENAME TO synced_supplier_transactions_old;
ALTER INDEX IF EXISTS synced_supplier_transactions_pkey RENAME TO synced_supplier_transactions_old_pkey;
ALTER INDEX IF EXISTS synced_supplier_transactions_store_id_local_id_key RENAME TO synced_supplier_transactions_old_store_local;
ALTER INDEX IF EXISTS idx_synced_supplier_transactions_uuid RENAME TO idx_synced_supplier_transactions_old_uuid;

CREATE TABLE synced_supplier_transactions (LIKE synced_supplier_transactions_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_supplier_transactions DROP CONSTRAINT IF EXISTS synced_supplier_transactions_old_pkey;
ALTER TABLE synced_supplier_transactions ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_supplier_transactions_uuid
  ON synced_supplier_transactions (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_supplier_transactions',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_supplier_tx_store_partkey
  ON synced_supplier_transactions (store_id, partition_key DESC);

-- ──────────────────────────────────────────────────────────────
-- 5/6: synced_product_transactions
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_product_transactions RENAME TO synced_product_transactions_old;
ALTER INDEX IF EXISTS synced_product_transactions_pkey RENAME TO synced_product_transactions_old_pkey;
ALTER INDEX IF EXISTS synced_product_transactions_store_id_local_id_key RENAME TO synced_product_transactions_old_store_local;
ALTER INDEX IF EXISTS idx_synced_product_transactions_uuid RENAME TO idx_synced_product_transactions_old_uuid;

CREATE TABLE synced_product_transactions (LIKE synced_product_transactions_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_product_transactions DROP CONSTRAINT IF EXISTS synced_product_transactions_old_pkey;
ALTER TABLE synced_product_transactions ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_product_transactions_uuid
  ON synced_product_transactions (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_product_transactions',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_product_tx_store_partkey
  ON synced_product_transactions (store_id, partition_key DESC);

-- ──────────────────────────────────────────────────────────────
-- 6/6: synced_cash_transactions
-- ──────────────────────────────────────────────────────────────

ALTER TABLE synced_cash_transactions RENAME TO synced_cash_transactions_old;
ALTER INDEX IF EXISTS synced_cash_transactions_pkey RENAME TO synced_cash_transactions_old_pkey;
ALTER INDEX IF EXISTS synced_cash_transactions_store_id_local_id_key RENAME TO synced_cash_transactions_old_store_local;
ALTER INDEX IF EXISTS idx_synced_cash_transactions_uuid RENAME TO idx_synced_cash_transactions_old_uuid;

CREATE TABLE synced_cash_transactions (LIKE synced_cash_transactions_old INCLUDING DEFAULTS INCLUDING CONSTRAINTS)
  PARTITION BY RANGE (partition_key);

ALTER TABLE synced_cash_transactions DROP CONSTRAINT IF EXISTS synced_cash_transactions_old_pkey;
ALTER TABLE synced_cash_transactions ADD PRIMARY KEY (id, partition_key);

CREATE UNIQUE INDEX IF NOT EXISTS idx_synced_cash_transactions_uuid
  ON synced_cash_transactions (store_id, uuid) WHERE uuid IS NOT NULL;

SELECT partman.create_parent(
  p_parent_table := 'public.synced_cash_transactions',
  p_control := 'partition_key',
  p_interval := '1 month',
  p_premake := 3
);

CREATE INDEX IF NOT EXISTS idx_cash_tx_store_partkey
  ON synced_cash_transactions (store_id, partition_key DESC);

-- ══════════════════════════════════════════════════════════════
-- Step 4: Re-apply RLS policies on new partitioned tables
-- (Sprint 153 policies were on old tables)
-- ══════════════════════════════════════════════════════════════

DO $$
DECLARE
  tbl TEXT;
BEGIN
  FOR tbl IN SELECT unnest(ARRAY[
    'synced_invoices', 'synced_invoice_items',
    'synced_customer_transactions', 'synced_supplier_transactions',
    'synced_product_transactions', 'synced_cash_transactions'
  ])
  LOOP
    EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', tbl);
    EXECUTE format('ALTER TABLE %I FORCE ROW LEVEL SECURITY', tbl);
    EXECUTE format('DROP POLICY IF EXISTS tenant_isolation ON %I', tbl);
    EXECUTE format(
      'CREATE POLICY tenant_isolation ON %I FOR ALL USING ('
      'current_setting(''app.current_store_id'', true) = '''' '
      'OR store_id = current_setting(''app.current_store_id'')::integer)', tbl
    );
  END LOOP;
END $$;

-- ══════════════════════════════════════════════════════════════
-- Done Sprint 156! Next: run migration_sprint157.sql
-- ══════════════════════════════════════════════════════════════
