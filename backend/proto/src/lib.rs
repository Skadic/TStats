pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tstats_descriptor");

pub mod tournaments {
    use model::tournament::Model;

    tonic::include_proto!("tournaments");

    impl From<Model> for Tournament {
        fn from(value: Model) -> Self {
            Self {
                id: Some(value.id),
                name: value.name,
                shorthand: value.shorthand,
                format: value.format as u32,
                bws: value.bws,
                rank_restrictions: vec![],
                country_restrictions: vec![],
                stages: vec![],
            }
        }
    }

    impl From<Tournament> for Model {
        fn from(value: Tournament) -> Self {
            Self {
                id: value.id(),
                name: value.name,
                shorthand: value.shorthand,
                format: value.format as i32,
                bws: value.bws,
            }
        }
    }

    impl From<model::tournament::RankRange> for RankRange {
        fn from(value: model::tournament::RankRange) -> Self {
            Self {
                min: value.min as u32,
                max: value.max as u32,
            }
        }
    }

    impl From<RankRange> for model::tournament::RankRange {
        fn from(value: RankRange) -> Self {
            Self {
                min: value.min as usize,
                max: value.max as usize,
            }
        }
    }
}

pub mod stages {
    use model::stage::Model;

    tonic::include_proto!("stages");

    impl From<model::stage::Model> for Stage {
        fn from(value: Model) -> Self {
            Stage {
                tournament_id: value.tournament_id,
                stage_order: value.stage_order as i32,
                name: value.name,
                best_of: value.best_of as i32,
                pool_brackets: vec![],
            }
        }
    }

    impl From<Stage> for model::stage::Model {
        fn from(value: Stage) -> Self {
            Model {
                tournament_id: value.tournament_id,
                stage_order: value.stage_order as i16,
                name: value.name,
                best_of: value.best_of as i16,
            }
        }
    }
}

pub mod pool_brackets {
    tonic::include_proto!("pool_brackets");
}
pub mod debug_data {
    tonic::include_proto!("debug");
}
