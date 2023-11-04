use sea_orm::{query::*, ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use tonic::{Request, Response, Status};

use model::{
    entities::{CountryRestrictionEntity, StageEntity, TournamentEntity},
    stage,
    tournament::{
        RankRestriction::{OpenRank, Single, Tiered},
        RankRange,
        TournamentFormat
    }
};
use proto::tournaments::tournament_service_server::TournamentService;
use proto::tournaments::{
    CreateTournamentResponse, DeleteTournamentRequest, DeleteTournamentResponse, GetAllTournamentsRequest,
    GetAllTournamentsResponse, GetTournamentRequest, UpdateTournamentRequest, UpdateTournamentResponse,
};

use crate::LocalAppState;

pub struct TournamentServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl TournamentService for TournamentServiceImpl {
    async fn get_all(
        &self,
        _request: Request<GetAllTournamentsRequest>,
    ) -> Result<Response<GetAllTournamentsResponse>, Status> {
        let tournaments = TournamentEntity::find()
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get all tournaments: {e}")))?
            .into_iter()
            .map(proto::tournaments::Tournament::from)
            .collect::<Vec<_>>();

        Ok(Response::new(GetAllTournamentsResponse { tournaments }))
    }

    async fn get(
        &self,
        request: Request<GetTournamentRequest>,
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
