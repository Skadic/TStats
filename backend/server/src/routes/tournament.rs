use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{query::*, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

use crate::routes::Id;
use crate::AppState;
use model::entities::{CountryRestrictionEntity, StageEntity, TournamentEntity};
use model::models::Tournament;
use model::stage;

/// Get all tournaments from the database
#[utoipa::path(
    get,
    path = "/api/tournament/all",
    responses(
        (status = 200, description = "Successfuly requested beatmap", body = [Tournament]),
        (status = 500, description = "Failed requesting tournaments from the database", body = String)
    )
)]
pub async fn get_all_tournaments(
    State(ref state): State<AppState>,
) -> Result<Json<Vec<Tournament>>, (StatusCode, String)> {
    let tournaments = TournamentEntity::find().all(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to get all tournaments: {e}"),
        )
    })?;
    Ok(Json(tournaments))
}

/// A stage with all primary key information stripped out
#[derive(Debug, Serialize, ToSchema)]
#[schema(example = json!({"name": "QF", "best_of": 7}))]
#[serde(rename_all = "camelCase")]
pub struct SlimStage {
    name: String,
    best_of: i16,
}

/// A tournament including all its stages and country restrictions
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTournamentResult {
    /// The tournament itself
    #[serde(flatten)]
    tournament: Tournament,
    /// The tournament's stages
    #[schema(example = json!([{"name": "RO16", "best_of": 5}, {"name": "QF", "best_of": 7}]))]
    stages: Vec<SlimStage>,
    /// The tournament's country restrictions as a vector of country codes.
    #[schema(example = json!(["UK", "NZ", "FR"]))]
    country_restrictions: Vec<String>,
}

/// Get a tournament by its ID including its stages
#[utoipa::path(
    get,
    path = "/api/tournament",
    params(
        Id
    ),
    responses(
        (status = 200, description = "Successfuly requested beatmap", body = ExtendedTournamentResult),
        (status = 404, description = "The tournament with the given id does not exist", body = String),
        (status = 500, description = "Failed requesting from the database", body = String)
    )
)]
pub async fn get_tournament(
    State(ref state): State<AppState>,
    Query(param): Query<Id>,
) -> Result<Json<Option<ExtendedTournamentResult>>, (StatusCode, String)> {
    // Find the tournament with the given ID
    let Some(tournament) = TournamentEntity::find_by_id(param.id)
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
            StatusCode::NOT_FOUND,
            format!("tournament with id '{}' not found", param.id),
        ));
    };

    // Find all stages of the tournament
    let stages = tournament
        .find_related(StageEntity)
        .order_by_asc(stage::Column::StageOrder)
        .all(&state.db)
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

    // Find all country restrictions for this tournament in the database
    let country_restrictions = tournament
        .find_related(CountryRestrictionEntity)
        .all(&state.db)
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
#[utoipa::path(
    post,
    path = "/api/tournament",
    request_body = Tournament,
    responses(
        (status = 201, description = "Successfully created tournament", body = Id, example = json!({ "id": 16 })),
        (status = 500, description = "Failed to create tournament", body = String)
    )
)]
pub async fn create_tournament(
    State(ref state): State<AppState>,
    Json(tournament): Json<Tournament>,
) -> Result<(StatusCode, Json<Id>), (StatusCode, String)> {
    let name = tournament.name.clone();

    let mut tournament = tournament.into_active_model();
    tournament.id = ActiveValue::NotSet;
    let tournament = tournament.insert(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to create tournament with name '{name}': {e}"),
        )
    })?;

    Ok((StatusCode::CREATED, Json(Id { id: tournament.id })))
}
