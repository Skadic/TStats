use std::ops::Deref;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use url::Url;

use utils::{cache::CacheResult, crypt::EncryptedToken, Cacheable};

use crate::RedisConnectionPool;

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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct OsuCsrfToken(pub oauth2::CsrfToken);

impl Deref for OsuCsrfToken {
    type Target = oauth2::CsrfToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OsuAuthCode {
    pub async fn request(client: &BasicClient, redis: &RedisConnectionPool) -> CacheResult<Url> {
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public".into()))
            .add_scope(Scope::new("identify".into()))
            .url();

        OsuCsrfToken(csrf_token).cache(redis, Some(300)).await?;

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

impl Cacheable for OsuCsrfToken {
    type KeyType = str;

    fn type_key() -> &'static str {
        "oauthcsrftoken"
    }

    fn key(&self) -> &Self::KeyType {
        self.0.secret().as_str()
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
    .set_redirect_uri(RedirectUrl::new("http://localdev.skadic.moe:5173/auth".to_string()).unwrap())
    .set_auth_type(oauth2::AuthType::RequestBody)
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub osu_user_id: u32,
}

impl Cacheable for Session {
    type KeyType = str;

    fn type_key() -> &'static str {
        "session"
    }

    fn key(&self) -> &Self::KeyType {
        self.session_id.as_str()
    }
}

impl Session {
    pub fn new(osu_user_id: u32) -> Self {
        Self {
            session_id: Self::generate_session_id(),
            osu_user_id,
        }
    }

    fn generate_session_id() -> String {
        let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
        let mut buf = [0u8; 16];
        rng.fill_bytes(&mut buf);
        BASE64_STANDARD.encode(buf)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OsuApiTokens {
    pub user_id: u32,
    pub access_token: EncryptedToken,
    pub refresh_token: EncryptedToken,
}

impl Cacheable for OsuApiTokens {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "osuapitokens"
    }

    fn key(&self) -> &Self::KeyType {
        &self.user_id
    }
}
