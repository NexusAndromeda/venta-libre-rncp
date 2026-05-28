use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::HeaderValue;

use crate::error::{map_auth_error, ApiError};
use crate::models::auth::{self, AuthError};
use crate::state::AppState;
use crate::util::types::{is_admin_active, UserRow};

pub async fn require_user(
    state: &AppState,
    auth_header: Option<&str>,
) -> Result<UserRow, AuthError> {
    auth::resolve_bearer(&state.pool, &state.config, auth_header).await
}

pub async fn require_admin(
    state: &AppState,
    auth_header: Option<&str>,
) -> Result<UserRow, AuthError> {
    let user = require_user(state, auth_header).await?;
    if !is_admin_active(&user) {
        return Err(AuthError::Forbidden);
    }
    Ok(user)
}

pub async fn optional_user(
    state: &AppState,
    auth_header: Option<&str>,
) -> Result<Option<UserRow>, AuthError> {
    let trimmed = auth_header.map(str::trim).filter(|s| !s.is_empty());
    if trimmed.is_none() {
        return Ok(None);
    }
    require_user(state, auth_header).await.map(Some)
}

pub struct AuthUser(pub UserRow);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(header_to_str);
        let user = require_user(state, header)
            .await
            .map_err(map_auth_error)?;
        Ok(AuthUser(user))
    }
}

pub struct AdminUser(pub UserRow);

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(header_to_str);
        let user = require_admin(state, header)
            .await
            .map_err(map_auth_error)?;
        Ok(AdminUser(user))
    }
}

pub struct OptionalAuth(pub Option<UserRow>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuth {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(header_to_str);
        let user = optional_user(state, header)
            .await
            .map_err(map_auth_error)?;
        Ok(OptionalAuth(user))
    }
}

fn header_to_str(value: &HeaderValue) -> Option<&str> {
    value.to_str().ok()
}
