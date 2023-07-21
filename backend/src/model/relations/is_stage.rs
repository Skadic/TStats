use async_trait::async_trait;
use crate::model::{TableRelation, TableType};
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::{Response, Surreal};
use crate::model::stage::Stage;
use crate::model::tournament::Tournament;

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

#[async_trait]
impl TableRelation<Tournament<'_>, Stage<'_>> for IsStage {
    fn input_relation(&self) -> &Thing {
        &self.tournament
    }

    fn output_relation(&self) -> &Thing {
        &self.stage
    }
}