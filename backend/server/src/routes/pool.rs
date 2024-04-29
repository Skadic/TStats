use super::tournament::find_stage;
use crate::AppState;
use futures::TryStreamExt;
use futures::{stream::FuturesOrdered, TryFutureExt};
use model::{pool_bracket, pool_map};
use proto::osu::api::get_map;
use proto::{
    keys::StageKey,
    pool::{
        pool_service_server::PoolService, update_pool_bracket_request::MapIds,
        CreatePoolBracketRequest, CreatePoolBracketResponse, DeletePoolBracketRequest,
        DeletePoolBracketResponse, DeletePoolRequest, DeletePoolResponse, GetPoolBracketRequest,
        GetPoolBracketResponse, GetPoolRequest, GetPoolResponse, Pool, PoolBracketMaps,
        UpdatePoolBracketRequest, UpdatePoolBracketResponse,
    },
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, FromQueryResult,
    IntoActiveModel, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};
use tonic::{Request, Response, Status};
use utils::LogStatus;

pub struct PoolServiceImpl(pub AppState);

#[tonic::async_trait]
impl PoolService for PoolServiceImpl {
    async fn get(
        &self,
        request: Request<GetPoolRequest>,
    ) -> Result<Response<GetPoolResponse>, Status> {
        let db = &self.0.db;
        let request = request.into_inner();
        let stage_key = request
            .stage_key
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;

        // Test if the tournament and stage exist
        let (_tournament, stage) = find_stage(&stage_key, db).await?;

        let pool = stage
            .find_related(pool_bracket::Entity)
            .find_with_related(pool_map::Entity)
            .order_by_asc(pool_bracket::Column::BracketOrder)
            .order_by_asc(pool_map::Column::MapOrder)
            .all(db)
            .await
            .map_err(|e| Status::internal(format!("error fetching pool: {e}")))
            .error_status()?;

        let brackets = pool
            .into_iter()
            .map(|(bracket, maps)| {
                // Get the map data from the osu api for each map
                maps.into_iter()
                    .map(|map| get_map(&self.0.redis, self.0.osu.as_ref(), map.map_id as u32))
                    .collect::<FuturesOrdered<_>>()
                    // Transform the beatmaps into the on-the-wire format
                    .try_collect::<Vec<_>>()
                    .map_ok(|maps| (bracket, maps))
                    // Transform the brackets into the on-the-wire format
                    .map_ok(|(bracket, maps)| proto::pool::PoolBracket {
                        bracket_order: bracket.bracket_order as u32,
                        name: bracket.name,
                        maps: Some(PoolBracketMaps { maps }),
                    })
            })
            // Collect each fetched bracket
            .collect::<FuturesOrdered<_>>()
            .try_collect::<Vec<_>>()
            .map_err(|e| Status::internal(format!("error fetching map info: {e}")))
            .await
            .error_status()?;

        Ok(Response::new(GetPoolResponse {
            pool: Some(Pool { brackets }),
        }))
    }

    async fn delete(
        &self,
        request: Request<DeletePoolRequest>,
    ) -> Result<Response<DeletePoolResponse>, Status> {
        let db = &self.0.db;
        let request = request.into_inner();
        let stage_key = request
            .stage_key
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;

        // Test if the tournament and stage exist
        let (tournament, stage) = find_stage(&stage_key, db).await?;

        pool_bracket::Entity::delete_many()
            .filter(pool_bracket::Column::TournamentId.eq(tournament.id))
            .filter(pool_bracket::Column::StageOrder.eq(stage.stage_order))
            .exec(db)
            .map_err(|e| Status::internal(format!("error deleting pool: {e}")))
            .await
            .error_status()?;

        Ok(Response::new(DeletePoolResponse {}))
    }

