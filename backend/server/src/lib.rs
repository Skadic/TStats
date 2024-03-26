use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use deadpool_redis::Config;
use http::{HeaderName, HeaderValue, Method};
use miette::{Context, IntoDiagnostic};
use proto::{
    osu_auth::osu_auth_service_server::OsuAuthServiceServer,
    pool::pool_service_server::PoolServiceServer, stages::stage_service_server::StageServiceServer,
};
use rosu_v2::prelude::*;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tonic::server::NamedService;
use tonic::Status;
use tonic_health::server::HealthReporter;
use tonic_web::GrpcWebLayer;
use tower_http::{
    cors::{AllowHeaders, CorsLayer, ExposeHeaders},
    trace::{self, TraceLayer},
};
use tracing::{debug, error, info, info_span, warn, Level};

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
use crate::routes::osu_auth::OsuAuthServiceImpl;
use crate::routes::pool::PoolServiceImpl;
use crate::routes::stage::StageServiceImpl;
use crate::routes::tournament::TournamentServiceImpl;

use utils::consts::*;

type RedisConnection = deadpool_redis::Connection;
type RedisConnectionPool = deadpool_redis::Pool;

mod osu;
mod routes;

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    osu: Arc<Osu>,
    redis: RedisConnectionPool,
}

impl AppState {
    pub async fn redis_connection(&self) -> Result<RedisConnection, tonic::Status> {
        self.redis.get().await.map_err(|e| {
            error!(source = %e, "could not get redis connection");
            Status::internal("could not connect to redis")
        })
    }
}

pub async fn run_server() -> miette::Result<()> {
    let server_setup_span = info_span!("server_setup").entered();
    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        warn!("could not read .env file. expecting environment variables to be defined: {e}");
    }

    //session::crypt::verify_aes_key().into_diagnostic()?;

    let (db, redis, osu) = tokio::join!(setup_database(), setup_redis(), setup_osu());
    let (db, redis, osu) = (db?, redis?, osu?);

    let state = AppState { db, redis, osu };

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

    let frontend_method = parse_env(FRONTEND_METHOD, || "http".to_owned())?;
    let frontend_host: String = parse_env(FRONTEND_HOST, || "localhost".to_owned())?;
    let frontend_port = parse_env(FRONTEND_PORT, || "5173".to_owned())?;
    let host: IpAddr = parse_env(BACKEND_HOST, || IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))?;
    let port = parse_env(BACKEND_PORT, || 3000)?;

    let frontend_addr: HeaderValue = format!("{frontend_method}://{frontend_host}:{frontend_port}")
        .parse()
        .into_diagnostic()
        .wrap_err("could not parse frontend url: {e}")?;
    let addr: SocketAddr = SocketAddr::new(host, port);

    info!("Serving at {addr}");
    info!("Allowing requests from {frontend_addr:?}");

    info!("Starting server");

    drop(server_setup_span);

    let auth_interceptor_state = state.clone();
    let auth_interceptor = move |req: tonic::Request<()>| {
        /*
                let auth_header_token =
                if let Some(c) = req.metadata().get("authorization") {
                    warn!("auth cookie is set!: {:?}", c);
                    c.to_str().map_err(|e|{ error!("non-unicode bearer token"); Status::unauthenticated("non-unicode bearer token")})?
                } else {
                    error!("auth cookie is not set!");
                    return Err(Status::unauthenticated("no bearer token"))
                };
        */
        //OsuAccessToken::get_cached(&0, auth_interceptor_state.redis.get().await.map_err(|err| {error!(src = %err, "could not get redis connection"); Status::internal("could not check authorization")})?);

        Ok(req)
    };

    // Build the gRPC server
    tonic::transport::server::Server::builder()
        .accept_http1(true)
        // Layers to apply to the gRPC services
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::DEBUG))
                .on_request(trace::DefaultOnRequest::new().level(Level::DEBUG))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_origin([frontend_addr])
                .allow_headers(AllowHeaders::any())
                .expose_headers(ExposeHeaders::list([
                    HeaderName::from_static("grpc-status"),
                    HeaderName::from_static("grpc-message"),
                ])),
        )
        .layer(GrpcWebLayer::new())
        .layer(tonic::service::interceptor(cors_interceptor))
        .layer(tonic::service::interceptor(auth_interceptor))
        // The gRPC services
        .add_service(reflection_server)
        .add_service(DebugServiceServer::new(DebugServiceImpl(state.clone())))
        .add_service(TournamentServiceServer::new(TournamentServiceImpl(
            state.clone(),
        )))
        .add_service(StageServiceServer::new(StageServiceImpl(state.clone())))
        .add_service(PoolServiceServer::new(PoolServiceImpl(state.clone())))
        .add_service(OsuAuthServiceServer::new(OsuAuthServiceImpl(
            state.clone(),
            osu::auth::get_auth_client(),
        )))
        .add_service(health_server)
        .serve(addr)
        .await
        .into_diagnostic()
}

/// Intercepts cors requests so they are not forwarded to the actual handler
fn cors_interceptor(req: tonic::Request<()>) -> tonic::Result<tonic::Request<()>> {
    debug!(?req);
    let is_cors = match req.metadata().get("sec-fetch-mode") {
        Some(fetch_mode) if fetch_mode == "cors" => true,
        Some(_) | None => false,
    };
    if is_cors {
        Err(Status::ok("cors request".to_string()))
    } else {
        Ok(req)
    }
}

/// Reads an environment variable and tries to parse it into the specified type.
/// If the variable is not set, this generates a default value from the given closure.
///
/// # Errors
///
/// If parsing fails, this will return an error.
#[tracing::instrument(skip(default_fn))]
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

#[tracing::instrument]
async fn setup_database() -> miette::Result<DatabaseConnection> {
    let database_url = std::env::var(DATABASE_URL)
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

#[tracing::instrument]
async fn setup_redis() -> miette::Result<deadpool_redis::Pool> {
    let redis_url = std::env::var(REDIS_URL)
        .into_diagnostic()
        .wrap_err("REDIS_URL not set")?;
    info!("connecting to redis");

    let cfg = Config::from_url(&redis_url);
    let pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .into_diagnostic()
        .wrap_err("could not create redis connection pool")?;

    /*
    let client = redis::Client::open(redis_url)
        .into_diagnostic()
        .wrap_err("error connecting to redis")?;

    let conn = client
        .get_multiplexed_tokio_connection()
        .await
        .into_diagnostic()
        .wrap_err("error connecting to redis")?;
    */
    info!("connection to redis successful");

    Ok(pool)
}

#[tracing::instrument]
async fn setup_osu() -> miette::Result<Arc<Osu>> {
    let osu_client_id = std::env::var(OSU_CLIENT_ID)
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID not set")?
        .parse::<u64>()
        .into_diagnostic()
        .wrap_err("OSU_CLIENT_ID must be a non-negative integer")?;

    let osu_client_secret = std::env::var(OSU_CLIENT_SECRET)
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
