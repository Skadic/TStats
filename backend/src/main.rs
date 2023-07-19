use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

use model::tournament::RankRange;

use crate::model::{
    map::PoolMap,
    stage::Stage,
    tournament::{Tournament, TournamentFormat},
    TableType,
};

mod model;
mod routes;

#[derive(Debug, Deserialize, Serialize)]
struct TestA {
    n: i32,
    b: TestB,
    c: Option<TournamentFormat>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestB {
    x: i32,
    y: i32,
}

#[derive(Debug, Deserialize, Serialize)]
enum TestC {
    VarA { value: i32 },
    VarB(String),
    VarC(bool),
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("default").use_db("tstats").await?;

    db.query(include_str!("../db/schema/tournament.sql"))
        .await?;
    db.query(include_str!("../db/schema/stage.sql")).await?;
    db.query(include_str!("../db/schema/map.sql")).await?;

    let a: Record = db
        .create("test")
        .content(TestA {
            n: 1,
            b: TestB { x: 2, y: 3 },
            c: Some(TournamentFormat::versus(1)),
        })
        .await?;

    let tn = Tournament {
        name: "Deutsche Meisterschaft 8".to_string(),
        shorthand: "DM8".to_string(),
        format: TournamentFormat::versus(1),
        rank_range: Some(RankRange::Single {
            rank_range: 750..5000,
        }),
        bws: false,
        country_restriction: Some(vec!["GER".to_string()]),
    };
    println!("{}", serde_json::to_string(&tn).unwrap());
    let tournament: Record = db.create(Tournament::table_name()).content(tn).await?;

    let stage: Record = db
        .create(Stage::table_name())
        .content(Stage {
            order: 0,
            name: "QF".to_string(),
            tournament: tournament.id.clone(),
            pool_brackets: vec!["NM".to_string(), "HD".to_string(), "HR".to_string()],
        })
        .await
        .unwrap();

    let map: Record = db
        .create(PoolMap::table_name())
        .content(PoolMap {
            map_id: 4110015,
            stage: stage.id.clone(),
            bracket: "NM".to_string(),
            bracket_order: 1,
        })
        .await
        .unwrap();

    dbg!(tournament);
    dbg!(stage);
    dbg!(map);

    //let db = Arc::new(db);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/api/tournament/get",
            get(routes::tournament::get_tournament),
        )
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
