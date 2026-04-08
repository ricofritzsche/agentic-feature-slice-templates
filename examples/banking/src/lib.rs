mod features;

use axum::{Json, Router, routing::get};
use factstore_memory::MemoryStore;
use factstore_sqlite::SqliteStore;
use std::{env, error::Error, fmt, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::info;

const SERVICE_NAME: &str = "banking-feature-slices-example";

pub struct AppConfig {
    pub address: SocketAddr,
    pub factstore_sqlite_path: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
        let port = env::var("APP_PORT").map_err(|_| ConfigError::MissingVar("APP_PORT"))?;
        let factstore_sqlite_path = env::var("FACTSTORE_SQLITE_PATH")
            .map_err(|_| ConfigError::MissingVar("FACTSTORE_SQLITE_PATH"))?;
        let address =
            format!("{host}:{port}")
                .parse()
                .map_err(|source| ConfigError::InvalidAddress {
                    value: format!("{host}:{port}"),
                    source,
                })?;

        Ok(Self {
            address,
            factstore_sqlite_path,
        })
    }
}

#[derive(Clone)]
pub struct AppState {
    store: Arc<Store>,
}

impl AppState {
    pub fn from_sqlite_path(
        factstore_sqlite_path: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let store = SqliteStore::open(factstore_sqlite_path)?;

        Ok(Self {
            store: Arc::new(Store::Sqlite(store)),
        })
    }

    pub fn from_memory_store(memory_store: Arc<Mutex<MemoryStore>>) -> Self {
        Self {
            store: Arc::new(Store::Memory(memory_store)),
        }
    }

    pub fn with_failing_append() -> Self {
        Self {
            store: Arc::new(Store::FailAppend),
        }
    }

    pub fn store(&self) -> &Store {
        self.store.as_ref()
    }
}

pub enum Store {
    Sqlite(SqliteStore),
    Memory(Arc<Mutex<MemoryStore>>),
    FailAppend,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingVar(&'static str),
    InvalidAddress {
        value: String,
        source: std::net::AddrParseError,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVar(var) => write!(f, "missing required environment variable {var}"),
            Self::InvalidAddress { value, source } => {
                write!(f, "invalid listen address {value}: {source}")
            }
        }
    }
}

impl Error for ConfigError {}

pub fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .merge(features::router())
        .with_state(app_state)
}

pub async fn run(listener: TcpListener, app_state: AppState) -> Result<(), std::io::Error> {
    let address = listener.local_addr()?;
    info!(service = SERVICE_NAME, %address, "service listening");

    axum::serve(listener, app(app_state)).await
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: SERVICE_NAME,
    })
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}
