use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A country which is allowed in a tournament
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, ToSchema)]
#[sea_orm(table_name = "country_restriction")]
#[schema(as = CountryRestriction)]
#[serde(rename_all="camelCase")]
pub struct Model {
    /// The id of the tournament the
    #[sea_orm(primary_key)]
    #[schema(example = 634)]
    pub tournament_id: i32,
    /// The country's ISO3166-1 alpha-2 code
    #[sea_orm(primary_key)]
    #[schema(example = "JP")]
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
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
