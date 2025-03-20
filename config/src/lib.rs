use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;
use tracing::info;

pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

pub struct ServerConfig {
    pub port: u16,
}

pub struct DatabaseConfig {
    pub url: String,
    pub run_migrations: bool,
}

pub struct LogConfig {
    pub level: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        info!("Loading configuration");
        // Load .env file if present
        let _ = dotenv();

        Ok(Self {
            server: ServerConfig {
                port: env::var("PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .context("Invalid PORT")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
                run_migrations: env::var("RUN_MIGRATIONS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .context("Invalid RUN_MIGRATIONS")?,
            },
            log: LogConfig {
                level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            },
        })
    }
}
