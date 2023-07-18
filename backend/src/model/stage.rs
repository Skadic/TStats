use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use super::TableType;

/// A stage in a tournament.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Stage {
    /// The order of the stage in the tournament. The first stage is 0, the second is 1, etc.
    pub order: usize,
    /// The stage's short name. For example, "QF", "RO16", etc.
    pub name: String,
    /// The tournament this stage is in.
    pub tournament: RecordId
}

impl TableType for Stage {
    fn table_name() -> &'static str {
        "stage"
    }
}
