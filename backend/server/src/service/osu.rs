use std::sync::Arc;

use miette::{Context, IntoDiagnostic};
use model::dto::osu::OsuUserDto;
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
        tracing::info!("fetching osu user");
        let osu = Arc::clone(&self.osu);
        let osu_user =
            OsuUserDto::get_cached_or(&self.redis, &user_id, Some(86400), || async move {
                osu.user(user_id).await.map(Into::into)
            })
            .await
            .into_diagnostic()
            .wrap_err("could not get osu user from api")?;
        tracing::info!("osu user fetched successfully");
        Ok(osu_user)
    }
}
