use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::HeaderValue;
use axum::http::Method;

use miette::{Context, IntoDiagnostic};
use proto::pool::pool_service_server::PoolServiceServer;
use proto::stages::stage_service_server::StageServiceServer;
use rosu_v2::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::RwLock;
use tonic::transport::NamedService;
use tonic_health::server::HealthReporter;

use tower_http::cors::AllowHeaders;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::{info, warn, Level};

use model::{
    create_table, drop_table,
    entities::{
        CountryRestrictionEntity, PoolBracketEntity, PoolMapEntity, RankRestrictionEntity,
        StageEntity, TournamentEntity,
    },
};
use proto::debug_data::debug_service_server::DebugServiceServer;
use proto::tournaments::tournament_service_server::TournamentServiceServer;

use crate::routes::debug::DebugServiceImpl;
use crate::routes::pool::PoolServiceImpl;
use crate::routes::stage::StageServiceImpl;
use crate::routes::tournament::TournamentServiceImpl;

#[allow(unused)]
mod cache;
mod osu;
mod routes;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    osu: Arc<Osu>,
    redis: redis::aio::MultiplexedConnection,
}

impl AppState {
    fn get_local_instance(&self) -> LocalAppState {
        LocalAppState {
            db: self.db.clone(),
            osu: self.osu.clone(),
            redis: RwLock::new(self.redis.clone()),
        }
    }
}

#[allow(unused)]
pub struct LocalAppState {
    db: DatabaseConnection,
    osu: Arc<Osu>,
    redis: RwLock<redis::aio::MultiplexedConnection>,
}

pub async fn run_server() -> miette::Result<()> {
    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        warn!("could not read .env file. expecting environment variables to be defined: {e}");
    }

    let (db, redis, osu) = tokio::join!(setup_database(), setup_redis(), setup_osu());
    let (db, redis, osu) = (db?, redis?, osu?);

    let state = AppState { db, redis, osu };

    /*
    // build our application
    let app = Router::new()
        .route("/beatmap", get(routes::debug::get_beatmap))
        .route("/api/pool", get(routes::pool::get_pool))
        .route("/user", get(routes::debug::get_user))


    info!("Starting server");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .into_diagnostic()
     */

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .into_diagnostic()
        .wrap_err("error creating the gRPC reflection server")?;

    let (mut health_reporter, health_server) = tonic_health::server::health_reporter();

    health_reporter
        .set_serving::<TournamentServiceServer<TournamentServiceImpl>>()
        .await;
    health_reporter
        .set_serving::<DebugServiceServer<DebugServiceImpl>>()
        .await;
    health_reporter
        .set_serving::<StageServiceServer<StageServiceImpl>>()
        .await;
    health_reporter
        .set_serving::<PoolServiceServer<PoolServiceImpl>>()
        .await;

    // Type fun
    async fn set_serving<T: NamedService>(rep: &mut HealthReporter, _: &T) {
        rep.set_serving::<T>().await;
    }
    set_serving(&mut health_reporter, &reflection_server).await;

    let frontend_method = parse_env("FRONTEND_METHOD", || "http".to_owned())?;
    let frontend_host: String = parse_env("FRONTEND_HOST", || "0.0.0.0".to_owned())?;
    let frontend_port = parse_env("FRONTEND_PORT", || "5173".to_owned())?;
    let host: IpAddr = parse_env("BACKEND_HOST", || IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))?;
    let port = parse_env("BACKEND_PORT", || 3000)?;

    let frontend_addr: HeaderValue = format!("{frontend_method}://{frontend_host}:{frontend_port}")
        .parse()
        .into_diagnostic()
        .wrap_err("could not parse frontend url: {e}")?;
    let addr: SocketAddr = SocketAddr::new(host, port);

    info!("Serving at {addr}");
    info!("Allowing requests from {frontend_addr:?}");

    info!("Starting server");

    tonic::transport::Server::builder()
        .accept_http1(true)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
                .on_request(trace::DefaultOnRequest::new().level(Level::DEBUG))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_origin([frontend_addr])
                .allow_headers(AllowHeaders::any()),
        )
        .add_service(tonic_web::enable(reflection_server))
        .add_service(tonic_web::enable(DebugServiceServer::new(
            DebugServiceImpl(state.get_local_instance()),
        )))
        .add_service(tonic_web::enable(TournamentServiceServer::new(
            TournamentServiceImpl(state.get_local_instance()),
        )))
        .add_service(tonic_web::enable(StageServiceServer::new(
            StageServiceImpl(state.get_local_instance()),
        )))
        .add_service(tonic_web::enable(PoolServiceServer::new(PoolServiceImpl(
            state.get_local_instance(),
        ))))
        .add_service(health_server)
        .serve(addr)
        .await
        .into_diagnostic()
}

fn parse_env<T>(env_var: &str, default_fn: impl FnOnce() -> T) -> miette::Result<T>
where
    T::Err: 'static + std::error::Error + Send + Sync,
    T: FromStr,
{
    match std::env::var(env_var) {
        Ok(value) => value
            .parse::<T>()
            .into_diagnostic()
            .wrap_err(format!("could not parse {env_var} (value is {value})")),
        Err(e) => {
            warn!("could not read {env_var} ({e}), using default");
            Ok(default_fn())
        }
    }
}

async fn setup_database() -> miette::Result<DatabaseConnection> {
    let database_url = std::env::var("DATABASE_URL")
        .into_diagnostic()
        .wrap_err("DATABASE_URL not set")?;
    info!("connecting to database...");
    let mut opt = ConnectOptions::new(database_url);
    opt.connect_timeout(Duration::from_secs(1));
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .into_diagnostic()
        .wrap_err("failed to connect to database")?;

    drop_table(&db, PoolMapEntity).await;
    drop_table(&db, PoolBracketEntity).await;
    drop_table(&db, RankRestrictionEntity).await;
    drop_table(&db, StageEntity).await;
    drop_table(&db, CountryRestrictionEntity).await;
    drop_table(&db, TournamentEntity).await;

    create_table(&db, TournamentEntity).await;
    create_table(&db, CountryRestrictionEntity).await;
    create_table(&db, StageEntity).await;
    create_table(&db, RankRestrictionEntity).await;
    create_table(&db, PoolBracketEntity).await;
    create_table(&db, PoolMapEntity).await;
    info!("connected to database and setup tables");

    Ok(db)
}

async fn setup_redis() -> miette::Result<redis::aio::MultiplexedConnection> {
    let redis_url = std::env::var("REDIS_URL")
        .into_diagnostic()
        .wrap_err("REDIS_URL not set")?;
    info!("connecting to redis");
    let client = redis::Client::open(redis_url)
        .into_diagnostic()
        .wrap_err("error connecting to redis")?;

    let conn = client
        .get_multiplexed_tokio_connection()
        .await
        .into_diagnostic()
        .wrap_err("error connecting to redis")?;
    info!("connection to redis successful");

    Ok(conn)
}

async fn setup_osu() -> miette::Result<Arc<Osu>> {
    let osu_client_id = std::env::var("OSU_CLIENT_ID")
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID not set")?
        .parse::<u64>()
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID must be a non-negative integer")?;

    let osu_client_secret = std::env::var("OSU_CLIENT_SECRET")
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_SECRET not set")?;
    info!("connecting to osu api...");
    let osu = Arc::new(
        Osu::new(osu_client_id, osu_client_secret)
            .await
            .into_diagnostic()
            .wrap_err("error connecting to osu api")?,
    );
    info!("connection to osu api successful");
    Ok(osu)
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
