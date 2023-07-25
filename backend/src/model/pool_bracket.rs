use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A bracket in a tournament pool, like "the NoMod bracket", "the Hidden bracket", etc.
#[derive(Clone, Debug, Serialize, Deserialize, DeriveEntityModel, PartialEq, Hash)]
#[sea_orm(table_name = "pool_bracket")]
pub struct Model {
    /// The id of the tournament this pool belongs to
    #[sea_orm(primary_key)]
    pub tournament_id: i32,
    /// The index of the stage in the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    pub stage_order: i16,
    /// The order of this bracket in the stage. E.g. if this is the first bracket in the pool, this is 0.
    #[sea_orm(primary_key)]
    pub bracket_order: i16,
    /// The name for this bracket, like "NM", "HD", etc.
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id"
    )]
    Tournament,
    #[sea_orm(
        belongs_to = "super::stage::Entity",
        from = "(Column::TournamentId, Column::StageOrder)",
        to = "(super::stage::Column::TournamentId, super::stage::Column::StageOrder)"
    )]
    Stage,
    #[sea_orm(has_many = "super::pool_map::Entity")]
    PoolMap,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::stage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl Related<super::pool_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolMap.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}