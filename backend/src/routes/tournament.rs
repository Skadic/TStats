use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{
    query::*, ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel,
    ModelTrait,
};
use serde::Serialize;

use crate::model::entities::{CountryRestrictionEntity, StageEntity, TournamentEntity};
use crate::model::models::Tournament;
use crate::model::stage;
use crate::routes::ById;

/// Get all tournaments
pub async fn get_all_tournaments(
    State(ref db): State<DatabaseConnection>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, String)> {
    let tournaments = TournamentEntity::find().all(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to get all tournaments: {e}"),
        )
    })?;
    Ok(Json(tournaments))
}

#[derive(Debug, Serialize)]
pub struct SlimStage {
    name: String,
    best_of: i16,
}

/// A tournament including all its stages
#[derive(Debug, Serialize)]
pub struct ExtendedTournamentResult {
    #[serde(flatten)]
    tournament: Tournament,
    stages: Vec<SlimStage>,
    country_restrictions: Vec<String>,
}

/// Get a tournament by its ID including its stages
pub async fn get_tournament(
    State(ref db): State<DatabaseConnection>,
    Query(param): Query<ById>,
) -> Result<Json<Option<ExtendedTournamentResult>>, (StatusCode, String)> {
    // Find the tournament with the given ID
    let Some(tournament) = TournamentEntity::find_by_id(param.id)
        .one(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get tournament: {e}"),
            )
        })? else {
        return Err((StatusCode::NOT_FOUND, format!("tournament with id '{}' not found", param.id)));
    };

    // Find all stages of the tournament
    let stages = tournament
        .find_related(StageEntity)
        .order_by_asc(stage::Column::StageOrder)
        .all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get stages: {e}"),
            )
        })?
        .into_iter()
        .map(|stage| SlimStage {
            name: stage.name,
            best_of: stage.best_of,
        })
        .collect();

    let country_restrictions = tournament
        .find_related(CountryRestrictionEntity)
        .all(db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to get country restrictions: {e}"),
            )
        })?
        .into_iter()
        .map(|cr| cr.name)
        .collect::<Vec<String>>();

    Ok(Json(Some(ExtendedTournamentResult {
        tournament,
        stages,
        country_restrictions,
    })))
}

/// Create a new tournament
pub async fn create_tournament(
    State(ref db): State<DatabaseConnection>,
    Json(tournament): Json<Tournament>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let name = tournament.name.clone();

    let mut tournament = tournament.into_active_model();
    tournament.id = ActiveValue::NotSet;
    let tournament = tournament.insert(db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create tournament with name '{name}': {e}"),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        format!(
            "tournament with name '{name}' created with id '{}'",
            tournament.id
        ),
    ))
}
