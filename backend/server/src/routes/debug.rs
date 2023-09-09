use std::sync::{Arc, OnceLock};

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::debug;
use rand::prelude::*;
use rosu_v2::prelude::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use model::tournament::{RankRestriction, TournamentFormat};
use model::*;
use crate::osu::map::get_map;

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
#[utoipa::path(
    post,
    path = "/api/test_data",
    responses(
        (status = CREATED, description = "Successfully created a test tournament")
    )
)]
pub async fn fill_test_data(State(ref db): State<DatabaseConnection>) -> StatusCode {
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
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(tournament_name),
        shorthand: ActiveValue::Set(shorthand),
        format: ActiveValue::Set(*FORMATS.choose(&mut rng).unwrap()),
        rank_range: ActiveValue::Set(rank_ranges.choose(&mut rng).unwrap().clone()),
        bws: ActiveValue::Set(rng.gen()),
    };

    let tournament = tournament.insert(db).await.unwrap();

    // We only add country restrictions or rank ranges sometimes
    if rng.gen_bool(0.5) {
        for country in restriction {
            let restriction = country_restriction::ActiveModel {
                tournament_id: ActiveValue::Set(tournament.id),
                name: ActiveValue::Set(country.to_string()),
            };

            restriction.insert(db).await.unwrap();
        }
    }

    // For each stage, we create a record and add some maps to its pool
    for (stage_order, &stage_name) in STAGES.iter().enumerate() {
        // Insert the stage
        let stage = stage::ActiveModel {
            name: ActiveValue::Set(stage_name.to_string()),
            tournament_id: ActiveValue::Set(tournament.id),
            stage_order: ActiveValue::Set(stage_order as i16),
            best_of: ActiveValue::Set(rng.gen_range(3..6) * 2 + 1),
        };

        let _stage = stage.insert(db).await.unwrap();

        // Add a few maps to the stage's pool
        for (bracket_order, &bracket_name) in BRACKETS.iter().enumerate() {
            // insert the pool bracket
            let _bracket = pool_bracket::ActiveModel {
                name: ActiveValue::Set(bracket_name.to_string()),
                tournament_id: ActiveValue::Set(tournament.id),
                stage_order: ActiveValue::Set(stage_order as i16),
                bracket_order: ActiveValue::Set(bracket_order as i16),
            }
            .insert(db)
            .await
            .unwrap();

            for map_order in 0..2 {
                // Choose a random map
                let choice = *MAP_IDS.choose(&mut rng).unwrap();
                // insert the map into the bracket
                let _map = pool_map::ActiveModel {
                    tournament_id: ActiveValue::Set(tournament.id),
                    stage_order: ActiveValue::Set(stage_order as i16),
                    bracket_order: ActiveValue::Set(bracket_order as i16),
                    map_order: ActiveValue::Set(map_order),
                    map_id: ActiveValue::Set(choice as i64),
                }
                .insert(db)
                .await
                .unwrap();
            }
        }
    }
    StatusCode::CREATED
}

/// Requests a test beatmap from the osu api.
#[utoipa::path(
    get,
    path = "/api/beatmap",
    responses(
        (status = 200, description = "Successfuly requested beatmap")
    )
)]
pub async fn get_beatmap(State(osu): State<Arc<Osu>>) -> Json<Vec<BeatmapCompact>> {
    Json(get_map(osu, 2088253).await)
}
