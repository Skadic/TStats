use std::borrow::BorrowMut;
use std::error::Error;

use rosu_v2::prelude::*;
use serde::{Deserialize, Serialize};

use crate::cache::{get_cached_or, CacheError, CacheResult, Cacheable};

use super::user::OsuUser;

/// A slimmer Beatmap struct
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SlimBeatmap {
    pub artist_name: String,
    pub name: String,
    pub diff_name: String,
    pub set_id: u32,
    pub map_id: u32,
    pub creator: OsuUser,
    pub difficulty: Difficulty,
}

/// Contains information about a beatmap's difficulty values
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub stars: f32,
    pub length: u32,
    pub bpm: f32,
    pub cs: f32,
    pub ar: f32,
    pub od: f32,
    pub hp: f32,
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

impl From<SlimBeatmap> for proto::osu::Beatmap {
    fn from(value: SlimBeatmap) -> Self {
        let Difficulty {
            stars,
            length,
            bpm,
            cs,
            ar,
            od,
            hp,
        } = value.difficulty;

        proto::osu::Beatmap {
            artist_name: value.artist_name,
            name: value.name,
            difficulty_name: value.diff_name,
            mapset_id: value.set_id,
            map_id: value.map_id as u64,
            creator: Some(proto::osu::User {
                user_id: value.creator.user_id,
                username: value.creator.username.into_string(),
                country: value.creator.country.into_string(),
                cover_url: value.creator.cover_url,
            }),
            difficulty: Some(proto::osu::Difficulty {
                stars,
                length,
                bpm,
                cs,
                ar,
                od,
                hp,
            }),
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TryFromSlimBeatmapError {
    #[error("missing difficulty")]
    MissingDifficulty,
    #[error("missing creator")]
    MissingCreator,
    #[error("username too long \"{0}\"")]
    UsernameTooLong(String),
    #[error("country code too long \"{0}\"")]
    CountryTooLong(String),
}

impl TryFrom<proto::osu::Beatmap> for SlimBeatmap {
    type Error = TryFromSlimBeatmapError;

    fn try_from(value: proto::osu::Beatmap) -> Result<Self, Self::Error> {
        let Some(proto::osu::Difficulty {
            stars,
            length,
            bpm,
            cs,
            ar,
            od,
            hp,
        }) = value.difficulty
        else {
            return Err(TryFromSlimBeatmapError::MissingDifficulty);
        };
        let Some(proto::osu::User {
            user_id,
            username,
            country,
            cover_url,
        }) = value.creator
        else {
            return Err(TryFromSlimBeatmapError::MissingCreator);
        };

        if username.len() > 15 {
            return Err(TryFromSlimBeatmapError::UsernameTooLong(username));
        }

        if country.len() > 2 {
            return Err(TryFromSlimBeatmapError::CountryTooLong(username));
        }

        Ok(SlimBeatmap {
            artist_name: value.artist_name,
            name: value.name,
            diff_name: value.difficulty_name,
            set_id: value.mapset_id,
            map_id: value.map_id as u32,
            creator: OsuUser {
                user_id,
                username: SmallString::from_str(username.as_str()),
                country: SmallString::from_str(country.as_str()),
                cover_url,
            },
            difficulty: Difficulty {
                stars,
                length,
                bpm,
                cs,
                ar,
                od,
                hp,
            },
        })
    }
}

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
    mut redis: redis::aio::MultiplexedConnection,
    osu: &Osu,
    map_id: u32,
) -> CacheResult<SlimBeatmap> {
    let redis = redis.borrow_mut();
    // Find the map's data
    let map = get_cached_or::<SlimBeatmap, CacheError, _,  _>(
        &mut redis.clone(),
        &map_id,
        Some(3600),
        || async {
            // If it doesn't exist in the cache, request it from the osu api
            let mapset = osu
                .beatmapset_from_map_id(map_id)
                .await
                .map_err(|e| -> Box<dyn Error + Send + Sync> { Box::new(e) })
                .map_err(CacheError::Request)?;
            let maps = mapset.maps.as_ref().unwrap();
            // beatmapset_from_map_id guarantees that maps has entries and we also know that the map with the given id exists
            let map = maps.iter().find(|map| map.map_id == map_id).unwrap();
            let creator = get_cached_or::<OsuUser, OsuError, _, _>(
                redis,
                &map.creator_id,
                Some(3600),
                || async { osu.user(map.creator_id).await.map(OsuUser::from) },
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
