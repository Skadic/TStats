//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "stage"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub tournament_id: i32,
    pub stage_order: i16,
    pub name: String,
    pub best_of: i16,
    pub start_date: Option<DateTime>,
    pub end_date: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TournamentId,
    StageOrder,
    Name,
    BestOf,
    StartDate,
    EndDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TournamentId,
    StageOrder,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i16);
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Match,
    PoolBracket,
    Tournament,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::TournamentId => ColumnType::Integer.def(),
            Self::StageOrder => ColumnType::SmallInteger.def(),
            Self::Name => ColumnType::String(StringLen::N(10u32)).def(),
            Self::BestOf => ColumnType::SmallInteger.def(),
            Self::StartDate => ColumnType::DateTime.def().null(),
            Self::EndDate => ColumnType::DateTime.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Match => Entity::has_many(super::r#match::Entity).into(),
            Self::PoolBracket => Entity::has_many(super::pool_bracket::Entity).into(),
            Self::Tournament => Entity::belongs_to(super::tournament::Entity)
                .from(Column::TournamentId)
                .to(super::tournament::Column::Id)
                .into(),
        }
    }
}

impl Related<super::r#match::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Match.def()
    }
}

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBracket.def()
    }
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
