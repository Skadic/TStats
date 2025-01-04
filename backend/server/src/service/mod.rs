use std::sync::Arc;

use crate::RedisConnectionPool;

mod auth;
mod stage;
mod tournament;

pub use auth::*;
use sea_orm::DatabaseConnection;
pub use stage::*;
pub use tournament::*;
use utils::TStatsPaths;

#[derive(Clone)]
pub struct TStatsServices {
    pub auth: Arc<AuthService>,
    pub tournaments: Arc<TournamentService>,
    pub stage: Arc<StageService>,
}

impl TStatsServices {
    pub fn new(
        db: &DatabaseConnection,
        redis: &RedisConnectionPool,
        tstats_paths: &TStatsPaths,
    ) -> Self {
        Self {
            auth: AuthService::new(redis),
            tournaments: TournamentService::new(db.clone(), tstats_paths.clone()),
            stage: StageService::new(db.clone()),
        }
    }
}
