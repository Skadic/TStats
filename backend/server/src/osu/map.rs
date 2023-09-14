use std::error::Error;

use rosu_v2::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cache::{get_cached_or, CacheError, CacheResult, Cacheable};

use super::user::OsuUser;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimBeatmap {
    artist_name: String,
    name: String,
    diff_name: String,
    set_id: u32,
    map_id: u32,
    creator: OsuUser,
    difficulty: Difficulty,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    stars: f32,
    length: u32,
    bpm: f32,
    cs: f32,
    ar: f32,
    od: f32,
    hp: f32,
}

impl Cacheable for SlimBeatmap {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "map"
    }

    fn key(&self) -> Self::KeyType {
        self.map_id
    }
}

impl SlimBeatmap {
    pub fn from_map_set_and_creator(map: &Beatmap, set: &Beatmapset, creator: &OsuUser) -> Self {
        Self {
            artist_name: set.artist.clone(),
            name: set.title.clone(),
            set_id: map.mapset_id,
            map_id: map.map_id,
            diff_name: map.version.clone(),
            creator: creator.clone(),
            difficulty: Difficulty {
                stars: map.stars,
                length: map.seconds_total,
                bpm: map.bpm,
                cs: map.cs,
                ar: map.ar,
                od: map.od,
                hp: map.hp,
            },
        }
    }
}

pub async fn get_map(
    redis: &mut redis::aio::MultiplexedConnection,
    osu: impl AsRef<Osu>,
    map_id: u32,
) -> CacheResult<SlimBeatmap> {
    // Find the map's data
    let map = get_cached_or::<SlimBeatmap, CacheError, _, _>(
        &mut redis.clone(),
        &map_id,
        Some(3600),
        || async {
            // If it doesn't exist in the cache, request it from the osu api
            let mapset = osu
                .as_ref()
                .beatmapset_from_map_id(map_id)
                .await
                .map_err(|e| -> Box<dyn Error> { Box::new(e) })
                .map_err(CacheError::Request)?;
            let maps = mapset.maps.as_ref().unwrap();
            // beatmapset_from_map_id guarantees that maps has entries and we also know that the map with the given id exists
            let map = maps.iter().find(|map| map.map_id == map_id).unwrap();
            let creator = get_cached_or::<OsuUser, OsuError, _, _>(
                redis,
                &map.creator_id,
                Some(3600),
                || async { osu.as_ref().user(map.creator_id).await.map(OsuUser::from) },
            )
            .await?;

            Ok(SlimBeatmap::from_map_set_and_creator(
                map, &mapset, &creator,
            ))
        },
    )
    .await?;

    Ok(map)
}
