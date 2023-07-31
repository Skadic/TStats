use axum::{
    http::Method,
    routing::{get, options, post},
    Router,
};
use log::{info, warn, LevelFilter};
use miette::{Context, IntoDiagnostic};
use rosu_v2::prelude::*;
use sea_orm::{
    sea_query::Table, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, EntityTrait,
    Schema,
};
use std::{fs::File, io::Write, sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model::entities::{
    CountryRestrictionEntity, PoolBracketEntity, PoolMapEntity, StageEntity, TournamentEntity,
};

mod model;
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

#[tokio::main]
async fn main() -> miette::Result<()> {
    // Setup logger
    tracing_log::LogTracer::builder()
        .with_max_level(LevelFilter::Trace)
        .ignore_crate("sqlx")
        .ignore_crate("hyper")
        .ignore_crate("rustls")
        .ignore_crate("h2")
        .init()
        .into_diagnostic()
        .wrap_err("failed to setup logger")?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .into_diagnostic()
        .wrap_err("setting default subscriber failed")?;

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
                    "http://localhost".parse().unwrap(),
                    "https://localhost".parse().unwrap(),
                    "http://tstats.skadic.moe".parse().unwrap(),
                    "https://tstats.skadic.moe".parse().unwrap(),
                ])
                .allow_headers(["content-type".parse().unwrap()]),
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

async fn drop_table<E: EntityTrait>(db: &DatabaseConnection, table: E) {
    let builder = db.get_database_backend();
    match db
        .execute(builder.build(Table::drop().table(table.table_ref())))
        .await
    {
        Ok(_) => info!("Dropped table '{}'", table.table_name()),
        Err(e) => info!("Failed to drop table '{}': {e}", table.table_name()),
    };
}

async fn create_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    match db
        .execute(builder.build(&schema.create_table_from_entity(entity)))
        .await
    {
        Ok(_) => info!("Created table '{}'", entity.table_name()),
        Err(e) => info!("Failed to create table '{}': {e}", entity.table_name()),
    };
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
