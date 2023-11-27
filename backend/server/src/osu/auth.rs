use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use redis::AsyncCommands;
use url::Url;

use crate::cache::{cache, CacheResult, Cacheable};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuRefreshToken {
    pub user_id: u32,
    pub token: oauth2::RefreshToken,
}

impl Cacheable for OsuRefreshToken {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osurefreshtoken"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuAccessToken {
    pub user_id: u32,
    pub token: oauth2::AccessToken,
}

impl Cacheable for OsuAccessToken {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuaccesstoken"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuAuthCode {
    pub user_id: u32,
    pub code: oauth2::AuthorizationCode,
}

impl OsuAuthCode {
    pub async fn request<Conn: AsyncCommands + Send + Sync>(
        client: &BasicClient,
        redis: &mut Conn,
    ) -> CacheResult<Url> {
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public".into()))
            .add_scope(Scope::new("identify".into()))
            .url();

        cache(redis, &csrf_token, Some(300)).await?;

        tracing::debug!("Browse to: {}", &auth_url);
        Ok(auth_url)
    }
}

impl Cacheable for OsuAuthCode {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuauthcode"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}

impl Cacheable for oauth2::CsrfToken {
    type KeyType = str;

    fn type_key() -> &'static str {
        "oauthcsrftoken"
    }

    fn key(&self) -> &Self::KeyType {
        self.secret().as_str()
    }
}

pub fn get_auth_client() -> BasicClient {
    // We know these exist and are valid. Otherwise, this app wouldn't be running
    let client_id = std::env::var(crate::OSU_CLIENT_ID).unwrap();
    let client_secret = std::env::var(crate::OSU_CLIENT_SECRET).unwrap();
    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        // These URLs are static. They will parse
        AuthUrl::new("https://osu.ppy.sh/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://osu.ppy.sh/oauth/token".to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new("http://0.0.0.0:5173/auth".to_string()).unwrap())
    .set_auth_type(oauth2::AuthType::RequestBody)
}
