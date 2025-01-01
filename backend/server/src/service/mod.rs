use std::sync::Arc;

use crate::RedisConnectionPool;

mod auth;

pub use auth::*;

#[derive(Clone)]
pub struct TStatsServices {
    pub auth: Arc<AuthService>,
}

impl TStatsServices {
    pub fn new(redis: &RedisConnectionPool) -> Self {
        Self {
            auth: AuthService::new(redis),
        }
    }
}
