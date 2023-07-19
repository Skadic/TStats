use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ById {
    id: String,
}

pub mod debug;
pub mod stage;
pub mod tournament;
