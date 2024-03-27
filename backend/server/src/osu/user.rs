use rosu_v2::{
    prelude::{CountryCode, OsuError, User, Username},
    Osu,
};
use serde::{Deserialize, Serialize};
use url::Url;

use utils::{cache::CacheResult, Cacheable};

use crate::RedisConnectionPool;

/// An osu user
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OsuUser {
    pub user_id: u32,
    pub username: Username,
    pub country: CountryCode,
    pub cover_url: String,
}

impl From<User> for OsuUser {
    fn from(value: User) -> Self {
        let url = match Url::parse(&value.cover.url) {
            Ok(url) => url
                .path_segments()
                .and_then(|iter| iter.last())
                .unwrap_or(&value.cover.url)
                .to_owned(),
            Err(_) => value.cover.url,
        };
        Self {
            user_id: value.user_id,
            username: value.username,
            country: value.country_code,
            cover_url: url,
        }
    }
}

impl Cacheable for OsuUser {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuuser"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}

pub async fn get_user(
    redis: &RedisConnectionPool,
    osu: &Osu,
    user_id: u32,
) -> CacheResult<OsuUser> {
    OsuUser::get_cached_or::<OsuError, _>(redis, &user_id, Some(60), || async {
        let usr = osu.user(user_id).await?;
        Ok(usr.into())
    })
    .await
}
