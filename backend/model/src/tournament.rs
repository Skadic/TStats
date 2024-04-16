use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A tournament with its associated data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "tournament")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The database id for this tournament
    #[sea_orm(primary_key, auto_increment = true, unique)]
    #[serde(skip_deserializing)]
    pub id: i32,
    /// The tournament's full name
    pub name: String,
    /// This tournament's shorthand name
    pub shorthand: String,
    /// The tournament format, i.e. how many players are playing at any one time. This should be a [`TournamentFormat`] value.
    pub format: i32,
    /// Whether this tournament uses badge-weighting to adjust player's ranks.
    #[sea_orm(default_value = true)]
    pub bws: bool,
    /// The mode of this tournament
    #[sea_orm(column_type = "TinyInteger")]
    pub mode: Mode,
}

#[derive(Debug, Serialize, Deserialize, EnumIter, DeriveRelation, Copy, Clone)]
pub enum Relation {
    #[sea_orm(has_many = "super::team::Entity")]
    Team,
    #[sea_orm(has_many = "super::rank_restriction::Entity")]
    RankRestriction,
    #[sea_orm(has_many = "super::country_restriction::Entity")]
    CountryRestriction,
    #[sea_orm(has_many = "super::stage::Entity")]
    Stage,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Serialize_repr, Deserialize_repr, EnumIter, DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
#[repr(i16)]
pub enum Mode {
    Osu = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<super::country_restriction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CountryRestriction.def()
    }
}

impl Related<super::rank_restriction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RankRestriction.def()
    }
}

impl Related<super::stage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
