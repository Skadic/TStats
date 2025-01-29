use std::sync::Arc;

use model::dto::auth::{
        AuthenticatedUserDto, DeliverAuthCodeRequestDto, DeliverAuthCodeResponseDto,
        RequestAuthCodeResponseDto,
    };
use poem::session::Session;
use poem_openapi::{param::Query, payload::Json, OpenApi};
use tracing::debug;
use utils::{consts::SESSION_CONTENT, LogErrorFuture, ToPoemErrorFuture};

use crate::service::{AuthService, SessionContent};

pub struct AuthApi {
    auth_service: Arc<AuthService>,
}

impl AuthApi {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self {
            auth_service,
        }
    }
}

#[OpenApi(prefix_path = "/auth")]
impl AuthApi {
    #[oai(path = "/", method = "get")]
    #[tracing::instrument(skip_all)]
    /// Request Auth Code
    async fn request_auth_code(
        &self,
        #[oai(name = "returnUrl")] Query(return_url): Query<String>,
    ) -> poem::Result<Json<RequestAuthCodeResponseDto>> {
        let auth_url = self
            .auth_service
            .request_auth_code(&return_url)
            .log_error("could not request auth code")
            .internal_server_error()
            .await?
            .to_string();
        Ok(Json(RequestAuthCodeResponseDto { auth_url }))
    }

    #[oai(path = "/", method = "post")]
    #[tracing::instrument(skip(self, session))]
    /// Deliver Auth Code
    async fn deliver_auth_code(
        &self,
        Json(request): Json<DeliverAuthCodeRequestDto>,
        session: &Session,
    ) -> poem::Result<Json<DeliverAuthCodeResponseDto>> {
        debug!("delivering auth code");
        let auth_result = self
            .auth_service
            .deliver_auth_code(&request.auth_code, &request.state)
            .log_error("could not deliver auth code")
            .internal_server_error()
            .await?;
        session.set(SESSION_CONTENT, &auth_result.session);

        Ok(Json(DeliverAuthCodeResponseDto {
            return_url: auth_result.return_url,
            user: auth_result.session.user,
        }))
    }

    #[oai(path = "/user", method = "get")]
    #[tracing::instrument(skip_all)]
    /// Get Authenticated User
    async fn get_authenticated_user(
        &self,
        session: &Session,
    ) -> poem::Result<Json<Option<AuthenticatedUserDto>>> {
        let Some(session) = session.get::<SessionContent>(SESSION_CONTENT) else {
            return Ok(Json(None));
        };

        Ok(Json(Some(session.user)))
    }
}
