use surrealdb::sql::Thing;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::{Connection, Surreal};
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Content;

pub mod map;
pub mod relations;
pub mod stage;
pub mod tournament;

/// A record in the database. This offers methods to retrieve the name of the table for the type as well as the id of the record.
pub trait TableType: Sized + Serialize + for<'a> Deserialize<'a> {
    /// The name of the table in the database for this type.
    fn table_name() -> &'static str;
    /// The id of the record in the database.
    fn database_id(&self) -> Option<&Thing>;

}

/// A record in the database that can be inserted.
#[async_trait]
pub trait TableRecord: TableType {
    /// Inserts the object into the database
    async fn insert<R>(self, db: &Surreal<Client>) -> surrealdb::Result<R> where
        R: DeserializeOwned + Send + Sync {
        db.create(Self::table_name())
            .content(self)
            .await
    }
}

/// A relation between two records in the database.
#[async_trait]
pub trait TableRelation<In: TableRecord, Out: TableRecord>: TableType + Sync {
    fn input_relation(&self) -> &Thing;
    fn output_relation(&self) -> &Thing;

    /// Relates two objects in the database
    async fn relate(self, db: &Surreal<Client>) -> surrealdb::Result<Self> {
        let mut res = db.query(format!("RELATE $in->{}->$out CONTENT $v;", Self::table_name()))
            .bind(("in", self.input_relation()))
            .bind(("out", self.output_relation()))
            .bind(("v", self))
            .await?;
        let opt: Option<Self> = res.take(0)?;

        Ok(opt.unwrap())
    }
}