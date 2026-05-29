use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::util::types::{to_public_user, AuthResponse, UserRow};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    email: String,
    name: String,
    is_admin: bool,
    exp: i64,
    iat: i64,
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn extract_bearer(auth_header: Option<&str>) -> Option<String> {
    let trimmed = auth_header?.trim();
    trimmed.strip_prefix("Bearer ").map(|t| t.to_string())
}

pub fn generate_token(user: &UserRow, config: &AppConfig) -> Result<String, ()> {
    let now = unix_now();
    let exp = now + config.jwt_expiration_hours * 3600;
    let claims = Claims {
        sub: user.id.clone(),
        email: user.email.clone(),
        name: user.display_name.clone(),
        is_admin: user.is_admin,
        exp,
        iat: now,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| ())
}

pub fn verify_token(token: &str, config: &AppConfig) -> Result<Claims, ()> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ())?;
    Ok(data.claims)
}

pub fn auth_response_for_user(user: &UserRow, config: &AppConfig) -> Result<AuthResponse, ()> {
    let token = generate_token(user, config)?;
    let now = unix_now();
    Ok(AuthResponse {
        token,
        user: to_public_user(user),
        expires_at: now + config.jwt_expiration_hours * 3600,
    })
}
