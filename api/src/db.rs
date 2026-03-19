use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}

/// Seed admin user on first run if ADMIN_PASSWORD env var is set
pub async fn seed_admin(pool: &PgPool) {
    let password = match std::env::var("ADMIN_PASSWORD") {
        Ok(p) => p,
        Err(_) => return, // No ADMIN_PASSWORD set, skip seeding
    };

    let admin_phone = std::env::var("ADMIN_PHONE").unwrap_or_else(|_| "0374222326".to_string());
    let hash = bcrypt::hash(&password, 12).expect("Failed to hash admin password");

    // Create admin store if not exists
    sqlx::query(
        "INSERT INTO stores (name, license_key, owner_name, phone, license_type, is_active) \
         VALUES ('Nodi Admin', 'ADMIN-MASTER-KEY', 'Admin', $1, 'lifetime', true) \
         ON CONFLICT (license_key) DO NOTHING"
    )
    .bind(&admin_phone)
    .execute(pool)
    .await
    .ok();

    // Get admin store id
    let store_id: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM stores WHERE license_key = 'ADMIN-MASTER-KEY'"
    )
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    let store_id = match store_id {
        Some(id) => id,
        None => {
            tracing::error!("Failed to find admin store");
            return;
        }
    };

    // Upsert admin user — always update password hash and phone
    sqlx::query(
        "INSERT INTO users (store_id, phone, password_hash, display_name, role) \
         VALUES ($1, $2, $3, 'Admin', 'admin') \
         ON CONFLICT (phone) DO UPDATE SET password_hash = $3, role = 'admin'"
    )
    .bind(store_id)
    .bind(&admin_phone)
    .bind(&hash)
    .execute(pool)
    .await
    .ok();

    tracing::info!("✅ Admin user seeded: phone={}", admin_phone);

    // Schema evolution — ensure new columns/tables exist
    sqlx::query("ALTER TABLE stores ADD COLUMN IF NOT EXISTS revoked_at TIMESTAMP")
        .execute(pool).await.ok();
    sqlx::query("ALTER TABLE stores ADD COLUMN IF NOT EXISTS duration_days INTEGER DEFAULT 30")
        .execute(pool).await.ok();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS license_payments (\
         id SERIAL PRIMARY KEY, store_id INTEGER REFERENCES stores(id), \
         amount INTEGER NOT NULL, payment_method TEXT DEFAULT 'BANK_TRANSFER', \
         period_start DATE, period_end DATE, note TEXT, created_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();

    // Support chat tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS support_tickets (\
         id SERIAL PRIMARY KEY, store_id INTEGER REFERENCES stores(id), \
         license_key TEXT NOT NULL, store_name TEXT, phone TEXT, \
         subject TEXT NOT NULL, status TEXT DEFAULT 'open' CHECK(status IN ('open','in_progress','resolved','closed')), \
         created_at TIMESTAMP DEFAULT NOW(), updated_at TIMESTAMP DEFAULT NOW(), resolved_at TIMESTAMP)"
    ).execute(pool).await.ok();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS support_messages (\
         id SERIAL PRIMARY KEY, ticket_id INTEGER REFERENCES support_tickets(id) ON DELETE CASCADE, \
         sender_type TEXT NOT NULL CHECK(sender_type IN ('customer','admin')), \
         sender_name TEXT, message TEXT NOT NULL, created_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tickets_status ON support_tickets(status)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tickets_license ON support_tickets(license_key)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_messages_ticket ON support_messages(ticket_id)").execute(pool).await.ok();

    // v2: closed_by / closed_at columns + system sender_type
    sqlx::query("ALTER TABLE support_tickets ADD COLUMN IF NOT EXISTS closed_by TEXT").execute(pool).await.ok();
    sqlx::query("ALTER TABLE support_tickets ADD COLUMN IF NOT EXISTS closed_at TIMESTAMP").execute(pool).await.ok();
    sqlx::query("ALTER TABLE support_messages DROP CONSTRAINT IF EXISTS support_messages_sender_type_check").execute(pool).await.ok();
    sqlx::query("ALTER TABLE support_messages ADD CONSTRAINT support_messages_sender_type_check CHECK(sender_type IN ('customer','admin','system'))").execute(pool).await.ok();

    // v3: read receipts
    sqlx::query("ALTER TABLE support_messages ADD COLUMN IF NOT EXISTS read_at TIMESTAMP").execute(pool).await.ok();

    // v4: audit log
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS audit_log (\
         id SERIAL PRIMARY KEY, action TEXT NOT NULL, actor TEXT NOT NULL, \
         target_type TEXT, target_id TEXT, details TEXT, ip TEXT, \
         created_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_created ON audit_log(created_at DESC)").execute(pool).await.ok();

    // v5: notifications
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS notifications (\
         id SERIAL PRIMARY KEY, title TEXT NOT NULL, message TEXT NOT NULL, \
         target_type TEXT DEFAULT 'all', target_id TEXT, \
         created_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();

    // v6: app version tracking
    sqlx::query("ALTER TABLE stores ADD COLUMN IF NOT EXISTS app_version TEXT").execute(pool).await.ok();
    sqlx::query("ALTER TABLE stores ADD COLUMN IF NOT EXISTS last_sync_at TIMESTAMP").execute(pool).await.ok();

    // v7: multi-device license support
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS devices (\
         id SERIAL PRIMARY KEY, \
         store_id INTEGER NOT NULL REFERENCES stores(id) ON DELETE CASCADE, \
         device_id TEXT NOT NULL, \
         device_type TEXT NOT NULL DEFAULT 'windows', \
         device_name TEXT, \
         is_active BOOLEAN DEFAULT true, \
         first_activated_at TIMESTAMP DEFAULT NOW(), \
         last_active_at TIMESTAMP DEFAULT NOW(), \
         created_at TIMESTAMP DEFAULT NOW(), \
         UNIQUE(store_id, device_id))"
    ).execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_devices_store_id ON devices(store_id)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_devices_device_id ON devices(device_id)").execute(pool).await.ok();

    // Fix TIMESTAMPTZ → TIMESTAMP if table was created with wrong type
    sqlx::query("ALTER TABLE devices ALTER COLUMN first_activated_at TYPE TIMESTAMP USING first_activated_at::TIMESTAMP").execute(pool).await.ok();
    sqlx::query("ALTER TABLE devices ALTER COLUMN last_active_at TYPE TIMESTAMP USING last_active_at::TIMESTAMP").execute(pool).await.ok();
    sqlx::query("ALTER TABLE devices ALTER COLUMN created_at TYPE TIMESTAMP USING created_at::TIMESTAMP").execute(pool).await.ok();

    // Migrate existing HWIDs from stores to devices table
    sqlx::query(
        "INSERT INTO devices (store_id, device_id, device_type, device_name, is_active, first_activated_at) \
         SELECT id, hwid, 'windows', 'PC (migrated)', true, COALESCE(activated_at, NOW()) \
         FROM stores WHERE hwid IS NOT NULL AND hwid != '' \
         ON CONFLICT (store_id, device_id) DO NOTHING"
    ).execute(pool).await.ok();

    // v8: loyalty sync support
    sqlx::query("ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS loyalty_points INTEGER DEFAULT 0").execute(pool).await.ok();
    sqlx::query("ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS total_spent REAL DEFAULT 0").execute(pool).await.ok();
    sqlx::query("ALTER TABLE synced_customers ADD COLUMN IF NOT EXISTS loyalty_tier TEXT DEFAULT 'bronze'").execute(pool).await.ok();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS synced_loyalty_transactions (\
         id SERIAL PRIMARY KEY, store_id INTEGER NOT NULL, local_id INTEGER NOT NULL, \
         customer_id INTEGER, transaction_type TEXT NOT NULL, \
         points INTEGER NOT NULL DEFAULT 0, amount REAL DEFAULT 0, \
         invoice_id INTEGER, description TEXT, \
         created_at TEXT, synced_at TIMESTAMP DEFAULT NOW(), \
         UNIQUE(store_id, local_id))"
    ).execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_synced_loyalty_tx_store ON synced_loyalty_transactions(store_id)").execute(pool).await.ok();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS synced_loyalty_settings (\
         id SERIAL PRIMARY KEY, store_id INTEGER NOT NULL UNIQUE, \
         enabled BOOLEAN DEFAULT false, \
         points_per_amount INTEGER DEFAULT 1000, \
         point_value REAL DEFAULT 1.0, \
         bronze_threshold REAL DEFAULT 0, \
         silver_threshold REAL DEFAULT 5000000, \
         gold_threshold REAL DEFAULT 20000000, \
         diamond_threshold REAL DEFAULT 50000000, \
         updated_at TEXT, synced_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();
}

