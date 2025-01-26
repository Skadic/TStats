use std::str::FromStr;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use deadpool_redis::Config;
use http::Method;
use miette::{Context, IntoDiagnostic, Result};
use poem::listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener};
use poem::middleware::Cors;
use poem::session::{CookieConfig, CookieSession};
use poem::web::cookie::SameSite;
use poem::{handler, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use rosu_v2::Osu;
use routes::auth::AuthApi;
use routes::debug::DebugApi;
use routes::stage::StageApi;
use routes::tournament::TournamentApi;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use service::TStatsServices;
use settings::TStatsConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::level_filters::LevelFilter;
use tracing::{error, info, info_span, warn};

use tracing_error::ErrorLayer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use utils::{consts::*, TStatsPaths};

type RedisConnection = deadpool_redis::Connection;
type RedisConnectionPool = deadpool_redis::Pool;

mod routes;
mod service;
mod settings;

static CONFIG: LazyLock<TStatsConfig> = LazyLock::new(|| TStatsConfig::read().unwrap());

pub fn tstats_config() -> &'static TStatsConfig {
    &CONFIG
}

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
    pub async fn redis_connection(&self) -> miette::Result<RedisConnection> {
        self.redis
            .get()
            .await
            .into_diagnostic()
            .wrap_err("could not connect to redis")
            .inspect_err(|e| {
                error!(error = %e, "could not get redis connection");
            })
    }
}

const KEY: &str = include_str!("../../../certs/private.key.pem");
const CERT: &str = include_str!("../../../certs/domain.cert.pem");

//#[tracing::instrument]
pub async fn run_server() -> Result<()> {
    setup_logger();
    let cfg = tstats_config();
    let server_setup_span = info_span!("setup").entered();
    // Load environment variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        warn!("could not read .env file. expecting environment variables to be defined: {e}");
    }

    let state = create_state().await?;
    drop(server_setup_span);

    let openapi_service = OpenApiService::new(
        (
            DebugApi::new(state.db.clone()),
            TournamentApi::new(state.services.tournament()),
            AuthApi::new(state.services.auth(), state.services.osu()),
            StageApi::new(state.services.stage()),
        ),
        "TStats API",
        "0.1",
    );
    let spec_endpoint = openapi_service.spec_endpoint();
    let swagger_ui = openapi_service.swagger_ui();

    let serve_addr = tstats_config().backend_addr();
    let frontend_origin = cfg.frontend_addr.origin().unicode_serialization();

    info!("Serving at {serve_addr}");
    info!("Allowing requests from {}", frontend_origin);

    info!("Starting server");

    let route = Route::new()
        //.at("/*", options(cors_handler))
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
                .allow_origin(frontend_origin)
                .allow_credentials(true),
        )
        .with(poem::middleware::Tracing)
        .with(CookieSession::new(
            CookieConfig::signed(cfg.session_signing_key.clone())
                .max_age(Some(Duration::from_secs(86400)))
                .same_site(Some(SameSite::None))
                .secure(true)
                .max_age(Duration::from_secs(86400))
                .domain(".skadic.moe"),
        ));

    Server::new(
        TcpListener::bind(serve_addr)
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

fn setup_logger() {
    let registry = tracing_subscriber::registry()
        .with(Targets::new().with_targets([
            ("server", LevelFilter::DEBUG),
            ("utils", LevelFilter::DEBUG),
            ("model", LevelFilter::DEBUG),
            ("rosu_v2", LevelFilter::INFO),
            ("tower_http", LevelFilter::INFO),
        ]))
        .with(ErrorLayer::default());
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
}

async fn create_state() -> Result<AppState> {
    let base_path = parse_env(TSTATS_DATA_DIR, || {
        std::env::current_dir()
            .expect("could not get working directory")
            .join("tsdata")
    })?;
    let paths = TStatsPaths::new(base_path)
        .into_diagnostic()
        .wrap_err("could not canonicalize path")?;
    let ((db, sqlx), redis, osu) = tokio::try_join!(setup_database(), setup_redis(), setup_osu())?;
    let services = TStatsServices::new(&db, &redis, &paths, Arc::clone(&osu));

    info!("Storing data in {:?}", paths.base());

    Ok(AppState {
        db,
        sqlx,
        redis,
        osu,
        paths,
        services,
    })
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

#[tracing::instrument(name = "database")]
async fn setup_database() -> miette::Result<(DatabaseConnection, PgPool)> {
    let database_url = tstats_config().postgres.url.as_str();
    info!("connecting to database...");
    let mut opt = ConnectOptions::new(database_url);
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

#[tracing::instrument(name = "redis")]
async fn setup_redis() -> miette::Result<deadpool_redis::Pool> {
    let redis_url = tstats_config().redis.url.clone();
    info!("connecting to redis");

    let cfg = Config::from_url(redis_url);
    let pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .into_diagnostic()
        .wrap_err("could not create redis connection pool")?;

    info!("connection to redis successful");

    Ok(pool)
}

#[tracing::instrument(name = "osu")]
async fn setup_osu() -> miette::Result<Arc<Osu>> {
    let osu_cfg = &tstats_config().osu;
    let osu_client_id = osu_cfg.client_id;
    let osu_client_secret = osu_cfg.client_secret.clone();
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
