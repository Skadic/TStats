use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A map in a tournament pool.
#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel, PartialEq, Hash)]
#[sea_orm(table_name = "pool_map")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tournament_id: i32,
    #[sea_orm(primary_key)]
    pub stage_order: i16,
    #[sea_orm(primary_key)]
    pub bracket_order: i16,
    /// The map's osu id. Note, that this is *not* the mapset id.
    #[sea_orm(primary_key)]
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