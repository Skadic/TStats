use rosu_v2::prelude::*;

pub async fn get_maps(osu: impl AsRef<Osu>, map_ids: impl IntoIterator<Item=u32>) -> Vec<BeatmapCompact> {
    osu.as_ref().beatmaps(map_ids).await.unwrap()
}
