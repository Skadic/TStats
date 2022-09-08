use rocket::{http::Status, log::private::error, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};

type DBPool = Pool<MySql>;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Tournament {
    id: Option<i32>,
    shorthand: String,
    full_name: String,
    play_format: i8,
    team_size: i8,
    score_version: i8,
}

#[post("/create", format = "application/json", data = "<tournament>")]
pub async fn create_tournament(tournament: Json<Tournament>, db_pool: &State<DBPool>) -> Status {
    let tournament = tournament.into_inner();

    let query_result = sqlx::query("INSERT INTO tournament(shorthand, full_name, play_format, team_size, score_version) VALUES (?, ?, ?, ?, ?)")
        .bind(tournament.shorthand)
        .bind(tournament.full_name)
        .bind(tournament.play_format)
        .bind(tournament.team_size)
        .bind(tournament.score_version)
        .execute(&**db_pool)
        .await;

    match query_result {
        Ok(_) => Status::Ok,
        Err(e) => {
            error!("Error creating tournament: {}", e);
            Status::InternalServerError
        }
    }
}

#[get("/<tournament_id>")]
pub async fn get(
    tournament_id: i32,
    db_pool: &State<DBPool>,
) -> (Status, Option<Json<Tournament>>) {
    println!("YES HE CALLED THIS");
    let result = sqlx::query_as::<_, Tournament>("SELECT * FROM tournament WHERE id=?")
        .bind(tournament_id)
        .fetch_optional(&**db_pool);
    match result.await {
        Ok(tournament) => {
            tournament.map_or((Status::NotFound, None), |t| (Status::Ok, Some(Json(t))))
        }
        Err(_) => (Status::InternalServerError, None),
    }
}

#[get("/all")]
pub async fn get_all(db_pool: &State<DBPool>) -> (Status, Option<Json<Vec<Tournament>>>) {
    let result =
        sqlx::query_as::<_, Tournament>("SELECT * FROM tournament").fetch_all(&**db_pool);
    match result.await {
        Ok(vec) => (Status::Ok, Some(Json(vec))),
        Err(e) => {
            error!("Error creating tournament: {}", e);
            (Status::InternalServerError, None)
        }
    }
}
