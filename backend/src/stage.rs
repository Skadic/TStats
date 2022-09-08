use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};

type DBPool = Pool<MySql>;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow, Default)]
pub struct Stage {
    id: Option<i32>,
    tournament_id: Option<i32>,
    idx: i32,
    stage_name: String,
    best_of: u8,
}

#[post(
    "/<tournament_id>/create",
    format = "application/json",
    data = "<stage>"
)]
pub async fn create(
    tournament_id: i32,
    stage: Json<Stage>,
    db_pool: &State<DBPool>,
) -> (Status, &'static str) {
    let query_result = sqlx::query(
        "INSERT INTO stage(tournament_id, idx, stage_name, best_of) VALUES (?, ?, ?, ?)",
    )
    .bind(tournament_id)
    .bind(stage.idx)
    .bind(&stage.stage_name)
    .bind(stage.best_of)
    .execute(&**db_pool)
    .await;

    match query_result {
        Ok(_) => (Status::Ok, "Successfully created stage."),
        Err(_) => (
            Status::InternalServerError,
            "Error creating stage. Maybe the tournament does not exist.",
        ),
    }
}

#[get("/<tournament_id>")]
pub async fn get_all(
    tournament_id: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Vec<Stage>>, (Status, String)> {
    let query_result = sqlx::query_as::<MySql, Stage>(
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of
        FROM tournament 
        INNER JOIN stage ON stage.tournament=tournament.id 
        WHERE tournament.id=?
        ORDER BY stage.stage_number ASC",
    )
    .bind(tournament_id)
    .fetch_all(&**db_pool)
    .await;

    query_result.map(Json).map_err(|err| {
        (
            Status::InternalServerError,
            format!("Error requesting stages for tournament {tournament_id}: {err}"),
        )
    })
}

#[get("/<tournament_id>/<stage_number>")]
pub async fn get(
    tournament_id: i32,
    stage_number: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Stage>, (Status, String)> {
    let query_result = sqlx::query_as::<MySql, Stage>(
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of
        FROM tournament 
        INNER JOIN stage ON stage.tournament=tournament.id 
        WHERE stage.stage_number=? AND tournament.id=?",
    )
    .bind(&stage_number)
    .bind(tournament_id)
    .fetch_optional(&**db_pool)
    .await;

    if let Err(err) = query_result {
        return Err((
            Status::InternalServerError,
            format!("Error querying stage {stage_number} of tournament {tournament_id}: {err}"),
        ));
    }

    query_result
        .unwrap()
        .map(Json)
        .ok_or((Status::NotFound, format!("Stage {tournament_id} not found")))
}
