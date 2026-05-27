use ventalibre_server::{
    build_app,
    config::AppConfig,
    db::create_pool,
    state::AppState,
};

#[tokio::main]
async fn main() {
    let config = AppConfig::load().expect("failed to load configuration");

    let pool = create_pool(&config.sqlite_database_url).await.expect("database pool");

    let state = AppState::new(pool, config);

    let address = state.config.socket_addr();

    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(address).await.expect("bind failed");

    println!("listening on http://{address}");
    
    axum::serve(listener, app).await.expect("server error");
}
