use std::ops::Range;

use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{ArrayType, ValueType, ValueTypeErr};
use serde::{Deserialize, Serialize};

use crate::model::tournament::RankRange::OpenRank;

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

/// A builder for tournament [`Model`] objects.
pub struct TournamentBuilder {
    tournament: Model,
}

impl TournamentBuilder {
    /// Creates a new [`TournamentBuilder`] with the given name, shorthand, format and whether the tournament uses badge-weighting..
    pub fn new(
        name: impl Into<String>,
        shorthand: impl Into<String>,
        format: TournamentFormat,
        bws: bool,
    ) -> Self {
        TournamentBuilder {
            tournament: Model {
                id: 0,
                name: name.into(),
                shorthand: shorthand.into(),
                format,
                rank_range: OpenRank,
                bws,
            },
        }
    }

    /// Sets this tournament's rank restriction to a single rank range.
    #[allow(dead_code)]
    pub fn single_rank_range(mut self, rank: Range<usize>) -> Self {
        self.tournament.rank_range = RankRange::Single(rank);
        self
    }

    /// Sets this tournament's rank restriction to a multiple rank ranges for tiered tournaments.
    #[allow(dead_code)]
    pub fn tiered_rank_range(mut self, ranks: impl IntoIterator<Item = Range<usize>>) -> Self {
        let mut iter = ranks.into_iter().peekable();
        if iter.peek().is_none() {
            self.tournament.rank_range = OpenRank;
            return self;
        }
        self.tournament.rank_range = RankRange::Tiered(iter.collect());
        self
    }

    /// Sets this tournament's rank restriction.
    pub fn with_rank_range(mut self, rank_range: RankRange) -> Self {
        match rank_range {
            RankRange::Tiered(v) => self.tiered_rank_range(v),
            RankRange::Single(range) => self.single_rank_range(range),
            RankRange::OpenRank => {
                self.tournament.rank_range = OpenRank;
                self
            }
        }
    }

    /// Finalizes the building process
    pub fn build(self) -> Model {
        self.tournament
    }
}

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

    /// Returns the number of tiers in the tournament. For a single rank range, this is just one.
    /// For tiered tournaments, it's the number of rank ranges.
    pub fn num_tiers(&self) -> usize {
        match self {
            RankRange::Single(_) => 1,
            RankRange::Tiered(ranges) => ranges.len(),
            RankRange::OpenRank => 1
        }
    }
}

impl Default for RankRange {
    fn default() -> Self {
        Self::OpenRank
    }
}
