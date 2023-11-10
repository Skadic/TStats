use std::borrow::Cow;

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

    fn key(&self) -> Self::KeyType {
        self.user_id
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

    fn key(&self) -> Self::KeyType {
        self.user_id
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuAuthCode {
    pub user_id: u32,
    pub code: oauth2::AuthorizationCode,
}

impl OsuAuthCode {
    pub async fn request<Conn: AsyncCommands + Send + Sync>(
        user_id: u32,
        client: &BasicClient,
        redis: &mut Conn,
    ) -> CacheResult<Url> {
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .set_redirect_uri(Cow::Owned(
                // TODO update hard-coded frontend IP
                RedirectUrl::new("http://localhost:5173".into()).unwrap(),
            ))
            .add_scope(Scope::new("public".into()))
            .add_scope(Scope::new("identify".into()))
            .url();

        let state = OsuAuthState {
            user_id,
            state: csrf_token,
        };

        cache(redis, &state, Some(300)).await?;

        println!("Browse to: {}", &auth_url);
        Ok(auth_url)
    }
}

impl Cacheable for OsuAuthCode {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuauthcode"
    }

    fn key(&self) -> Self::KeyType {
        self.user_id
    }
}

/// A state string associated with an osu user, used in an authorization grant
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuAuthState {
    pub user_id: u32,
    pub state: oauth2::CsrfToken,
}

impl Cacheable for OsuAuthState {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuauthstate"
    }

    fn key(&self) -> Self::KeyType {
        self.user_id
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
    .set_redirect_uri(RedirectUrl::new("http://localhost:5173".to_string()).unwrap())
}
