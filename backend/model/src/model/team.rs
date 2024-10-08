//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "team"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub tournament_id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TournamentId,
    Name,
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
    QualifierRun,
    TeamMember,
    Tournament,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::TournamentId => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(StringLen::N(40u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::QualifierRun => Entity::has_many(super::qualifier_run::Entity).into(),
            Self::TeamMember => Entity::has_many(super::team_member::Entity).into(),
            Self::Tournament => Entity::belongs_to(super::tournament::Entity)
                .from(Column::TournamentId)
                .to(super::tournament::Column::Id)
                .into(),
        }
    }
}

impl Related<super::qualifier_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QualifierRun.def()
    }
}

impl Related<super::team_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamMember.def()
    }
}

impl Related<super::tournament::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tournament.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
