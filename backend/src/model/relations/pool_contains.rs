use crate::model::TableType;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::{Response, Surreal};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PoolContains<'a> {
    pub id: Option<Thing>,
    #[serde(rename = "in")]
    pub stage: Thing,
    #[serde(rename = "out")]
    pub map: Thing,
    pub bracket: Cow<'a, str>,
    pub bracket_order: usize,
}

impl<'a> PoolContains<'a> {
    #[inline]
    pub fn new(
        stage: &Thing,
        map: &Thing,
        bracket: impl Into<Cow<'a, str>>,
        bracket_order: usize,
    ) -> Self {
        Self {
            id: None,
            map: map.clone(),
            stage: stage.clone(),
            bracket: bracket.into(),
            bracket_order,
        }
    }

    #[inline]
    pub async fn relate(db: &Surreal<Client>, stage: &Thing, map: &Thing, bracket: impl Into<Cow<'a, str>>, bracket_order: usize) -> surrealdb::Result<()> {
        let _: Response = db
            .query("RELATE $in->pool_contains->$out SET bracket = $bracket, bracket_order = $bracket_order;")
            .bind(PoolContains::new(
                stage,
                map,
                bracket,
                bracket_order,
            ))
            .await?;
        Ok(())
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
