use futures::TryFutureExt;
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use oauth2::{
    basic::BasicClient,
    reqwest::{self, async_http_client},
    AuthorizationCode, TokenResponse,
};
use proto::osu_auth::{
    osu_auth_service_server::OsuAuthService, DeliverAuthCodeRequest, DeliverAuthCodeResponse,
    RequestAuthCodeRequest, RequestAuthCodeResponse,
};
use tonic::{Request, Response, Status};
use url::Url;

use crate::{
    cache::{cache, get_cached},
    osu::auth::{OsuAccessToken, OsuAuthCode, OsuAuthState, OsuRefreshToken},
    LocalAppState,
};

pub struct OsuAuthServiceImpl(pub LocalAppState, pub BasicClient);

#[tonic::async_trait]
impl OsuAuthService for OsuAuthServiceImpl {
    async fn request_auth_code(
        &self,
        request: Request<RequestAuthCodeRequest>,
    ) -> Result<Response<RequestAuthCodeResponse>, Status> {
        let request = request.into_inner();
        let user_id = request.user_id;
        let mut redis = self.0.redis.write().await;

        let url = OsuAuthCode::request(user_id, &self.1, &mut *redis)
            .map_err(|e| Status::internal(format!("error requesting auth code: {e}")))
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
        let auth_code = AuthorizationCode::new(request.auth_code);
        let csrf_token = request.state;
        let mut redis = self.0.redis.write().await;

        let client = &self.1;

        let token = client
            .exchange_code(auth_code)
            .request_async(async_http_client)
            .map_err(|e| Status::internal(format!("could not get token from osu API: {e:?}")))
            .await?;

        let access_token = token.access_token();
        let refresh_token = token
            .refresh_token()
            .ok_or_else(|| Status::internal("osu API did not send refresh token"))?;
        let expiry = token
            .expires_in()
            .ok_or_else(|| Status::internal("osu API did not send token expiry"))?;

        let response_bytes = request_user_data(access_token.secret().as_str()).await?;
        // We should have now received the user data. If not, we're probably not authenticated yet
        let body_content = String::from_utf8_lossy(response_bytes.as_slice());
        let user = serde_json::from_str::<rosu_v2::model::user::User>(body_content.as_ref())
            .map_err(|e| {
                Status::unauthenticated(format!("could not get user data from osu API: {e}"))
            })?;

        let user_id = user.user_id;

        // Check whether the CSRF token received from the server matches the one from the cache
        let cached_csrf_token: OsuAuthState = get_cached(&mut *redis, &user_id)
            .map_err(|e| Status::internal(format!("error fetching CSRF token: {e}")))
            .await?
            .ok_or_else(|| Status::unauthenticated("missing CSRF token"))?;

        if cached_csrf_token.state.secret() != &csrf_token {
            return Err(Status::unauthenticated("CSRF token mismatch"));
        }

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
    let resp = reqwest::async_http_client(oauth2::HttpRequest {
        url: Url::parse("https://osu.ppy.sh/api/v2/me").unwrap(),
        method: Method::GET,
        headers,
        body: vec![],
    })
    .await
    .map_err(|e| Status::internal(format!("could not request user data using token: {e}")))?;

    Ok(resp.body)
}
