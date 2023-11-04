use std::ops::Range;

use crate::tournament::RankRestriction::{OpenRank, Single, Tiered};
use sea_orm::entity::prelude::*;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A tournament with its associated data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, ToSchema)]
#[sea_orm(table_name = "tournament")]
#[schema(as = Tournament)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// The database id for this tournament
    #[sea_orm(primary_key, auto_increment = true, unique)]
    #[serde(skip_deserializing)]
    #[schema(example = 1, required = false)]
    pub id: i32,
    /// The tournament's full name
    #[schema(example = "My Great Tournament 3")]
    pub name: String,
    /// This tournament's shorthand name
    #[schema(example = "MGT3")]
    pub shorthand: String,
    /// The tournament format, i.e. how many players are playing at any one time. This should be a [`TournamentFormat`] value.
    pub format: i32,
    /// Whether this tournament uses badge-weighting to adjust player's ranks.
    #[sea_orm(default_value = true)]
    #[schema(example = true)]
    pub bws: bool,
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

/// The tournament format, detailing the format of a match.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy, FromJsonQueryResult, ToSchema)]
#[schema(example = json!({"Versus": 3}))]
pub enum TournamentFormat {
    /// A simple versus match. The parameter is the number of players playing for each team at any
    /// one time. So for a 4v4, this parameter is 4.
    #[schema(example = 1)]
    Versus(usize),
    /// A battle royale style tournament, the parameter being the number of players.
    #[schema(example = 10)]
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone, FromJsonQueryResult, ToSchema)]
#[schema(example = json!({"Single": {"min": 500, "max": 10000}}))]
pub enum RankRestriction {
    /// Everyone can participate. There is no rank restriction
    OpenRank,
    /// This tournament has a single rank range
    Single(RankRange),
    /// This tournament has multiple tiers and therefore multiple rank ranges
    Tiered(Vec<RankRange>),
}

/// A rank range with a lower and upper bound.
#[derive(
    Debug, PartialEq, Serialize, Deserialize, Hash, Clone, Copy, FromJsonQueryResult, ToSchema,
)]
#[schema(example = json!({"min": 500, "max": 10000}))]
pub struct RankRange {
    /// The rank range's lower bound
    pub min: usize,
    /// The rank range's upper bound
    pub max: usize,
}

impl From<Range<usize>> for RankRange {
    fn from(value: Range<usize>) -> Self {
        Self {
            min: value.start,
            max: value.end,
        }
    }
}

impl RankRestriction {
    /// Creates a new single rank range.
    pub fn single(rank_range: impl Into<RankRange>) -> Self {
        RankRestriction::Single(rank_range.into())
    }

    /// Creates a new tiered rank range with an iterator of rank ranges as input.
    pub fn tiered(rank_ranges: impl IntoIterator<Item = impl Into<RankRange>>) -> Self {
        RankRestriction::Tiered(rank_ranges.into_iter().map(Into::into).collect())
    }

    /// Returns the number of tiers in the tournament. For a single rank range or an open rank tournament, this is just one.
    /// For tiered tournaments, it's the number of rank ranges.
    pub fn num_tiers(&self) -> usize {
        match self {
            RankRestriction::Tiered(ranges) => ranges.len(),
            _ => 1,
        }
    }
}

impl Default for RankRestriction {
    fn default() -> Self {
        Self::OpenRank
    }
}

impl<T> From<T> for RankRestriction
where
    T: IntoIterator<Item = RankRange>,
    <T as IntoIterator>::IntoIter: ExactSizeIterator,
{
    fn from(value: T) -> Self {
        let mut iter = value.into_iter();
        match iter.len() {
            0 => OpenRank,
            1 => Single(iter.next().unwrap()),
            _ => Tiered(iter.collect()),
        }
    }
}

impl From<RankRestriction> for Vec<RankRange> {
    fn from(value: RankRestriction) -> Self {
        match value {
            OpenRank => vec![],
            Single(rr) => vec![rr],
            Tiered(vec) => vec,
        }
    }
}
