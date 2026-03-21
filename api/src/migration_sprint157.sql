-- ============================================================
-- Sprint 157: Backfill + Cutover
-- Run MANUALLY via psql AFTER migration_sprint156.sql
-- ============================================================

-- ══════════════════════════════════════════════════════════════
-- Step 1: Backfill old → new (partitioned)
-- Each INSERT reads from _old and inserts into the partitioned
-- table, which auto-routes to the correct monthly partition.
-- ══════════════════════════════════════════════════════════════

-- 1/6: synced_invoices
INSERT INTO synced_invoices SELECT * FROM synced_invoices_old;

-- 2/6: synced_invoice_items
INSERT INTO synced_invoice_items SELECT * FROM synced_invoice_items_old;

-- 3/6: synced_customer_transactions
INSERT INTO synced_customer_transactions SELECT * FROM synced_customer_transactions_old;

-- 4/6: synced_supplier_transactions
INSERT INTO synced_supplier_transactions SELECT * FROM synced_supplier_transactions_old;

-- 5/6: synced_product_transactions
INSERT INTO synced_product_transactions SELECT * FROM synced_product_transactions_old;

-- 6/6: synced_cash_transactions
INSERT INTO synced_cash_transactions SELECT * FROM synced_cash_transactions_old;

-- ══════════════════════════════════════════════════════════════
-- Step 2: Verify row counts match
-- Run these manually and compare!
-- ══════════════════════════════════════════════════════════════

SELECT 'synced_invoices' AS tbl,
  (SELECT COUNT(*) FROM synced_invoices_old) AS old_count,
  (SELECT COUNT(*) FROM synced_invoices) AS new_count;

SELECT 'synced_invoice_items' AS tbl,
  (SELECT COUNT(*) FROM synced_invoice_items_old) AS old_count,
  (SELECT COUNT(*) FROM synced_invoice_items) AS new_count;

SELECT 'synced_customer_transactions' AS tbl,
  (SELECT COUNT(*) FROM synced_customer_transactions_old) AS old_count,
  (SELECT COUNT(*) FROM synced_customer_transactions) AS new_count;

SELECT 'synced_supplier_transactions' AS tbl,
  (SELECT COUNT(*) FROM synced_supplier_transactions_old) AS old_count,
  (SELECT COUNT(*) FROM synced_supplier_transactions) AS new_count;

SELECT 'synced_product_transactions' AS tbl,
  (SELECT COUNT(*) FROM synced_product_transactions_old) AS old_count,
  (SELECT COUNT(*) FROM synced_product_transactions) AS new_count;

SELECT 'synced_cash_transactions' AS tbl,
  (SELECT COUNT(*) FROM synced_cash_transactions_old) AS old_count,
  (SELECT COUNT(*) FROM synced_cash_transactions) AS new_count;

-- ══════════════════════════════════════════════════════════════
-- Step 3: Drop old tables (ONLY after verifying counts match!)
-- ══════════════════════════════════════════════════════════════

-- ⚠️ UNCOMMENT THESE ONLY AFTER VERIFYING COUNTS ABOVE ⚠️

-- DROP TABLE IF EXISTS synced_invoices_old CASCADE;
-- DROP TABLE IF EXISTS synced_invoice_items_old CASCADE;
-- DROP TABLE IF EXISTS synced_customer_transactions_old CASCADE;
-- DROP TABLE IF EXISTS synced_supplier_transactions_old CASCADE;
-- DROP TABLE IF EXISTS synced_product_transactions_old CASCADE;
-- DROP TABLE IF EXISTS synced_cash_transactions_old CASCADE;

-- ══════════════════════════════════════════════════════════════
-- Step 4: Run initial maintenance
-- ══════════════════════════════════════════════════════════════

SELECT partman.run_maintenance_proc();

-- ══════════════════════════════════════════════════════════════
-- Step 5: Verify partitions exist
-- ══════════════════════════════════════════════════════════════

SELECT parent_table, partition_interval, premake
FROM partman.part_config
ORDER BY parent_table;

-- List child partitions
SELECT inhrelid::regclass AS partition_name
FROM pg_inherits
WHERE inhparent = 'synced_invoices'::regclass
ORDER BY partition_name;