    async fn create_bracket(
        &self,
        request: Request<CreatePoolBracketRequest>,
    ) -> Result<Response<CreatePoolBracketResponse>, Status> {
        use ActiveValue as A;
        let db = &self.0.db;
        let request = request.into_inner();
        let stage_key = request
            .stage_key
            .ok_or_else(|| Status::invalid_argument("missing stage key"))?;

        // We don't allow empty bracket names
        if request.name.trim().is_empty() {
            return Err(Status::invalid_argument("empty bracket name"));
        }

        // Test if the tournament and stage exist
        let (tournament, stage) = find_stage(&stage_key, db).await.error_status()?;

        #[allow(unused)]
        #[derive(FromQueryResult, Debug)]
        struct MaxBracket {
            tournament_id: i32,
            stage_order: i16,
            bracket_order: i16,
        }

        // Find the max bracket_order for this pool so far
        let max_bracket = pool_bracket::Entity::find()
            .select_only()
            .column(pool_bracket::Column::TournamentId)
            .column(pool_bracket::Column::StageOrder)
            .column_as(
                Expr::col(pool_bracket::Column::BracketOrder).max(),
                "bracket_order",
            )
            .filter(pool_bracket::Column::TournamentId.eq(tournament.id))
            .filter(pool_bracket::Column::StageOrder.eq(stage.stage_order))
            .group_by(pool_bracket::Column::TournamentId)
            .group_by(pool_bracket::Column::StageOrder)
            .into_model::<MaxBracket>()
            .one(db)
            .map_err(|e| Status::internal(format!("error fetching bracket info: {e}")))
            .await
            .error_status()?;

        // Insert the bracket into the database
        let bracket = pool_bracket::ActiveModel {
            tournament_id: A::Set(tournament.id),
            stage_order: A::Set(stage_key.stage_order as i16),
            bracket_order: A::Set(
                // If there already is a bracket, use a bracket order one higher than the highest
                // one that exists. Otherwise, just use 0
                max_bracket
                    .map(|max| max.bracket_order + 1)
                    .unwrap_or_default(),
            ),
            name: A::Set(request.name),
        };

        let bracket = bracket
            .insert(db)
            .map_err(|e| Status::internal(format!("error inserting bracket: {e}")))
            .await
            .error_status()?;

        Ok(Response::new(CreatePoolBracketResponse {
            key: Some(proto::keys::PoolBracketKey {
                stage_key: Some(StageKey {
                    tournament_key: Some(proto::keys::TournamentKey {
                        id: bracket.tournament_id as i32,
                    }),
                    stage_order: bracket.stage_order as u32,
                }),
                bracket_order: bracket.bracket_order as u32,
            }),
        }))
    }

    async fn get_bracket(
        &self,
        request: Request<GetPoolBracketRequest>,
    ) -> Result<Response<GetPoolBracketResponse>, Status> {
        let db = &self.0.db;
        let request = request.into_inner();
        let bracket_key = request
            .key
            .ok_or_else(|| Status::invalid_argument("missing bracket key"))?;
        let stage_key = bracket_key
            .stage_key
            .ok_or_else(|| Status::invalid_argument("missing stage key in bracket key"))?;

        // Test if the tournament and stage exist
        let (tournament, stage) = find_stage(&stage_key, db).await?;

        // Find the bracket
        let bracket = stage
            .find_related(pool_bracket::Entity)
            .filter(pool_bracket::Column::BracketOrder.eq(bracket_key.bracket_order))
            .one(db)
            .map_err(|e| Status::internal(format!("error fetching pool bracket: {e}")))
            .await?
            .ok_or_else(|| {
                Status::not_found(format!(
                    "bracket {} in stage {} of tournament {} not found",
                    bracket_key.bracket_order, stage.stage_order, tournament.id
                ))
            })
            .error_status()?;

        let maps = bracket
            .find_related(pool_map::Entity)
            .order_by_asc(pool_map::Column::MapOrder)
            .all(db)
            .map_err(|e| Status::internal(format!("error fetching pool maps: {e}")))
            .await
            .error_status()?;

        let maps = maps
            .into_iter()
            .map(|map| get_map(&self.0.redis, self.0.osu.as_ref(), map.map_id as u32))
            .collect::<FuturesOrdered<_>>()
            .map_err(|e| Status::internal(format!("error fetching map info: {e}")))
            .try_collect::<Vec<_>>()
            .await
            .error_status()?;

        Ok(Response::new(GetPoolBracketResponse {
            bracket: Some(proto::pool::PoolBracket {
                bracket_order: bracket.bracket_order as u32,
                name: bracket.name,
                maps: Some(PoolBracketMaps { maps }),
            }),
        }))
    }

