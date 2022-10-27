use crate::{
    map::structs::{MinimizedBeatmapset, PoolSlot},
    util::{get_cached, get_cached_opt},
    DBPool,
};
use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex, State};
use rosu_v2::Osu;

use self::util::{get_mapset_cached, get_stage_id_cached};

pub mod structs;
pub mod util;

#[get("/test_map")]
pub async fn get_test_map(
    osu: &State<Osu>,
    redis_client: &State<Mutex<redis::aio::Connection>>,
) -> (Status, Result<Json<MinimizedBeatmapset>, String>) {
    let mut lock = redis_client.lock().await;

    let mapset = match get_cached(&mut lock, "mapset:662395", || async {
        osu.beatmapset(662395).await.map(MinimizedBeatmapset::from)
    })
    .await
    {
        Ok(set) => set,
        Err(err) => {
            return (
                Status::InternalServerError,
                Err(format!(
                    "Error during retrieval of cached value with key \"mapset:662395\": {}",
                    err
                )),
            )
        }
    };

    (Status::Ok, Ok(Json(mapset)))
}

#[post("/<tournament_id>/stage/<stage_idx>/pool/set?<map_slot>&<map_id>")]
pub async fn set_map(
    tournament_id: u32,
    stage_idx: u32,
    map_slot: u32,
    map_id: u32,
    osu: &State<Osu>,
    db_pool: &State<DBPool>,
    redis_client: &State<Mutex<redis::aio::Connection>>,
) -> Result<Status, (Status, String)> {
    let mut client = redis_client.lock().await;

    let stage_id_request = get_stage_id_cached(&mut *client, &**db_pool, tournament_id, stage_idx);

    let stage_id = match stage_id_request.await? {
        Some(stage_id) => stage_id,
        None => {
            return Err((
                Status::NotFound,
                format!("Stage {stage_idx} of tournament {tournament_id} not found"),
            ))
        }
    };

    // Get the map from the osu api and cache it
    let map_cache_key = format!("map:{map_id}");
    let map_request = get_cached(&mut client, &map_cache_key, || async {
        osu.beatmaps([map_id]).await
    })
    .await?
    .into_iter()
    .next();

    let map = match map_request {
        Some(map) => map,
        None => {
            return Err((
                Status::NotFound,
                format!("The map with id {map_id} does not exist"),
            ))
        }
    };

    let mapset_id = map.mapset.unwrap().mapset_id;

    let res = sqlx::query!(
        "
        INSERT INTO mappoolslot(stage_id, map_slot, mapset_id, map_id) VALUES (?, ?, ?, ?) ON DUPLICATE KEY UPDATE mapset_id=?, map_id=?
        ",
        stage_id,
        map_slot,
        mapset_id,
        map.map_id,
        mapset_id,
        map.map_id
    )
    .execute(&**db_pool)
    .await;

    match res {
        Ok(_) => Ok(Status::Ok),
        Err(e) => Err((Status::InternalServerError, format!("Error setting map with id {map_id} as map {map_slot} in stage {stage_idx} of tournament {tournament_id}: {e}")))
    }
}

#[get("/<tournament_id>/stage/<stage_idx>/pool/<map_slot>")]
pub async fn get_map_by_slot(
    tournament_id: u32,
    stage_idx: u32,
    map_slot: u8,
    osu: &State<Osu>,
    db_pool: &State<DBPool>,
    redis_client: &State<Mutex<redis::aio::Connection>>,
) -> Result<Json<PoolSlot>, (Status, String)> {
    let client = redis_client.lock();

    // Get the mapset and map id for the given pool slot
    let record = {
        let res = sqlx::query!(
            "
            SELECT stage.id AS stage_id, mappoolslot.mapset_id, mappoolslot.map_id FROM mappoolslot
            INNER JOIN stage ON stage.id = mappoolslot.stage_id
            INNER JOIN tournament ON tournament.id = stage.tournament_id
            WHERE tournament.id = ? AND stage.idx = ? AND mappoolslot.map_slot = ? 
            ",
            tournament_id,
            stage_idx,
            map_slot
        )
        .fetch_optional(&**db_pool)
        .await;

        // Handle errors from the database
        if let Err(e) = res {
            let err_msg = format!("Error retrieving map slot {map_slot} in stage of index {stage_idx} in tournament with id {tournament_id}: {e}");
            error!("{}", err_msg);
            return Err((Status::InternalServerError, err_msg));
        }

        match res.unwrap() {
            Some(record) => record,
            None => {
                let err_msg = format!("Map in slot {map_slot} of stage {stage_idx} in tournament with id {tournament_id} not found.");
                error!("{}", err_msg);
                return Err((Status::NotFound, err_msg));
            }
        }
    };

    let mapset = {
        get_mapset_cached(&mut *client.await, &**osu, record.mapset_id as u32)
            .await
            .map_err(|err| {
                let err_msg = format!(
                "Error retrieving mapset {} from osu api. Maybe the mapset does not exist: {err}",
                record.mapset_id
            );
                error!("{}", err_msg);
                (Status::InternalServerError, err_msg)
            })?
    };

    match mapset {
        Some(set) => Ok(Json(PoolSlot {
            map_id: record.map_id as u32,
            mapset: set,
        })),
        None => Err((Status::NotFound, format!("Not found"))),
    }
}

#[get("/<tournament_id>/stage/<stage_idx>/pool")]
pub async fn get_mappool(
    tournament_id: u32,
    stage_idx: u32,
    osu: &State<Osu>,
    db_pool: &State<DBPool>,
    redis_client: &State<Mutex<redis::aio::Connection>>,
) -> Result<Json<Vec<PoolSlot>>, (Status, String)> {
    let mut client = redis_client.lock().await;

    let records = {
        let res = sqlx::query!(
        "
        SELECT map_slot, mapset_id, map_id FROM mappoolslot INNER JOIN stage ON stage.id=mappoolslot.stage_id WHERE stage.tournament_id=? AND stage.idx=? 
        ",
        tournament_id,
        stage_idx)
        .fetch_all(&**db_pool)
        .await;

        // Handle errors from the database
        if let Err(e) = res {
            let err_msg = format!("Error retrieving maps for stage of index {stage_idx} in tournament with id {tournament_id}: {e}");
            error!("{}", err_msg);
            return Err((Status::InternalServerError, err_msg));
        }

        // Get each map's data from the osu api or Redis if it is cached
        let mut result = vec![];
        for record in res.unwrap().into_iter() {
            let mapset = get_mapset_cached(&mut *client, &**osu, record.mapset_id as u32).await;
            // Did an error occur while caching?
            match mapset {
                // Was the map found?
                Ok(opt) => match opt {
                    Some(map) => result.push(PoolSlot {
                        map_id: record.map_id as u32,
                        mapset: map,
                    }),
                    None => {
                        let err_msg = format!("map not found while retrieving mappool");
                        error!("{}", err_msg);
                        return Err((Status::NotFound, err_msg));
                    }
                },
                Err(e) => {
                    let err_msg = format!("Error retrieving maps for stage of index {stage_idx} in tournament with id {tournament_id}: {e}");
                    error!("{}", err_msg);
                    return Err((Status::NotFound, err_msg));
                }
            }
        }
        result
    };

    Ok(Json(records))
}
