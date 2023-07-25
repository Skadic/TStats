use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use log::{debug, error, info, warn};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use crate::model::models::{Stage, Tournament};

/// Get all tournaments
pub async fn get_all_tournaments(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, String)> {
    todo!("implement get_all_tournaments");
}

/// A tournament including all its stages
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedTournament {
    #[serde(flatten)]
    tournament: Tournament,
    stages: Vec<Stage>,
}

/// Get a tournament by its ID including its stages
pub async fn get_tournament(
    State(db): State<DatabaseConnection>,
    Query(param): Query<String>,
) -> Result<Json<Option<ExtendedTournament>>, (StatusCode, String)> {
    // Find the tournament with the given ID including all its stages
    todo!("implement get_tournament");
}

/// Create a new tournament
pub async fn create_tournament(
    State(db): State<DatabaseConnection>,
    Json(tournament): Json<Tournament>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let name = tournament.name.clone();

    todo!("implement create_tournament");
}
