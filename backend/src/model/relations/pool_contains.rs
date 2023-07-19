use crate::model::TableType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoolContains<'a> {
    pub id: Option<Thing>,
    #[serde(rename = "in")]
    pub tournament: Thing,
    #[serde(rename = "out")]
    pub stage: Thing,
    pub bracket: Cow<'a, str>,
    pub bracket_order: usize,
}

impl<'a> PoolContains<'a> {
    pub fn new(
        tournament: &Thing,
        stage: &Thing,
        bracket: impl Into<Cow<'a, str>>,
        bracket_order: usize,
    ) -> Self {
        Self {
            id: None,
            tournament: tournament.clone(),
            stage: stage.clone(),
            bracket: bracket.into(),
            bracket_order,
        }
    }
}

impl TableType for PoolContains<'_> {
    fn table_name() -> &'static str {
        "pool_contains"
    }

    fn database_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}
