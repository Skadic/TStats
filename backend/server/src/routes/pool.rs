use std::sync::Arc;

use model::dto::pool::PoolDto;
use poem_openapi::{param::Path, payload::Json, OpenApi};
use utils::{LogErrorDiagnosticFuture, ToPoemErrorFuture};

use crate::service::PoolService;


pub struct PoolApi {
    pool_service: Arc<PoolService>,
}

impl PoolApi {
    pub fn new(pool_service: Arc<PoolService>) -> Self {
        Self { pool_service }
    }
}

#[OpenApi(prefix_path = "/tournament")]
impl PoolApi {
    
    /// Fetches the pool for a tournament's stage.
    #[oai(path = "/:tournament_id/stage/:stage_order/pool", method = "get")]
    #[tracing::instrument(skip_all, name = "get_pool")]
    async fn get(&self, Path(tournament_id): Path<usize>, Path(stage_order): Path<usize>) -> poem::Result<Json<PoolDto>> {
        self.pool_service
            .get_all(tournament_id, stage_order)
            .log_error("could not fetch pool")
            .internal_server_error()
            .await
            .map(Json)
    }
}