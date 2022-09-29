use rocket::{http::Status, log::private::warn, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool};
use std::collections::HashMap;

type DBPool = Pool<MySql>;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Stage {
    id: i32,
    tournament_id: i32,
    idx: i8,
    stage_name: String,
    best_of: i8,
    pool_format: sqlx::types::JsonValue,
}

#[post(
    "/<tournament_id>/stage/create",
    format = "application/json",
    data = "<stage>"
)]
pub async fn create(
    tournament_id: i32,
    stage: Json<Stage>,
    db_pool: &State<DBPool>,
) -> (Status, String) {
    let query_result = sqlx::query!(
        "INSERT INTO stage(tournament_id, idx, stage_name, best_of, pool_format) VALUES (?, ?, ?, ?, ?)",
        tournament_id,
        stage.idx,
        &stage.stage_name,
        stage.best_of,
        &stage.pool_format
    )
    .execute(&**db_pool)
    .await;

    match query_result {
        Ok(_) => (Status::Ok, "Successfully created stage.".to_owned()),
        Err(e) => {
            let err_msg = format!("Error creating stage. Maybe the tournament does not exist. {e}");
            warn!("{}", err_msg);
            (Status::UnprocessableEntity, err_msg)
        }
    }
}

#[get("/<tournament_id>/stage")]
pub async fn get_all(
    tournament_id: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Vec<Stage>>, (Status, String)> {
    let query_result = sqlx::query_as!(
        Stage,
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of, stage.pool_format
        FROM tournament 
        INNER JOIN stage ON stage.tournament_id=tournament.id 
        WHERE tournament.id=?
        ORDER BY stage.idx ASC",
        tournament_id
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

#[get("/<tournament_id>/stage/<stage_idx>")]
pub async fn get(
    tournament_id: i32,
    stage_idx: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<Stage>, (Status, String)> {
    let query_result = sqlx::query_as!(Stage,
        "SELECT stage.id, stage.tournament_id, stage.idx, stage.stage_name, stage.best_of, stage.pool_format
        FROM tournament 
        INNER JOIN stage ON stage.tournament_id=tournament.id 
        WHERE stage.idx=? AND tournament.id=?",
        &stage_idx,
        tournament_id
    )
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

#[post(
    "/<tournament_id>/stage/<stage_idx>/pool_format",
    format = "application/json",
    data = "<format>"
)]
pub async fn set_pool_format(
    tournament_id: i32,
    stage_idx: i32,
    format: String,
    db_pool: &State<DBPool>,
) -> (Status, String) {
    // Check if the json is only a mapping to non-negative integers
    if serde_json::from_str::<HashMap<String, u8>>(&format).is_err() {
        return (Status::UnprocessableEntity, "Invalid pool format. This should only be a mapping from mod bracket names to number of maps in the bracket".to_owned());
    }

    debug!("Pool format: {:?}", format);

    let res = sqlx::query!(
        "UPDATE stage SET pool_format=? WHERE tournament_id=? AND idx=?",
        format,
        tournament_id,
        stage_idx
    )
    .execute(&**db_pool)
    .await;

    match res {
        Ok(_) => (Status::Ok, "Successfully set pool format".to_owned()),
        Err(e) => (
            Status::InternalServerError,
            format!("Error setting pool format: {e}"),
        ),
    }
}

#[get("/<tournament_id>/stage/<stage_idx>/pool_format")]
pub async fn get_pool_format(
    tournament_id: i32,
    stage_idx: i32,
    db_pool: &State<DBPool>,
) -> Result<Json<HashMap<String, u8>>, (Status, String)> {
    let res = sqlx::query!(
        "SELECT pool_format FROM stage WHERE tournament_id=? AND idx=?",
        tournament_id,
        stage_idx
    )
    .fetch_optional(&**db_pool)
    .await;

    // Catch any error from the database
    let res = match res {
        Ok(r) => r,
        Err(e) => {
            return Err((
                Status::InternalServerError,
                format!("Error getting pool format: {e}"),
            ))
        }
    };

    // Extract the value and handle possible errors
    match res {
        Some(format) => serde_json::from_value::<HashMap<String, u8>>(format.pool_format)
            .map(Json)
            .map_err(|e| {
                (
                    Status::InternalServerError,
                    format!("Error deserializing pool format: {e}"),
                )
            }),
        None => Err((
            Status::NotFound,
            format!("Pool Format not found for stage {stage_idx} of tournament {tournament_id}"),
        )),
    }
}
