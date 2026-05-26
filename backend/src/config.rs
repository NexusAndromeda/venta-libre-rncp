use std::env;
use std::net::{IpAddr, SocketAddr};
use dotenvy::dotenv;

fn require_env(key: &str) -> Result<String, String> {
    env::var(key)
        .map_err(|_| format!("{key} must be set in .env"))
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub sqlite_database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppConfig {

    pub fn load() -> Result<Self, String> {
        dotenv().ok();

        let port: u16 = require_env("PORT")?
            .parse()
            .map_err(|_| "PORT must be a valid integer".to_string())?;

        let host: IpAddr = require_env("HOST")?
            .parse()
            .map_err(|_| "HOST must be a valid IP address".to_string())?;

        let jwt_expiration_hours: i64 = require_env("JWT_EXPIRATION_HOURS")?
            .parse()
            .map_err(|_| "JWT_EXPIRATION_HOURS must be a valid integer".to_string())?;

        Ok(Self {
            host,
            port,
            sqlite_database_url: require_env("SQLITE_DATABASE_URL")?,
            jwt_secret: require_env("JWT_SECRET")?,
            jwt_expiration_hours,
        })
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    }
}
