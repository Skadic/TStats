//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use super::sea_orm_active_enums::MatchType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "versus_match"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub match_id: i32,
    pub team_red: i32,
    pub team_blue: i32,
    pub score_red: Option<i16>,
    pub score_blue: Option<i16>,
    pub match_type: MatchType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    MatchId,
    TeamRed,
    TeamBlue,
    ScoreRed,
    ScoreBlue,
    MatchType,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    MatchId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Match,
    Team2,
    Team1,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::MatchId => ColumnType::Integer.def(),
            Self::TeamRed => ColumnType::Integer.def(),
            Self::TeamBlue => ColumnType::Integer.def(),
            Self::ScoreRed => ColumnType::SmallInteger.def().null(),
            Self::ScoreBlue => ColumnType::SmallInteger.def().null(),
            Self::MatchType => MatchType::db_type().def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Match => Entity::belongs_to(super::r#match::Entity)
                .from((Column::MatchId, Column::MatchType))
                .to((
                    super::r#match::Column::Id,
                    super::r#match::Column::MatchType,
                ))
                .into(),
            Self::Team2 => Entity::belongs_to(super::team::Entity)
                .from(Column::TeamBlue)
                .to(super::team::Column::Id)
                .into(),
            Self::Team1 => Entity::belongs_to(super::team::Entity)
                .from(Column::TeamRed)
                .to(super::team::Column::Id)
                .into(),
        }
    }
}

impl Related<super::r#match::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Match.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
