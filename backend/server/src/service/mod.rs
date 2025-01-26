use std::sync::Arc;

use crate::RedisConnectionPool;

mod auth;
mod osu;
mod stage;
mod tournament;

pub use auth::*;
pub use osu::*;
use rosu_v2::Osu;
use sea_orm::DatabaseConnection;
pub use stage::*;
pub use tournament::*;
use utils::TStatsPaths;

#[derive(Clone)]
pub struct TStatsServices {
    auth: Arc<AuthService>,
    tournament: Arc<TournamentService>,
    stage: Arc<StageService>,
    osu: Arc<OsuService>,
}

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

impl TStatsServices {
    pub fn new(
        db: &DatabaseConnection,
        redis: &RedisConnectionPool,
        tstats_paths: &TStatsPaths,
        osu: Arc<Osu>,
    ) -> Self {
        Self {
            auth: AuthService::new(redis),
            tournament: TournamentService::new(db.clone(), tstats_paths.clone()),
            stage: StageService::new(db.clone()),
            osu: OsuService::new(osu, redis.clone()),
        }
    }

    service_getter! {
        auth -> AuthService,
        tournament -> TournamentService,
        stage -> StageService,
        osu -> OsuService,
    }
}
