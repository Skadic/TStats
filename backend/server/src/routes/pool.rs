use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use futures::future::join_all;
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use model::entities::{PoolBracketEntity, PoolMapEntity, StageEntity};
use model::models::PoolBracket;

use crate::osu::map::{find_map_info, SlimBeatmap};
use crate::routes::TournamentIdAndStageOrder;
use crate::AppState;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullPoolBracket {
    #[serde(flatten)]
    pub bracket: PoolBracket,
    pub maps: Vec<SlimBeatmap>,
}

#[utoipa::path(
    get,
    path = "/api/pool",
    params(TournamentIdAndStageOrder),
    responses(
        (status = 200, description = "Return the contents of the pool for the given stage in a tournament", body = [FullPoolBracket]),
        (status = 404, description = "The tournament or stage does not exist", body = String),
        (status = 500, description = "Error communicating with the database", body = String),
    )
)]
#[axum_macros::debug_handler]
pub async fn get_pool(
    State(state): State<AppState>,
    Query(TournamentIdAndStageOrder {
        tournament_id,
        stage_order,
    }): Query<TournamentIdAndStageOrder>,
) -> Result<Json<Vec<FullPoolBracket>>, (StatusCode, String)> {
    let db = &state.db;

    let stage = StageEntity::find_by_id((tournament_id, stage_order))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get stage: {e}"),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                "stage or tournament does not exist".to_owned(),
            )
        })?;

    let pool = stage
        .find_related(PoolBracketEntity)
        .find_with_related(PoolMapEntity)
        .all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get pool brackets: {e}"),
            )
        })?;

    let mut full_pool = Vec::with_capacity(pool.len());
    for (bracket, maps) in pool {
        full_pool.push(find_map_info(&state, bracket, maps));
    }

    Ok(Json(join_all(full_pool).await))
}
