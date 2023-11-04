pub mod tournaments {
    use model::tournament::{Model, TournamentFormat};

    use crate::tournaments::format::FormatType;

    tonic::include_proto!("tournaments");

    impl From<Model> for Tournament {
        fn from(value: Model) -> Self {
            Self {
                id: Some(value.id),
                name: value.name,
                shorthand: value.shorthand,
                format: Some(value.format.into()),
                bws: value.bws,
                rank_restrictions: Vec::<model::tournament::RankRange>::from(value.rank_range)
                    .into_iter()
                    .map(|rr| RankRange {
                        min: rr.min as u32,
                        max: rr.max as u32,
                    })
                    .collect(),
                country_restrictions: vec![],
                stages: vec![],
            }
        }
    }

    impl TryFrom<Tournament> for Model {
        type Error = prost::DecodeError;

        fn try_from(value: Tournament) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id(),
                name: value.name,
                shorthand: value.shorthand,
                format: TournamentFormat::try_from(value.format.unwrap())?,
                bws: value.bws,
                rank_range: value
                    .rank_restrictions
                    .into_iter()
                    .map(RankRange::into)
                    .collect::<Vec<_>>()
                    .into(),
            })
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

    impl TryFrom<Format> for TournamentFormat {
        type Error = prost::DecodeError;

        fn try_from(value: Format) -> Result<Self, Self::Error> {
            match FormatType::try_from(value.format_type)? {
                FormatType::Versus => Ok(Self::Versus(value.players as usize)),
                FormatType::BattleRoyale => Ok(Self::BattleRoyale(value.players as usize)),
            }
        }
    }

    impl From<TournamentFormat> for Format {
        fn from(value: TournamentFormat) -> Self {
            match value {
                TournamentFormat::Versus(players) => Self {
                    format_type: FormatType::Versus as i32,
                    players: players as u32,
                },
                TournamentFormat::BattleRoyale(players) => Self {
                    format_type: FormatType::BattleRoyale as i32,
                    players: players as u32,
                },
            }
        }
    }
}

pub mod stages {
    use model::stage::Model;

    tonic::include_proto!("stages");

    impl From<model::stage::Model> for Stage {
        fn from(value: Model) -> Self {
            Stage {}
        }
    }
}
pub mod debug_data {
    tonic::include_proto!("debug");
}
