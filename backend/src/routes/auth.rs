use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Json;
use serde_json::{json, Value};

use crate::auth_util::AuthUser;
use crate::error::{map_auth_error, ApiError};
use crate::models::auth;
use crate::state::AppState;
use crate::util::types::{to_public_user, LoginRequest, RegisterRequest};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
        .route("/logout", post(logout))
}

async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let out = auth::register(&state.pool, &state.config, body)
        .await
        .map_err(map_auth_error)?;
    Ok((StatusCode::CREATED, Json(serde_json::to_value(out).unwrap())))
}

async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<Value>, ApiError> {
    let out = auth::login(&state.pool, &state.config, body)
        .await
        .map_err(map_auth_error)?;
    Ok(Json(serde_json::to_value(out).unwrap()))
}

async fn me(AuthUser(user): AuthUser) -> Json<Value> {
    Json(serde_json::to_value(to_public_user(&user)).unwrap())
}

async fn logout() -> Json<Value> {
    Json(json!({ "message": "Sesión cerrada exitosamente" }))
}
