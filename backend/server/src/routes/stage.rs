use futures::{stream::FuturesOrdered, StreamExt};
use proto::stages::{
    stage_service_server::StageService, CreateStageRequest, CreateStageResponse,
    DeleteStageRequest, DeleteStageResponse, GetAllStagesRequest, GetAllStagesResponse,
    GetStageRequest, GetStageResponse, UpdateStageRequest, UpdateStageResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, LoaderTrait, ModelTrait,
    QueryFilter, QueryOrder,
};
use tonic::{Request, Response, Status};

use crate::{
    osu::map::{get_map, Difficulty, SlimBeatmap},
    LocalAppState,
};

pub struct StageServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl StageService for StageServiceImpl {
    type GetAllStream =
        futures::stream::Iter<std::vec::IntoIter<Result<GetAllStagesResponse, Status>>>;

    async fn get_all(
        &self,
        request: Request<GetAllStagesRequest>,
    ) -> Result<Response<Self::GetAllStream>, Status> {
        let tournament_key = request
            .get_ref()
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key"))?;

        // find the tournament with its related stages
        let res = model::tournament::Entity::find_by_id(tournament_key.id)
            .find_with_related(model::stage::Entity)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not load tournament and stage: {e}")))?;

        let stages = match res.into_iter().next() {
            Some((_, stages)) => stages,
            None => {
                return Err(Status::not_found(format!(
                    "tournament with id {} does not exist",
                    tournament_key.id
                )))
            }
        };

        // Convert the stages into the "on-the-wire" format
        let stages = stages
            .into_iter()
            .map(|stage| GetAllStagesResponse {
                key: Some(proto::keys::StageKey {
                    tournament_key: Some(tournament_key.clone()),
                    stage_order: stage.stage_order as u32,
                }),
                stage: Some(proto::stages::Stage {
                    name: stage.name.clone(),
                    best_of: stage.best_of as u32,
                    stage_order: stage.stage_order as u32,
                }),
            })
            .map(Result::Ok)
            .collect::<Vec<_>>();

        // Send the reques
        Ok(Response::new(futures::stream::iter(stages)))
    }

