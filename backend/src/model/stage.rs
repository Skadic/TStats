use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use super::TableType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Stage {
    pub order: usize,
    pub name: String,
    pub tournament: RecordId
}

impl TableType for Stage {
    fn table_name() -> &'static str {
        "stage"
    }
}
