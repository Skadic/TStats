use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A map in a tournament pool.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, DeriveEntityModel, PartialEq, ToSchema)]
#[sea_orm(table_name = "rank_restriction")]
#[schema(as = RankRestriction)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The id of the tournament this pool belongs to.
    #[sea_orm(primary_key)]
    #[schema(example = 435)]
    pub tournament_id: i32,
    /// Especially used in tiered tournaments, this is the tier this rank range is for, starting at 0 being the highest tier.
    #[sea_orm(primary_key)]
    #[schema(example = 2)]
    pub tier: i32,
    /// The minimum rank for this range
    #[schema(example = 100)]
    pub min: i32,
    /// The maximum rank for this range
    #[schema(example = 1000)]
    pub max: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
    belongs_to = "super::tournament::Entity",
    from = "Column::TournamentId",
    to = "super::tournament::Column::Id"
    on_delete = "Cascade",
    on_update = "Cascade",
    )]
    Tournament,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
