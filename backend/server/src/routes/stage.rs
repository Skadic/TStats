use std::sync::Arc;

use futures::TryFutureExt;
use http::StatusCode;
use model::dto::stage::StageDto;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use utils::{LogErrorDiagnosticFuture, ToPoemErrorFuture};

use crate::service::StageService;

pub struct StageApi {
    stage_service: Arc<StageService>,
}

impl StageApi {
    pub fn new(stage_service: Arc<StageService>) -> Self {
        Self { stage_service }
    }
}

#[OpenApi(prefix_path = "tournament/:tournament_id/stage")]
impl StageApi {
    
    /// Fetch all stages for a tournament.
    #[oai(path = "/", method = "get")]
    #[tracing::instrument(skip_all, name = "get_all_stages")]
    async fn get_all(&self, Path(tournament_id): Path<usize>) -> poem::Result<Json<Vec<StageDto>>> {
        self.stage_service
            .get_all(tournament_id)
            .log_error("could not fetch tournaments")
            .internal_server_error()
            .await
            .map(Json)
    }

    /// Fetch a single stage for a tournament.
    #[oai(path = "/:stage_order", method = "get")]
    #[tracing::instrument(skip_all, name = "get_stage_by_id")]
    async fn get(
        &self,
        Path(tournament_id): Path<usize>,
        Path(stage_order): Path<usize>,
    ) -> poem::Result<Json<StageDto>> {
        self.stage_service
            .get_by_id(tournament_id, stage_order)
            .log_error("could not fetch tournament")
            .internal_server_error()
            .and_then(|opt| async { opt.ok_or(poem::Error::from_status(StatusCode::NOT_FOUND)) })
            .await
            .map(Json)
    }
}
