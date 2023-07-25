use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DatabaseConnection;

use crate::model::models::Stage;

#[derive(Debug, serde::Deserialize)]
pub struct ByTournamentId {
    tournament_id: String,
}

#[derive(Debug, serde::Deserialize)]
struct Stages {
    stages: Vec<Stage>,
}

/// Get all stages
pub async fn get_all_stages(
    State(db): State<DatabaseConnection>,
    Query(param): Query<ByTournamentId>,
) -> Result<Json<Vec<Stage>>, (StatusCode, String)>  {
    todo!("implement get_all_stages")
}

pub async fn create_stage(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ByTournamentId>,
    Json(stage): Json<Stage>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!("implement create_stage")
}
