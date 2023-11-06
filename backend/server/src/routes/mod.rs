use utoipa::{IntoParams, ToSchema};

#[allow(unused)]
pub mod debug;
pub mod pool;
pub mod stage;
pub mod tournament;

/// A struct containing a tournament id and stage order used for querying
#[derive(Debug, Clone, Copy, serde::Deserialize, ToSchema, IntoParams)]
#[schema(example = json!({ "tournament_id": 152, "stage_order": 2 }))]
pub struct TournamentIdAndStageOrder {
    tournament_id: i32,
    stage_order: i16,
}
