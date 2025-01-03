use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use deadpool_redis::Config;
use http::{HeaderValue, Method};
use miette::{miette, Context, IntoDiagnostic};
use poem::listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener};
use poem::middleware::Cors;
use poem::session::{CookieConfig, CookieSession};
use poem::web::cookie::{CookieKey, SameSite};
use poem::{handler, options, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use rosu_v2::Osu;
use routes::auth::AuthApi;
use routes::tournament::TournamentApi;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use service::TStatsServices;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{error, info, info_span, warn};

use routes::debug::DebugApi;

use utils::{consts::*, TStatsPaths};

type RedisConnection = deadpool_redis::Connection;
type RedisConnectionPool = deadpool_redis::Pool;

mod routes;
mod service;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub sqlx: PgPool,
    pub osu: Arc<Osu>,
    pub redis: RedisConnectionPool,
    pub paths: TStatsPaths,
    pub services: TStatsServices,
}

impl AppState {
    pub async fn redis_connection(&self) -> Result<RedisConnection, miette::Error> {
        self.redis.get().await.map_err(|e| {
            error!(source = %e, "could not get redis connection");
            miette!("could not connect to redis")
        })
    }
}

const KEY: &str = include_str!("../../../certs/private.key.pem");
const CERT: &str = include_str!("../../../certs/domain.cert.pem");

#[tracing::instrument]
pub async fn run_server() -> miette::Result<()> {
    let server_setup_span = info_span!("server_setup").entered();
    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        warn!("could not read .env file. expecting environment variables to be defined: {e}");
    }
    let session_signing_key = parse_env(SESSION_SIGNING_KEY, || String::new())
        .and_then(|s| BASE64_STANDARD.decode(s).into_diagnostic())
        .map(|v| CookieKey::from(&v))?;

    let (db, redis, osu) = tokio::join!(setup_database(), setup_redis(), setup_osu());
    let ((db, sqlx), redis, osu) = (db?, redis?, osu?);
    let services = TStatsServices::new(&redis);

    let base_path = parse_env(TSTATS_DATA_DIR, || {
        std::env::current_dir()
            .expect("could not get working directory")
            .join("tsdata")
    })?;
    let paths = TStatsPaths::new(base_path)
        .into_diagnostic()
        .wrap_err("could not canonicalize path")?;
    info!("Storing data in {:?}", paths.base());

    let state = AppState {
        db,
        sqlx,
        redis,
        osu,
        paths,
        services,
    };

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

    let openapi_service = OpenApiService::new(
        (
            DebugApi(state.clone()),
            TournamentApi(state.clone()),
            AuthApi {
                auth_service: Arc::clone(&state.services.auth),
            },
        ),
        "TStats API",
        "0.1",
    );
    let spec_endpoint = openapi_service.spec_endpoint();
    let swagger_ui = openapi_service.swagger_ui();

    let route = Route::new()
        .at("/*", options(cors_handler))
        .nest("/api", openapi_service)
        .nest("/swagger", swagger_ui)
        .nest("/spec", spec_endpoint)
        .with(
            Cors::new()
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([
                    "Authorization",
                    "content-length",
                    "Origin",
                    "Content-Type",
                    "X-Auth-Token",
                ])
                .allow_origin(frontend_addr)
                .allow_credentials(true),
        )
        .with(poem::middleware::Tracing)
        .with(CookieSession::new(
            CookieConfig::signed(session_signing_key)
                .max_age(Some(Duration::from_secs(86400)))
                .same_site(Some(SameSite::None))
                .secure(true)
                .max_age(Duration::from_secs(86400))
                .domain(".skadic.moe"),
        ));

    Server::new(
        TcpListener::bind(addr)
            .rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(KEY).cert(CERT))),
    )
    .run(route)
    .await
    .into_diagnostic()
}

#[handler]
fn cors_handler() -> http::StatusCode {
    http::StatusCode::OK
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
            .wrap_err_with(|| format!("could not parse {env_var} (value is {value})")),
        Err(e) => {
            warn!("could not read {env_var} ({e}), using default");
            Ok(default_fn())
        }
    }
}

#[tracing::instrument]
async fn setup_database() -> miette::Result<(DatabaseConnection, PgPool)> {
    let database_url = std::env::var(DATABASE_URL)
        .into_diagnostic()
        .wrap_err("DATABASE_URL not set")?;
    info!("connecting to database...");
    let mut opt = ConnectOptions::new(&database_url);
    opt.connect_timeout(Duration::from_secs(1));
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .into_diagnostic()
        .wrap_err("failed to connect to database")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .into_diagnostic()
        .wrap_err("failed to connect to database")?;

    model::migrate(&pool)
        .await
        .into_diagnostic()
        .wrap_err("could not migrate database")?;

    info!("connected to and setup database");

    Ok((db, pool))
}

#[tracing::instrument]
async fn setup_redis() -> miette::Result<deadpool_redis::Pool> {
    let redis_url = std::env::var(REDIS_URL)
        .into_diagnostic()
        .wrap_err("REDIS_URL not set")?;
    info!("connecting to redis");

    let cfg = Config::from_url(redis_url);
    let pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .into_diagnostic()
        .wrap_err("could not create redis connection pool")?;

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
