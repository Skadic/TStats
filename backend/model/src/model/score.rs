//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "score"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub player_id: i32,
    pub tournament_id: i32,
    pub stage_order: i16,
    pub bracket_order: i16,
    pub map_order: i16,
    pub match_id: i32,
    pub score: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    PlayerId,
    TournamentId,
    StageOrder,
    BracketOrder,
    MapOrder,
    MatchId,
    Score,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    PlayerId,
    TournamentId,
    StageOrder,
    BracketOrder,
    MapOrder,
    MatchId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32, i16, i16, i16, i32);
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Match,
    PoolMap,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::PlayerId => ColumnType::Integer.def(),
            Self::TournamentId => ColumnType::Integer.def(),
            Self::StageOrder => ColumnType::SmallInteger.def(),
            Self::BracketOrder => ColumnType::SmallInteger.def(),
            Self::MapOrder => ColumnType::SmallInteger.def(),
            Self::MatchId => ColumnType::Integer.def(),
            Self::Score => ColumnType::BigInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Match => Entity::belongs_to(super::r#match::Entity)
                .from(Column::MatchId)
                .to(super::r#match::Column::Id)
                .into(),
            Self::PoolMap => Entity::belongs_to(super::pool_map::Entity)
                .from((
                    Column::TournamentId,
                    Column::StageOrder,
                    Column::BracketOrder,
                    Column::MapOrder,
                ))
                .to((
                    super::pool_map::Column::TournamentId,
                    super::pool_map::Column::StageOrder,
                    super::pool_map::Column::BracketOrder,
                    super::pool_map::Column::MapOrder,
                ))
                .into(),
        }
    }
}

impl Related<super::r#match::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Match.def()
    }
}

impl Related<super::pool_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolMap.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
