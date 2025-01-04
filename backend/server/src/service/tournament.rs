use std::sync::Arc;

use futures::future::OptionFuture;
use futures::FutureExt;
use futures::StreamExt;
use futures::TryStreamExt;
use itertools::izip;
use miette::Context;
use miette::IntoDiagnostic;
use model::db::country_restriction;
use model::db::prelude::*;
use model::db::rank_restriction;
use model::db::tournament;
use model::dto::tournament::TournamentDto;
use sea_orm::{DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait, QueryOrder};
use utils::TStatsPaths;

#[derive(Clone)]
pub struct TournamentService {
    db: DatabaseConnection,
    paths: TStatsPaths,
}

impl TournamentService {
    pub fn new(db: DatabaseConnection, paths: TStatsPaths) -> Arc<Self> {
        Arc::new(Self { db, paths })
    }

    /// Fetch all tournaments.
    ///
    /// # Errors
    ///
    /// This function will return an internal server error if there was an error communicating with the database,
    /// or the tournament does not exist.
    pub async fn get_all(&self) -> miette::Result<Vec<TournamentDto>> {
        let db = &self.db;
        let tournaments: Vec<tournament::Model> = Tournament::find()
            .all(db)
            .await
            .into_diagnostic()
            .wrap_err("failed to get tournaments")?;

        // Get rank restrictions
        let rank_restrictions = tournaments
            .load_many(
                RankRestriction::find().order_by_asc(rank_restriction::Column::Tier),
                db,
            )
            .map(|res| {
                res.into_diagnostic()
                    .wrap_err("failed to get rank restrictions")
            });

        // Get country restrictions
        let country_restrictions = tournaments
            .load_many(
                CountryRestriction::find().order_by_asc(country_restriction::Column::CountryCode),
                db,
            )
            .map(|res| {
                res.into_diagnostic()
                    .wrap_err("failed to get country restrictions")
            });

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions) =
            tokio::try_join!(rank_restrictions, country_restrictions)?;

        Ok(
            futures::stream::iter(izip!(tournaments, rank_restrictions, country_restrictions,))
                .then(
                    |(tournament, rank_restriction, country_restriction)| async move {
                        let rank_restrictions = rank_restriction.iter().map(Into::into).collect();
                        let country_restrictions =
                            country_restriction.iter().map(Into::into).collect();
                        let banner = self.read_banner(tournament.banner).await?;

                        Ok::<_, miette::Error>(TournamentDto {
                            id: tournament.id,
                            name: tournament.name,
                            shorthand: tournament.shorthand,
                            format: tournament.format as u32,
                            bws: tournament.bws,
                            mode: tournament.mode.into(),
                            banner,
                            start_date: tournament.start_date.map(Into::into),
                            end_date: tournament.end_date.map(Into::into),
                            rank_restrictions,
                            country_restrictions,
                        })
                    },
                )
                .try_collect::<Vec<_>>()
                .await?,
        )
    }

    /// Fetch a single Tournament by its id.
    ///
    /// # Errors
    ///
    /// This function will return an internal server error if there was an error communicating with the database,
    /// or the tournament does not exist.
    pub async fn get_by_id(&self, tournament_id: i32) -> miette::Result<Option<TournamentDto>> {
        let db = &self.db;
        let Some(tournament) = Tournament::find_by_id(tournament_id)
            .one(db)
            .await
            .into_diagnostic()
            .wrap_err("could not load tournament")?
        else {
            return Ok(None);
        };

        // Get rank restrictions
        let rank_restrictions = tournament
            .find_related(RankRestriction)
            .order_by_asc(rank_restriction::Column::Tier)
            .all(db)
            .map(|res| {
                res.into_diagnostic()
                    .wrap_err("failed to get rank restrictions")
            });

        // Get country restrictions
        let country_restrictions = tournament
            .find_related(CountryRestriction)
            .order_by_asc(country_restriction::Column::CountryCode)
            .all(db)
            .map(|res| {
                res.into_diagnostic()
                    .wrap_err("failed to get country restrictions")
            });

        let banner = self.read_banner(tournament.banner);

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions, banner) =
            tokio::try_join!(rank_restrictions, country_restrictions, banner)?;

        Ok(Some(TournamentDto {
            id: tournament.id,
            name: tournament.name,
            shorthand: tournament.shorthand,
            format: tournament.format as u32,
            bws: tournament.bws,
            mode: tournament.mode.into(),
            banner,
            start_date: tournament.start_date.map(Into::into),
            end_date: tournament.end_date.map(Into::into),
            rank_restrictions: rank_restrictions.into_iter().map(Into::into).collect(),
            country_restrictions: country_restrictions.into_iter().map(Into::into).collect(),
        }))
    }

    async fn read_banner(&self, banner_name: Option<String>) -> miette::Result<Option<Vec<u8>>> {
        OptionFuture::from(
            banner_name.map(|banner_name| tokio::fs::read(self.paths.banner(&banner_name))),
        )
        .map(|v| {
            v.transpose()
                .into_diagnostic()
                .wrap_err("could not read banner file")
        })
        .await
    }
}
