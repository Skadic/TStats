use std::sync::OnceLock;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use futures::future::join_all;
use futures::future::FutureExt;
use rand::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

use model::tournament::{RankRestriction, TournamentFormat};
use model::*;
use proto::debug_data::debug_service_server::DebugService;

use crate::osu::map::{get_map, SlimBeatmap};
use crate::osu::user::OsuUser;
use crate::{AppState, LocalAppState};

// These three tables are for generating a random tournament name.
const MODIFIER_1: [&str; 5] = ["Amazing", "Mysterious", "Incredible", "Osu", "Great"];
const MODIFIER_2: [&str; 8] = [
    "Newcomer", "Spring", "Fall", "World", "European", "Map", "Anime", "Waifu",
];
const TOURNAMENT: [&str; 5] = ["Cup", "Tournament", "Brawl", "Festival", "Showdown"];

const COUNTRIES: [&str; 15] = [
    "GE", "FR", "IT", "ES", "UK", "US", "CA", "RU", "JP", "CN", "KR", "AU", "NZ", "BR", "AR",
];

const BRACKETS: [&str; 3] = ["NM", "HD", "HR"];

const STAGES: [&str; 6] = ["Q", "RO16", "QF", "SF", "F", "GF"];

static RANK_RANGES: OnceLock<[RankRestriction; 9]> = OnceLock::new();

const FORMATS: [TournamentFormat; 4] = [
    TournamentFormat::versus(1),
    TournamentFormat::versus(2),
    TournamentFormat::versus(4),
    TournamentFormat::battle_royale(10),
];

const MAP_IDS: [usize; 9] = [
    3883456, 4192228, 4189337, 3917025, 4141288, 4186607, 3876751, 4130092, 4149939,
];

/// Fills the database with test data including a tournament, a few stages, maps for its pools.

/// Requests a test beatmap from the osu api.
#[utoipa::path(
    get,
    path = "/api/beatmap",
    responses(
        (status = 200, description = "Successfuly requested beatmap")
    )
)]
pub async fn get_beatmap(State(mut state): State<AppState>) -> Json<SlimBeatmap> {
    Json(
        get_map(&mut state.redis, &state.osu, 2088253)
            .await
            .unwrap(),
    )
}

pub async fn get_user(State(mut state): State<AppState>) -> Json<OsuUser> {
    Json(crate::osu::user::get_user(&mut state.redis, &state.osu, 1235015).await)
}
pub struct DebugServiceImpl(pub LocalAppState);

#[tonic::async_trait]
impl DebugService for DebugServiceImpl {
    async fn fill_data(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        use ActiveValue as A;
        let db = &self.0.db;
        let rank_ranges = RANK_RANGES.get_or_init(|| {
            [
                RankRestriction::OpenRank,
                RankRestriction::OpenRank,
                RankRestriction::OpenRank,
                RankRestriction::single(50..1000),
                RankRestriction::single(1500..5000),
                RankRestriction::single(10000..100000),
                RankRestriction::tiered([1000..1500, 1500..5000, 5000..10000]),
                RankRestriction::tiered([1..750, 750..2000]),
                RankRestriction::tiered([100..1000, 1000..10000, 10000..100000]),
            ]
        });

        // The following section generates a tournament with a random name, format, and country
        let mut rng = StdRng::from_entropy();
        let tournament_name = format!(
            "{} {} {} {}",
            MODIFIER_1.choose(&mut rng).unwrap(),
            MODIFIER_2.choose(&mut rng).unwrap(),
            TOURNAMENT.choose(&mut rng).unwrap(),
            rng.gen_range(0..10)
        );
        let shorthand = tournament_name
            .split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect::<String>();

        let num_restrictions = rng.gen_range(1..5);
        // a vector of random length containing multiple country codes
        let restriction = COUNTRIES
            .choose_multiple(&mut rng, num_restrictions)
            .copied()
            .collect::<Vec<&str>>();

        debug!("Inserting test data into database");
        let tournament = tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set(tournament_name),
            shorthand: A::Set(shorthand),
            format: A::Set(*FORMATS.choose(&mut rng).unwrap()),
            rank_range: A::Set(rank_ranges.choose(&mut rng).unwrap().clone()),
            bws: A::Set(rng.gen()),
        };

