use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A map in a tournament pool.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, DeriveEntityModel, PartialEq, ToSchema)]
#[sea_orm(table_name = "pool_map")]
#[schema(as = PoolMap)]
#[serde(rename_all="camelCase")]
pub struct Model {
    /// The id of the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    #[schema(example = 435)]
    pub tournament_id: i32,
    /// The index of the stage in the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    #[schema(example = 2)]
    pub stage_order: i16,
    /// The order of this bracket in the stage. E.g. if this is the first bracket in the pool, this is 0.
    #[sea_orm(primary_key)]
    #[schema(example = 1)]
    pub bracket_order: i16,
    /// The number of the map in the bracket. Note, that this is *zero indexed*, so e.g. NM1 will have map_order 0, NM2 will have map_order 1, etc.
    #[sea_orm(primary_key)]
    #[schema(example = 3)]
    pub map_order: i16,
    /// The map's osu id. Note, that this is *not* the mapset id.
    #[schema(example = 1009363)]
    pub map_id: i64,
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
    #[sea_orm(
        belongs_to = "super::pool_bracket::Entity",
        from = "(Column::TournamentId, Column::StageOrder, Column::BracketOrder)",
        to = "(super::pool_bracket::Column::TournamentId, super::pool_bracket::Column::StageOrder, super::pool_bracket::Column::BracketOrder)"
    )]
    PoolBracket,
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

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBracket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
