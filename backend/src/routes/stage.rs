use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter, QueryOrder,
};
use serde::{Serialize};

use crate::model::entities::{PoolBracketEntity, PoolMapEntity};
use crate::model::{entities::StageEntity, models::Stage, *};
use crate::model::models::{PoolBracket, PoolMap};

#[derive(Debug, serde::Deserialize)]
pub struct ByTournamentId {
    tournament_id: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ByTournamentIdAndStageOrder {
    tournament_id: i32,
    stage_order: i16,
}

/// Get all stages for a given tournament
pub async fn get_all_stages(
    State(ref db): State<DatabaseConnection>,
    Query(ByTournamentId { tournament_id }): Query<ByTournamentId>,
) -> Result<Json<Vec<Stage>>, (StatusCode, String)> {
    let stages = StageEntity::find()
        .filter(stage::Column::TournamentId.eq(tournament_id))
        .order_by_asc(stage::Column::StageOrder)
        .all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get all stages: {e}"),
            )
        })?;

    Ok(Json(stages))
}

#[derive(Debug, Serialize)]
pub struct ExtendedStage {
    #[serde(flatten)]
    stage: Stage,
    brackets: Vec<ExtendedPoolBracket>,
}

#[derive(Debug, Serialize)]
pub struct ExtendedPoolBracket {
    #[serde(flatten)]
    bracket: PoolBracket,
    maps: Vec<PoolMap>,
}

pub async fn get_stage(
    State(ref db): State<DatabaseConnection>,
    Query(ByTournamentIdAndStageOrder {
        tournament_id,
        stage_order,
    }): Query<ByTournamentIdAndStageOrder>,
) -> Result<(StatusCode, Json<Option<ExtendedStage>>), (StatusCode, String)> {
    let Some(stage) = StageEntity::find_by_id((tournament_id, stage_order))
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get stage: {e}"),
            )
        })? else {
        return Ok((StatusCode::NOT_FOUND, Json(None)));
    };

    let brackets = stage
        .find_related(PoolBracketEntity)
        .order_by_asc(pool_bracket::Column::BracketOrder)
        .find_with_related(PoolMapEntity)
        .order_by_asc(pool_map::Column::MapOrder)
        .all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get brackets: {e}"),
            )
        })?
        .into_iter()
        .map(|(bracket, maps)| ExtendedPoolBracket { bracket, maps })
        .collect::<Vec<_>>();

    Ok((StatusCode::OK, Json(Some(ExtendedStage { stage, brackets }))))
}

/// Creates a new stage in a tournament
pub async fn create_stage(
    State(ref db): State<DatabaseConnection>,
    Json(stage): Json<Stage>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    stage.into_active_model().insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create stage: {e}"),
        )
    })?;

    Ok((StatusCode::CREATED, "stage created".to_owned()))
}
