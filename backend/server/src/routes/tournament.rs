use futures::{stream::FuturesOrdered, StreamExt};
use sea_orm::{
    query::*, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
};
use tonic::{Request, Response, Status};

use model::{
    entities::{CountryRestrictionEntity, StageEntity, TournamentEntity},
    *,
};
use proto::tournaments::{tournament_service_server::TournamentService, RankRange};
use proto::tournaments::{
    CreateTournamentRequest, CreateTournamentResponse, DeleteTournamentRequest,
    DeleteTournamentResponse, GetAllTournamentsRequest, GetAllTournamentsResponse,
    GetTournamentRequest, UpdateTournamentRequest, UpdateTournamentResponse,
};

use crate::LocalAppState;

pub struct TournamentServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl TournamentService for TournamentServiceImpl {
    type GetAllStream =
        futures::stream::Iter<std::vec::IntoIter<Result<GetAllTournamentsResponse, Status>>>;

    async fn get_all(
        &self,
        _request: Request<GetAllTournamentsRequest>,
    ) -> Result<Response<Self::GetAllStream>, Status> {
        let tournaments = TournamentEntity::find()
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get all tournaments: {e}")))?;

        // Get rank restrictions
        let mut futures = tournaments
            .iter()
            .map(|tournament| {
                rank_restriction::Entity::find()
                    .filter(rank_restriction::Column::TournamentId.eq(tournament.id))
                    .all(&self.0.db)
            })
            .collect::<FuturesOrdered<_>>();
        let mut rank_restrictions = Vec::with_capacity(tournaments.len());
        while let Some(restriction) = futures.next().await {
            rank_restrictions.push(
                restriction.map_err(|e| {
                    Status::internal(format!("failed to get rank restriction: {e}"))
                })?,
            );
        }

        // Get country restrictions
        let mut futures = tournaments
            .iter()
            .map(|tournament| {
                country_restriction::Entity::find()
                    .filter(country_restriction::Column::TournamentId.eq(tournament.id))
                    .all(&self.0.db)
            })
            .collect::<FuturesOrdered<_>>();
        let mut country_restrictions = Vec::with_capacity(tournaments.len());
        while let Some(countries) = futures.next().await {
            country_restrictions.push(
                countries.map_err(|e| {
                    Status::internal(format!("failed to get rank restriction: {e}"))
                })?,
            );
        }

        let mut response = Vec::with_capacity(tournaments.len());

        for i in 0..tournaments.len() {
            let tournament = &tournaments[i];
            let rank_restriction = &rank_restrictions[i];
            let country_restriction = &country_restrictions[i];
            let res = GetAllTournamentsResponse {
                id: tournament.id as i32,
                name: tournament.name.clone(),
                shorthand: tournament.shorthand.clone(),
                format: tournament.format as u32,
                bws: tournament.bws,
                rank_restrictions: rank_restriction
                    .into_iter()
                    .map(|r| RankRange {
                        min: r.min as u32,
                        max: r.max as u32,
                    })
                    .collect(),
                country_restrictions: country_restriction
                    .into_iter()
                    .map(|c| c.name.clone())
                    .collect(),
            };

            response.push(Ok(res));
        }

        Ok(Response::new(futures::stream::iter(response)))
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

        let rank_ranges = tournament
            .find_related(rank_restriction::Entity)
            .order_by_asc(rank_restriction::Column::Tier)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get rank restriction: {e}")))?
            .into_iter()
            .map(|r| RankRange {
                min: r.min as u32,
                max: r.max as u32,
            }).collect();


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
        tournament.rank_restrictions = rank_ranges;
        tournament.stages = stages;

        Ok(Response::new(tournament))
    }

    async fn create(
        &self,
        request: Request<CreateTournamentRequest>,
    ) -> Result<Response<CreateTournamentResponse>, Status> {
        use ActiveValue as A;
        let tournament = request.get_ref();
        let name = tournament.name.clone();
        let format = tournament.format as i32;

        // TODO Validate stuff like the rank ranges being in the right order

        let tournament_model = model::tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set(tournament.name.clone()),
            shorthand: A::Set(tournament.shorthand.clone()),
            format: A::Set(format),
            bws: A::Set(tournament.bws),
        };
        let tournament_model = tournament_model.insert(&self.0.db).await.map_err(|e| {
            Status::internal(format!(
                "failed to create tournament with name '{name}': {e}"
            ))
        })?;

        for (i, range) in tournament.rank_restrictions.iter().enumerate() {
            let restriction = model::rank_restriction::ActiveModel {
                tournament_id: A::Set(tournament_model.id),
                tier: A::Set(i as i32),
                min: A::Set(range.min as i32),
                max: A::Set(range.max as i32),
            };

            restriction
                .insert(&self.0.db)
                .await
                .map_err(|e| Status::internal(format!("failed to create rank restriction: {e}")))?;
        }

        for country in tournament.country_restrictions.iter() {
            let restriction = model::country_restriction::ActiveModel {
                tournament_id: A::Set(tournament_model.id),
                name: A::Set(country.clone()),
            };

            restriction.insert(&self.0.db).await.map_err(|e| {
                Status::internal(format!("failed to create country restriction: {e}"))
            })?;
        }

        Ok(Response::new(CreateTournamentResponse {
            id: tournament_model.id,
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
            .map(|r| r.ranges.as_slice())
        {
            // TODO Probably validate rank ranges
            rank_restriction::Entity::delete_many()
                .filter(rank_restriction::Column::TournamentId.eq(request.get_ref().id))
                .exec(&self.0.db)
                .await
                .map_err(|e| {
                    Status::internal(format!("could not delete rank restrictions: {e}"))
                })?;

            for (i, range) in ranges.iter().enumerate() {
                let restriction = rank_restriction::ActiveModel {
                    tournament_id: A::Set(request.get_ref().id),
                    tier: A::Set(i as i32),
                    min: A::Set(range.min as i32),
                    max: A::Set(range.max as i32),
                };

                restriction.insert(&self.0.db).await.map_err(|e| {
                    Status::internal(format!("failed to create rank restriction: {e}"))
                })?;
            }
        }

        if let Some(format) = request.get_ref().format {
            model.format = A::Set(format as i32);
        }

        if let Some(bws) = request.get_ref().bws {
            model.bws = A::Set(bws.clone());
        }

        model
            .update(&self.0.db)
            .await
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
