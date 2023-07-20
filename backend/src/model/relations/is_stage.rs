use crate::model::TableType;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::{Response, Surreal};

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

    pub async fn relate(db: &Surreal<Client>, tournament: &Thing, stage: &Thing) -> surrealdb::Result<()> {
        let _: Response = db
            .query("RELATE $in->is_stage->$out;")
            .bind(IsStage::new(tournament, stage))
            .await?;
        Ok(())
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
