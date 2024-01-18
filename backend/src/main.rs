use miette::IntoDiagnostic;
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    prelude::*,
};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let registry = tracing_subscriber::registry().with(Targets::new().with_targets([
        ("server", LevelFilter::DEBUG),
        ("model", LevelFilter::DEBUG),
        ("rosu_v2", LevelFilter::INFO),
        ("tower_http", LevelFilter::INFO),
    ]));
    if let Ok(pretty_logging_enabled) = std::env::var("LOG_PRETTY")
        .into_diagnostic()
        .and_then(|v| v.parse::<bool>().into_diagnostic())
    {
        if pretty_logging_enabled {
            registry
                .with(tracing_subscriber::fmt::layer().without_time().pretty())
                .init();
        }
    } else {
        registry
            .with(tracing_subscriber::fmt::layer().without_time().compact())
            .init();
    };

    // Setup logger

    server::run_server().await
}
