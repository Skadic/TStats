use std::borrow::Cow;
use std::ops::Range;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::TableType;

/// A tournament with its associated data.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Tournament<'a> {
    pub id: Option<Thing>,
    /// The tournament's full name
    pub name: Cow<'a, str>,
    /// This tournament's shorthand name
    pub shorthand: Cow<'a, str>,
    /// The tournament format, i.e. how many players are playing at any one time
    pub format: TournamentFormat,
    /// The tournament's rank range.
    pub rank_range: Option<RankRange>,
    /// Whether this tournament uses badge-weighting to adjust player's ranks.
    pub bws: bool,
    /// Contains the countries the tournament is restricted to, if any.
    pub country_restriction: Option<Vec<Cow<'a, str>>>,
}

/// A builder for [`Tournament`] objects.
pub struct TournamentBuilder<'a> {
    tournament: Tournament<'a>,
}

impl<'a> TournamentBuilder<'a> {
    /// Creates a new [`TournamentBuilder`] with the given name, shorthand, format and whether the tournament uses badge-weighting..
    pub fn new(
        name: impl Into<Cow<'a, str>>,
        shorthand: impl Into<Cow<'a, str>>,
        format: TournamentFormat,
        bws: bool,
    ) -> Self {
        TournamentBuilder {
            tournament: Tournament {
                id: None,
                name: name.into(),
                shorthand: shorthand.into(),
                format,
                rank_range: None,
                bws,
                country_restriction: None,
            },
        }
    }

    /// Sets this tournament's rank restriction to a single rank range.
    #[allow(dead_code)]
    pub fn single_rank_range(mut self, rank: Range<usize>) -> Self {
        self.tournament.rank_range = Some(RankRange::Single(rank));
        self
    }

    /// Sets this tournament's rank restriction to a multiple rank ranges for tiered tournaments.
    #[allow(dead_code)]
    pub fn tiered_rank_range(mut self, ranks: impl IntoIterator<Item = Range<usize>>) -> Self {
        self.tournament.rank_range = Some(RankRange::Tiered(ranks.into_iter().collect()));
        // If there are no tiers, we have no rank ranges
        if self.tournament.rank_range.as_ref().unwrap().num_tiers() == 0 {
            self.tournament.rank_range = None;
        }
        self
    }

    /// Sets this tournament's rank restriction.
    pub fn with_rank_range(mut self, rank_range: RankRange) -> Self {
        self.tournament.rank_range = Some(rank_range);
        self
    }

    /// Sets this tournament's country restriction, which is an iterator of ISO-3361 alpha-2 country codes.
    /// If the iterator is empty, the country restriction is set to `None`.
    pub fn country_restriction(mut self, countries: impl IntoIterator<Item = &'a str>) -> Self {
        self.tournament.country_restriction =
            Some(countries.into_iter().map(|c| c.into()).collect());
        if self
            .tournament
            .country_restriction
            .as_ref()
            .unwrap()
            .is_empty()
        {
            self.tournament.country_restriction = None;
        }
        self
    }

    /// Finalizes the building process
    pub fn build(self) -> Tournament<'a> {
        self.tournament
    }
}

/// The tournament format, detailing the format of a match.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone, Copy)]
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Clone)]
//#[serde(untagged)]
pub enum RankRange {
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
        }
    }
}

impl TableType for Tournament<'_> {
    fn table_name() -> &'static str {
        "tournament"
    }

    fn database_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}
