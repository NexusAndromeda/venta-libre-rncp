use sqlx::Row;

use crate::config::AppConfig;
use crate::db::DbPool;
use crate::util::jwt::{auth_response_for_user, extract_bearer, verify_token};
use crate::util::password::{hash_password, verify_password};
use crate::util::types::{user_from_sql, AuthResponse, LoginRequest, RegisterRequest, UserRow};
use crate::util::uuid::new_uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    MissingToken,
    InvalidFormat,
    InvalidToken,
    UserNotFound,
    InvalidName,
    InvalidEmail,
    WeakPassword,
    EmailExists,
    HashError,
    CreateUserError,
    TokenError,
    InvalidCredentials,
    UserInactive,
    VerificationError,
    Database,
    Forbidden,
}

pub async fn register(
    pool: &DbPool,
    config: &AppConfig,
    request: RegisterRequest,
) -> Result<AuthResponse, AuthError> {
    if let Some(err) = validate_register(&request) {
        return Err(err);
    }

    let email_lc = request.email.trim().to_lowercase();

    let exists: i64 = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = ?) AS e")
        .bind(&email_lc)
        .fetch_one(pool)
        .await
        .map_err(|_| AuthError::Database)?;

    if exists != 0 {
        return Err(AuthError::EmailExists);
    }

    let password_hash = hash_password(&request.password).map_err(|_| AuthError::HashError)?;
    let id = new_uuid();

    sqlx::query(
        "INSERT INTO users (id, email, display_name, password_hash, is_active, is_admin)
         VALUES (?, ?, ?, ?, 1, 0)",
    )
    .bind(&id)
    .bind(&email_lc)
    .bind(request.name.trim())
    .bind(&password_hash)
    .execute(pool)
    .await
    .map_err(|_| AuthError::CreateUserError)?;

    let user = fetch_active_user_by_id(pool, &id)
        .await
        .map_err(|_| AuthError::Database)?
        .ok_or(AuthError::CreateUserError)?;

    auth_response_for_user(&user, config).map_err(|_| AuthError::TokenError)
}

pub async fn login(
    pool: &DbPool,
    config: &AppConfig,
    request: LoginRequest,
) -> Result<AuthResponse, AuthError> {
    let email_lc = request.email.trim().to_lowercase();

    let user = fetch_user_by_email(pool, &email_lc)
        .await
        .map_err(|_| AuthError::Database)?
        .ok_or(AuthError::InvalidCredentials)?;

    if !user.is_active {
        return Err(AuthError::UserInactive);
    }

    let ok = verify_password(&request.password, &user.password_hash)
        .map_err(|_| AuthError::VerificationError)?;

    if !ok {
        return Err(AuthError::InvalidCredentials);
    }

    auth_response_for_user(&user, config).map_err(|_| AuthError::TokenError)
}

pub async fn resolve_bearer(
    pool: &DbPool,
    config: &AppConfig,
    auth_header: Option<&str>,
) -> Result<UserRow, AuthError> {
    let trimmed = auth_header.map(str::trim).filter(|s| !s.is_empty());
    if trimmed.is_none() {
        return Err(AuthError::MissingToken);
    }

    let token = extract_bearer(trimmed).ok_or(AuthError::InvalidFormat)?;
    let claims = verify_token(&token, config).map_err(|_| AuthError::InvalidToken)?;

    if claims.sub.is_empty() {
        return Err(AuthError::InvalidToken);
    }

    fetch_active_user_by_id(pool, &claims.sub)
        .await
        .map_err(|_| AuthError::Database)?
        .ok_or(AuthError::UserNotFound)
}

fn validate_register(request: &RegisterRequest) -> Option<AuthError> {
    if request.name.trim().is_empty() {
        return Some(AuthError::InvalidName);
    }
    if !request.email.trim().contains('@') {
        return Some(AuthError::InvalidEmail);
    }
    if request.password.len() < 6 {
        return Some(AuthError::WeakPassword);
    }
    None
}

async fn fetch_active_user_by_id(pool: &DbPool, user_id: &str) -> Result<Option<UserRow>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, email, display_name, password_hash, is_active, is_admin, created_at, updated_at
         FROM users WHERE id = ? AND is_active = 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_user_row).transpose()
}

async fn fetch_user_by_email(pool: &DbPool, email: &str) -> Result<Option<UserRow>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, email, display_name, password_hash, is_active, is_admin, created_at, updated_at
         FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    row.as_ref().map(map_user_row).transpose()
}

fn map_user_row(row: &sqlx::sqlite::SqliteRow) -> Result<UserRow, sqlx::Error> {
    Ok(user_from_sql(
        row.try_get(0)?,
        row.try_get(1)?,
        row.try_get(2)?,
        row.try_get(3)?,
        row.try_get(4)?,
        row.try_get(5)?,
        row.try_get(6)?,
        row.try_get(7)?,
    ))
}