    async fn get(
        &self,
        request: Request<GetStageRequest>,
    ) -> Result<Response<GetStageResponse>, Status> {
        let stage_key = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let tournament_key = stage_key
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

        // Find the tournament and the associated stage
        let res = model::tournament::Entity::find_by_id(tournament_key.id)
            .find_also_related(model::stage::Entity)
            .filter(model::stage::Column::StageOrder.eq(stage_key.stage_order))
            .one(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not load tournament and stage: {e}")))?;

        let (_tournament, stage) = match res {
            Some((tournament, Some(stage))) => (tournament, stage),
            Some(_) => {
                return Err(Status::not_found(format!(
                    "stage {} in tournament with id {} does not exist",
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

        // Find associated pool brackets
        let brackets = stage
            .find_related(model::pool_bracket::Entity)
            .order_by_asc(model::pool_bracket::Column::BracketOrder)
            .all(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not load pool brackets: {e}")))?;

        let maps = brackets
            .load_many(
                model::pool_map::Entity::find().order_by_asc(model::pool_map::Column::MapOrder),
                &self.0.db,
            )
            .await
            .map_err(|e| Status::internal(format!("could not load pool maps: {e}")))?;

        let mut fetched_maps = Vec::with_capacity(maps.len());
        let redis = self.0.redis.read().await;

        // Fetch map info from the osu api and transform them to the on-the-wire format
        for bracket in maps {
            // Fetch info for all maps
            let n = bracket.len();
            let results = bracket
                .into_iter()
                .map(|map| get_map(redis.clone(), self.0.osu.as_ref(), map.map_id as u32))
                .collect::<FuturesOrdered<_>>()
                .collect::<Vec<_>>()
                .await;
            // Create a new vec to hold the fetched maps
            fetched_maps.push(Vec::with_capacity(n));
            // Check for each map whether there was an error and transform them to the on-the-wire format
            let fetched = fetched_maps.last_mut().unwrap();
            for map in results {
                let map =
                    map.map_err(|e| Status::internal(format!("error fetching map data: {e}")))?;

                let SlimBeatmap {
                    artist_name,
                    name,
                    diff_name,
                    set_id,
                    map_id,
                    creator,
                    difficulty,
                } = map;

                let Difficulty {
                    stars,
                    length,
                    bpm,
                    cs,
                    ar,
                    od,
                    hp,
                } = difficulty;

                let pool_map = proto::osu::Beatmap {
                    artist_name,
                    name,
                    difficulty_name: diff_name,
                    mapset_id: set_id,
                    map_id: map_id as u64,
                    creator: Some(proto::osu::User {
                        user_id: creator.user_id,
                        username: creator.username.into_string(),
                        country: creator.country.into_string(),
                        cover_url: creator.cover_url,
                    }),
                    difficulty: Some(proto::osu::Difficulty {
                        stars,
                        length,
                        bpm,
                        cs,
                        ar,
                        od,
                        hp,
                    }),
                };
                fetched.push(pool_map);
            }
        }

        // Compose the response
        let response = GetStageResponse {
            key: Some(stage_key.clone()),
            stage: Some(proto::stages::Stage {
                name: stage.name.clone(),
                best_of: stage.best_of as u32,
                stage_order: stage_key.stage_order as u32,
            }),
            pool: Some(proto::pool::Pool {
                brackets: brackets
                    .into_iter()
                    .zip(fetched_maps)
                    .map(|(bracket, maps)| proto::pool::PoolBracket {
                        bracket_order: bracket.bracket_order as u32,
                        name: bracket.name.clone(),
                        maps: Some(proto::pool::PoolBracketMaps { maps }),
                    })
                    .collect(),
            }),
        };

        Ok(Response::new(response))
    }

    async fn create(
        &self,
        request: Request<CreateStageRequest>,
    ) -> Result<Response<CreateStageResponse>, Status> {
        use sea_orm::ActiveValue as A;
        let stage = request
            .get_ref()
            .stage
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage"))?;
        let stage_key = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let tournament_key = stage_key
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

        let stage = model::stage::ActiveModel {
            tournament_id: A::Set(tournament_key.id),
            name: A::Set(stage.name.clone()),
            stage_order: A::Set(stage_key.stage_order as i16),
            best_of: A::Set(stage.best_of as i16),
        };

        stage
            .insert(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to create stage: {e}")))?;

        Ok(Response::new(CreateStageResponse {}))
    }

    async fn update(
        &self,
        request: Request<UpdateStageRequest>,
    ) -> Result<Response<UpdateStageResponse>, Status> {
        use sea_orm::ActiveValue as A;
        let req = request.get_ref();
        let stage_key = req
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let tournament_key = stage_key
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

        // Find the tournament and the associated stage
        let res = model::tournament::Entity::find_by_id(tournament_key.id)
            .find_also_related(model::stage::Entity)
            .filter(model::stage::Column::StageOrder.eq(stage_key.stage_order))
            .one(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not load tournament and stage: {e}")))?;

        // Check if tournament/stage exists
        let (_tournament, stage) = match res {
            Some((tournament, Some(stage))) => (tournament, stage),
            Some(_) => {
                return Err(Status::not_found(format!(
                    "stage {} in tournament with id {} does not exist",
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

        // Update values
        let mut stage = stage.into_active_model();
        if let Some(ref name) = req.name {
            stage.name = A::Set(name.clone());
        }
        if let Some(best_of) = req.best_of {
            stage.best_of = A::Set(best_of as i16);
        }

        // Update in database
        stage
            .update(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not update stage: {e}")))?;

        Ok(Response::new(UpdateStageResponse {}))
    }

    async fn delete(
        &self,
        request: Request<DeleteStageRequest>,
    ) -> Result<Response<DeleteStageResponse>, Status> {
        // Unpack keys from request
        let req = request.get_ref();
        let stage_key = req
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let tournament_key = stage_key
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

        // Delete Stage
        let delete_result =
            model::stage::Entity::delete_by_id((tournament_key.id, stage_key.stage_order as i16))
                .exec(&self.0.db)
                .await
                .map_err(|e| Status::internal(format!("could not delete stage: {e}")))?;

        // If no stage was delete, that means that it didn't exist
        if delete_result.rows_affected == 0 {
            return Err(Status::not_found(format!(
                "could not find stage {} in tournament {}",
                stage_key.stage_order, tournament_key.id,
            )));
        }

        Ok(Response::new(DeleteStageResponse {}))
    }
}
