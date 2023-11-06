use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A map in a tournament pool.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "pool_map")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The id of the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    pub tournament_id: i32,
    /// The index of the stage in the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    pub stage_order: i16,
    /// The order of this bracket in the stage. E.g. if this is the first bracket in the pool, this is 0.
    #[sea_orm(primary_key)]
    pub bracket_order: i16,
    /// The number of the map in the bracket. Note, that this is *zero indexed*, so e.g. NM1 will have map_order 0, NM2 will have map_order 1, etc.
    #[sea_orm(primary_key)]
    pub map_order: i16,
    /// The map's osu id. Note, that this is *not* the mapset id.
    pub map_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pool_bracket::Entity",
        from = "(Column::TournamentId, Column::StageOrder, Column::BracketOrder)",
        to = "(super::pool_bracket::Column::TournamentId, super::pool_bracket::Column::StageOrder, super::pool_bracket::Column::BracketOrder)"
        on_delete = "Cascade",
        on_update = "Cascade",
    )]
    PoolBracket,
}

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBracket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