    async fn update_bracket(
        &self,
        request: Request<UpdatePoolBracketRequest>,
    ) -> Result<Response<UpdatePoolBracketResponse>, Status> {
        use ActiveValue as A;
        let db = &self.0.db;
        let request = request.into_inner();
        let pool_bracket_key = request
            .key
            .ok_or_else(|| Status::invalid_argument("missing pool bracket key"))?;
        let stage_key = pool_bracket_key
            .stage_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key in pool bracket key"))?;
        // Test if the tournament and stage exist
        let (tournament, stage) = find_stage(stage_key, db).await?;

        let mut bracket = pool_bracket::Entity::find_by_id((
            tournament.id,
            stage.stage_order,
            pool_bracket_key.bracket_order as i16,
        ))
        .one(db)
        .map_err(|e| Status::not_found(format!("error fetching pool bracket: {e}")))
        .await?
        .ok_or_else(|| {
            Status::not_found(format!(
                "bracket {} in stage {} of tournament {} does not exist",
                pool_bracket_key.bracket_order, stage.stage_order, tournament.id
            ))
        })?
        .into_active_model();

        // Update bracket name
        if let Some(name) = request.name {
            // We don't allow empty bracket names
            if name.trim().is_empty() {
                return Err(Status::invalid_argument("empty bracket name"));
            }
            bracket.name = A::Set(name);
        }

        // Update bracket order
        if let Some(bracket_order) = request.bracket_order {
            bracket.bracket_order = A::Set(bracket_order as i16);
        }

        // Update maps
        if let Some(MapIds { maps }) = request.maps {
            let tournament_id = tournament.id;
            let stage_order = stage_key.stage_order as i16;
            let bracket_order = pool_bracket_key.bracket_order as i16;

            // Delete all old maps
            pool_map::Entity::delete_many()
                .filter(pool_map::Column::TournamentId.eq(tournament_id))
                .filter(pool_map::Column::StageOrder.eq(stage_order))
                .filter(pool_map::Column::BracketOrder.eq(bracket_order))
                .exec(db)
                .map_err(|e| Status::internal(format!("error deleting old pool maps: {e}")))
                .await
                .error_status()?;

            // Insert the new maps
            pool_map::Entity::insert_many(maps.into_iter().enumerate().map(
                |(map_order, map_id)| pool_map::ActiveModel {
                    tournament_id: A::Set(tournament_id),
                    stage_order: A::Set(stage_order),
                    bracket_order: A::Set(bracket_order),
                    map_order: A::Set(map_order as i16),
                    map_id: A::Set(map_id as i64),
                },
            ))
            .exec(db)
            .map_err(|e| Status::internal(format!("error inserting pool maps: {e}")))
            .await
            .error_status()?;
        }

        // We want to get the update bracket back
        let GetPoolBracketResponse { bracket } = self
            .get_bracket(Request::new(GetPoolBracketRequest {
                key: Some(pool_bracket_key),
            }))
            .await
            .error_status()?
            .into_inner();

        Ok(Response::new(UpdatePoolBracketResponse { bracket }))
    }

    async fn delete_bracket(
        &self,
        request: Request<DeletePoolBracketRequest>,
    ) -> Result<Response<DeletePoolBracketResponse>, Status> {
        let db = &self.0.db;
        let request = request.into_inner();
        let pool_bracket_key = request
            .key
            .ok_or_else(|| Status::invalid_argument("missing pool bracket key"))?;
        let stage_key = pool_bracket_key
            .stage_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing stage key in pool bracket key"))?;
        // Test if the tournament and stage exist
        let (tournament, stage) = find_stage(stage_key, db).await.error_status()?;

        let delete_res = pool_bracket::Entity::delete_by_id((
            tournament.id,
            stage.stage_order,
            pool_bracket_key.bracket_order as i16,
        ))
        .exec(db)
        .map_err(|e| Status::not_found(format!("error fetching pool bracket: {e}")))
        .await
        .error_status()?;

        if delete_res.rows_affected == 0 {
            return Err(Status::not_found(format!(
                "bracket {} in stage {} of tournament {} not found",
                pool_bracket_key.bracket_order, stage_key.stage_order, tournament.id
            )));
        }

        Ok(Response::new(DeletePoolBracketResponse {}))
    }
}
