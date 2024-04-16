use futures::TryFutureExt;
use itertools::izip;
use sea_orm::{
    query::*, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait,
};
use tonic::{Request, Response, Status};

use model::{
    entities::{CountryRestrictionEntity, StageEntity, TournamentEntity},
    *,
};
use proto::{
    keys::StageKey,
    tournaments::{
        Country, CountryList, CreateTournamentRequest, CreateTournamentResponse,
        DeleteTournamentRequest, DeleteTournamentResponse, GetAllTournamentsRequest,
        GetAllTournamentsResponse, GetTournamentRequest, RangeList, UpdateTournamentRequest,
        UpdateTournamentResponse,
    },
};
use proto::{
    keys::TournamentKey,
    tournaments::{
        tournament_service_server::TournamentService, GetTournamentResponse, RankRange, Tournament,
    },
};

use crate::AppState;

pub async fn find_stage(
    stage_key: &StageKey,
    db: &DatabaseConnection,
) -> tonic::Result<(tournament::Model, stage::Model)> {
    let tournament_key = stage_key
        .tournament_key
        .as_ref()
        .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

    let res = tournament::Entity::find_by_id(tournament_key.id)
        .find_also_related(stage::Entity)
        .filter(stage::Column::StageOrder.eq(stage_key.stage_order))
        .one(db)
        .await
        .map_err(|e| Status::internal(format!("error fetching tournament: {e}")))?;

    // Test if the tournament and stage exist
    let (tournament, stage) = match res {
        Some((tournament, Some(stage))) => (tournament, stage),
        Some((_, None)) => {
            return Err(Status::not_found(format!(
                "stage {} in tournament {} does not exist",
                stage_key.stage_order, tournament_key.id
            )))
        }
        None => {
            return Err(Status::not_found(format!(
                "tournament with id {} does not exist",
                tournament_key.id
            )))
        }
    };

    Ok((tournament, stage))
}

pub struct TournamentServiceImpl(pub AppState);

#[tonic::async_trait]
impl TournamentService for TournamentServiceImpl {
    type GetAllStream = futures::stream::Iter<
        Box<dyn Iterator<Item = Result<GetAllTournamentsResponse, Status>> + Send + Sync>,
    >;
    //futures::stream::Iter<std::vec::IntoIter<Result<GetAllTournamentsResponse, Status>>>;

    async fn get_all(
        &self,
        _request: Request<GetAllTournamentsRequest>,
    ) -> Result<Response<Self::GetAllStream>, Status> {
        let db = &self.0.db;
        let tournaments = TournamentEntity::find()
            .all(db)
            .map_err(|e| Status::internal(format!("failed to get all tournaments: {e}")))
            .await?;

        // Get rank restrictions
        let rank_restrictions = tournaments
            .load_many(
                rank_restriction::Entity::find().order_by_asc(rank_restriction::Column::Tier),
                db,
            )
            .map_err(|e| Status::internal(format!("failed to get rank restrictions: {e}")));

        // Get country restrictions
        let country_restrictions = tournaments
            .load_many(
                country_restriction::Entity::find().order_by_asc(country_restriction::Column::Name),
                db,
            )
            .map_err(|e| Status::internal(format!("failed to get country restrictions: {e}")));

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions) =
            tokio::join!(rank_restrictions, country_restrictions);
        let (rank_restrictions, country_restrictions) = (rank_restrictions?, country_restrictions?);

        let iter: Box<
            dyn Iterator<Item = Result<GetAllTournamentsResponse, Status>> + Send + Sync,
        > = Box::new(
            izip!(tournaments, rank_restrictions, country_restrictions).map(
                |(tournament, rank_restriction, country_restriction)| {
                    let rank_restrictions = Some(RangeList {
                        ranges: rank_restriction
                            .iter()
                            .map(|r| RankRange {
                                min: r.min as u32,
                                max: r.max as u32,
                            })
                            .collect(),
                    });
                    let country_restrictions = Some(CountryList {
                        countries: country_restriction
                            .iter()
                            .map(|c| Country {
                                name: c.name.clone(),
                            })
                            .collect(),
                    });
                    Ok(GetAllTournamentsResponse {
                        tournament: Some(Tournament {
                            key: Some(TournamentKey { id: tournament.id }),
                            name: tournament.name,
                            shorthand: tournament.shorthand,
                            format: tournament.format as u32,
                            bws: tournament.bws,
                        }),
                        rank_restrictions,
                        country_restrictions,
                    })
                },
            ),
        );

