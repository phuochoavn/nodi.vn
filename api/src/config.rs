/// Sprint 163: Read a secret from a Docker Secret file, falling back to an env var.
/// In production (Docker), secrets are mounted at /run/secrets/<name> (tmpfs, never on disk).
/// In local dev (no Docker), the env var fallback allows `dotenvy` / shell env to work.
pub fn load_secret(secret_path: &str, env_fallback: &str) -> String {
    let path = std::path::Path::new(secret_path);
    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(val) => {
                let trimmed = val.trim().to_string();
                tracing::info!("🔑 Loaded secret from {}", secret_path);
                trimmed
            }
            Err(e) => {
                tracing::error!("Failed to read secret file {}: {}", secret_path, e);
                std::env::var(env_fallback)
                    .unwrap_or_else(|_| panic!("{} must be set (secret file read failed)", env_fallback))
            }
        }
    } else {
        tracing::warn!("Secret file {} not found, falling back to env var {}", secret_path, env_fallback);
        std::env::var(env_fallback)
            .unwrap_or_else(|_| panic!("{} must be set (no secret file or env var)", env_fallback))
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Sprint 163: JWT secret from Docker Secret file
        let jwt_secret = load_secret("/run/secrets/jwt_secret", "JWT_SECRET");

        // Sprint 163: Construct DATABASE_URL from secret + env vars
        // This avoids storing the full connection string with embedded password in .env
        let database_url = if std::path::Path::new("/run/secrets/db_password").exists() {
            let db_password = load_secret("/run/secrets/db_password", "DB_PASSWORD");
            let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| "pgbouncer".to_string());
            let db_port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| "nodi".to_string());
            let db_user = std::env::var("DB_USER").unwrap_or_else(|_| "nodi_app".to_string());
            format!("postgres://{}:{}@{}:{}/{}", db_user, db_password, db_host, db_port, db_name)
        } else {
            tracing::warn!("No /run/secrets/db_password, falling back to DATABASE_URL env var");
            std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set (no secret file available)")
        };

        Self {
            database_url,
            jwt_secret,
        }
    }
}
