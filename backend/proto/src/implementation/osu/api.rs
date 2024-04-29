use deadpool_redis::Pool as RedisConnectionPool;
use miette::IntoDiagnostic;
use rosu_v2::{error::OsuError, Osu};
use utils::{cache::{CacheError, CacheResult}, Cacheable};


/// Gets information about a map from the osu API.
///
/// # Panics
///
/// Panics if the map has no beatmapset.
///
/// # Errors
///
/// This function will return an error if something goes wrong during cacheing or communicating with the osu api.
pub async fn get_map(
    redis: &RedisConnectionPool,
    osu: &Osu,
    map_id: u32,
) -> CacheResult<crate::osu::Beatmap> {
    // Find the map's data
    let map =
        crate::osu::Beatmap::get_cached_or::<CacheError, _>(redis, &map_id, Some(3600), || async {
            // If it doesn't exist in the cache, request it from the osu api
            let mapset = osu
                .beatmapset_from_map_id(map_id)
                .await
                .into_diagnostic()
                .map_err(CacheError::Request)?;
            let maps = mapset.maps.as_ref().unwrap();
            // beatmapset_from_map_id guarantees that maps has entries and we also know that the map with the given id exists
            let map = maps.iter().find(|map| map.map_id == map_id).unwrap();
            let creator = crate::osu::User::get_cached_or::<OsuError, _>(
                redis,
                &map.creator_id,
                Some(3600),
                || async { osu.user(map.creator_id).await.map(crate::osu::User::from) },
            )
            .await?;

            Ok(crate::osu::Beatmap::from_map_set_and_creator(
                map, &mapset, &creator,
            ))
        })
        .await?;

    Ok(map)
}

pub async fn get_user(
    redis: &RedisConnectionPool,
    osu: &Osu,
    user_id: u32,
) -> CacheResult<crate::osu::User> {
    crate::osu::User::get_cached_or::<OsuError, _>(redis, &user_id, Some(60), || async {
        let usr = osu.user(user_id).await?;
        Ok(usr.into())
    })
    .await
}