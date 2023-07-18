use model::tournament::TournamentFormat;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Ws,
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

use crate::model::{
    stage::Stage,
    tournament::Tournament,
    TableType, map::PoolMap,
};

mod model;

#[derive(Deserialize, Serialize)]
pub enum UserStatus {
    Active,
    Inactive,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    user_id: usize,
    name: String,
    status: UserStatus,
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

    let tournament: Record = db
        .create(Tournament::table_name())
        .content(Tournament {
            name: "Deutsche Meisterschaft 8".to_string(),
            shorthand: "DM8".to_string(),
            format: TournamentFormat::versus(1),
            rank_range: None,
            bws: false,
            country_restriction: Some(vec!["GER".to_string()]),
        })
        .await?;

    let stage: Record = db
        .create(Stage::table_name())
        .content(Stage {
            order: 0,
            name: "QF".to_string(),
            tournament: tournament.id.clone(),
        })
        .await.unwrap();

    let map: Record = db
        .create(PoolMap::table_name())
        .content(PoolMap {
            map_id: 4110015,
            stage: stage.id.clone(),
            bracket: "NM".to_string(),
            bracket_order: 1,
        })
        .await.unwrap();

    dbg!(tournament);
    dbg!(stage);
    dbg!(map);

    Ok(())
}
