use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A stage in a tournament.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "stage")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The id of the tournament the stage belongs to.
    #[sea_orm(primary_key)]
    pub tournament_id: i32,
    /// The stage's short name. For example, "QF", "RO16", etc.
    pub name: String,
    /// The stage's order in the tournament. For example, the first stage is 0, the second stage is 1, etc.
    #[sea_orm(primary_key, column_type = "TinyInteger")]
    pub stage_order: i16,
    /// The best-of of this stage's matches.
    #[sea_orm(column_type = "TinyInteger")]
    pub best_of: i16,
}

#[derive(Debug, Serialize, Deserialize, EnumIter, DeriveRelation, Copy, Clone)]
pub enum Relation {
    /// A tournament can have many stages.
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Tournament,
    /// A stage has multiple pool brackets
    #[sea_orm(has_many = "super::pool_bracket::Entity")]
    PoolBrackets,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBrackets.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new Stage with the given name, order and pool brackets.
    pub fn new(
        name: impl Into<String>,
        stage_order: usize,
        best_of: usize,
        //pool_brackets: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            tournament_id: 0,
            name: name.into(),
            stage_order: stage_order as i16,
            //pool_brackets: pool_brackets.into_iter().map(|s| s.into()).collect(),
            best_of: best_of as i16,
        }
    }
}
