use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,      // user id
    pub store_id: i32, // store id
    pub role: String,
    pub exp: usize,    // expiration timestamp
    pub iat: usize,    // issued at
}

pub fn create_token(user_id: i32, store_id: i32, role: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    #[allow(deprecated)]
    let expiry = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        store_id,
        role: role.to_string(),
        exp: expiry.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))
}

#[allow(dead_code)]
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}
