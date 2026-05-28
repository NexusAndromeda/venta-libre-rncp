use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::{json, Value};

use crate::models::auth::AuthError;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: Value,
}

impl ApiError {
    fn new(status: StatusCode, error: &str, message: &str) -> Self {
        Self {
            status,
            body: json!({ "error": error, "message": message }),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

pub fn map_auth_error(err: AuthError) -> ApiError {
    let (status, error, message) = match err {
        AuthError::MissingToken => (
            StatusCode::UNAUTHORIZED,
            "missing_token",
            "Token de autorización requerido",
        ),
        AuthError::InvalidFormat => (
            StatusCode::UNAUTHORIZED,
            "invalid_format",
            "Formato de token inválido",
        ),
        AuthError::InvalidToken => (
            StatusCode::UNAUTHORIZED,
            "invalid_token",
            "Token inválido o expirado",
        ),
        AuthError::UserNotFound => (
            StatusCode::NOT_FOUND,
            "user_not_found",
            "Usuario no encontrado",
        ),
        AuthError::InvalidName => (
            StatusCode::BAD_REQUEST,
            "invalid_name",
            "El nombre es requerido",
        ),
        AuthError::InvalidEmail => (
            StatusCode::BAD_REQUEST,
            "invalid_email",
            "Email inválido",
        ),
        AuthError::WeakPassword => (
            StatusCode::BAD_REQUEST,
            "weak_password",
            "La contraseña debe tener al menos 6 caracteres",
        ),
        AuthError::EmailExists => (
            StatusCode::CONFLICT,
            "email_exists",
            "El email ya está registrado",
        ),
        AuthError::HashError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "hash_error",
            "Error al procesar contraseña",
        ),
        AuthError::CreateUserError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "create_user_error",
            "Error al crear usuario",
        ),
        AuthError::TokenError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "token_error",
            "Error al generar token",
        ),
        AuthError::InvalidCredentials => (
            StatusCode::UNAUTHORIZED,
            "invalid_credentials",
            "Credenciales inválidas",
        ),
        AuthError::UserInactive => (
            StatusCode::UNAUTHORIZED,
            "user_inactive",
            "Usuario inactivo",
        ),
        AuthError::VerificationError => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "verification_error",
            "Error al verificar contraseña",
        ),
        AuthError::Database => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database_error",
            "Error de base de datos",
        ),
        AuthError::Forbidden => (
            StatusCode::FORBIDDEN,
            "forbidden",
            "Acción no permitida",
        ),
    };
    ApiError::new(status, error, message)
}
