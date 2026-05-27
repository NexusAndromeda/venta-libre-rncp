use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use crate::routes;
use crate::state::AppState;

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/v1", routes::v1_router())
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(state.config.cors_allowed_origin.clone()))
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state)
}
