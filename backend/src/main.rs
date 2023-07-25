use axum::http::Method;
use axum::{routing::get, routing::post, Router};
use log::info;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, EntityName, Schema};
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
    tracing_log::LogTracer::init().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Connecting to database...");

    let db: DatabaseConnection = Database::connect("postgres://root:root@localhost:5432/postgres")
        .await
        .unwrap();

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    db.execute(builder.build(Table::drop().table(PoolMapEntity.table_ref())))
        .await
        .unwrap();
    db.execute(builder.build(Table::drop().table(PoolBracketEntity.table_ref())))
        .await
        .unwrap();
    db.execute(builder.build(Table::drop().table(StageEntity.table_ref())))
        .await
        .unwrap();
    db.execute(builder.build(Table::drop().table(CountryRestrictionEntity.table_ref())))
        .await
        .unwrap();
    db.execute(builder.build(Table::drop().table(TournamentEntity.table_ref())))
        .await
        .unwrap();

    db.execute(builder.build(&schema.create_table_from_entity(TournamentEntity)))
        .await
        .unwrap();
    db.execute(builder.build(&schema.create_table_from_entity(CountryRestrictionEntity)))
        .await
        .unwrap();
    db.execute(builder.build(&schema.create_table_from_entity(StageEntity)))
        .await
        .unwrap();
    db.execute(builder.build(&schema.create_table_from_entity(PoolBracketEntity)))
        .await
        .unwrap();
    db.execute(builder.build(&schema.create_table_from_entity(PoolMapEntity)))
        .await
        .unwrap();

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
        .route("/api/stage", post(routes::stage::create_stage))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(["http://localhost:5173".parse().unwrap()])
                .allow_headers(["content-type".parse().unwrap()]),
        )
        .with_state(db);

    info!("Starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
