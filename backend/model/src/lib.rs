use sea_orm::{sea_query::Table, ConnectionTrait, DatabaseConnection, EntityTrait, Schema};

use tracing::{info, warn};

pub mod country_restriction;
pub mod player;
pub mod pool_bracket;
pub mod pool_map;
pub mod rank_restriction;
pub mod stage;
pub mod team;
pub mod tournament;

#[allow(unused)]
pub mod models {
    pub use super::country_restriction::Model as CountryRestriction;
    pub use super::player::Model as Player;
    pub use super::pool_bracket::Model as PoolBracket;
    pub use super::pool_map::Model as PoolMap;
    pub use super::rank_restriction::Model as RankRestriction;
    pub use super::stage::Model as Stage;
    pub use super::team::Model as Team;
    pub use super::tournament::Model as Tournament;
}
#[allow(unused)]
pub mod entities {
    pub use super::country_restriction::Entity as CountryRestrictionEntity;
    pub use super::player::Entity as PlayerEntity;
    pub use super::pool_bracket::Entity as PoolBracketEntity;
    pub use super::pool_map::Entity as PoolMapEntity;
    pub use super::rank_restriction::Entity as RankRestrictionEntity;
    pub use super::stage::Entity as StageEntity;
    pub use super::team::Entity as TeamEntity;
    pub use super::tournament::Entity as TournamentEntity;
}

#[tracing::instrument(skip(db, table), fields(table_name = table.table_name()))]
pub async fn drop_table<E: EntityTrait>(db: &DatabaseConnection, table: E) {
    let builder = db.get_database_backend();
    match db
        .execute(builder.build(Table::drop().table(table.table_ref())))
        .await
    {
        Ok(_) => info!("dropped table"),
        Err(error) => warn!(%error, "failed to drop table"),
    };
}

#[tracing::instrument(skip(db, entity), fields(table_name = entity.table_name()))]
pub async fn create_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    match db
        .execute(builder.build(&schema.create_table_from_entity(entity)))
        .await
    {
        Ok(_) => info!("created table"),
        Err(error) => warn!(%error, "failed to create table"),
    };
}
