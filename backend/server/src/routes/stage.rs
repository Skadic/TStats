use super::tournament::find_stage;
use proto::osu::api::get_map;
use crate::{ routes::convert_start_end, AppState};
use futures::{stream::FuturesOrdered, TryFutureExt, TryStreamExt};
use model::stage;
use proto::stages::{
    stage_service_server::StageService, CreateStageRequest, CreateStageResponse,
    DeleteStageRequest, DeleteStageResponse, GetAllStagesRequest, GetAllStagesResponse,
    GetStageRequest, GetStageResponse, UpdateStageRequest, UpdateStageResponse,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, IntoActiveModel,
    LoaderTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};
use tonic::{Request, Response, Status};

pub struct StageServiceImpl(pub AppState);

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
                    start_date: stage.start_date.map(Into::into),
                    end_date: stage.end_date.map(Into::into),
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
        let db = &self.0.db;
        let stage_key = request
            .get_ref()
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        // Find the tournament and the associated stage
        let (_tournament, stage) = find_stage(stage_key, db).await?;

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

        // Fetch map info from the osu api and transform them to the on-the-wire format
        let fetched_maps = maps
            .into_iter()
            .map(|bracket| {
                bracket
                    .into_iter()
                    .map(|map| get_map(&self.0.redis, self.0.osu.as_ref(), map.map_id as u32))
                    .collect::<FuturesOrdered<_>>()
                    .map_ok(proto::osu::Beatmap::from)
                    .map_err(|e| Status::internal(format!("error fetching map data: {e}")))
                    .try_collect::<Vec<_>>()
            })
            .collect::<FuturesOrdered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        // Compose the response
        let response = GetStageResponse {
            key: Some(stage_key.clone()),
            stage: Some(proto::stages::Stage {
                name: stage.name.clone(),
                best_of: stage.best_of as u32,
                stage_order: stage_key.stage_order,
                start_date: stage.start_date.map(Into::into),
                end_date: stage.end_date.map(Into::into),
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
        let request = request.into_inner();
        let tournament_key = request
            .tournament_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tournament key"))?;

        #[allow(unused)]
        #[derive(FromQueryResult, Debug)]
        struct MaxStage {
            tournament_id: i32,
            stage_order: i16,
        }

        // Find the max bracket_order for this pool so far
        let max_bracket = stage::Entity::find()
            .select_only()
            .column(stage::Column::TournamentId)
            .column_as(Expr::col(stage::Column::StageOrder).max(), "bracket_order")
            .filter(stage::Column::TournamentId.eq(tournament_key.id))
            .group_by(stage::Column::TournamentId)
            .into_model::<MaxStage>()
            .one(&self.0.db)
            .map_err(|e| Status::internal(format!("error fetching stage info: {e}")))
            .await?;
        let new_stage_order = max_bracket
            .map(|max| max.stage_order + 1)
            .unwrap_or_default();

        let (start_date, end_date) = convert_start_end(request.start_date, request.end_date)?;

        let stage = model::stage::ActiveModel {
            tournament_id: A::Set(tournament_key.id),
            name: A::Set(request.name),
            stage_order: A::Set(new_stage_order),
            best_of: A::Set(request.best_of as i16),
            start_date: A::Set(start_date),
            end_date: A::Set(end_date),
        };

        stage
            .insert(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("failed to create stage: {e}")))?;

        Ok(Response::new(CreateStageResponse {
            stage_order: new_stage_order as u32,
        }))
    }

    async fn update(
        &self,
        request: Request<UpdateStageRequest>,
    ) -> Result<Response<UpdateStageResponse>, Status> {
        use sea_orm::ActiveValue as A;
        let db = &self.0.db;
        let req = request.into_inner();
        let stage_key = req
            .key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let (_tournament, stage) = find_stage(stage_key, db).await?;

        // Update values
        let mut stage = stage.into_active_model();
        if let Some(name) = req.name {
            stage.name = A::Set(name);
        }
        if let Some(best_of) = req.best_of {
            stage.best_of = A::Set(best_of as i16);
        }

        // Update in database
        let stage = stage
            .update(&self.0.db)
            .await
            .map_err(|e| Status::internal(format!("could not update stage: {e}")))?;
        
        Ok(Response::new(UpdateStageResponse {
            stage: Some(proto::stages::Stage {
                name: stage.name,
                best_of: stage.best_of as u32,
                stage_order: stage.stage_order as u32,
                start_date: stage.start_date.map(Into::into),
                end_date: stage.end_date.map(Into::into),
            }),
        }))
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
