use crate::AppState;
use itertools::izip;
use miette::miette;
use model::db::prelude::*;
use model::db::{country_restriction, rank_restriction, stage, tournament};
use model::dto::tournament::TournamentDto;
use poem::error::NotFoundError;
use poem::session::Session;
use poem::web::Path;
use poem_openapi::{payload::Json, OpenApi};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait, QueryFilter, QueryOrder,
};
use utils::{LogPoemError, LogPoemErrorFuture};

pub struct TournamentApi(pub AppState);

async fn find_stage(
    tournament_id: i32,
    stage_order: i32,
    db: &DatabaseConnection,
) -> miette::Result<(tournament::Model, stage::Model)> {
    let res = Tournament::find_by_id(tournament_id)
        .find_also_related(Stage)
        .filter(stage::Column::StageOrder.eq(stage_order))
        .one(db)
        .await
        .map_err(|e| miette!("error fetching tournament: {e}"))?;

    // Test if the tournament and stage exist
    let (tournament, stage) = match res {
        Some((tournament, Some(stage))) => (tournament, stage),
        Some((_, None)) => {
            return Err(miette!(
                "stage {stage_order} in tournament {tournament_id} does not exist",
            ))
        }
        None => {
            return Err(miette!(
                "tournament with id {} does not exist",
                tournament_id
            ))
        }
    };

    Ok((tournament, stage))
}

#[OpenApi(prefix_path = "/tournaments")]
impl TournamentApi {
    #[oai(path = "/", method = "get")]
    #[tracing::instrument(skip_all)]
    async fn get_all(&self, session: &Session) -> poem::Result<Json<Vec<TournamentDto>>> {
        session.set("my_msg", "HALLO");
        let db = &self.0.db;
        let tournaments: Vec<tournament::Model> = Tournament::find()
            .all(db)
            .log_internal_server_error("failed to get tournaments")
            .await?;

        // Get rank restrictions
        let rank_restrictions = tournaments
            .load_many(
                RankRestriction::find().order_by_asc(rank_restriction::Column::Tier),
                db,
            )
            .log_internal_server_error("failed to get rank restrictions");

        // Get country restrictions
        let country_restrictions = tournaments
            .load_many(
                CountryRestriction::find().order_by_asc(country_restriction::Column::CountryCode),
                db,
            )
            .log_internal_server_error("failed to get country restrictions");

        let banners = tournaments
            .iter()
            .map(|tournament| match &tournament.banner {
                Some(banner_name) => {
                    return std::fs::read(self.0.paths.banner(banner_name)).map(Some);
                }
                None => Ok(None),
            })
            .collect::<Result<Vec<_>, _>>()
            .log_internal_server_error("could not read banner image")?;

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions) =
            tokio::try_join!(rank_restrictions, country_restrictions)
                .log_internal_server_error("failed to get country_restrictions")?;

        let iter = izip!(
            tournaments,
            rank_restrictions,
            country_restrictions,
            banners,
        )
        .map(
            |(tournament, rank_restriction, country_restriction, banner)| {
                let rank_restrictions = rank_restriction.iter().map(Into::into).collect();
                let country_restrictions = country_restriction.iter().map(Into::into).collect();

                TournamentDto {
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
                }
            },
        );
        Ok(Json(iter.collect()))
    }

    #[oai(path = "/:id", method = "get")]
    #[tracing::instrument(skip_all)]
    async fn get(&self, Path(id): Path<i32>) -> poem::Result<Json<TournamentDto>> {
        let db = &self.0.db;
        let tournament: tournament::Model = Tournament::find_by_id(id as i32)
            .one(db)
            .log_internal_server_error("could not load tournament")
            .await?
            .ok_or(NotFoundError)?;

        // Get rank restrictions
        let rank_restrictions = tournament
            .find_related(RankRestriction)
            .order_by_asc(rank_restriction::Column::Tier)
            .all(db)
            .log_internal_server_error("failed to get rank restrictions");

        // Get country restrictions
        let country_restrictions = tournament
            .find_related(CountryRestriction)
            .order_by_asc(country_restriction::Column::CountryCode)
            .all(db)
            .log_internal_server_error("failed to get country restrictions");

        let banner = match &tournament.banner {
            Some(banner_name) => Some(
                std::fs::read(self.0.paths.banner(banner_name))
                    .log_internal_server_error("could not read banner image")?,
            ),
            None => None,
        };

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions) =
            tokio::try_join!(rank_restrictions, country_restrictions)
                .log_internal_server_error("failed to get country_restrictions")?;

        Ok(Json(TournamentDto {
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
}

mod test {

    #[sqlx::test]
    async fn test_basic() {}
}
