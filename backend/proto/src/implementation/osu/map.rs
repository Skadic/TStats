use rosu_v2::prelude::*;
use utils::Cacheable;

impl Cacheable for crate::osu::Beatmap {
    type KeyType = u32;

    fn type_key() -> &'static str {
        "map"
    }

    fn key(&self) -> &Self::KeyType {
        &self.map_id
    }
}

impl crate::osu::Beatmap {
    pub fn from_map_set_and_creator(
        map: &BeatmapExtended,
        set: &BeatmapsetExtended,
        creator: &crate::osu::User,
    ) -> Self {
        Self {
            artist_name: set.artist.clone(),
            name: set.title.clone(),
            mapset_id: map.mapset_id,
            map_id: map.map_id,
            difficulty_name: map.version.clone(),
            creator: Some(creator.clone()),
            difficulty: Some(crate::osu::Difficulty {
                stars: map.stars,
                length: map.seconds_total,
                bpm: map.bpm,
                cs: map.cs,
                ar: map.ar,
                od: map.od,
                hp: map.hp,
            }),
        }
    }
}
