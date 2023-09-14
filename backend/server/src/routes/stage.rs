use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryOrder};
use serde::Serialize;
use utoipa::ToSchema;

use model::{
    entities::{PoolBracketEntity, PoolMapEntity, StageEntity, TournamentEntity},
    models::Stage,
    *,
};

use crate::routes::{TournamentId, TournamentIdAndStageOrder};
use crate::AppState;

/// Get all stages for a given tournament
#[utoipa::path(
    get,
    path = "/api/stage/all",
    params(TournamentId),
    responses(
        (status = 200, description = "Return all stages for the given tournament", body = [Stage]),
        (status = 404, description = "The tournament does not exist", body = String),
        (status = 500, description = "Error communicating with the database", body = String),
    )
)]
pub async fn get_all_stages(
    State(ref state): State<AppState>,
    Query(TournamentId { tournament_id }): Query<TournamentId>,
) -> Result<Json<Vec<Stage>>, (StatusCode, String)> {
    // Find the tournament with the given id
    let Some(tournament) = TournamentEntity::find_by_id(tournament_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get tournament: {e}"),
            )
        })?
    else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("tournament with id '{tournament_id}' does not exist"),
        ));
    };

    // Get all stages belonging to the tournament
    let stages = tournament
        .find_related(StageEntity)
        .all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get all stages: {e}"),
            )
        })?;

    Ok(Json(stages))
}

/// A stage with its associated pool brackets and their maps
#[derive(Debug, Serialize, ToSchema)]
pub struct ExtendedStageResult {
    #[serde(flatten)]
    stage: Stage,
    /// The pool brackets with their maps
    brackets: Vec<ExtendedPoolBracket>,
}

/// A pool bracket consisting of a name and its associated maps
#[derive(Debug, Serialize, ToSchema)]
pub struct ExtendedPoolBracket {
    /// The name of the pool bracket
    #[schema(example = "HR")]
    name: String,
    /// The map ids of the maps that make up this pool bracket
    #[schema(example = json!([ 131891, 126645, 3853099 ]))]
    maps: Vec<usize>,
}

/// Get a stage together with its pool brackets and their maps
#[utoipa::path(
    get,
    path = "/api/stage",
    params(TournamentIdAndStageOrder),
    responses(
        (status = 200, description = "Return the given stage with extra data", body = ExtendedStageResult),
        (status = 404, description = "The tournament or stage does not exist", body = String),
        (status = 500, description = "Error communicating with the database", body = String),
    )
)]
pub async fn get_stage(
    State(ref state): State<AppState>,
    Query(TournamentIdAndStageOrder {
        tournament_id,
        stage_order,
    }): Query<TournamentIdAndStageOrder>,
) -> Result<(StatusCode, Json<Option<ExtendedStageResult>>), (StatusCode, String)> {
    // Find the stage with the given id
    let Some(stage) = StageEntity::find_by_id((tournament_id, stage_order))
        .one(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get stage: {e}"),
            )
        })?
    else {
        return Ok((StatusCode::NOT_FOUND, Json(None)));
    };

    // Find the pool brackets associated with the stage and the maps inside the brackets
    let brackets = stage
        .find_related(PoolBracketEntity)
        .order_by_asc(pool_bracket::Column::BracketOrder)
        .find_with_related(PoolMapEntity)
        .order_by_asc(pool_map::Column::MapOrder)
        .all(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get brackets: {e}"),
            )
        })?
        .into_iter()
        // For the brackets, we only want the name and for the maps we only want the id
        .map(|(bracket, maps)| ExtendedPoolBracket {
            name: bracket.name,
            maps: maps.into_iter().map(|m| m.map_id as usize).collect(),
        })
        .collect::<Vec<_>>();

    Ok((
        StatusCode::OK,
        Json(Some(ExtendedStageResult { stage, brackets })),
    ))
}

/// Creates a new stage in a tournament
#[utoipa::path(
    post,
    path = "/api/stage",
    request_body = Stage,
    responses(
        (status = 201, description = "Stage Created"),
        (status = 500, description = "Error communicating with the database", body = String),
    )
)]
pub async fn create_stage(
    State(ref state): State<AppState>,
    Json(stage): Json<Stage>,
) -> Result<StatusCode, (StatusCode, String)> {
    stage
        .into_active_model()
        .insert(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to create stage: {e}"),
            )
        })?;

    Ok(StatusCode::CREATED)
}
