use proto::{
    pool_brackets::PoolBracket,
    stages::{
        stage_service_server::StageService, CreateStageRequest, CreateStageResponse,
        DeleteStageRequest, DeleteStageResponse, GetAllStagesRequest, GetAllStagesResponse,
        GetStageRequest, GetStageResponse, UpdateStageRequest, UpdateStageResponse,
    },
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use tonic::{Request, Response, Status};

use crate::LocalAppState;

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
                stage: Some(proto::stages::Stage {
                    key: Some(proto::keys::StageKey {
                        tournament_key: Some(tournament_key.clone()),
                        stage_order: stage.stage_order as u32,
                    }),
                    name: stage.name.clone(),
                    best_of: stage.best_of as u32,
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

        let response = GetStageResponse {
            stage: Some(proto::stages::Stage {
                key: Some(stage_key.clone()),
                name: stage.name.clone(),
                best_of: stage.best_of as u32,
            }),
            pool_brackets: brackets
                .into_iter()
                .map(|bracket| PoolBracket {
                    key: Some(proto::keys::PoolBracketKey {
                        stage_key: Some(stage_key.clone()),
                        bracket_order: bracket.bracket_order as u32,
                    }),
                    name: bracket.name.clone(),
                })
                .collect(),
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
        let stage_key = stage
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
        _request: Request<UpdateStageRequest>,
    ) -> Result<Response<UpdateStageResponse>, Status> {
        Err(Status::unimplemented("Update unimplemented"))
    }

    async fn delete(
        &self,
        _request: Request<DeleteStageRequest>,
    ) -> Result<Response<DeleteStageResponse>, Status> {
        Err(Status::unimplemented("Delete unimplemented"))
    }
}
