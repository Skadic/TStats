use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{query::*, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use serde::Serialize;
use tonic::{Request, Response, Status};
use utoipa::ToSchema;

use model::entities::{CountryRestrictionEntity, StageEntity, TournamentEntity};
use model::models::{Stage, Tournament};
use model::stage;
use model::tournament::RankRestriction::{OpenRank, Single, Tiered};
use model::tournament::{RankRange, TournamentFormat};
use proto::tournaments::tournament_service_server::TournamentService;
use proto::tournaments::{
    CreateTournamentResponse, DeleteTournamentRequest, DeleteTournamentResponse, GetAllRequest,
    GetAllResponse, GetRequest, UpdateTournamentRequest, UpdateTournamentResponse,
};

use crate::routes::Id;
use crate::{AppState, LocalAppState};

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

/// A tournament including all its stages and country restrictions
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedTournamentResult {
    /// The tournament itself
    #[serde(flatten)]
    tournament: Tournament,
    /// The tournament's stages
    #[schema(example = json!([{"tournamentId": 1, "stageOrder": 2, "name": "RO16", "bestOf": 5}, {"tournamentId": 1, "stageOrder": 2, "name": "QF", "bestOf": 7}]))]
    stages: Vec<Stage>,
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

pub struct TournamentServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl TournamentService for TournamentServiceImpl {
    async fn get_all(
        &self,
        _request: Request<GetAllRequest>,
    ) -> Result<Response<GetAllResponse>, Status> {
        let tournaments = TournamentEntity::find()
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get all tournaments: {e}")))?
            .into_iter()
            .map(proto::tournaments::Tournament::from)
            .collect::<Vec<_>>();

        Ok(Response::new(GetAllResponse { tournaments }))
    }

    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<proto::tournaments::Tournament>, Status> {
        let id = request.get_ref().id;
        let Some(tournament) = TournamentEntity::find_by_id(id)
            .one(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get tournament: {e}")))?
        else {
            return Err(Status::not_found(format!(
                "tournament with id '{id}' not found"
            )));
        };

        // Find all stages of the tournament
        let stages = tournament
            .find_related(StageEntity)
            .order_by_asc(stage::Column::StageOrder)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get stages: {e}")))?
            .into_iter()
            .map(proto::stages::Stage::from)
            .collect::<Vec<_>>();

        // Find all country restrictions for this tournament in the database
        let country_restrictions = tournament
            .find_related(CountryRestrictionEntity)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get country restrictions: {e}")))?
            .into_iter()
            .map(|cr| cr.name)
            .collect::<Vec<String>>();

        let mut tournament = proto::tournaments::Tournament::from(tournament);
        tournament.country_restrictions = country_restrictions;
        tournament.stages = stages;

        Ok(Response::new(tournament))
    }

    async fn create(
        &self,
        request: Request<proto::tournaments::Tournament>,
    ) -> Result<Response<CreateTournamentResponse>, Status> {
        use ActiveValue as A;
        let tournament = request.get_ref();
        let name = tournament.name.clone();
        let format = tournament
            .format
            .clone()
            .map(TournamentFormat::try_from)
            .ok_or_else(|| Status::invalid_argument(format!("empty tournament format")))?
            .map_err(|_| Status::invalid_argument(format!("could not decode tournament")))?;

        let mut tournament = model::tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set(tournament.name.clone()),
            shorthand: A::Set(tournament.shorthand.clone()),
            format: A::Set(format),
            rank_range: A::Set(match tournament.rank_restrictions.len() {
                0 => OpenRank,
                1 => Single(tournament.rank_restrictions[0].clone().into()),
                _ => Tiered(
                    tournament
                        .rank_restrictions
                        .iter()
                        .cloned()
                        .map(RankRange::from)
                        .collect(),
                ),
            }),
            bws: A::Set(tournament.bws),
        };
        tournament.id = ActiveValue::NotSet;
        let tournament = tournament.insert(&self.0.db).await.map_err(|e| {
            Status::internal(format!(
                "failed to create tournament with name '{name}': {e}"
            ))
        })?;

        Ok(Response::new(CreateTournamentResponse {
            id: tournament.id,
        }))
    }

    async fn update(
        &self,
        request: Request<UpdateTournamentRequest>,
    ) -> Result<Response<UpdateTournamentResponse>, Status> {
        use ActiveValue as A;
        let model = TournamentEntity::find_by_id(request.get_ref().id)
            .one(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to fetch tournament: {e}")))?
            .ok_or_else(|| {
                Status::not_found(format!(
                    "tournament with id {} not found",
                    request.get_ref().id
                ))
            })?;

        let mut model = model.into_active_model();

        if let Some(name) = request.get_ref().name.as_ref() {
            model.name = A::Set(name.clone());
        }

        if let Some(shorthand) = request.get_ref().shorthand.as_ref() {
            model.shorthand = A::Set(shorthand.clone());
        }

        if let Some(ranges) = request
            .get_ref()
            .rank_restrictions
            .as_ref()
            .map(|r| &r.ranges)
        {
            A::Set(match ranges.len() {
                0 => OpenRank,
                1 => Single(ranges[0].clone().into()),
                _ => Tiered(ranges.iter().cloned().map(RankRange::from).collect()),
            });
        }

        if let Some(format) = request.get_ref().format.as_ref() {
            model.format =
                A::Set(format.clone().try_into().map_err(|e| {
                    Status::invalid_argument("tournament format not decodeable: {e}")
                })?);
        }

        if let Some(bws) = request.get_ref().bws {
            model.bws = A::Set(bws.clone());
        }

        model.update(&self.0.db).await
            .map_err(|e| Status::internal(format!("failed to fetch tournament: {e}")))?;

        Ok(Response::new(UpdateTournamentResponse {}))
    }

    async fn delete(
        &self,
        request: Request<DeleteTournamentRequest>,
    ) -> Result<Response<DeleteTournamentResponse>, Status> {
        TournamentEntity::delete_by_id(request.get_ref().id)
            .exec(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not delete tournament: {e}")))?;

        Ok(Response::new(DeleteTournamentResponse {}))
    }
}
