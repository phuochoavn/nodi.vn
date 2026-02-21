use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub license_key: String,
    pub owner_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub license_type: Option<String>,
    pub license_expires_at: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub hwid: Option<String>,
    pub activated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