/// ─── Sync V2: Migration for merge engine tables ───
/// Called once on startup. All statements are idempotent (IF NOT EXISTS / ADD COLUMN IF NOT EXISTS).
pub async fn sync_v2_init(pool: &PgPool) {
    tracing::info!("🔄 Running Sync V2 migrations...");

    // ── 1. Add uuid, device_id, updated_at to every synced_* table ───────────
    let synced_tables = [
        "synced_customers",
        "synced_products",
        "synced_invoices",
        "synced_invoice_items",
        "synced_invoice_payments",
        "synced_suppliers",
        "synced_purchase_orders",
        "synced_purchase_items",
        "synced_customer_transactions",
        "synced_supplier_transactions",
        "synced_cash_transactions",
        "synced_product_units",
        "synced_product_batches",
        "synced_product_transactions",
        "synced_payment_vouchers",
        "synced_store_settings",
        "synced_promotions",
        "synced_vouchers",
        "synced_daily_closings",
        "synced_returns",
        "synced_return_items",
        "synced_loyalty_transactions_v2",
    ];

    for table in &synced_tables {
        // uuid column + unique index
        let sql_uuid = format!(
            "ALTER TABLE {} ADD COLUMN IF NOT EXISTS uuid TEXT", table
        );
        sqlx::query(&sql_uuid).execute(pool).await.ok();

        // Create unique index on (store_id, uuid) — partial, only non-null
        let idx_name = format!("idx_{}_uuid", table);
        let sql_idx = format!(
            "CREATE UNIQUE INDEX IF NOT EXISTS {} ON {} (store_id, uuid) WHERE uuid IS NOT NULL",
            idx_name, table
        );
        sqlx::query(&sql_idx).execute(pool).await.ok();

        // device_id column
        let sql_dev = format!(
            "ALTER TABLE {} ADD COLUMN IF NOT EXISTS device_id TEXT", table
        );
        sqlx::query(&sql_dev).execute(pool).await.ok();

        // updated_at column (use TIMESTAMP to match existing pattern, avoid TIMESTAMPTZ decode issues)
        let sql_upd = format!(
            "ALTER TABLE {} ADD COLUMN IF NOT EXISTS updated_at_v2 TIMESTAMP DEFAULT NOW()", table
        );
        sqlx::query(&sql_upd).execute(pool).await.ok();
    }

    // ── 2. sync_inbox — staging table for incoming changes ──────────────────
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sync_inbox (\
         id BIGSERIAL PRIMARY KEY, \
         store_id INTEGER NOT NULL, \
         device_id TEXT NOT NULL, \
         batch_id TEXT NOT NULL, \
         table_name TEXT NOT NULL, \
         record_uuid TEXT NOT NULL, \
         operation TEXT NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')), \
         payload JSONB NOT NULL, \
         received_at TIMESTAMP DEFAULT NOW(), \
         processed_at TIMESTAMP, \
         status TEXT DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'PROCESSED', 'FAILED', 'SKIPPED')), \
         error_message TEXT)"
    ).execute(pool).await.ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sync_inbox_status ON sync_inbox(status)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sync_inbox_store ON sync_inbox(store_id)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sync_inbox_batch ON sync_inbox(batch_id)").execute(pool).await.ok();

    // ── 3. sync_journal — global change log (cursor source) ─────────────────
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sync_journal (\
         id BIGSERIAL PRIMARY KEY, \
         store_id INTEGER NOT NULL, \
         table_name TEXT NOT NULL, \
         record_uuid TEXT NOT NULL, \
         operation TEXT NOT NULL CHECK (operation IN ('INSERT', 'UPDATE', 'DELETE')), \
         source_device_id TEXT, \
         created_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sj_store_id ON sync_journal(store_id, id)").execute(pool).await.ok();
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sj_table ON sync_journal(table_name)").execute(pool).await.ok();

    // ── 4. sync_conflicts — conflict audit log ──────────────────────────────
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sync_conflicts (\
         id BIGSERIAL PRIMARY KEY, \
         store_id INTEGER NOT NULL, \
         table_name TEXT NOT NULL, \
         record_uuid TEXT NOT NULL, \
         device_a TEXT NOT NULL, \
         device_b TEXT NOT NULL, \
         field_name TEXT, \
         value_a TEXT, \
         value_b TEXT, \
         resolution TEXT NOT NULL CHECK (resolution IN ('SERVER_WINS', 'CLIENT_WINS', 'MERGED')), \
         resolved_at TIMESTAMP DEFAULT NOW())"
    ).execute(pool).await.ok();

    // ── 5. sync_devices — device registry per store ─────────────────────────
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sync_devices (\
         id BIGSERIAL PRIMARY KEY, \
         store_id INTEGER NOT NULL, \
         device_id TEXT NOT NULL, \
         device_name TEXT, \
         last_push_at TIMESTAMP, \
         last_pull_at TIMESTAMP, \
         pull_cursor BIGINT DEFAULT 0, \
         created_at TIMESTAMP DEFAULT NOW(), \
         UNIQUE(store_id, device_id))"
    ).execute(pool).await.ok();

    // ── 6. log_sync_journal PG function ─────────────────────────────────────
    sqlx::query(
        "CREATE OR REPLACE FUNCTION log_sync_journal(\
         p_store_id INTEGER, p_table_name TEXT, p_record_uuid TEXT, \
         p_operation TEXT, p_device_id TEXT) RETURNS BIGINT AS $$ \
         DECLARE new_id BIGINT; \
         BEGIN \
           INSERT INTO sync_journal (store_id, table_name, record_uuid, operation, source_device_id) \
           VALUES (p_store_id, p_table_name, p_record_uuid, p_operation, p_device_id) \
           RETURNING id INTO new_id; \
           RETURN new_id; \
         END; \
         $$ LANGUAGE plpgsql"
    ).execute(pool).await.ok();

    tracing::info!("✅ Sync V2 migrations complete");
}
