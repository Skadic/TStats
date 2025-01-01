use std::sync::Arc;

use model::dto::auth::{
    DeliverAuthCodeRequestDto, DeliverAuthCodeResponseDto, RequestAuthCodeResponseDto,
};
use poem::session::Session;
use poem_openapi::{payload::Json, OpenApi};
use utils::consts::OSU_SESSION;

use crate::service::AuthService;

pub struct AuthApi {
    pub auth_service: Arc<AuthService>,
}

#[OpenApi(prefix_path = "/auth")]
impl AuthApi {
    #[oai(path = "/", method = "get")]
    #[tracing::instrument(skip_all)]
    async fn request_auth_code(&self) -> poem::Result<Json<RequestAuthCodeResponseDto>> {
        let auth_url = self.auth_service.request_auth_code().await?.to_string();
        Ok(Json(RequestAuthCodeResponseDto { auth_url }))
    }

    #[oai(path = "/", method = "post")]
    #[tracing::instrument(skip_all)]
    async fn deliver_auth_code(
        &self,
        Json(request): Json<DeliverAuthCodeRequestDto>,
        session: &Session,
    ) -> poem::Result<DeliverAuthCodeResponseDto> {
        let auth_result = self
            .auth_service
            .deliver_auth_code(&request.auth_code, &request.state)
            .await?;
        session.set(OSU_SESSION, auth_result);

        Ok(DeliverAuthCodeResponseDto::Redirect(
            "https://localdev.skadic.moe:3000".to_owned(),
        ))
    }
}
