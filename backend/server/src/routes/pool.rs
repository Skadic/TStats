use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{EntityTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use model::entities::{PoolBracketEntity, PoolMapEntity, StageEntity};
use model::models::{PoolBracket, PoolMap};

use crate::osu::map::{get_map, SlimBeatmap};
use crate::routes::TournamentIdAndStageOrder;
use crate::AppState;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullPoolBracket {
    #[serde(flatten)]
    bracket: PoolBracket,
    maps: Vec<SlimBeatmap>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    brackets: Vec<PoolBracket>,
}

pub async fn get_pool(
    State(mut state): State<AppState>,
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
        .ok_or_else(|| (StatusCode::NOT_FOUND, "stage does not exist".to_owned()))?;

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
        full_pool.push(find_map_info(&mut state, bracket, maps).await);
    }

    Ok(Json(full_pool))
}

async fn find_map_info(
    state: &mut AppState,
    bracket: PoolBracket,
    maps: Vec<PoolMap>,
) -> FullPoolBracket {
    let mut full_maps = Vec::new();

    for map in maps {
        let map_id = map.map_id as u32;
        full_maps.push(get_map(&mut state.redis, &state.osu, map_id).await.unwrap());
    }

    FullPoolBracket {
        bracket,
        maps: full_maps,
    }
}
