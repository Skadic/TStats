use crate::model::TableType;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsStage {
    pub id: Option<Thing>,
    #[serde(rename = "in")]
    pub tournament: Thing,
    #[serde(rename = "out")]
    pub stage: Thing,
}

impl IsStage {
    pub fn new(tournament: &Thing, stage: &Thing) -> Self {
        Self {
            id: None,
            tournament: tournament.clone(),
            stage: stage.clone(),
        }
    }
}

impl TableType for IsStage {
    fn table_name() -> &'static str {
        "is_stage"
    }

    fn database_id(&self) -> Option<&Thing> {
        self.id.as_ref()
    }
}
