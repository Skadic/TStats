use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    prelude::*,
};

#[tokio::main]
async fn main() -> miette::Result<()> {
    // Setup logger
    tracing_subscriber::registry()
        .with(Targets::new().with_targets([
            ("server", LevelFilter::DEBUG),
            ("model", LevelFilter::DEBUG),
            ("rosu_v2", LevelFilter::INFO),
            ("tower_http", LevelFilter::INFO),
        ]))
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    server::run_server().await
}
