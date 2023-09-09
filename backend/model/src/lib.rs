use sea_orm::{EntityTrait, DatabaseConnection, ConnectionTrait, sea_query::Table, Schema};

use log::info;

pub mod country_restriction;
pub mod pool_bracket;
pub mod pool_map;
pub mod stage;
pub mod tournament;

#[allow(unused)]
pub mod models {
    pub use super::country_restriction::Model as CountryRestriction;
    pub use super::pool_bracket::Model as PoolBracket;
    pub use super::pool_map::Model as PoolMap;
    pub use super::stage::Model as Stage;
    pub use super::tournament::Model as Tournament;
}
#[allow(unused)]
pub mod entities {
    pub use super::country_restriction::Entity as CountryRestrictionEntity;
    pub use super::pool_bracket::Entity as PoolBracketEntity;
    pub use super::pool_map::Entity as PoolMapEntity;
    pub use super::stage::Entity as StageEntity;
    pub use super::tournament::Entity as TournamentEntity;
}


pub async fn drop_table<E: EntityTrait>(db: &DatabaseConnection, table: E) {
    let builder = db.get_database_backend();
    match db
        .execute(builder.build(Table::drop().table(table.table_ref())))
        .await
    {
        Ok(_) => info!("Dropped table '{}'", table.table_name()),
        Err(e) => info!("Failed to drop table '{}': {e}", table.table_name()),
    };
}

pub async fn create_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    match db
        .execute(builder.build(&schema.create_table_from_entity(entity)))
        .await
    {
        Ok(_) => info!("Created table '{}'", entity.table_name()),
        Err(e) => info!("Failed to create table '{}': {e}", entity.table_name()),
    };
}