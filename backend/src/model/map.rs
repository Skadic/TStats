use serde::{Deserialize, Serialize};

use surrealdb::sql::Thing;
use crate::model::TableRecord;

use super::TableType;

/// A map in a tournament pool.
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash)]
pub struct PoolMap {
    /// The map's osu id which also serves as its data base id. Note, that this is *not* the mapset id.
    pub id: Option<Thing>,
}

impl TableType for PoolMap {
    fn table_name() -> &'static str {
        "map"
    }

    fn database_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}

impl TableRecord for PoolMap {}
