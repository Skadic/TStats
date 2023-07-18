use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::TableType;

/// A tournament with its associated data.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Tournament {
    /// The tournament's full name
    pub name: String,
    /// This tournament's shorthand name
    pub shorthand: String,
    /// The tournament format, i.e. how many players are playing at any one time
    pub format: TournamentFormat,
    /// The tournament's rank range.
    pub rank_range: Option<RankRange>,
    /// Whether this tournament uses badge-weighting to adjust player's ranks.
    pub bws: bool,
    /// Contains the countries the tournament is restricted to, if any.
    pub country_restriction: Option<Vec<String>>,
}

/// The tournament format, detailing the format of a match.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub enum TournamentFormat {
    /// A simple versus match. The parameter is the number of players playing for each team at any
    /// one time. So for a 4v4, this parameter is 4.
    Versus { team_size: usize },
    /// A battle royale style tournament.
    BattleRoyale,
}

impl TournamentFormat {
    pub fn versus(team_size: usize) -> Self {
        TournamentFormat::Versus { team_size }
    }
}

/// A rank range for the current tournament which determines which players are allowed into the
/// tournament
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum RankRange {
    /// This tournament has a single rank range
    Single { rank_range: Range<usize> },
    /// This tournament has multiple tiers and therefore multiple rank ranges
    Tiered { tiers: Vec<Range<usize>> },
}

impl RankRange {
    pub fn single(rank_range: Range<usize>) -> Self {
        RankRange::Single { rank_range }
    }

    pub fn tiered(rank_ranges: impl IntoIterator<Item = Range<usize>>) -> Self {
        RankRange::Tiered {
            tiers: rank_ranges.into_iter().collect(),
        }
    }
}

impl TableType for Tournament {
    fn table_name() -> &'static str {
        "tournament"
    }
}
