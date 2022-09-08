use std::fmt::Display;

use redis::RedisError;
use rosu_v2::prelude::OsuError;

/// Implements simple from conversions for Error enum types
macro_rules! quick_from_err {
    ($err:path, $src:path, $var:ident) => {
        impl From<$src> for $err {
            fn from(err: $src) -> Self {
                Self::$var(err)
            }
        }
    };
}

/// An error that might occur when trying to retrieve a cached value using [crate::util::get_cached]
#[derive(Debug)]
pub enum CacheError {
    Redis(RedisError),
    Json(serde_json::error::Error),
    Osu(OsuError),
}

quick_from_err!(CacheError, RedisError, Redis);
quick_from_err!(CacheError, serde_json::error::Error, Json);
quick_from_err!(CacheError, OsuError, Osu);

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