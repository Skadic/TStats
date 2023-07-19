use surrealdb::sql::Thing;

pub mod map;
pub mod relations;
pub mod stage;
pub mod tournament;

/// A record in the database. This offers methods to retrieve the name of the table for the type as well as the id of the record.
pub trait TableType {
    /// The name of the table in the database for this type.
    fn table_name() -> &'static str;
    /// The id of the record in the database.
    fn database_id(&self) -> Option<&Thing>;
}
