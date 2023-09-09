use rosu_v2::prelude::*;

pub async fn get_map(osu: impl AsRef<Osu>, map_id: u32) -> Vec<BeatmapCompact> {
    osu.as_ref().beatmaps([map_id]).await.unwrap()
}
