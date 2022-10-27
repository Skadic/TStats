use std::borrow::{Borrow, BorrowMut};

use redis::AsyncCommands;
use rosu_v2::{prelude::OsuError, Osu};

use crate::{error::CacheError, util::get_cached_opt, DBPool};

use super::structs::{MinimizedBeatmapset, MinimizedBeatmap};

pub(crate) async fn get_stage_id_cached(
    mut client: impl BorrowMut<redis::aio::Connection>,
    db_pool: impl Borrow<DBPool>,
    tournament_id: u32,
    stage_idx: u32,
) -> Result<Option<u32>, CacheError> {
    // Find the stage id and cache it if needs to be retrieved from the database
    let cache_key = format!("stage:{tournament_id}:{stage_idx}:id");
    get_cached_opt(client.borrow_mut(), &cache_key, || async {
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
        .fetch_optional(db_pool.borrow())
        .await
        .map(|opt| opt.map(|record| record.id as u32))
    })
    .await
}

pub(crate) async fn get_mapset_cached(
    mut client: impl BorrowMut<redis::aio::Connection>,
    osu: impl Borrow<Osu>,
    mapset_id: u32,
) -> Result<Option<MinimizedBeatmapset>, CacheError> {
    let mapset_key = format!("mapset:{}", mapset_id);
    crate::util::get_cached_opt(client.borrow_mut(), &mapset_key, || async {
        let res = osu
            .borrow()
            .beatmapset(mapset_id)
            .await
            .map(MinimizedBeatmapset::from)
            .map(Some);
        match res {
            Err(OsuError::NotFound) => Ok(None),
            _ => res,
        }
    })
    .await
}

/*
pub(crate) async fn get_maps_cached(
    mut client: impl BorrowMut<redis::aio::Connection>,
    osu: impl Borrow<Osu>,
    map_ids: &[u32],
) -> Result<Vec<Option<MinimizedBeatmap>>, CacheError> {
    let mut maps: Vec<Option<MinimizedBeatmap>> = vec![None; map_ids.len()];
    let mut to_request = Vec::with_capacity(map_ids.len() / 2);

    for (i, &map_id) in map_ids.into_iter().enumerate() {
        let map_key = format!("map:{}", map_id);
        // Try to find the value in the cache
        let val = if let Ok(cached) = client.borrow_mut().get::<_, String>(&map_key).await {
            // If found, deserialize and return it
            maps[i] = Some(serde_json::from_str(&cached)?);
        } else {
            to_request.push(map_id);
        };
    }

    let requested = osu.borrow().beatmaps(to_request).await.unwrap().into_iter().map(MinimizedBeatmap::from);

    for opt in maps.iter_mut().filter(|v| v.is_none()) {
        *opt = requested.next();
    }

    Ok(vec![])
}
*/
