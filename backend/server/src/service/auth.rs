use std::{ops::Deref, sync::Arc, time::Duration};

use futures::TryFutureExt;
use miette::{Context, IntoDiagnostic};
use model::dto::auth::AuthenticatedUserDto;
use oauth2::{
    basic::BasicClient, AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use url::Url;

use utils::{
    cache::{CacheError, CacheResult},
    crypt::EncryptedToken,
    Cacheable,
};

use crate::RedisConnectionPool;

type OAuthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

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
    oauth_client: OAuthClient,
    http_client: oauth2::reqwest::Client,
    redis: RedisConnectionPool,
}

impl AuthService {
    pub fn new(redis: &RedisConnectionPool) -> Arc<Self> {
        Arc::new(Self {
            oauth_client: get_auth_client(),
            http_client: oauth2::reqwest::Client::new(),
            redis: redis.clone(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionContent {
    pub user: AuthenticatedUserDto,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub expires_in: Duration,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("token missing in response from osu api: {0}")]
    NoTokenFromOsuApi(&'static str),
    #[error("error in osu oauth flow: {0}")]
    OAuthError(&'static str),
    #[error("CSRF token missing in cache")]
    NoCsrfToken,
    #[error("CSRF token mismatch")]
    CsrfTokenMismatch,
    #[error("error fetching osu user")]
    ErrorFetchingOsuUser,
    #[error(transparent)]
    CacheError(#[from] CacheError),
}

#[derive(Debug)]
pub struct AuthResult {
    pub session: SessionContent,
    pub return_url: String,
}

impl AuthService {
    #[tracing::instrument(skip_all)]
    pub async fn request_auth_code(&self, return_url: &str) -> Result<Url, AuthError> {
        let auth_url = OsuAuthCode::request(return_url, &self.oauth_client, &self.redis)
            .map_err(|_| AuthError::OAuthError("error requesting auth code"))
            .await?;
        Ok(auth_url)
    }

    #[tracing::instrument(skip_all)]
    pub async fn deliver_auth_code(
        &self,
        auth_code: &str,
        csrf_token: &str,
    ) -> Result<AuthResult, AuthError> {
        let auth_code = AuthorizationCode::new(auth_code.to_string());
        let redis = &self.redis;
        let client = &self.oauth_client;

        info!(state = csrf_token, "trying to sign in user");

        // Check whether the CSRF token received from the server matches the one from the cache
        let cached_csrf_token = OsuCsrfToken::uncache(redis, csrf_token)
            .await?
            .ok_or(AuthError::NoCsrfToken)?;

        if cached_csrf_token.secret() != &csrf_token {
            return Err(AuthError::CsrfTokenMismatch);
        }

        // Request Auth Token from osu API
        let token = client
            .exchange_code(auth_code)
            .request_async(&self.http_client)
            .map_err(|_| AuthError::OAuthError("could not get token from osu API"))
            .await?;

        let access_token = token.access_token().clone();
        let refresh_token = token
            .refresh_token()
            .ok_or(AuthError::OAuthError("osu API did not send refresh token"))?
            .clone();
        let expires_in = token
            .expires_in()
            .ok_or(AuthError::OAuthError("osu API did not send refresh token"))?;

        let user = self
            .request_user_data(access_token.secret().as_str())
            .await
            .map_err(|_| AuthError::ErrorFetchingOsuUser)?;
        let user_id = user.user_id;

        tracing::debug!(user_id, "successfully authenticated user");

        // All is well, so we save the accesss token and refresh token
        return Ok(AuthResult {
            session: SessionContent {
                user: user.into(),
                access_token,
                refresh_token,
                expires_in,
            },
            return_url: cached_csrf_token.1,
        });
    }

    #[tracing::instrument(skip_all)]
    async fn request_user_data(
        &self,
        access_token: &str,
    ) -> miette::Result<rosu_v2::model::user::User> {
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

        let resp = self
            .http_client
            .get(Url::parse("https://osu.ppy.sh/api/v2/me").unwrap())
            .headers(headers)
            .send()
            .await
            .into_diagnostic()
            .wrap_err("could not fetch osu user")?;

        let body = resp
            .bytes()
            .await
            .into_diagnostic()
            .wrap_err("could not get bytes of osu user")?;

        // We should have now received the user data. If not, we're probably not authenticated yet
        let body_content = String::from_utf8_lossy(&body);
        let user = serde_json::from_str::<rosu_v2::model::user::User>(body_content.as_ref())
            .into_diagnostic()
            .wrap_err("could not parse data from osu API")?;

        Ok(user)
    }
}

impl OsuAuthCode {
    pub async fn request(
        return_url: impl AsRef<str>,
        client: &OAuthClient,
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

pub fn get_auth_client() -> OAuthClient {
    // We know these exist and are valid. Otherwise, this app wouldn't be running
    let client_id = std::env::var(crate::OSU_CLIENT_ID).unwrap();
    let client_secret = std::env::var(crate::OSU_CLIENT_SECRET).unwrap();
    BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new("https://osu.ppy.sh/oauth/authorize".to_string()).unwrap())
        .set_token_uri(TokenUrl::new("https://osu.ppy.sh/oauth/token".to_string()).unwrap())
        .set_redirect_uri(
            RedirectUrl::new("http://localdev.skadic.moe:5173/auth".to_string()).unwrap(),
        )
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