        Ok(Response::new(futures::stream::iter(iter)))
    }

    async fn get(
        &self,
        request: Request<GetTournamentRequest>,
    ) -> Result<Response<GetTournamentResponse>, Status> {
        let id = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament id"))?
            .id;
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
            .map(|stage| proto::stages::Stage {
                name: stage.name,
                best_of: stage.best_of as u32,
                stage_order: stage.stage_order as u32,
            })
            .collect::<Vec<_>>();

        let ranges = tournament
            .find_related(rank_restriction::Entity)
            .order_by_asc(rank_restriction::Column::Tier)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get rank restriction: {e}")))?
            .into_iter()
            .map(|r| RankRange {
                min: r.min as u32,
                max: r.max as u32,
            })
            .collect::<Vec<_>>();

        // Find all country restrictions for this tournament in the database
        let countries = tournament
            .find_related(CountryRestrictionEntity)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to get country restrictions: {e}")))?
            .into_iter()
            .map(|cr| Country { name: cr.name })
            .collect::<Vec<_>>();

        let tournament = GetTournamentResponse {
            tournament: Some(Tournament {
                key: Some(TournamentKey { id: tournament.id }),
                name: tournament.name,
                shorthand: tournament.shorthand,
                format: tournament.format as u32,
                bws: tournament.bws,
            }),
            country_restrictions: Some(CountryList { countries }),
            rank_restrictions: Some(RangeList { ranges }),
            stages,
        };

        Ok(Response::new(tournament))
    }

    async fn create(
        &self,
        request: Request<CreateTournamentRequest>,
    ) -> Result<Response<CreateTournamentResponse>, Status> {
        use ActiveValue as A;
        let tournament = request
            .get_ref()
            .tournament
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament"))?;
        let name = tournament.name.clone();
        let format = tournament.format as i32;

        // TODO Validate stuff like the rank ranges being in the right order

        let tournament_model = model::tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set(tournament.name.clone()),
            shorthand: A::Set(tournament.shorthand.clone()),
            format: A::Set(format),
            bws: A::Set(tournament.bws),
            // TODO Actually get the mode from the API
            mode: A::Set(tournament::Mode::Osu)
        };
        let tournament_model = tournament_model.insert(&self.0.db).await.map_err(|e| {
            Status::internal(format!(
                "failed to create tournament with name '{name}': {e}"
            ))
        })?;

        if let Some(ref rank_restrictions) = request.get_ref().rank_restrictions {
            for (i, range) in rank_restrictions.ranges.iter().enumerate() {
                let restriction = model::rank_restriction::ActiveModel {
                    tournament_id: A::Set(tournament_model.id),
                    tier: A::Set(i as i32),
                    min: A::Set(range.min as i32),
                    max: A::Set(range.max as i32),
                };

                restriction.insert(&self.0.db).await.map_err(|e| {
                    Status::internal(format!("failed to create rank restriction: {e}"))
                })?;
            }
        }

        if let Some(ref country_restrictions) = request.get_ref().country_restrictions {
            for country in country_restrictions.countries.iter() {
                let restriction = model::country_restriction::ActiveModel {
                    tournament_id: A::Set(tournament_model.id),
                    name: A::Set(country.name.clone()),
                };

                restriction.insert(&self.0.db).await.map_err(|e| {
                    Status::internal(format!("failed to create country restriction: {e}"))
                })?;
            }
        }

        Ok(Response::new(CreateTournamentResponse {
            key: Some(TournamentKey {
                id: tournament_model.id,
            }),
        }))
    }

    async fn update(
        &self,
        request: Request<UpdateTournamentRequest>,
    ) -> Result<Response<UpdateTournamentResponse>, Status> {
        let tournament_id = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament id"))?
            .id;
        use ActiveValue as A;
        let model = TournamentEntity::find_by_id(tournament_id)
            .one(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to fetch tournament: {e}")))?
            .ok_or_else(|| {
                Status::not_found(format!("tournament with id {} not found", tournament_id))
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
                .filter(rank_restriction::Column::TournamentId.eq(tournament_id))
                .exec(&self.0.db)
                .await
                .map_err(|e| {
                    Status::internal(format!("could not delete rank restrictions: {e}"))
                })?;

            for (i, range) in ranges.iter().enumerate() {
                let restriction = rank_restriction::ActiveModel {
                    tournament_id: A::Set(tournament_id),
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
            model.bws = A::Set(bws);
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
        let id = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament id"))?
            .id;
        TournamentEntity::delete_by_id(id)
            .exec(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not delete tournament: {e}")))?;

        Ok(Response::new(DeleteTournamentResponse {}))
    }
}
