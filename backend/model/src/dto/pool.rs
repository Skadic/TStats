use poem_openapi::Object;
use rosu_v2::prelude::{BeatmapExtended, BeatmapsetExtended};
use serde::{Deserialize, Serialize};
use utils::Cacheable;
use miette::Diagnostic;

use super::osu::OsuUserDto;

#[derive(Object, PartialEq, Debug, Clone)]
#[oai(rename = "Pool", rename_all = "camelCase")]
pub struct PoolDto {
    pub tournament_id: usize,
    pub stage_order: usize,
    pub brackets: Vec<PoolBracketDto>,
}

#[derive(Object, PartialEq, Debug, Clone)]
#[oai(rename = "PoolBracket", rename_all = "camelCase")]
pub struct PoolBracketDto {
    pub order: usize,
    pub name: String,
    pub maps: Vec<BeatmapDto>,
}

#[derive(Object, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[oai(rename = "Beatmap", rename_all = "camelCase")]
pub struct BeatmapDto {
    /// The song's artist's name
    pub artist_name: String,
    /// The song's name
    pub title: String,
    /// The beatmap's difficulty name
    pub difficulty_name: String,
    /// The id of the mapset
    pub mapset_id: u32,
    /// The id of the beatmap
    pub map_id: u32,
    /// The creator of the beatmap
    pub creator: OsuUserDto,
    /// This map's difficulty
    pub difficulty: DifficultyDto,
}

#[derive(Object, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[oai(rename = "Difficulty", rename_all = "camelCase")]
pub struct DifficultyDto {
    stars: f32,
    /// The map's length in seconds
    total_length: u32,
    /// The map's drain time in seconds
    drain_time: u32,
    max_combo: u32,
    bpm: f32,
    cs: f32,
    ar: f32,
    od: f32,
    hp: f32,
}

#[derive(Debug, Clone, Copy, thiserror::Error, Diagnostic)]
pub enum TryFromBeatmapError {
    #[error("missing attribute: '{0}'")]
    MissingAttribute(&'static str),
    #[error("no mapset")]
    MissingMapset,
    #[error("no map: {0}")]
    MissingMap(u32),
}

impl BeatmapDto {
    pub fn new(
        map_id: u32,
        mapset: BeatmapsetExtended,
        creator: OsuUserDto,
    ) -> Result<Self, TryFromBeatmapError> {
        let map = mapset
            .maps
            .ok_or(TryFromBeatmapError::MissingMap(map_id))?
            .into_iter()
            .filter(|map| map.map_id == map_id)
            .next()
            .ok_or(TryFromBeatmapError::MissingMap(map_id))?;
        let difficulty = DifficultyDto {
            stars: map.stars,
            total_length: map.seconds_total,
            drain_time: map.seconds_drain,
            max_combo: map
                .max_combo
                .ok_or(TryFromBeatmapError::MissingAttribute("map.max_combo"))?,
            bpm: map.bpm,
            cs: map.cs,
            ar: map.ar,
            od: map.od,
            hp: map.hp,
        };
        let artist_name = mapset.artist.clone();
        let title = mapset.title.clone();
        let difficulty_name = map.version;
        let mapset_id = map.mapset_id;
        let map_id = map.map_id;

        Ok(BeatmapDto {
            artist_name,
            title,
            difficulty_name,
            mapset_id,
            map_id,
            creator,
            difficulty,
        })
    }
}

impl Cacheable for BeatmapDto {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "beatmap"
    }

    fn key(&self) -> &Self::KeyType {
        &self.map_id
    }
}
