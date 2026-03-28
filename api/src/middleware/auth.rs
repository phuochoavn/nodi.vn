use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,      // user id
    pub store_id: i32, // store id
    pub role: String,
    #[serde(default)]
    pub plan_type: String, // "free", "trial", "pro", "lifetime" (default empty for backward compat)
    #[serde(default)]
    pub token_type: String, // "access" or "refresh" (default empty for backward compat)
    pub exp: usize,    // expiration timestamp
    pub iat: usize,    // issued at
}

/// Create access token (24h)
pub fn create_token(user_id: i32, store_id: i32, role: &str, plan_type: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    #[allow(deprecated)]
    let expiry = now + Duration::hours(24);

    let claims = Claims {
        sub: user_id,
        store_id,
        role: role.to_string(),
        plan_type: plan_type.to_string(),
        token_type: "access".to_string(),
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

/// Create refresh token (90 days)
pub fn create_refresh_token(user_id: i32, store_id: i32, role: &str, plan_type: &str, secret: &str) -> Result<String, AppError> {
    let now = Utc::now();
    #[allow(deprecated)]
    let expiry = now + Duration::days(90);

    let claims = Claims {
        sub: user_id,
        store_id,
        role: role.to_string(),
        plan_type: plan_type.to_string(),
        token_type: "refresh".to_string(),
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

/// Verify access token
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?;

    // Accept both access tokens and tokens without token_type (backward compatibility)
    if claims.token_type != "access" && !claims.token_type.is_empty() {
        return Err(AppError::Unauthorized("Invalid token type".into()));
    }

    Ok(claims)
}

/// Verify refresh token
pub fn verify_refresh_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid refresh token: {}", e)))?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized("Not a refresh token".into()));
    }

    Ok(claims)
}
