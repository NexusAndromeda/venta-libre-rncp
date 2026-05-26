use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use ventalibre_server::config::AppConfig;

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {

    let config = AppConfig::load().expect("failed to load configuration");
    let address = config.socket_addr();

    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(address).await.expect("bind failed");

    println!("listening on http://{address}");
    axum::serve(listener, app).await.expect("server error");
}