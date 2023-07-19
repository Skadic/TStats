use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;
use surrealdb::sql::Thing;

use super::TableType;

/// A stage in a tournament.
#[derive(Debug, PartialEq, Serialize, Deserialize, Hash)]
pub struct Stage<'a> {
    /// The stage's ID.
    pub id: Option<Thing>,
    /// The stage's short name. For example, "QF", "RO16", etc.
    pub name: Cow<'a, str>,
    /// The stage's order in the tournament. For example, the first stage is 0, the second stage is 1, etc.
    pub order: usize,
    /// The brackets in this pool in the order they should appear, e.g. most commonly for std tournaments, this is ["NM", "HD", "HR", "DT", "FM", "TB"].
    pub pool_brackets: Vec<Cow<'a, str>>,
}

impl<'a> Stage<'a> {
    pub fn new(
        name: &'a str,
        order: usize,
        pool_brackets: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            order,
            pool_brackets: pool_brackets
                .into_iter()
                .map(|s| s.into())
                .collect(),
        }
    }
}

impl TableType for Stage<'_> {
    fn table_name() -> &'static str {
        "stage"
    }

    fn database_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}
