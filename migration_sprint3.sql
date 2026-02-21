-- Sprint 3 Migration: Sync + Backup tables
-- =========================================

-- Cập nhật synced_customers
ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS credit_limit DOUBLE PRECISION;
ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS cccd TEXT;
ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS cccd_front_img TEXT;
ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS cccd_back_img TEXT;

-- Cập nhật synced_products
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS sku TEXT;
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS base_unit TEXT;
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS cost_price DOUBLE PRECISION;
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS sell_price DOUBLE PRECISION;
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS expiry_date TEXT;
ALTER TABLE synced_products ADD COLUMN IF NOT EXISTS updated_at TEXT;

-- Cập nhật synced_invoices
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS customer_id BIGINT;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS discount_amount DOUBLE PRECISION DEFAULT 0;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS final_amount DOUBLE PRECISION DEFAULT 0;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS customer_name TEXT;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS customer_phone TEXT;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS status TEXT;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS customer_pay DOUBLE PRECISION DEFAULT 0;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS change_money DOUBLE PRECISION DEFAULT 0;
ALTER TABLE synced_invoices ADD COLUMN IF NOT EXISTS notes TEXT;

-- Cập nhật synced_invoice_items
ALTER TABLE synced_invoice_items ADD COLUMN IF NOT EXISTS local_id BIGINT;
ALTER TABLE synced_invoice_items ADD COLUMN IF NOT EXISTS product_sku TEXT;
ALTER TABLE synced_invoice_items ADD COLUMN IF NOT EXISTS exchange_value DOUBLE PRECISION DEFAULT 1;

-- Purchase Orders
CREATE TABLE IF NOT EXISTS synced_purchase_orders (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    supplier_id BIGINT,
    supplier_name TEXT,
    total_amount DOUBLE PRECISION DEFAULT 0,
    status TEXT DEFAULT 'COMPLETED',
    import_date TEXT,
    is_tax_invoice BOOLEAN DEFAULT false,
    invoice_type TEXT DEFAULT 'OFFICIAL',
    note TEXT,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

CREATE TABLE IF NOT EXISTS synced_purchase_items (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    purchase_order_id BIGINT NOT NULL,
    local_id BIGINT NOT NULL,
    product_id BIGINT,
    product_name TEXT,
    unit TEXT,
    exchange_value DOUBLE PRECISION DEFAULT 1,
    quantity DOUBLE PRECISION DEFAULT 0,
    import_price DOUBLE PRECISION DEFAULT 0,
    subtotal DOUBLE PRECISION DEFAULT 0,
    expiry_date TEXT,
    batch_number TEXT,
    UNIQUE(store_id, local_id)
);

-- Customer Transactions
CREATE TABLE IF NOT EXISTS synced_customer_transactions (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    customer_id BIGINT,
    amount DOUBLE PRECISION DEFAULT 0,
    transaction_type TEXT,
    note TEXT,
    season TEXT,
    invoice_id BIGINT,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Supplier Transactions
CREATE TABLE IF NOT EXISTS synced_supplier_transactions (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    supplier_id BIGINT,
    amount DOUBLE PRECISION DEFAULT 0,
    ref_type TEXT,
    ref_id BIGINT,
    note TEXT,
    balance_after DOUBLE PRECISION DEFAULT 0,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Product Units
CREATE TABLE IF NOT EXISTS synced_product_units (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    product_id BIGINT,
    unit_name TEXT,
    exchange_value DOUBLE PRECISION DEFAULT 1,
    price DOUBLE PRECISION DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TEXT,
    UNIQUE(store_id, local_id)
);

-- Store Funds
CREATE TABLE IF NOT EXISTS synced_store_funds (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id) UNIQUE,
    current_balance BIGINT DEFAULT 0,
    updated_at TEXT
);

-- Cash Transactions
CREATE TABLE IF NOT EXISTS synced_cash_transactions (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    amount BIGINT DEFAULT 0,
    flow_type TEXT,
    category TEXT,
    ref_id BIGINT,
    balance_after BIGINT DEFAULT 0,
    note TEXT,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Product Batches
CREATE TABLE IF NOT EXISTS synced_product_batches (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    product_id BIGINT,
    purchase_item_id BIGINT,
    expiry_date TEXT,
    import_date TEXT,
    quantity DOUBLE PRECISION DEFAULT 0,
    remaining_quantity DOUBLE PRECISION DEFAULT 0,
    created_at TEXT,
    updated_at TEXT,
    UNIQUE(store_id, local_id)
);

-- Payment Vouchers
CREATE TABLE IF NOT EXISTS synced_payment_vouchers (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    voucher_code TEXT,
    supplier_id BIGINT,
    amount BIGINT DEFAULT 0,
    payment_method TEXT,
    note TEXT,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Store Settings
CREATE TABLE IF NOT EXISTS synced_store_settings (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id) UNIQUE,
    name TEXT,
    address TEXT,
    phone TEXT,
    security_pin TEXT,
    updated_at TEXT
);

-- Product Transactions
CREATE TABLE IF NOT EXISTS synced_product_transactions (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id BIGINT NOT NULL,
    product_id BIGINT,
    transaction_type TEXT,
    quantity DOUBLE PRECISION DEFAULT 0,
    reference_type TEXT,
    reference_id BIGINT,
    note TEXT,
    balance_after DOUBLE PRECISION DEFAULT 0,
    created_at TEXT,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Invoice Payments
CREATE TABLE IF NOT EXISTS synced_invoice_payments (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    invoice_local_id BIGINT NOT NULL,
    amount DOUBLE PRECISION DEFAULT 0,
    payment_method TEXT
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_sync_customers_store ON synced_customers(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_products_store ON synced_products(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_invoices_store ON synced_invoices(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_po_store ON synced_purchase_orders(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_cust_txn_store ON synced_customer_transactions(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_cash_txn_store ON synced_cash_transactions(store_id);
CREATE INDEX IF NOT EXISTS idx_sync_prod_txn_store ON synced_product_transactions(store_id);
