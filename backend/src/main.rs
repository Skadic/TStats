use axum::{routing::get, routing::post, Router};
use axum::http::Method;
use log::info;

use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};
use tower_http::cors::CorsLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod model;
mod routes;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    tracing_log::LogTracer::init().unwrap();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Connecting to database...");
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
        .await?;

    db.use_ns("default").use_db("tstats").await?;

    info!("Setting up table schemas");
    db.query("BEGIN TRANSACTION")
        .query(include_str!("../db/schema/tournament.sql"))
        .query(include_str!("../db/schema/stage.sql"))
        .query(include_str!("../db/schema/is_stage.sql"))
        .query(include_str!("../db/schema/map.sql"))
        .query(include_str!("../db/schema/pool_contains.sql"))
        .query("COMMIT TRANSACTION")
        .await?;

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
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(["http://localhost:5173".parse().unwrap()]))
        .with_state(db);

    info!("Starting server");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
