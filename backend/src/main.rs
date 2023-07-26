use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use log::{info, LevelFilter};
use sea_orm::{
    sea_query::Table, ConnectionTrait, Database, DatabaseConnection, EntityTrait, Schema,
};
use tower_http::cors::CorsLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::model::entities::{
    CountryRestrictionEntity, PoolBracketEntity, PoolMapEntity, StageEntity, TournamentEntity,
};

mod model;
mod routes;

#[tokio::main]
async fn main() {
    tracing_log::LogTracer::builder()
        .with_max_level(LevelFilter::Trace)
        .ignore_crate("sqlx")
        .ignore_crate("hyper")
        .init()
        .unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Connecting to database...");

    let db: DatabaseConnection = Database::connect("postgres://root:root@localhost:5432/postgres")
        .await
        .unwrap();

    drop_table(&db, PoolMapEntity).await;
    drop_table(&db, PoolBracketEntity).await;
    drop_table(&db, StageEntity).await;
    drop_table(&db, CountryRestrictionEntity).await;
    drop_table(&db, TournamentEntity).await;

    create_table(&db, TournamentEntity).await;
    create_table(&db, CountryRestrictionEntity).await;
    create_table(&db, StageEntity).await;
    create_table(&db, PoolBracketEntity).await;
    create_table(&db, PoolMapEntity).await;

    // build our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test_data", post(routes::debug::fill_test_data))
        .route(
            "/api/tournament/all",
            get(routes::tournament::get_all_tournaments),
        )
        .route(
            "/api/tournament",
            get(routes::tournament::get_tournament).post(routes::tournament::create_tournament),
        )
        .route("/api/stage/all", get(routes::stage::get_all_stages))
        .route(
            "/api/stage",
            get(routes::stage::get_stage).post(routes::stage::create_stage),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(["http://localhost:4173".parse().unwrap(), "http://localhost:5173".parse().unwrap(),])
                .allow_headers(["content-type".parse().unwrap()]),
        )
        .with_state(db);

    info!("Starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn drop_table<E: EntityTrait>(db: &DatabaseConnection, table: E) {
    let builder = db.get_database_backend();
    match db
        .execute(builder.build(Table::drop().table(table.table_ref())))
        .await
    {
        Ok(_) => info!("Dropped table '{}'", table.table_name()),
        Err(e) => info!("Failed to drop table '{}': {e}", table.table_name()),
    };
}

async fn create_table<E: EntityTrait>(db: &DatabaseConnection, entity: E) {
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
