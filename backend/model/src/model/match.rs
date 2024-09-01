//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use super::sea_orm_active_enums::MatchType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "match"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub tournament_id: i32,
    pub stage_order: i16,
    pub date: DateTime,
    pub match_type: MatchType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TournamentId,
    StageOrder,
    Date,
    MatchType,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    MatchLink,
    QualifierRun,
    Score,
    Stage,
    VersusMatch,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::TournamentId => ColumnType::Integer.def(),
            Self::StageOrder => ColumnType::SmallInteger.def(),
            Self::Date => ColumnType::DateTime.def(),
            Self::MatchType => MatchType::db_type().def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::MatchLink => Entity::has_many(super::match_link::Entity).into(),
            Self::QualifierRun => Entity::has_many(super::qualifier_run::Entity).into(),
            Self::Score => Entity::has_many(super::score::Entity).into(),
            Self::Stage => Entity::belongs_to(super::stage::Entity)
                .from((Column::TournamentId, Column::StageOrder))
                .to((
                    super::stage::Column::TournamentId,
                    super::stage::Column::StageOrder,
                ))
                .into(),
            Self::VersusMatch => Entity::has_many(super::versus_match::Entity).into(),
        }
    }
}

impl Related<super::match_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MatchLink.def()
    }
}

impl Related<super::qualifier_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QualifierRun.def()
    }
}

impl Related<super::score::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Score.def()
    }
}

impl Related<super::stage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl Related<super::versus_match::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VersusMatch.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
