use rosu_v2::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cache::{get_cached_or, CacheResult, Cacheable};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimBeatmap {
    artist_name: String,
    name: String,
    set_id: u32,
    map_id: u32,
    creator_id: u32,
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
    pub fn from_map_and_set(map: &Beatmap, set: &Beatmapset) -> Self {
        Self {
            artist_name: set.artist.clone(),
            name: set.title.clone(),
            set_id: map.mapset_id,
            map_id: map.map_id,
            creator_id: map.creator_id,
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
    let map = get_cached_or::<SlimBeatmap, OsuError, _, _>(redis, &map_id, Some(3600), || async {
        let mapset = osu.as_ref().beatmapset_from_map_id(map_id).await?;
        let maps = mapset.maps.as_ref().unwrap();
        // beatmapset_from_map_id guarantees that maps has entries and we also know that the map with the given id exists
        let map = maps.iter().find(|map| map.map_id == map_id).unwrap();
        Ok(SlimBeatmap::from_map_and_set(map, &mapset))
    })
    .await?;

    Ok(map)
}
