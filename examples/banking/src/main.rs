use banking_feature_slices_example::{AppConfig, AppState, run};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    configure_logging();

    if let Err(error) = start().await {
        error!(%error, "startup failed");
        std::process::exit(1);
    }
}

async fn start() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("starting banking feature slices example");

    let config = AppConfig::from_env()?;
    info!(address = %config.address, "configuration loaded");
    let app_state = AppState::from_sqlite_path(&config.factstore_sqlite_path)?;
    info!(path = %config.factstore_sqlite_path, "factstore sqlite opened");

    let listener = TcpListener::bind(config.address).await?;
    run(listener, app_state).await?;
    Ok(())
}

fn configure_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt().json().with_env_filter(filter).init();
}