        let tournament = tournament.insert(db).await.unwrap();

        // We only add country restrictions or rank ranges sometimes
        if rng.gen_bool(0.5) {
            for country in restriction {
                let restriction = country_restriction::ActiveModel {
                    tournament_id: A::Set(tournament.id),
                    name: A::Set(country.to_string()),
                };

                restriction.insert(db).await.unwrap();
            }
        }

        // For each stage, we create a record and add some maps to its pool
        for (stage_order, &stage_name) in STAGES.iter().enumerate() {
            // Insert the stage
            let stage = stage::ActiveModel {
                name: A::Set(stage_name.to_string()),
                tournament_id: A::Set(tournament.id),
                stage_order: A::Set(stage_order as i16),
                best_of: A::Set(rng.gen_range(3..6) * 2 + 1),
            };

            let _stage = stage.insert(db).await.unwrap();

            // Add a few maps to the stage's pool
            for (bracket_order, &bracket_name) in BRACKETS.iter().enumerate() {
                // insert the pool bracket
                let _bracket = pool_bracket::ActiveModel {
                    name: A::Set(bracket_name.to_string()),
                    tournament_id: A::Set(tournament.id),
                    stage_order: A::Set(stage_order as i16),
                    bracket_order: A::Set(bracket_order as i16),
                }
                .insert(db)
                .await
                .unwrap();

                for map_order in 0..2 {
                    // Choose a random map
                    let choice = *MAP_IDS.choose(&mut rng).unwrap();
                    // insert the map into the bracket
                    let _map = pool_map::ActiveModel {
                        tournament_id: A::Set(tournament.id),
                        stage_order: A::Set(stage_order as i16),
                        bracket_order: A::Set(bracket_order as i16),
                        map_order: A::Set(map_order),
                        map_id: A::Set(choice as i64),
                    }
                    .insert(db)
                    .await
                    .unwrap();
                }
            }
        }

        Ok(Response::new(()))
    }

    async fn dm8(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        use ActiveValue as A;

        let db = &self.0.db;

        let dm8 = tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set("Deutsche Meisterschaft 8".to_owned()),
            shorthand: A::Set("DM8".to_owned()),
            format: A::Set(TournamentFormat::Versus(1)),
            rank_range: A::Set(RankRestriction::OpenRank),
            bws: A::Set(false),
        }
        .insert(db)
        .await
        .unwrap();

        macro_rules! pool {
            {$pool:ident, $bracket:ident => $($maps:literal),+} => {
                [$($maps),+].iter().copied().enumerate().map(|(i, map_id)| {
                    pool_map::ActiveModel {
                        tournament_id: A::Set(dm8.id),
                        stage_order: A::Set($pool.stage_order),
                        bracket_order: A::Set($bracket.bracket_order),
                        map_id: A::Set(map_id as i64),
                        map_order: A::Set(i as i16),
                    }.insert(db).map(Result::unwrap)
                })
            };
            {$pool:ident, $bracket:ident => $($maps:literal),+; $($other_brackets:ident => $($other_maps:literal),+);+} => {
                pool!($pool, $($other_brackets => $($other_maps),+);+).chain(
                pool!($pool, $bracket => $($maps),+))
            };
        }

        // Qualis
        {
            let qualis = stage::ActiveModel {
                tournament_id: A::Set(dm8.id),
                name: A::Set("Q".to_owned()),
                best_of: A::Set(0),
                stage_order: A::Set(0),
            }
            .insert(db)
            .await
            .unwrap();

            let nm = pool_bracket::ActiveModel {
                bracket_order: A::Set(0),
                name: A::Set("NM".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let res = join_all(pool! { qualis,
                nm => 2230996, 3263098, 2593243, 3142496, 3129534;
                hd => 3544219, 2588430;
                hr => 2314568, 434438;
                dt => 429797, 3153512
            })
            .await;

            error!("{:?}", res);
        }

        Ok(Response::new(()))
    }
}
