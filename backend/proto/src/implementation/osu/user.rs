use rosu_v2::prelude::UserExtended;
use url::Url;
use utils::Cacheable;

impl From<UserExtended> for crate::osu::User {
    fn from(value: UserExtended) -> Self {
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
            username: value.username.to_string(),
            country: value.country_code.to_string(),
            cover_url: url,
        }
    }
}

impl Cacheable for crate::osu::User {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuuser"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}
