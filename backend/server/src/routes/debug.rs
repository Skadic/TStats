use futures::future::join_all;
use futures::future::FutureExt;
use model::tournament::Mode;
use rand::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue};
use tonic::{Request, Response, Status};
use tracing::debug;

use model::*;
use proto::debug_data::debug_service_server::DebugService;

use crate::osu::map::SlimBeatmap;
use crate::AppState;

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

static RANK_RANGES: [(i32, i32); 3] = [(100, 2500), (2500, 5000), (5000, 10000)];

const FORMATS: [i32; 4] = [1, 2, 3, 4];

const MAP_IDS: [usize; 9] = [
    3883456, 4192228, 4189337, 3917025, 4141288, 4186607, 3876751, 4130092, 4149939,
];

pub struct DebugServiceImpl(pub AppState);

#[tonic::async_trait]
impl DebugService for DebugServiceImpl {
    async fn fill_data(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        use ActiveValue as A;
        let db = &self.0.db;

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
            //rank_range: A::Set(rank_ranges.choose(&mut rng).unwrap().clone()),
            bws: A::Set(rng.gen()),
            mode: A::Set(tournament::Mode::Osu),
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

        for i in 0..rng.gen_range(0..4) {
            let rank_restriction = rank_restriction::ActiveModel {
                tournament_id: A::Set(tournament.id),
                tier: A::Set(i),
                min: A::Set(RANK_RANGES[i as usize].0),
                max: A::Set(RANK_RANGES[i as usize].1),
            };

            rank_restriction.insert(db).await.unwrap();
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

    async fn owc23(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        use ActiveValue as A;

        let db = &self.0.db;

        let owc23 = tournament::ActiveModel {
            id: A::NotSet,
            name: A::Set("Osu World Cup 2023".to_owned()),
            shorthand: A::Set("OWC23".to_owned()),
            format: A::Set(4),
            bws: A::Set(false),
            mode: A::Set(Mode::Osu),
        }
        .insert(db)
        .await
        .unwrap();

        let add_team_member = |id, player_id| async move {
            team_member::ActiveModel {
                team_id: A::Set(id),
                user_id: A::Set(player_id),
            }
            .insert(db)
            .await
            .unwrap();
        };

        let team_germany = team::ActiveModel {
            id: A::NotSet,
            tournament_id: A::Set(owc23.id),
            name: A::Set("Germany".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        add_team_member(team_germany.id, 8116659).await;
        add_team_member(team_germany.id, 4504101).await;
        add_team_member(team_germany.id, 3765989).await;
        add_team_member(team_germany.id, 14385814).await;
        add_team_member(team_germany.id, 13300203).await;
        add_team_member(team_germany.id, 12952320).await;
        add_team_member(team_germany.id, 11921197).await;

        let team_usa = team::ActiveModel {
            id: A::NotSet,
            tournament_id: A::Set(owc23.id),
            name: A::Set("USA".to_string()),
        }
        .insert(db)
        .await
        .unwrap();

        add_team_member(team_usa.id, 7075211).await;
        add_team_member(team_usa.id, 7813296).await;
        add_team_member(team_usa.id, 4108547).await;
        add_team_member(team_usa.id, 2590257).await;
        add_team_member(team_usa.id, 4787150).await;
        add_team_member(team_usa.id, 13380270).await;
        add_team_member(team_usa.id, 3533958).await;
        add_team_member(team_usa.id, 4830687).await;

        macro_rules! pool {
            {$pool:ident, $bracket:ident => $($maps:literal),+} => {
                [$($maps),+].iter().copied().enumerate().map(|(i, map_id)| {
                    pool_map::ActiveModel {
                        tournament_id: A::Set(owc23.id),
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

        {
            let qualis = stage::ActiveModel {
                tournament_id: A::Set(owc23.id),
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
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(qualis.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let _res = join_all(pool! { qualis,
                nm => 4344435, 4344451, 4344441, 4344442;
                hd => 4344469, 4344423;
                hr => 4344412, 4344450;
                dt => 4344474, 4344475, 4344422
            })
            .await;
        }
        {
            let ro32 = stage::ActiveModel {
                tournament_id: A::Set(owc23.id),
                name: A::Set("RO32".to_owned()),
                best_of: A::Set(9),
                stage_order: A::Set(1),
            }
            .insert(db)
            .await
            .unwrap();

            let nm = pool_bracket::ActiveModel {
                bracket_order: A::Set(0),
                name: A::Set("NM".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let fm = pool_bracket::ActiveModel {
                bracket_order: A::Set(4),
                name: A::Set("FM".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let tb = pool_bracket::ActiveModel {
                bracket_order: A::Set(5),
                name: A::Set("TB".to_owned()),
                tournament_id: A::Set(owc23.id),
                stage_order: A::Set(ro32.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let _res = join_all(pool! { ro32,
                nm => 4352819, 4352824,4351786,3332588;
                hd => 4352411,4352324;
                hr => 1414172,2020374;
                dt => 4352790,3840580,2149694;
                fm => 2583501,4351866,4352856;
                tb => 3121101
            })
            .await;
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
            format: A::Set(1),
            bws: A::Set(false),
            mode: A::Set(Mode::Osu),
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

            let _res = join_all(pool! { qualis,
                nm => 2230996, 3263098, 2593243, 3142496, 3129534;
                hd => 3544219, 2588430;
                hr => 2314568, 434438;
                dt => 429797, 3153512
            })
            .await;
        }

        // RO64
        {
            let ro64 = stage::ActiveModel {
                tournament_id: A::Set(dm8.id),
                name: A::Set("RO64".to_owned()),
                best_of: A::Set(9),
                stage_order: A::Set(1),
            }
            .insert(db)
            .await
            .unwrap();

            let nm = pool_bracket::ActiveModel {
                bracket_order: A::Set(0),
                name: A::Set("NM".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(ro64.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hd = pool_bracket::ActiveModel {
                bracket_order: A::Set(1),
                name: A::Set("HD".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(ro64.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let hr = pool_bracket::ActiveModel {
                bracket_order: A::Set(2),
                name: A::Set("HR".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(ro64.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let dt = pool_bracket::ActiveModel {
                bracket_order: A::Set(3),
                name: A::Set("DT".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(ro64.stage_order),
            }
            .insert(db)
            .await
            .unwrap();
            let tb = pool_bracket::ActiveModel {
                bracket_order: A::Set(4),
                name: A::Set("TB".to_owned()),
                tournament_id: A::Set(dm8.id),
                stage_order: A::Set(ro64.stage_order),
            }
            .insert(db)
            .await
            .unwrap();

            let _res = join_all(pool! { ro64,
                nm => 3160790,3650832,2465287, 637391;
                hd => 3830079, 1957037, 2134428;
                hr => 1982100, 886269, 3167107;
                dt => 2188430, 3457575, 3541087;
                tb => 1233051, 1721284, 1295837
            })
            .await;
        }

        Ok(Response::new(()))
    }
}
