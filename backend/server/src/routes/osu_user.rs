use futures::TryFutureExt;
use proto::osu::{osu_user_service_server::OsuUserService, GetUserRequest};
use tonic::{async_trait, Request, Response, Status};
use utils::Cacheable;

use crate::{
    osu::{auth::Session, user::get_user},
    AppState, RedisConnectionPool,
};

pub struct OsuUserServiceImpl(pub AppState);

#[tracing::instrument(skip(redis))]
pub async fn get_authenticated_user<T: std::fmt::Debug>(
    request: &Request<T>,
    redis: &RedisConnectionPool,
) -> tonic::Result<Session> {
    let Some(tok) = request.metadata().get("authorization") else {
        tracing::error!("tried to get authentication without authorization token");
        return Err(Status::unauthenticated("session token not set"));
    };

    let token_string = String::from_utf8_lossy(tok.as_bytes());
    // Remove "Bearer " from the token
    let token_string = &token_string[7..];
    let Some(session) = Session::get_cached(token_string, redis)
        .map_err(|e| {
            tracing::error!(error = %e, "could not get session");
            Status::internal("error getting session")
        })
        .await?
    else {
        tracing::error!(token = %token_string, "no session found for token");
        return Err(Status::unauthenticated("no session found"));
    };

    Ok(session)
}

#[async_trait]
impl OsuUserService for OsuUserServiceImpl {
    #[tracing::instrument(skip_all)]
    async fn get(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<proto::osu::User>, Status> {
        let Session { osu_user_id, .. } = get_authenticated_user(&request, &self.0.redis).await?;

        let user = get_user(&self.0.redis, &self.0.osu, osu_user_id)
            .await
            .map(proto::osu::User::from)
            .map_err(|e| {
                tracing::error!(error = %e, "could not get osu user");
                Status::internal("could not get osu user")
            })?;

        Ok(Response::new(user))
    }
}
