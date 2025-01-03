use std::{ops::Deref, sync::Arc, time::Duration};

use http::StatusCode;
use oauth2::{
    basic::BasicClient, AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use url::Url;

use utils::{
    cache::CacheResult, crypt::EncryptedToken, Cacheable, LogPoemError, LogPoemErrorFuture,
};

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
pub struct OsuCsrfToken(pub oauth2::CsrfToken, pub String);

impl Deref for OsuCsrfToken {
    type Target = oauth2::CsrfToken;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct AuthService {
    oauth_client: BasicClient,
    redis: RedisConnectionPool,
}

impl AuthService {
    pub fn new(redis: &RedisConnectionPool) -> Arc<Self> {
        Arc::new(Self {
            oauth_client: get_auth_client(),
            redis: redis.clone(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContent {
    pub user_id: u32,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub expires_in: Duration,
}

#[derive(Debug)]
pub struct AuthResult {
    pub session: SessionContent,
    pub return_url: String,
}

impl AuthService {
    #[tracing::instrument(skip_all)]
    pub async fn request_auth_code(&self, return_url: &str) -> poem::Result<Url> {
        let auth_url = OsuAuthCode::request(return_url, &self.oauth_client, &self.redis)
            .log_internal_server_error("error requesting auth code")
            .await?;
        Ok(auth_url)
    }

    #[tracing::instrument(skip_all)]
    pub async fn deliver_auth_code(
        &self,
        auth_code: &str,
        csrf_token: &str,
    ) -> poem::Result<AuthResult> {
        let auth_code = AuthorizationCode::new(auth_code.to_string());
        let redis = &self.redis;
        let client = &self.oauth_client;

        // Check whether the CSRF token received from the server matches the one from the cache
        let cached_csrf_token = OsuCsrfToken::uncache(redis, csrf_token)
            .log_internal_server_error("error fetching CSRF token")
            .await?
            .log_error("missing CSRF token in cache", StatusCode::UNAUTHORIZED)?;

        if cached_csrf_token.secret() != &csrf_token {
            return None.log_internal_server_error("CSRF token mismatch");
        }

        // Request Auth Token from osu API
        let token = client
            .exchange_code(auth_code)
            .request_async(oauth2::reqwest::async_http_client)
            .log_internal_server_error("could not get token from osu API")
            .await?;

        let access_token = token.access_token().clone();
        let refresh_token = token
            .refresh_token()
            .log_internal_server_error("osu API did not send refresh token")?
            .clone();
        let expires_in = token
            .expires_in()
            .log_internal_server_error("osu API did not send token expiry")?;

        let user = request_user_data(access_token.secret().as_str()).await?;
        let user_id = user.user_id;

        tracing::info!(user_id, "successfully authenticated user");

        // All is well, so we save the accesss token and refresh token
        return Ok(AuthResult {
            session: SessionContent {
                user_id,
                access_token,
                refresh_token,
                expires_in,
            },
            return_url: cached_csrf_token.1,
        });
    }
}

#[tracing::instrument(skip_all)]
async fn request_user_data(access_token: &str) -> poem::Result<rosu_v2::model::user::User> {
    use oauth2::http::{HeaderMap, HeaderName, HeaderValue};
    // Try to request the current user's data
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("accept"),
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {access_token}")).unwrap(),
    );
    let resp = oauth2::reqwest::async_http_client(oauth2::HttpRequest {
        url: Url::parse("https://osu.ppy.sh/api/v2/me").unwrap(),
        method: oauth2::http::Method::GET,
        headers,
        body: vec![],
    })
    .await
    .log_error(
        "could not request user data using token",
        StatusCode::UNAUTHORIZED,
    )?;

    // We should have now received the user data. If not, we're probably not authenticated yet
    let body_content = String::from_utf8_lossy(resp.body.as_slice());
    let user = serde_json::from_str::<rosu_v2::model::user::User>(body_content.as_ref())
        .log_error(
            "could not parse data from osu API",
            StatusCode::UNAUTHORIZED,
        )?;

    Ok(user)
}

impl OsuAuthCode {
    pub async fn request(
        return_url: impl AsRef<str>,
        client: &BasicClient,
        redis: &RedisConnectionPool,
    ) -> CacheResult<Url> {
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public".into()))
            .add_scope(Scope::new("identify".into()))
            .url();

        OsuCsrfToken(csrf_token, return_url.as_ref().to_string())
            .cache(redis, Some(300))
            .await?;

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
