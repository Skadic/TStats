pub mod debug;
pub mod stage;
pub mod tournament;

/// A struct used to query for a specific entity by its id.
#[derive(Debug, serde::Deserialize)]
pub struct ById {
    pub id: i32
}
