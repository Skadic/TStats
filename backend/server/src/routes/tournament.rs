use std::sync::Arc;

use crate::service::TournamentService;
use futures::TryFutureExt;
use http::StatusCode;
use model::dto::tournament::TournamentDto;
use poem_openapi::param::Path;
use poem_openapi::{payload::Json, OpenApi};
use utils::{LogErrorDiagnosticFuture, ToPoemErrorFuture};

pub struct TournamentApi {
    tournament_service: Arc<TournamentService>,
}

impl TournamentApi {
    pub fn new(tournament_service: Arc<TournamentService>) -> Self {
        Self { tournament_service }
    }
}

#[OpenApi(prefix_path = "/tournament")]
impl TournamentApi {
    #[oai(path = "/", method = "get")]
    #[tracing::instrument(skip_all, name = "get_all_tournaments")]
    async fn get_all(&self) -> poem::Result<Json<Vec<TournamentDto>>> {
        self.tournament_service
            .get_all()
            .log_error("could not fetch tournaments")
            .internal_server_error()
            .await
            .map(Json)
    }

    #[oai(path = "/:id", method = "get")]
    #[tracing::instrument(skip_all,  name = "get_tournament_by_id")]
    async fn get(&self, Path(id): Path<i32>) -> poem::Result<Json<TournamentDto>> {
        self.tournament_service
            .get_by_id(id)
            .log_error("could not fetch tournament")
            .internal_server_error()
            .and_then(|opt| async { opt.ok_or(poem::Error::from_status(StatusCode::NOT_FOUND)) })
            .await
            .map(Json)
    }
}

mod test {
    #[sqlx::test]
    async fn test_basic() {}
}
