use std::path::PathBuf;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

pub type DbPool = SqlitePool;

fn backend_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub async fn create_pool(database_path: &str) -> Result<DbPool, sqlx::Error> {
    let path = backend_root().join(database_path);
    let options = SqliteConnectOptions::new()
        .filename(path)
        .pragma("foreign_keys", "ON");

    SqlitePool::connect_with(options).await
}
