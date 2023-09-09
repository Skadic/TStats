use axum::{
    http::Method,
    routing::{get, options, post},
    Router,
};
use log::{info, warn};
use miette::{Context, IntoDiagnostic};
use rosu_v2::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{fs::File, io::Write, sync::Arc, time::Duration};
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use model::{entities::{
    CountryRestrictionEntity, PoolBracketEntity, PoolMapEntity, StageEntity, TournamentEntity,
}, drop_table, create_table};

mod osu;
mod routes;

async fn cors() -> StatusCode {
    StatusCode::OK
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::debug::fill_test_data,
        routes::debug::get_beatmap,
        routes::tournament::get_all_tournaments,
        routes::tournament::get_tournament,
        routes::tournament::create_tournament,
        routes::stage::get_all_stages,
        routes::stage::get_stage,
        routes::stage::create_stage,
    ),
    components(
        schemas(
            model::tournament::Model,
            model::tournament::RankRestriction,
            model::tournament::TournamentFormat,
            model::tournament::RankRange,
            model::stage::Model,
            model::pool_bracket::Model,
            model::pool_map::Model,
            model::country_restriction::Model,
            routes::Id,
            routes::tournament::ExtendedTournamentResult,
            routes::tournament::SlimStage,
            routes::stage::TournamentId,
            routes::stage::TournamentIdAndStageOrder,
            routes::stage::ExtendedStageResult,
            routes::stage::ExtendedPoolBracket,
        )
    ),
    tags(
        (name = "tstats", description = "Backend API for managing tournaments and the associated data")
    )
)]
struct ApiDoc;

pub async fn run_server() -> miette::Result<()> {
    {
        let api_yaml = ApiDoc::openapi()
            .to_yaml()
            .into_diagnostic()
            .wrap_err("error serializing api docs to yaml")?;
        let mut api_doc_file = File::create("apidoc.yaml")
            .into_diagnostic()
            .wrap_err("could not open apidoc.yaml file")?;
        api_doc_file
            .write_all(api_yaml.as_bytes())
            .into_diagnostic()
            .wrap_err("could not write to the apidoc.yaml")?;
        info!("API documentation written to apidoc.yaml")
    }

    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        warn!("could not read .env file. expecting environment variables to be defined: {e}");
    }
    let osu_client_id = std::env::var("OSU_CLIENT_ID")
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID not set")?
        .parse::<u64>()
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID must be a non-negative integer")?;

    let osu_client_secret = std::env::var("OSU_CLIENT_SECRET")
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_SECRET not set")?;
    let database_url = std::env::var("DATABASE_URL")
        .into_diagnostic()
        .wrap_err("DATABASE_URL not set")?;

    info!("Connecting to database...");
    //let mut opt = ConnectOptions::new("postgres://root:root@127.0.0.1:5432/postgres");
    let mut opt = ConnectOptions::new(database_url);
    opt.connect_timeout(Duration::from_secs(1));
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .into_diagnostic()
        .wrap_err("failed to connect to database")?;

    drop_table(&db, PoolMapEntity).await;
    drop_table(&db, PoolBracketEntity).await;
    drop_table(&db, StageEntity).await;
    drop_table(&db, CountryRestrictionEntity).await;
    drop_table(&db, TournamentEntity).await;

    create_table(&db, TournamentEntity).await;
    create_table(&db, CountryRestrictionEntity).await;
    create_table(&db, StageEntity).await;
    create_table(&db, PoolBracketEntity).await;
    create_table(&db, PoolMapEntity).await;
    info!("Connected to database and setup tables");

    info!("Connecting to osu api...");

    let osu = Osu::new(osu_client_id, osu_client_secret)
        .await
        .into_diagnostic()
        .wrap_err("error connecting to osu api")?;
    info!("Connection successful!");

    // build our application
    let app = Router::new()
        .merge(SwaggerUi::new("/api/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", options(cors))
        .route("/*path", options(cors))
        .route("/api", get(|| async { "Hello, World!" }))
        .route("/api/test_data", post(routes::debug::fill_test_data))
        .route(
            "/api/tournament/all",
            get(routes::tournament::get_all_tournaments),
        )
        .route(
            "/api/tournament",
            get(routes::tournament::get_tournament).post(routes::tournament::create_tournament),
        )
        .route("/api/stage/all", get(routes::stage::get_all_stages))
        .route(
            "/api/stage",
            get(routes::stage::get_stage).post(routes::stage::create_stage),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_origin([
                    "http://localhost:5173".parse().unwrap(),
                    "https://localhost:5173".parse().unwrap(),
                    "http://tstats.skadic.moe".parse().unwrap(),
                    "https://tstats.skadic.moe".parse().unwrap(),
                ])
                .allow_headers(["content-type".parse().unwrap()]),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::INFO)),
        )
        .with_state(db)
        .route("/beatmap", get(routes::debug::get_beatmap))
        .with_state(Arc::new(osu));

    info!("Starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .into_diagnostic()
}

// source: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        info!("Shutdown request from Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
        info!("Shutdown request from SIGTERM");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
