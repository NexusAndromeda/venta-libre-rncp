use std::env;
use std::net::{IpAddr, SocketAddr};

use dotenvy::dotenv;
use http::HeaderValue;

fn require_env(key: &str) -> Result<String, String> {
    env::var(key)
        .map_err(|_| format!("{key} must be set in .env"))
}

fn env_parse<T: std::str::FromStr>(key: &str, err: &str) -> Result<T, String> {
    require_env(key)?
        .parse()
        .map_err(|_| err.to_string())
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub cors_allowed_origin: HeaderValue,
    pub sqlite_database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {

    pub fn load() -> Result<Self, String> {
        dotenv().ok();

        let port = env_parse("PORT", "PORT must be a valid integer")?;
        let host = env_parse("HOST", "HOST must be a valid IP address")?;
        let jwt_expiration_hours =
            env_parse("JWT_EXPIRATION_HOURS", "JWT_EXPIRATION_HOURS must be a valid integer")?;
        let cors_allowed_origin = env_parse(
            "CORS_ALLOWED_ORIGIN",
            "CORS_ALLOWED_ORIGIN must be a valid origin (e.g. http://localhost:8080)",
        )?;

        Ok(Self {
            host,
            port,
            cors_allowed_origin,
            sqlite_database_url: require_env("SQLITE_DATABASE_URL")?,
            jwt_secret: require_env("JWT_SECRET")?,
            jwt_expiration_hours,
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}
