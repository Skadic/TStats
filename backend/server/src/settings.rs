use std::{
    fmt::Display,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use config::{Config, Environment, File};
use miette::{miette, Context, IntoDiagnostic};
use poem::web::cookie::CookieKey;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};
use tracing::info;
use url::Url;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostgresConfig {
    pub url: Url,
    pub username: String,
    pub password: String,
    #[serde(skip)]
    _priv: (),
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: Url,
    #[serde(skip)]
    _priv: (),
}

#[derive(Debug, Deserialize)]
pub struct OsuConfig {
    pub client_id: u64,
    pub client_secret: String,
    #[serde(skip)]
    _priv: (),
}

#[derive(Debug, Deserialize)]
pub struct TStatsConfig {
    pub backend_host: IpAddr,
    pub backend_port: u16,
    pub frontend_addr: Url,
    #[serde(deserialize_with = "deserialize_cookie_key")]
    pub session_signing_key: CookieKey,
    pub log_pretty: bool,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
    pub osu: OsuConfig,
    #[serde(skip)]
    _priv: (),
}

impl TStatsConfig {
    pub fn backend_addr(&self) -> SocketAddr {
        SocketAddr::new(self.backend_host, self.backend_port)
    }
}

fn deserialize_cookie_key<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<CookieKey, D::Error> {
    let s = String::deserialize(deserializer)?;
    let bytes = BASE64_STANDARD
        .decode(s)
        .map_err(|e| serde::de::Error::custom(e))?;
    Ok(CookieKey::from(&bytes))
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TStatsProfile {
    DEV,
    PROD,
}

impl FromStr for TStatsProfile {
    type Err = miette::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "dev" => Self::DEV,
            "prod" => Self::PROD,
            _ => return Err(miette!("invalid profile: {s}")),
        })
    }
}

impl Display for TStatsProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEV => write!(f, "dev"),
            Self::PROD => write!(f, "prod"),
        }
    }
}

impl TStatsConfig {
    pub fn read() -> miette::Result<Self> {
        let profile = std::env::var("TSTATS_PROFILE")
            .ok()
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or_else(|| TStatsProfile::DEV);

        let base_config_path = Self::cfg_file_path(None::<TStatsProfile>)?;
        let profile_config_path = Self::cfg_file_path(Some(profile))?;
        let local_config_path = Self::cfg_file_path(Some("local"))?;

        info!("reading config from {:?}", base_config_path);

        let config = Config::builder()
            .add_source(File::with_name(base_config_path.to_string_lossy().as_ref()).required(true))
            .add_source(
                File::with_name(profile_config_path.to_string_lossy().as_ref()).required(false),
            )
            .add_source(
                File::with_name(local_config_path.to_string_lossy().as_ref()).required(false),
            )
            .add_source(Environment::with_prefix("TSTATS"))
            .build()
            .into_diagnostic()
            .wrap_err("error building config")?;

        config
            .try_deserialize()
            .into_diagnostic()
            .wrap_err("error deserializing config")
    }

    fn cfg_file_path<S: Display>(profile: Option<S>) -> miette::Result<PathBuf> {
        const BASE_FILE_NAME: &str = "config";

        let mut config_location = std::env::var("TSTATS_CONFIG_LOCATION")
            .ok()
            .map(|s| s.parse())
            .transpose()
            .into_diagnostic()
            .wrap_err("config location is not valid path")?
            .unwrap_or_else(|| PathBuf::from("config/tstats"));

        match profile {
            Some(profile) => config_location.push(format!("{BASE_FILE_NAME}-{profile}")),
            None => config_location.push(format!("{BASE_FILE_NAME}")),
        }
        Ok(config_location)
    }
}
