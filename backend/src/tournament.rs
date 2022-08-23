
use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Tournament {
    id: Option<i32>,
    shorthand: String,
    full_name: String,
}

#[post("/create", format = "application/json", data = "<tournament>")]
pub async fn create_tournament(
    tournament: Json<Tournament>,
    db_pool: &State<Pool<Postgres>>,
) -> Status {
    let tournament = tournament.into_inner();

    let query_result = sqlx::query("INSERT INTO Tournament(shorthand, full_name) VALUES ($1, $2)")
        //.bind(tournament.id)
        .bind(tournament.shorthand)
        .bind(tournament.full_name)
        .execute(&**db_pool)
        .await;

    match query_result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[get("/by_shorthand?<shorthand>")]
pub async fn get_by_shorthand(
    shorthand: Vec<String>,
    db_pool: &State<Pool<Postgres>>,
) -> (Status, Option<Json<Vec<Tournament>>>) {
    let result = sqlx::query_as::<Postgres, Tournament>("SELECT * FROM Tournament WHERE shorthand = ANY($1)")
        .bind(&shorthand)
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(vec) => (Status::Ok, Some(Json(vec))),
        Err(_) => (Status::InternalServerError, None),
    }
}

#[get("/all")]
pub async fn get_all(sqlite_pool: &State<Pool<Postgres>>) -> (Status, Option<Json<Vec<Tournament>>>) {
    let result =
        sqlx::query_as::<_, Tournament>("SELECT * FROM Tournament").fetch_all(&**sqlite_pool);
    match result.await {
        Ok(vec) => (Status::Ok, Some(Json(vec))),
        Err(_) => (Status::InternalServerError, None),
    }
}


