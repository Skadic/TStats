mod implementation;
mod model;

pub use model::*;
use sqlx::PgPool;

/// Migrate the database to be up to date with the SQL migration scripts.
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}
