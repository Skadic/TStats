use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A country which is allowed in a tournament
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, ToSchema)]
#[sea_orm(table_name = "team_mate")]
#[schema(as = Teammate)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[schema(example = 727)]
    team_id: i32,
    #[schema(example = 124493)]
    user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id"
    )]
    Team,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Creates a new player with the given id in the given team.
    pub fn new(
        team_id: i32,
        user_id: i32,
        //pool_brackets: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self { team_id, user_id }
    }
}
