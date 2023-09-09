pub mod debug;
pub mod stage;
pub mod tournament;

/// A struct containing a simple numeric id, used among other things to query for a specific entity by its id.
#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct Id {
    pub id: i32,
}
