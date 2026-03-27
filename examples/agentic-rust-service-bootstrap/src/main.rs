use my_example_operations_service::{AppConfig, run};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    match try_main().await {
        Ok(()) => info!("service stopped"),
        Err(error) => {
            error!(error = %error, "startup failed");
            std::process::exit(1);
        }
    }
}

async fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env()?;
    init_logging(config.log_filter())?;

    info!(
        service = "my-example-operations-service",
        "starting service"
    );
    info!(
        bind_address = %config.bind_address(),
        log_filter = config.log_filter(),
        "configuration loaded"
    );

    run(config).await?;
    Ok(())
}

fn init_logging(log_filter: &str) -> Result<(), tracing_subscriber::filter::FromEnvError> {
    let env_filter = EnvFilter::try_new(log_filter)?;
    fmt()
        .json()
        .with_env_filter(env_filter)
        .with_current_span(false)
        .with_span_list(false)
        .init();
    Ok(())
}
