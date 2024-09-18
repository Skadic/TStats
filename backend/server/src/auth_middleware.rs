use crate::osu::auth::ApiToken;
use crate::AppState;
use futures::TryFutureExt;
use http::{Request, Response};
use http_body::Frame;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::codegen::{Body, BoxFuture};
use tonic::Status;
use tonic_middleware::RequestInterceptor;
use tower_http::auth::AsyncAuthorizeRequest;
use utils::Cacheable;

#[derive(Clone)]
pub struct AuthLayer {
    state: AppState,
    included_services: Arc<Vec<String>>,
}

impl AuthLayer
where
    AuthLayer: Send + Sync,
{
    pub fn new(
        state: AppState,
        included_service_names: impl IntoIterator<Item: AsRef<str>>,
    ) -> Self {
        Self {
            state,
            included_services: Arc::new(
                included_service_names
                    .into_iter()
                    .map(|v| v.as_ref().to_owned())
                    .collect(),
            ),
        }
    }
}

#[tonic::async_trait]
impl RequestInterceptor for AuthLayer {
    #[tracing::instrument(skip(self), rename = "cors", level = "trace")]
    async fn intercept(
        &self,
        req: tonic::codegen::http::Request<BoxBody>,
    ) -> Result<http::Request<BoxBody>, Status> {
        let state = self.state.clone();
        let included_services = Arc::clone(&self.included_services);
        if !included_services.iter().any(|service| {
            let path = req.uri().path();
            path.contains(service)
        }) {
            return Ok(req);
        }

        let auth_header_token = req
            .headers()
            .get("authorization")
            .ok_or(Status::unauthenticated("authorization cookie not set"))
            .and_then(|value| {
                value
                    .to_str()
                    .map_err(|_| Status::unauthenticated("non-unicode session token"))
            })?;

        if !auth_header_token.starts_with("Bearer ") {
            return Err(Status::unauthenticated("invalid session token"));
        }
        let auth_header_token = &auth_header_token[7..];

        if let Some(_) = ApiToken::get_cached(auth_header_token, &state.redis)
            .map_err(|_| Status::internal("error reading session token"))
            .await?
        {
            // All is good. We have found a valid session token, so call the service
            return Ok(req);
        };

        // In this case, the session token is either unknown or expired.
        // We try to deserialize it and if successful, try to fetch a new token from the api
        Err(Status::unauthenticated("expired or unknown session token"))
    }
}

impl<B> AsyncAuthorizeRequest<B> for AuthLayer
where
    B: Send + 'static,
{
    type RequestBody = B;
    type ResponseBody = tonic::body::BoxBody;
    type Future = BoxFuture<Request<B>, Response<Self::ResponseBody>>;

    fn authorize(&mut self, req: Request<B>) -> Self::Future {
        let state = self.state.clone();
        let included_services = Arc::clone(&self.included_services);
        Box::pin(async move {
            if !included_services.iter().any(|service| {
                let path = req.uri().path();
                path.contains(service)
            }) {
                return Ok(req);
            }

            let auth_header_token = req
                .headers()
                .get("authorization")
                .ok_or(Status::unauthenticated("authorization cookie not set"))
                .and_then(|value| {
                    value
                        .to_str()
                        .map_err(|_| Status::unauthenticated("non-unicode session token"))
                })
                .map_err(status_to_box_body_response)?;

            if !auth_header_token.starts_with("Bearer ") {
                return Err(Status::unauthenticated("invalid session token"))
                    .map_err(status_to_box_body_response);
            }
            let auth_header_token = &auth_header_token[7..];

            if let Some(_) = ApiToken::get_cached(auth_header_token, &state.redis)
                .map_err(|_| Status::internal("error reading session token"))
                .map_err(status_to_box_body_response)
                .await?
            {
                // All is good. We have found a valid session token, so call the service
                return Ok(req);
            };

            // In this case, the session token is either unknown or expired.
            // We try to deserialize it and if successful, try to fetch a new token from the api
            Err(Status::unauthenticated("expired or unknown session token"))
                .map_err(status_to_box_body_response)
        })
    }
}

fn status_to_box_body_response(status: Status) -> Response<BoxBody> {
    Response::new(BoxBody::new(ErrBody::new(status)))
}

/// A body that always returns a tonic status as an error
pub struct ErrBody {
    err: Status,
}

impl ErrBody {
    pub fn new(err: Status) -> Self {
        Self { err }
    }
}

impl Body for ErrBody {
    type Data = bytes::Bytes;
    type Error = Status;

    fn poll_frame(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        Poll::Ready(Some(Err(self.err.clone())))
    }
}
