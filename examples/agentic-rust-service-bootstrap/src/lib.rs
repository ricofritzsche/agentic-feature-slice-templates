use axum::{Json, Router, routing::get};
use std::{env, error::Error, fmt, net::SocketAddr};
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppConfig {
    bind_address: SocketAddr,
    log_filter: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = read_env("APP_HOST").unwrap_or_else(|| "127.0.0.1".to_string());
        let port = match read_env("APP_PORT") {
            Some(raw) => raw
                .parse::<u16>()
                .map_err(|source| ConfigError::InvalidPort { raw, source })?,
            None => 3000,
        };
        let bind_address = format!("{host}:{port}")
            .parse::<SocketAddr>()
            .map_err(|source| ConfigError::InvalidBindAddress {
                host: host.clone(),
                port,
                source,
            })?;
        let log_filter = read_env("APP_LOG").unwrap_or_else(|| "info".to_string());

        Ok(Self {
            bind_address,
            log_filter,
        })
    }

    pub fn bind_address(&self) -> SocketAddr {
        self.bind_address
    }

    pub fn log_filter(&self) -> &str {
        &self.log_filter
    }
}

fn read_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidPort {
        raw: String,
        source: std::num::ParseIntError,
    },
    InvalidBindAddress {
        host: String,
        port: u16,
        source: std::net::AddrParseError,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPort { raw, .. } => {
                write!(f, "APP_PORT must be a valid u16 value, got {raw:?}")
            }
            Self::InvalidBindAddress { host, port, .. } => {
                write!(
                    f,
                    "invalid bind address from APP_HOST={host:?} and APP_PORT={port}"
                )
            }
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPort { source, .. } => Some(source),
            Self::InvalidBindAddress { source, .. } => Some(source),
        }
    }
}

pub fn build_router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub async fn run(config: AppConfig) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(config.bind_address()).await?;
    let local_address = listener.local_addr()?;

    info!(bind_address = %local_address, "service listening");

    axum::serve(listener, build_router()).await
}
