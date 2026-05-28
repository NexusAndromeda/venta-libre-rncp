mod auth;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::state::AppState;

async fn v1_info() -> Json<Value> {
    Json(json!({ "api": "v1" }))
}

pub fn v1_router() -> Router<AppState> {
    Router::new()
        .route("/", get(v1_info))
        .nest("/auth", auth::router())
}
