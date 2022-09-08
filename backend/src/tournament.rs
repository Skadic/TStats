use rocket::{http::Status, log::private::error, response::status, serde::json::Json, State, futures::FutureExt};
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
pub async fn create_tournament(
    tournament: Json<Tournament>,
    db_pool: &State<DBPool>,
) -> Result<Status, (Status, String)> {
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
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err((
            Status::InternalServerError,
            format!("Error creating tournament: {e}"),
        )),
    }
}

#[get("/<tournament_id>")]
pub async fn get(
    tournament_id: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Tournament>, (Status, String)> {
    let result = sqlx::query_as::<_, Tournament>("SELECT * FROM tournament WHERE id=?")
        .bind(tournament_id)
        .fetch_optional(&**db_pool);

    match result.await {
        Ok(tournament) => tournament.map(Json).ok_or((
            Status::NotFound,
            format!("Tournament with id {tournament_id} does not exist"),
        )),
        Err(err) => Err((
            Status::InternalServerError,
            format!("Error getting tournament with id {tournament_id}: {err}"),
        )),
    }
}

#[get("/all")]
pub async fn get_all(db_pool: &State<DBPool>) -> Result<Json<Vec<Tournament>>, (Status, String)> {
    let result = sqlx::query_as::<_, Tournament>("SELECT * FROM tournament").fetch_all(&**db_pool);

    result.await
        .map(Json)
        .map_err(|e| (Status::InternalServerError, format!("Error creating tournament: {e}")))
}
