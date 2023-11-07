use crate::{osu::map::get_map, LocalAppState};
use futures::{stream::FuturesOrdered, FutureExt, TryFutureExt, TryStreamExt};
use model::{pool_bracket, pool_map, stage, tournament};
use proto::pool::{
    pool_service_server::PoolService, CreatePoolBracketRequest, CreatePoolBracketResponse,
    DeletePoolBracketRequest, DeletePoolBracketResponse, DeletePoolRequest, DeletePoolResponse,
    GetPoolRequest, GetPoolResponse, Pool, PoolBracketMaps, UpdatePoolBracketRequest,
    UpdatePoolBracketResponse,
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use tonic::{Request, Response, Status};

pub struct PoolServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl PoolService for PoolServiceImpl {
    async fn get(
        &self,
        request: Request<GetPoolRequest>,
    ) -> Result<Response<GetPoolResponse>, Status> {
        let db = &self.0.db;
        let redis = self.0.redis.read().await;
        let request = request.into_inner();
        let stage_key = request
            .stage_key
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;
        let tournament_key = stage_key
            .tournament_key
            .ok_or_else(|| Status::invalid_argument("missing tournament key in stage key"))?;

        let res = tournament::Entity::find_by_id(tournament_key.id)
            .find_also_related(stage::Entity)
            .filter(stage::Column::StageOrder.eq(stage_key.stage_order))
            .one(db)
            .await
            .map_err(|e| Status::internal(format!("error fetching tournament: {e}")))?;

        // Test if the tournament and stage exist
        let (_tournament, stage) = match res {
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

        let pool = stage
            .find_related(pool_bracket::Entity)
            .find_with_related(pool_map::Entity)
            .order_by_asc(pool_bracket::Column::BracketOrder)
            .order_by_asc(pool_map::Column::MapOrder)
            .all(db)
            .await
            .map_err(|e| Status::internal(format!("error fetching pool: {e}")))?;

        let brackets = pool
            .into_iter()
            .map(|(bracket, maps)| {
                // Get the map data from the osu api for each map
                maps.into_iter()
                    .map(|map| {
                        get_map(redis.clone(), self.0.osu.as_ref(), map.map_id as u32)
                            .map(|res| res.map(proto::osu::Beatmap::from))
                    })
                    // Collect them into a map
                    .collect::<FuturesOrdered<_>>()
                    .try_collect::<Vec<_>>()
                    .map(|maps_res| maps_res.map(|maps| (bracket, maps)))
            })
            // Collect each fetched bracket 
            .collect::<FuturesOrdered<_>>()
            .try_collect::<Vec<_>>()
            .map(|res| {
                res.map(|brackets| {
                    brackets
                        .into_iter()
                        .map(|(bracket, maps)| proto::pool::PoolBracket {
                            bracket_order: bracket.bracket_order as u32,
                            name: bracket.name,
                            maps: Some(PoolBracketMaps { maps }),
                        })
                        .collect::<Vec<_>>()
                })
            })
            .map_err(|e| Status::internal(format!("error fetching map info: {e}")))
            .await?;

        Ok(Response::new(GetPoolResponse {
            pool: Some(Pool { brackets }),
        }))
    }

    async fn delete(
        &self,
        _request: Request<DeletePoolRequest>,
    ) -> Result<Response<DeletePoolResponse>, Status> {
        todo!()
    }

    async fn create_bracket(
        &self,
        _request: Request<CreatePoolBracketRequest>,
    ) -> Result<Response<CreatePoolBracketResponse>, Status> {
        todo!()
    }

    async fn update_bracket(
        &self,
        _request: Request<UpdatePoolBracketRequest>,
    ) -> Result<Response<UpdatePoolBracketResponse>, Status> {
        todo!()
    }

    async fn delete_bracket(
        &self,
        _request: Request<DeletePoolBracketRequest>,
    ) -> Result<Response<DeletePoolBracketResponse>, Status> {
        todo!()
    }
}
