use std::ops::Range;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// A tournament with its associated data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "tournament")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true, unique)]
    #[serde(skip_deserializing)]
    pub id: i32,
    /// The tournament's full name
    pub name: String,
    /// This tournament's shorthand name
    pub shorthand: String,
    /// The tournament format, i.e. how many players are playing at any one time. This should be a [`TournamentFormat`] value.
    pub format: TournamentFormat,
    /// The tournament's rank range. This should be a [`RankRange`] value.
    pub rank_range: RankRange,
    /// Whether this tournament uses badge-weighting to adjust player's ranks.
    #[sea_orm(default_value = true)]
    pub bws: bool,
}

#[derive(Debug, Serialize, Deserialize, EnumIter, DeriveRelation, Copy, Clone)]
pub enum Relation {
    #[sea_orm(has_many = "super::country_restriction::Entity")]
    CountryRestriction,
    #[sea_orm(has_many = "super::stage::Entity")]
    Stage,
    #[sea_orm(has_many = "super::pool_bracket::Entity")]
    PoolBracket,
    #[sea_orm(has_many = "super::pool_map::Entity")]
    PoolMap,
}

impl Related<super::country_restriction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CountryRestriction.def()
    }
}

impl Related<super::stage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stage.def()
    }
}

impl Related<super::pool_bracket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolBracket.def()
    }
}

impl Related<super::pool_map::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PoolMap.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// The tournament format, detailing the format of a match.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone, Copy, FromJsonQueryResult)]
pub enum TournamentFormat {
    /// A simple versus match. The parameter is the number of players playing for each team at any
    /// one time. So for a 4v4, this parameter is 4.
    Versus(usize),
    /// A battle royale style tournament, the parameter being the number of players.
    BattleRoyale(usize),
}

impl TournamentFormat {
    /// Creates a new versus variant with the given team size.
    pub const fn versus(team_size: usize) -> Self {
        TournamentFormat::Versus(team_size)
    }

    /// Creates a new battle royale variant with the given player count.
    pub const fn battle_royale(player_count: usize) -> Self {
        TournamentFormat::BattleRoyale(player_count)
    }
}

/// A rank range for the current tournament which determines which players are allowed into the
/// tournament
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone, FromJsonQueryResult)]
//#[serde(untagged)]
pub enum RankRange {
    /// Everyone can participate. There is no rank restriction
    OpenRank,
    /// This tournament has a single rank range
    Single(Range<usize>),
    /// This tournament has multiple tiers and therefore multiple rank ranges
    Tiered(Vec<Range<usize>>),
}

impl RankRange {
    /// Creates a new single rank range.
    pub fn single(rank_range: Range<usize>) -> Self {
        RankRange::Single(rank_range)
    }

    /// Creates a new tiered rank range with an iterator of rank ranges as input..
    pub fn tiered(rank_ranges: impl IntoIterator<Item = Range<usize>>) -> Self {
        RankRange::Tiered(rank_ranges.into_iter().collect())
    }

    /// Returns the number of tiers in the tournament. For a single rank range or an open rank tournament, this is just one.
    /// For tiered tournaments, it's the number of rank ranges.
    pub fn num_tiers(&self) -> usize {
        match self {
            RankRange::Tiered(ranges) => ranges.len(),
            _ => 1
        }
    }
}

impl Default for RankRange {
    fn default() -> Self {
        Self::OpenRank
    }
}
