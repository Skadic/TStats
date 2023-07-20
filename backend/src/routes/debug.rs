use std::sync::OnceLock;

use axum::{extract::State, http::StatusCode};
use log::{debug, warn};
use rand::{
    prelude::{SliceRandom, StdRng},
    Rng, SeedableRng,
};
use surrealdb::{engine::remote::ws::Client, Response, Surreal};

use crate::model::{
    map::PoolMap,
    relations::{is_stage::IsStage, pool_contains::PoolContains},
    stage::Stage,
    tournament::{RankRange, Tournament, TournamentBuilder, TournamentFormat},
    TableType,
};
use crate::Record;

// These three tables are for generating a random tournament name.
const MODIFIER_1: [&str; 5] = ["Amazing", "Mysterious", "Incredible", "Osu", "Great"];
const MODIFIER_2: [&str; 8] = [
    "Newcomer", "Spring", "Fall", "World", "European", "Map", "Anime", "Waifu",
];
const TOURNAMENT: [&str; 5] = ["Cup", "Tournament", "Brawl", "Festival", "Showdown"];

const COUNTRIES: [&str; 15] = [
    "GE", "FR", "IT", "ES", "UK", "US", "CA", "RU", "JP", "CN", "KR", "AU", "NZ", "BR", "AR",
];

const STAGES: [&str; 6] = ["Q", "RO16", "QF", "SF", "F", "GF"];

static RANK_RANGES: OnceLock<[RankRange; 6]> = OnceLock::new();

const FORMATS: [TournamentFormat; 4] = [
    TournamentFormat::versus(1),
    TournamentFormat::versus(2),
    TournamentFormat::versus(4),
    TournamentFormat::battle_royale(10),
];

const MAP_IDS: [usize; 6] = [3883456, 4192228, 4189337, 3917025, 4141288, 4186607];

/// Fills the database with test data including a tournament, a few stages, maps for its pools.
pub async fn fill_test_data(State(db): State<Surreal<Client>>) -> StatusCode {
    let rank_ranges = RANK_RANGES.get_or_init(|| {
        [
            RankRange::single(50..1000),
            RankRange::single(1500..5000),
            RankRange::single(10000..100000),
            RankRange::tiered([1000..1500, 1500..5000, 5000..10000]),
            RankRange::tiered([1..750, 750..2000]),
            RankRange::tiered([100..1000, 1000..10000, 10000..100000]),
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
    let mut builder = TournamentBuilder::new(
        tournament_name,
        shorthand,
        FORMATS[rng.gen_range(0..FORMATS.len())],
        rng.gen(),
    );

    // We only add country restrictions or rank ranges sometimes
    if rng.gen_bool(0.5) {
        builder = builder.country_restriction(restriction);
    }
    if rng.gen_bool(0.75) {
        builder = builder.with_rank_range(rank_ranges.choose(&mut rng).unwrap().clone());
    }

    debug!("Inserting test data into database");
    let tournament: Record = db
        .create(Tournament::table_name())
        .content(builder.build())
        .await
        .unwrap();

    // For each stage, we create a record and add some maps to its pool
    for (stage_order, &stage_name) in STAGES.iter().enumerate() {
        // Insert the stage
        let stage: Record = db
            .create(Stage::table_name())
            .content(Stage::new(stage_name, stage_order, ["NM", "HD", "HR"]))
            .await
            .unwrap();

        // Generate the connection between the tournament and the stage
        IsStage::relate(&db, &tournament.id, &stage.id).await.unwrap();
        /*
        let _: Record = db
            .create(IsStage::table_name())
            .content(IsStage::new(&tournament.id, &stage.id))
            .await
            .unwrap();
         */
        // Add a few maps to the stage's pool
        for bracket_order in 0..3 {
            // Choose a random map
            let choice = MAP_IDS.choose(&mut rng).unwrap().to_string();

            // We try to insert the map into the database, and if it already exists, we just get it
            let map: PoolMap = match db.update((PoolMap::table_name(), &choice)).await {
                Ok(map) => map,
                Err(e) => {
                    dbg!(&e);
                    warn!("error updating map: {choice}, {e}");
                    continue;
                }
            };

            // Generate the connection between the stage and the map
            PoolContains::relate(&db, &stage.id, map.database_id().unwrap(), "NM", bracket_order)
                .await
                .unwrap();
            /*
            let _: Record = db
                .create(PoolContains::table_name())
                .content(PoolContains::new(
                    &stage.id,
                    map.database_id().unwrap(),
                    "NM",
                    bracket_order,
                ))
                .await
                .unwrap();

             */
        }
    }
    StatusCode::OK
}
