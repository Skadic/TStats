use futures::TryFutureExt;
use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client,
    AuthorizationCode, CsrfToken, TokenResponse,
};
use proto::osu_auth::{
    osu_auth_service_server::OsuAuthService, DeliverAuthCodeRequest, DeliverAuthCodeResponse,
    RequestAuthCodeRequest, RequestAuthCodeResponse,
};
use tonic::{Request, Response, Status};
use url::Url;

use crate::{
    cache::{cache, get_cached},
    osu::auth::{OsuAccessToken, OsuAuthCode, OsuRefreshToken},
    LocalAppState,
};

pub struct OsuAuthServiceImpl(pub LocalAppState, pub BasicClient);

#[tonic::async_trait]
impl OsuAuthService for OsuAuthServiceImpl {
    async fn request_auth_code(
        &self,
        _request: Request<RequestAuthCodeRequest>,
    ) -> Result<Response<RequestAuthCodeResponse>, Status> {
        let mut redis = self.0.redis.write().await;

        let url = OsuAuthCode::request(&self.1, &mut *redis)
            .map_err(|e| {
                tracing::error!("error requesting auth code: {e}");
                Status::internal(format!("error requesting auth code: {e}"))
            })
            .await?;

        Ok(Response::new(RequestAuthCodeResponse {
            auth_url: url.to_string(),
        }))
    }

    async fn deliver_auth_code(
        &self,
        request: Request<DeliverAuthCodeRequest>,
    ) -> Result<Response<DeliverAuthCodeResponse>, Status> {
        let request = request.into_inner();

        let auth_code = AuthorizationCode::new(request.auth_code.to_string());
        let csrf_token = request.state;
        let mut redis = self.0.redis.write().await;

        let client = &self.1;

        // Check whether the CSRF token received from the server matches the one from the cache
        let cached_csrf_token: CsrfToken = get_cached(&mut *redis, csrf_token.as_str())
            .map_err(|e| {
                tracing::error!("error fetching CSRF token: {e}");
                Status::internal(format!("error fetching CSRF token: {e}"))
            })
            .await?
            .ok_or_else(|| {
                tracing::error!("missing CSRF token in cache");
                Status::unauthenticated("missing CSRF token in cache")
            })?;

        if cached_csrf_token.secret() != &csrf_token {
            tracing::warn!("CSRF token mismatch");
            return Err(Status::unauthenticated("CSRF token mismatch"));
        }

        // Request Auth Token from osu API
        let token = client.exchange_code(auth_code)
            .request_async(async_http_client)
            .map_err(|e| {
                tracing::warn!("could not get token from osu API: {e:?}");
                Status::internal(format!("could not get token from osu API: {e:?}"))
            })
            .await?;


        let access_token = token.access_token();
        let refresh_token = token.refresh_token().ok_or_else(|| {
            tracing::error!("osu API did not send refresh token");
            Status::internal("osu API did not send refresh token")
        })?;
        let expiry = token.expires_in().ok_or_else(|| {
            tracing::error!("osu API did not send token expiry");
            Status::internal("osu API did not send token expiry")
        })?;

        let response_bytes = request_user_data(access_token.secret().as_str()).await?;
        // We should have now received the user data. If not, we're probably not authenticated yet
        let body_content = String::from_utf8_lossy(response_bytes.as_slice());
        let user = serde_json::from_str::<rosu_v2::model::user::User>(body_content.as_ref())
            .map_err(|e| {
                tracing::error!("could not get user data from osu API: {e}");
                Status::unauthenticated(format!("could not get user data from osu API: {e}"))
            })?;

        let user_id = user.user_id;

        tracing::info!(user_id, "Successfully authenticated user");

        // All is well, so we save the accesss token and refresh token
        cache(
            &mut *redis,
            &OsuAccessToken {
                user_id,
                token: access_token.clone(),
            },
            Some(expiry.as_secs() as usize - 30),
        )
        .map_err(|e| Status::internal(format!("error caching access token: {e}")))
        .await?;

        cache(
            &mut *redis,
            &OsuRefreshToken {
                user_id,
                token: refresh_token.clone(),
            },
            None,
        )
        .map_err(|e| Status::internal(format!("error caching access token: {e}")))
        .await?;

        Ok(Response::new(DeliverAuthCodeResponse {}))
    }
}

async fn request_user_data(access_token: &str) -> tonic::Result<Vec<u8>> {
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
        method: reqwest::Method::GET,
        headers,
        body: vec![],
    })
    .await
    .map_err(|e| Status::internal(format!("could not request user data using token: {e}")))?;

    Ok(resp.body)
}
