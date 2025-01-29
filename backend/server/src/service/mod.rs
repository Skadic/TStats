use std::sync::Arc;

use crate::RedisConnectionPool;

mod auth;
mod osu;
mod pool;
mod stage;
mod tournament;

pub use auth::*;
pub use osu::*;
pub use pool::*;
pub use stage::*;
pub use tournament::*;

use rosu_v2::Osu;
use sea_orm::DatabaseConnection;
use utils::TStatsPaths;

macro_rules! service_getter {
    { $service:ident -> $t:ty $(,)? } => {
        pub fn $service(&self) -> Arc<$t> {
            Arc::clone(&self.$service)
        }
    };
    { $service:ident -> $t:ty, $($services:ident -> $ts:ident),+ $(,)? } => {
        service_getter! { $service -> $t, }
        service_getter! { $($services -> $ts),+ }
    }
}
#[derive(Clone)]
pub struct TStatsServices {
    auth: Arc<AuthService>,
    tournament: Arc<TournamentService>,
    stage: Arc<StageService>,
    pool: Arc<PoolService>,
    osu: Arc<OsuService>,
}

impl TStatsServices {
    pub fn new(
        db: &DatabaseConnection,
        redis: &RedisConnectionPool,
        tstats_paths: &TStatsPaths,
        osu: Arc<Osu>,
    ) -> Self {
        let osu_service = OsuService::new(osu, redis.clone());
        Self {
            auth: AuthService::new(redis),
            tournament: TournamentService::new(db.clone(), tstats_paths.clone()),
            stage: StageService::new(db.clone()),
            osu: Arc::clone(&osu_service),
            pool: PoolService::new(db.clone(), osu_service)
        }
    }

    service_getter! {
        auth -> AuthService,
        tournament -> TournamentService,
        stage -> StageService,
        osu -> OsuService,
        pool -> PoolService
    }
}