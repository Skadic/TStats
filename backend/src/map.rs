use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex, State};
use rosu_v2::{
    prelude::{Beatmap, Beatmapset, BeatmapsetCovers, GameMode, RankStatus},
    Osu,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use crate::util::{get_cached, get_cached_opt};

type DBPool = Pool<MySql>;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MinimizedBeatmapset {
    mapset_id: u32,
    artist: String,
    title: String,
    covers: BeatmapsetCovers,
    converts: Vec<MinimizedBeatmap>,
    creator_name: String,
    creator_id: u32,
    maps: Vec<MinimizedBeatmap>,
}

impl From<Beatmapset> for MinimizedBeatmapset {
    fn from(s: Beatmapset) -> Self {
        Self {
            mapset_id: s.mapset_id,
            artist: s.artist,
            title: s.title,
            covers: s.covers,
            converts: s
                .converts
                .unwrap_or_default()
                .into_iter()
                .map(|m| m.into())
                .collect(),
            creator_name: s.creator_name.into_string(),
            creator_id: s.creator_id,
            maps: s
                .maps
                .unwrap_or_default()
                .into_iter()
                .map(|m| m.into())
                .collect(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MinimizedBeatmap {
    map_id: u32,
    mapset_id: u32,
    difficulty: String,
    ar: f32,
    cs: f32,
    hp: f32,
    od: f32,
    bpm: f32,
    stars: f32,
    max_combo: u32,
    status: RankStatus,
    seconds_total: u32,
    mode: GameMode,
}

impl From<Beatmap> for MinimizedBeatmap {
    fn from(m: Beatmap) -> Self {
        Self {
            map_id: m.map_id,
            mapset_id: m.mapset_id,
            difficulty: m.version,
            ar: m.ar,
            cs: m.cs,
            hp: m.hp,
            od: m.od,
            bpm: m.bpm,
            stars: m.stars,
            max_combo: m.max_combo.unwrap_or(0),
            status: m.status,
            seconds_total: m.seconds_total,
            mode: m.mode,
        }
    }
}

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

#[post("/<tournament_id>/stage/<stage_idx>/set_map?<map_slot>&<map_id>")]
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

    // Find the stage id and cache it if needs to be retrieved from the database
    let cache_key = format!("stage:{tournament_id}:{stage_idx}:id");
    let stage_id = match get_cached_opt(&mut client, &cache_key, || async {
        // find stage id
        sqlx::query!(
            "
            SELECT stage.id FROM stage 
            INNER JOIN tournament ON tournament.id = stage.tournament_id
            WHERE tournament.id=? AND stage.idx=?
            ",
            tournament_id,
            stage_idx
        )
        .fetch_optional(&**db_pool)
        .await
        .map(|opt| opt.map(|record| record.id))
    })
    .await?
    {
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
    let map = match get_cached(&mut client, &map_cache_key, || async {
        osu.beatmaps([map_id]).await
    })
    .await?
    .into_iter()
    .next()
    {
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
