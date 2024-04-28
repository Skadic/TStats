//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "pool_map"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub tournament_id: i32,
    pub stage_order: i16,
    pub bracket_order: i16,
    pub map_order: i16,
    pub map_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    TournamentId,
    StageOrder,
    BracketOrder,
    MapOrder,
    MapId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    TournamentId,
    StageOrder,
    BracketOrder,
    MapOrder,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i16, i16, i16);
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    PoolBracket,
    Score,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::TournamentId => ColumnType::Integer.def(),
            Self::StageOrder => ColumnType::SmallInteger.def(),
            Self::BracketOrder => ColumnType::SmallInteger.def(),
            Self::MapOrder => ColumnType::SmallInteger.def(),
            Self::MapId => ColumnType::BigInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::PoolBracket => Entity::belongs_to(super::pool_bracket::Entity)
                .from((
                    Column::TournamentId,
                    Column::StageOrder,
                    Column::BracketOrder,
                ))
                .to((
                    super::pool_bracket::Column::TournamentId,
                    super::pool_bracket::Column::StageOrder,
                    super::pool_bracket::Column::BracketOrder,
                ))
                .into(),
            Self::Score => Entity::has_many(super::score::Entity).into(),
        }
    }
}

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBracket.def()
    }
}

impl Related<super::score::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Score.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
