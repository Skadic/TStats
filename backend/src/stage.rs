use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, Clone, Deserialize, Serialize, FromRow, Default)]
pub struct Stage {
    id: Option<i32>,
    tournament: Option<i32>,
    stage_number: i32,
    name: String,
}

#[post(
    "/<tournament_id>/create",
    format = "application/json",
    data = "<stage>"
)]
pub async fn create(
    tournament_id: i32,
    stage: Json<Stage>,
    db_pool: &State<Pool<Postgres>>,
) -> (Status, &'static str) {
    let query_result =
        sqlx::query("INSERT INTO stage(tournament, name, stage_number) VALUES ($1, $2, $3)")
            .bind(tournament_id)
            .bind(&stage.name)
            .bind(stage.stage_number)
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
    db_pool: &State<Pool<Postgres>>,
) -> (Status, Option<Json<Vec<Stage>>>) {
    let query_result = sqlx::query_as::<Postgres, Stage>(
        "SELECT stage.id, stage.tournament, stage.stage_number, stage.name
        FROM tournament 
        INNER JOIN stage ON stage.tournament=tournament.id 
        WHERE tournament.id=$1
        ORDER BY stage.stage_number ASC",
    )
    .bind(tournament_id)
    .fetch_all(&**db_pool)
    .await;

    match query_result {
        Ok(stages) => (Status::Ok, Some(Json(stages))),
        Err(_) => (Status::InternalServerError, None),
    }
}

#[get("/<tournament_id>/<stage_number>")]
pub async fn get(
    tournament_id: i32,
    stage_number: i32,
    db_pool: &State<Pool<Postgres>>,
) -> (Status, Option<Json<Stage>>) {
    let query_result = sqlx::query_as::<Postgres, Stage>(
        "SELECT stage.id, stage.tournament, stage.stage_number, stage.name
        FROM tournament 
        INNER JOIN stage ON stage.tournament=tournament.id 
        WHERE stage.stage_number=$1 AND tournament.id=$2",
    )
    .bind(&stage_number)
    .bind(tournament_id)
    .fetch_optional(&**db_pool)
    .await;

    if query_result.is_err() {
        return (Status::InternalServerError, None);
    }

    let query_result = query_result.unwrap();
    match query_result {
        Some(stage) => (Status::Ok, Some(Json(stage))),
        None => (Status::NotFound, None),
    }
}
