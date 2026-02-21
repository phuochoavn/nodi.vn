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
}
