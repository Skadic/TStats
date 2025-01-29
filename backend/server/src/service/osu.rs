use std::sync::Arc;

use miette::{Context, IntoDiagnostic};
use model::dto::{osu::OsuUserDto, pool::BeatmapDto};
use rosu_v2::Osu;
use utils::Cacheable;

use crate::RedisConnectionPool;

#[derive(Clone)]
pub struct OsuService {
    osu: Arc<Osu>,
    redis: RedisConnectionPool,
}

impl OsuService {
    pub fn new(osu: Arc<Osu>, redis: RedisConnectionPool) -> Arc<Self> {
        Arc::new(Self { osu, redis })
    }

    #[tracing::instrument(skip(self), name = "get_osu_user")]
    pub async fn get_user(&self, user_id: u32) -> miette::Result<OsuUserDto> {
        tracing::info!("fetching osu user {user_id}");
        let osu = Arc::clone(&self.osu);
        let osu_user =
            OsuUserDto::get_cached_or(&self.redis, &user_id, Some(86400), || async move {
                osu.user(user_id)
                    .await
                    .map(Into::into)
                    .into_diagnostic()
                    .wrap_err_with(|| {
                        format!("could not get osu user with id '{user_id}' from api")
                    })
            })
            .await?;
        Ok(osu_user)
    }

    //#[tracing::instrument(skip(self), name = "get_beatmap")]
    pub async fn get_beatmap(&self, map_id: u32) -> miette::Result<BeatmapDto> {
        tracing::info!("fetching beatmap {map_id}");
        BeatmapDto::get_cached_or(&self.redis, &map_id, Some(86400), || async move {
            let bms = self
                .osu
                .beatmapset_from_map_id(map_id)
                .await
                .into_diagnostic()
                .wrap_err("could not fetch beatmap")?;
            let creator = self.get_user(bms.creator_id).await?;
            BeatmapDto::new(map_id, bms, creator)
                .wrap_err("could not convert beatmap and user into beatmap dto")
        })
        .await
        .wrap_err_with(|| format!("could not get beatmap with id {map_id}"))
    }
}
