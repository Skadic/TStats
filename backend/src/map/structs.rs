use rosu_v2::prelude::{Beatmap, Beatmapset, BeatmapsetCovers, GameMode, RankStatus, BeatmapCompact};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct PoolSlot {
    pub map_id: u32,
    pub mapset: MinimizedBeatmapset,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct MinimizedBeatmapset {
    pub mapset_id: u32,
    pub artist: String,
    pub title: String,
    pub covers: BeatmapsetCovers,
    pub converts: Vec<MinimizedBeatmap>,
    pub creator_name: String,
    pub creator_id: u32,
    pub maps: Vec<MinimizedBeatmap>,
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct MinimizedBeatmap {
    pub map_id: u32,
    pub mapset_id: u32,
    pub difficulty: String,
    pub ar: f32,
    pub cs: f32,
    pub hp: f32,
    pub od: f32,
    pub bpm: f32,
    pub stars: f32,
    pub max_combo: u32,
    pub status: RankStatus,
    pub seconds_total: u32,
    pub mode: GameMode,
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
