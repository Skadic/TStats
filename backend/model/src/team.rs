use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A country which is allowed in a tournament
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tournament_id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tournament::Entity",
        from = "Column::TournamentId",
        to = "super::tournament::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    Tournament,
    /// A team has (potentially multiple) players
    #[sea_orm(has_many = "super::team_member::Entity")]
    Members,
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl Related<super::team_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new team with the given name.
    pub fn new(
        tournament_id: i32,
        name: impl Into<String>,
        //pool_brackets: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            id: 0,
            tournament_id,
            name: name.into(),
        }
    }
}
