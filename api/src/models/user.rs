use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub store_id: Option<i32>,
    pub phone: String,
    pub password_hash: String,
    pub role: Option<String>,
    pub display_name: Option<String>,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}
