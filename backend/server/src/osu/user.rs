use rosu_v2::{
    prelude::{CountryCode, OsuError, User, Username},
    Osu,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::cache::{get_cached_or, Cacheable};

/// An osu user
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OsuUser {
    user_id: u32,
    username: Username,
    country: CountryCode,
    cover_url: String,
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

    fn key(&self) -> Self::KeyType {
        self.user_id
    }
}

pub async fn get_user(
    redis: &mut redis::aio::MultiplexedConnection,
    osu: impl AsRef<Osu>,
    user_id: u32,
) -> OsuUser {
    get_cached_or::<OsuUser, OsuError, _, _>(redis, &user_id, Some(60), || async {
        let usr = osu.as_ref().user(user_id).await?;
        Ok(usr.into())
    })
    .await
    .unwrap()
}
