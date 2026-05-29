use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::HeaderValue;

use crate::error::{map_auth_error, ApiError};
use crate::models::auth;
use crate::state::AppState;
use crate::util::types::{is_admin_active, UserRow};

fn header_to_str(value: &HeaderValue) -> Option<&str> {
    value.to_str().ok()
}

pub struct AuthUser(pub UserRow);

pub struct AdminUser(pub UserRow);

pub struct OptionalAuth(pub Option<UserRow>);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(header_to_str);
        let user = auth::resolve_bearer(&state.pool, &state.config, header)
            .await
            .map_err(map_auth_error)?;
        Ok(AuthUser(user))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let authenticated = AuthUser::from_request_parts(parts, state).await?;
        let AuthUser(user) = authenticated;
        if !is_admin_active(&user) {
            return Err(ApiError::forbidden());
        }
        Ok(AdminUser(user))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuth {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(header_to_str);
        let trimmed = header.map(str::trim).filter(|s| !s.is_empty());
        if trimmed.is_none() {
            return Ok(OptionalAuth(None));
        }
        let user = auth::resolve_bearer(&state.pool, &state.config, header)
            .await
            .map_err(map_auth_error)?;
        Ok(OptionalAuth(Some(user)))
    }
}
