use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder};

use crate::model::{entities::StageEntity, models::Stage, *};

#[derive(Debug, serde::Deserialize)]
pub struct ByTournamentId {
    tournament_id: i32,
}

/// Get all stages for a given tournament
pub async fn get_all_stages(
    State(ref db): State<DatabaseConnection>,
    Query(param): Query<ByTournamentId>,
) -> Result<Json<Vec<Stage>>, (StatusCode, String)> {
    let stages = StageEntity::find()
        .filter(stage::Column::TournamentId.eq(param.tournament_id))
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
