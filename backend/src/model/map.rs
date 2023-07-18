
use serde::{Serialize, Deserialize};
use surrealdb::opt::RecordId;

use super::TableType;

/// A map in a tournament pool.
#[derive(Debug, Serialize, Deserialize, PartialEq, Hash)]
pub struct PoolMap {
    /// The map's osu id. Note, that this is *not* the mapset id.
    pub map_id: usize,
    /// A reference to the stage, whose pool this map is in.
    pub stage: RecordId,
    /// The identifier of bracket this map is in, e.g. "NM", "HD", etc.
    pub bracket: String,
    /// The number of the map in the bracket, e.g. if this is NM1, then this is 1.
    pub bracket_order: usize
}

impl TableType for PoolMap {
    fn table_name() -> &'static str {
        "map"
    }
}