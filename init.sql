-- Nodi Platform Database Schema

-- Bảng stores (cửa hàng)
CREATE TABLE stores (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    license_key VARCHAR(50) UNIQUE NOT NULL,
    owner_name VARCHAR(255),
    phone VARCHAR(20),
    address TEXT,
    province VARCHAR(100),
    district VARCHAR(100),
    license_type VARCHAR(50) DEFAULT 'basic',  -- basic, pro, lifetime
    license_expires_at TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Bảng users (đăng nhập web)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    store_id INTEGER REFERENCES stores(id),
    phone VARCHAR(20) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) DEFAULT 'store_owner',  -- store_owner, admin
    last_login_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Bảng backup_files
CREATE TABLE backup_files (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    file_path TEXT NOT NULL,
    backup_type VARCHAR(20) DEFAULT 'auto',  -- auto, manual
    created_at TIMESTAMP DEFAULT NOW()
);

-- Synced data tables (data từ cửa hàng sync lên)
CREATE TABLE synced_products (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    barcode TEXT,
    category TEXT,
    manufacturer TEXT,
    description TEXT,
    stock_quantity DECIMAL(10,2) DEFAULT 0,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

CREATE TABLE synced_customers (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    phone TEXT,
    address TEXT,
    tax_code TEXT,
    total_debt DECIMAL(15,2) DEFAULT 0,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

CREATE TABLE synced_invoices (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id INTEGER NOT NULL,
    invoice_number TEXT,
    customer_local_id INTEGER,
    total_amount DECIMAL(15,2) DEFAULT 0,
    payment_method TEXT,
    payment_status TEXT,
    created_at TIMESTAMP,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

CREATE TABLE synced_invoice_items (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    invoice_local_id INTEGER NOT NULL,
    product_local_id INTEGER NOT NULL,
    product_name TEXT,
    quantity DECIMAL(10,2),
    unit_name TEXT,
    unit_price DECIMAL(15,2),
    total DECIMAL(15,2),
    synced_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE synced_suppliers (
    id SERIAL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES stores(id),
    local_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    phone TEXT,
    address TEXT,
    company TEXT,
    total_debt DECIMAL(15,2) DEFAULT 0,
    synced_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(store_id, local_id)
);

-- Indexes cho Market Intelligence queries
CREATE INDEX idx_synced_invoices_store_created ON synced_invoices(store_id, created_at);
CREATE INDEX idx_synced_products_category ON synced_products(store_id, category);
CREATE INDEX idx_synced_invoice_items_product ON synced_invoice_items(store_id, product_local_id);
CREATE INDEX idx_stores_province ON stores(province);
CREATE INDEX idx_stores_license_key ON stores(license_key);

-- Admin user (tạo sau khi có password hash)
-- INSERT INTO users (phone, password_hash, role) VALUES ('admin_phone', 'hash', 'admin');

-- E-Invoice & Tax Config tables (Sprint: Kế toán & Thuế)
CREATE TABLE IF NOT EXISTS einvoice_config (
    store_id INTEGER PRIMARY KEY,
    provider VARCHAR(50) DEFAULT 'none',
    api_key VARCHAR(500),
    api_secret VARCHAR(500),
    tax_code VARCHAR(20),
    series_symbol VARCHAR(20),
    is_active BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tax_config (
    store_id INTEGER PRIMARY KEY,
    business_type VARCHAR(50) DEFAULT 'retail',
    vat_rate NUMERIC(5,2) DEFAULT 1.0,
    pit_rate NUMERIC(5,2) DEFAULT 0.5,
    tax_period VARCHAR(20) DEFAULT 'quarterly',
    updated_at TIMESTAMP DEFAULT NOW()
);
