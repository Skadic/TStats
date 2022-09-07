use std::fmt::Display;

use redis::RedisError;
use rosu_v2::prelude::OsuError;

#[derive(Debug)]
pub enum CacheError {
    Redis(RedisError),
    Json(serde_json::error::Error),
    Osu(OsuError),
}

impl From<RedisError> for CacheError {
    fn from(err: RedisError) -> Self {
        Self::Redis(err)
    }
}

impl From<serde_json::error::Error> for CacheError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::Json(err)
    }
}

impl From<OsuError> for CacheError {
    fn from(err: OsuError) -> Self {
        Self::Osu(err)
    }
}

impl Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CacheError::*;
        write!(
            f,
            "{}",
            match &self {
                Redis(_) => "Error communicating with the Redis cache",
                Json(_) => "Error (de-)serializing the requested value",
                Osu(_) => "Error communicating with the Osu Api",
            }
        )
    }
}
