use utoipa::{IntoParams, ToSchema};

pub mod debug;
pub mod pool;
pub mod stage;
pub mod tournament;

/// A struct containing a simple numeric id, used among other things to query for a specific entity by its id.
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct Id {
    pub id: i32,
}

/// A struct containing a tournament id used for querying
#[derive(Debug, Clone, Copy, serde::Deserialize, ToSchema, IntoParams)]
#[schema(example = json!({ "tournament_id": 152 }))]
pub struct TournamentId {
    tournament_id: i32,
}

/// A struct containing a tournament id and stage order used for querying
#[derive(Debug, Clone, Copy, serde::Deserialize, ToSchema, IntoParams)]
#[schema(example = json!({ "tournament_id": 152, "stage_order": 2 }))]
pub struct TournamentIdAndStageOrder {
    tournament_id: i32,
    stage_order: i16,
}
