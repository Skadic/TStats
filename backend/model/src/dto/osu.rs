use poem_openapi::Object;
use rosu_v2::prelude::UserExtended;
use url::Url;
use utils::Cacheable;

/// An osu user
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Object)]
#[oai(rename = "OsuUser", rename_all = "camelCase")]
pub struct OsuUserDto {
    /// The osu user id
    pub user_id: u32,
    /// The username with a maximum length of 15 characters
    pub username: String,
    /// 2-Character country code
    pub country: String,
    /// The file name of the user's profile banner
    pub cover_url: String,
}

impl From<UserExtended> for OsuUserDto {
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

impl Cacheable for OsuUserDto {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuuser"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}
