use std::str::FromStr;

use futures::TryFutureExt;
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse};
use proto::osu_auth::{
    osu_auth_service_server::OsuAuthService, DeliverAuthCodeRequest, DeliverAuthCodeResponse,
    RequestAuthCodeRequest, RequestAuthCodeResponse,
};
use tonic::{metadata::MetadataValue, Request, Response, Status};
use tracing::error;
use url::Url;
use utils::{crypt::EncryptedToken, Cacheable};

use crate::{
    osu::auth::{OsuApiTokens, OsuAuthCode, OsuCsrfToken, Session},
    AppState,
};

pub struct OsuAuthServiceImpl(pub AppState, pub BasicClient);

#[tonic::async_trait]
impl OsuAuthService for OsuAuthServiceImpl {
    #[tracing::instrument(skip(self, _request))]
    async fn request_auth_code(
        &self,
        _request: Request<RequestAuthCodeRequest>,
    ) -> Result<Response<RequestAuthCodeResponse>, Status> {
        let url = OsuAuthCode::request(&self.1, &self.0.redis)
            .map_err(|error| {
                tracing::error!(%error, "error requesting auth code");
                Status::internal(format!("error requesting auth code: {error}"))
            })
            .await?;

        Ok(Response::new(RequestAuthCodeResponse {
            auth_url: url.to_string(),
        }))
    }

    #[tracing::instrument(skip(self, request))]
    async fn deliver_auth_code(
        &self,
        request: Request<DeliverAuthCodeRequest>,
    ) -> Result<Response<DeliverAuthCodeResponse>, Status> {
        let request = request.into_inner();

        let auth_code = AuthorizationCode::new(request.auth_code.to_string());
        let csrf_token = request.state;
        let redis = &self.0.redis;

        let client = &self.1;

        // Check whether the CSRF token received from the server matches the one from the cache
        let cached_csrf_token = OsuCsrfToken::uncache(redis, csrf_token.as_str())
            .map_err(|error| {
                tracing::error!(%error, "error fetching CSRF token");
                Status::internal(format!("error fetching CSRF token: {error}"))
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
        let token = client
            .exchange_code(auth_code)
            .request_async(async_http_client)
            .map_err(|error| {
                tracing::warn!("could not get token from osu API");
                Status::internal(format!("could not get token from osu API: {error}"))
            })
            .await?;

        let access_token = token.access_token();
        let refresh_token = token.refresh_token().ok_or_else(|| {
            tracing::error!(error = "osu API did not send refresh token");
            Status::internal("osu API did not send refresh token")
        })?;
        let expiry = token.expires_in().ok_or_else(|| {
            tracing::error!(error = "osu API did not send token expiry");
            Status::internal("osu API did not send token expiry")
        })?;

        let user = request_user_data(access_token.secret().as_str()).await?;
        let user_id = user.user_id;

        tracing::info!(user_id, "successfully authenticated user");

        // All is well, so we save the accesss token and refresh token
        OsuApiTokens {
            user_id,
            access_token: EncryptedToken::new(access_token.secret()).map_err(|error| {
                error!("error caching access token: {error}");
                Status::internal("error caching access token")
            })?,
            refresh_token: EncryptedToken::new(refresh_token.secret()).map_err(|error| {
                error!("error caching refresh token: {error}");
                Status::internal("error caching refresh token")
            })?,
        }
        .cache(redis, Some(expiry.as_secs() as usize - 30))
        .map_err(|error| Status::internal(format!("error caching access tokens: {error}")))
        .await?;
        let session = Session::new(user_id);

        session
            .cache(redis, Some(600))
            .await
            .map_err(|e| Status::internal(format!("error caching session token: {e}")))?;

        let mut resp = Response::new(DeliverAuthCodeResponse {
            access_token: session.session_id,
        });
        let cookie = format!("mycookie={}", access_token.secret());
        resp.metadata_mut().append(
            "set-cookie",
            MetadataValue::from_str(&cookie)
                .map_err(|_| Status::internal("could not set cookie"))?,
        );
        Ok(resp)
    }
}

#[tracing::instrument(skip_all)]
async fn request_user_data(access_token: &str) -> tonic::Result<rosu_v2::model::user::User> {
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
    .map_err(|e| Status::internal(format!("could not request user data using token: {e}")))?;

    // We should have now received the user data. If not, we're probably not authenticated yet
    let body_content = String::from_utf8_lossy(resp.body.as_slice());
    let user = serde_json::from_str::<rosu_v2::model::user::User>(body_content.as_ref()).map_err(
        |error| {
            tracing::error!(%error, "could not parse data from osu API");
            Status::unauthenticated(format!("could not parse user data from osu API: {error}"))
        },
    )?;

    Ok(user)
}
