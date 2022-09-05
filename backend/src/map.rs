use redis::AsyncCommands;
use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex, State};
use rosu_v2::{
    prelude::{Beatmap, Beatmapset, BeatmapsetCovers, GameMode, RankStatus},
    Osu,
};
use serde::{Deserialize, Serialize};

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
) -> (Status, Option<Json<MinimizedBeatmapset>>) {
    let mut lock = redis_client.lock().await;

    let set = if let Ok(cached) = lock.get::<_, String>("mapset:662395").await {
        match serde_json::from_str(&cached) {
            Ok(res) => res,
            Err(_) => return (Status::InternalServerError, None),
        }
    } else {
        let set = match osu.beatmapset(662395).await {
            Ok(res) => MinimizedBeatmapset::from(res),
            Err(_) => return (Status::InternalServerError, None),
        };

        let json_str = match serde_json::to_string(&set) {
            Ok(res) => res,
            Err(_) => return (Status::InternalServerError, None),
        };

        match lock
            .set::<&str, String, String>("mapset:662395", json_str)
            .await
        {
            Ok(_) => {}
            Err(_) => return (Status::InternalServerError, None),
        }

        set
    };

    (Status::Ok, Some(Json(set)))
}
