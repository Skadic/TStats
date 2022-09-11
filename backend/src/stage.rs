use rocket::{http::Status, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;

type DBPool = Pool<MySql>;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow, Default)]
pub struct Stage {
    id: i32,
    tournament_id: i32,
    idx: i8,
    stage_name: String,
    best_of: i8,
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
    let query_result = sqlx::query!(
        "INSERT InTO stage(tournament_id, idx, stage_name, best_of) VALUES (?, ?, ?, ?)",
        tournament_id,
        stage.idx,
        &stage.stage_name,
        stage.best_of
    )
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
    let query_result = sqlx::query_as!(
        Stage,
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of
        FROM tournament 
        INNER JOIN stage ON stage.tournament_id=tournament.id 
        WHERE tournament.id=?
        ORDER BY stage.stage_number ASC",
        Some(tournament_id)
    )
    .fetch_all(&**db_pool)
    .await;

    query_result.map(Json).map_err(|err| {
        (
            Status::InternalServerError,
            format!("Error requesting stages for tournament {tournament_id}: {err}"),
        )
    })
}

#[get("/<tournament_id>/<stage_idx>")]
pub async fn get(
    tournament_id: i32,
    stage_idx: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Stage>, (Status, String)> {
    let query_result = sqlx::query_as::<MySql, Stage>(
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of
        FROM tournament 
        INNER JOIN stage ON stage.tournament=tournament.id 
        WHERE stage.stage_number=? AND tournament.id=?",
    )
    .bind(&stage_idx)
    .bind(tournament_id)
    .fetch_optional(&**db_pool)
    .await;

    if let Err(err) = query_result {
        return Err((
            Status::InternalServerError,
            format!("Error querying stage {stage_idx} of tournament {tournament_id}: {err}"),
        ));
    }

    query_result
        .unwrap()
        .map(Json)
        .ok_or((Status::NotFound, format!("Stage {tournament_id} not found")))
}


// TODO Yeah this api is pretty screwed lol. I gotta change that sometime
#[post("/<tournament_id>/<stage_idx>/pool_format", format = "application/json", data = "<format>")]
pub async fn set_pool_format(
    tournament_id: i32,
    stage_idx: i32,
    format: String,
    db_pool: &State<DBPool>,
) -> (Status, String) {
    // Check if the json is only a mapping to non-negative integers
    if let Err(_) = serde_json::from_str::<HashMap<String, u8>>(&format) {
        return (Status::UnprocessableEntity, "Invalid pool format. This should only be a mapping from mod bracket names to number of maps in the bracket".to_owned());
    }

    println!("{:?}", format);

    let res = sqlx::query!("UPDATE stage SET pool_format=? WHERE tournament_id=? AND idx=?", format, tournament_id, stage_idx)
        .execute(&**db_pool)
        .await;

    match res {
        Ok(_) => (Status::Ok, "Successfully set pool format".to_owned()),
        Err(e) => (Status::InternalServerError, format!("Error setting pool format: {e}"))
    }

}