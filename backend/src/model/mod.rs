use surrealdb::sql::Thing;

pub mod tournament;
pub mod stage;
pub mod map;
pub mod relations;

pub trait TableType {
    fn table_name() -> &'static str;
    fn database_id(&self) -> Option<&Thing>;
}
