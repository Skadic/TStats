use proto::{
    keys::PoolMapKey,
    scores::{score_service_server::ScoreService, GetScoresRequest, GetScoresResponse},
};
use tonic::{Request, Response, Status};
use tracing::error;

use crate::AppState;
use proto::osu::api::get_map;

struct ScoreServiceImpl(AppState);

#[tonic::async_trait]
impl ScoreService for ScoreServiceImpl {
    #[tracing::instrument(skip_all)]
    async fn get(
        &self,
        request: Request<GetScoresRequest>,
    ) -> tonic::Result<Response<GetScoresResponse>> {
        let state = &self.0;
        let request = request.into_inner();
        let ExtractedPoolMapKey {
            tournament_id,
            stage_order,
            bracket_order,
            map_order,
        } = extract_pool_map_key(request.pool_map_key)?;

        let v = sqlx::query!(
            "
            SELECT tournament_id, stage_order, bracket_order, map_order, map_id, player_id, score FROM pool_map
            LEFT JOIN score USING (tournament_id, stage_order, bracket_order, map_order)
            WHERE tournament_id = $1 AND stage_order = $2 AND bracket_order = $3 AND map_order = $4
            ", tournament_id, stage_order as i32, bracket_order as i32, map_order as i32
        )
        .fetch_all(&state.sqlx)
        .await
        .map_err(|error| {
            error!(%error, "could not query database for scores");
            Status::internal("could not get scores")
        })?;

        if v.is_empty() {
            error!(
                tournament_id,
                stage_order, bracket_order, map_order, "map does not exist in pool"
            );
            return Err(Status::not_found("map does not exist in pool"));
        }

        let map_id = v[0].map_id;
        let map = get_map(&self.0.redis, &self.0.osu, map_id as u32)
            .await
            .map_err(|error| {
                error!(%error, map_id, "error getting map from osu api");
                Status::internal("could not get map from osu api")
            })?;

        //proto::osu::Beatmap {
        //
        //      }

        todo!()
    }
}

struct ExtractedPoolMapKey {
    tournament_id: i32,
    stage_order: u32,
    bracket_order: u32,
    map_order: u32,
}

fn extract_pool_map_key(key: Option<PoolMapKey>) -> tonic::Result<ExtractedPoolMapKey> {
    let Some(pool_map_key) = key else {
        return Err(Status::invalid_argument("no pool map key"));
    };
    let Some(bracket_key) = pool_map_key.bracket_key else {
        return Err(Status::invalid_argument("no bracket key"));
    };
    let Some(stage_key) = bracket_key.stage_key else {
        return Err(Status::invalid_argument("no stage key"));
    };
    let Some(tournament_key) = stage_key.tournament_key else {
        return Err(Status::invalid_argument("no tournament key"));
    };
    let map_order = pool_map_key.map_order;
    let bracket_order = bracket_key.bracket_order;
    let stage_order = stage_key.stage_order;
    let tournament_id = tournament_key.id;

    Ok(ExtractedPoolMapKey {
        map_order,
        bracket_order,
        stage_order,
        tournament_id,
    })
}
