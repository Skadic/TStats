use rosu_v2::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cache::Cacheable;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PoolMap {
    artist_name: String,
    name: String,
    set_id: u32,
    map_id: u32,
    difficulty: Difficulty,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Difficulty {
    stars: f32,
    length: u32,
    bpm: f32,
    cs: f32,
    ar: f32,
    od: f32,
    hp: f32,
}

pub async fn get_maps(
    osu: impl AsRef<Osu>,
    map_ids: impl IntoIterator<Item = u32>,
) -> Vec<BeatmapCompact> {
    osu.as_ref().beatmaps(map_ids).await.unwrap()
}

impl Cacheable for PoolMap {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "map"
    }

    fn key(&self) -> Self::KeyType {
        self.map_id
    }
}
