use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ById {
    id: String,
}

pub mod tournament;
pub mod stage;
pub mod debug;