use crate::dto::tournament::{Country, Tournament};
use crate::{
    dto::tournament::{GetAllTournamentsResponse, RankRange},
    AppState,
};
use futures::TryFutureExt;
use itertools::izip;
use miette::miette;
use model::{country_restriction, rank_restriction, stage, tournament};
use poem::error::InternalServerError;
use poem_openapi::{payload::Json, OpenApi};
use sea_orm::{
    query::*, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, ModelTrait,
};

pub struct TournamentApi(pub AppState);

async fn find_stage(
    tournament_id: i32,
    stage_order: i32,
    db: &DatabaseConnection,
) -> miette::Result<(tournament::Model, stage::Model)> {
    let res = tournament::Entity::find_by_id(tournament_id)
        .find_also_related(stage::Entity)
        .filter(stage::Column::StageOrder.eq(stage_order))
        .one(db)
        .await
        .map_err(|e| miette!("error fetching tournament: {e}"))?;

    // Test if the tournament and stage exist
    let (tournament, stage) = match res {
        Some((tournament, Some(stage))) => (tournament, stage),
        Some((_, None)) => {
            return Err(miette!(
                "stage {} in tournament {} does not exist",
                stage_order,
                tournament_id
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
    async fn get_all(&self) -> poem::Result<Json<Vec<GetAllTournamentsResponse>>> {
        let db = &self.0.db;
        let tournaments = tournament::Entity::find()
            .all(db)
            .map_err(InternalServerError)
            .await?;

        // Get rank restrictions
        let rank_restrictions = tournaments
            .load_many(
                rank_restriction::Entity::find().order_by_asc(rank_restriction::Column::Tier),
                db,
            )
            .inspect_err(|error| tracing::error!(%error, "failed to get rank restrictions"))
            .map_err(InternalServerError);

        // Get country restrictions
        let country_restrictions = tournaments
            .load_many(
                country_restriction::Entity::find()
                    .order_by_asc(country_restriction::Column::CountryCode),
                db,
            )
            .inspect_err(|error| tracing::error!(%error, "failed to get country restrictions"))
            .map_err(InternalServerError);

        // Wait for the queries and unpack them
        let (rank_restrictions, country_restrictions) =
            tokio::join!(rank_restrictions, country_restrictions);
        let (rank_restrictions, country_restrictions) = (rank_restrictions?, country_restrictions?);

        let banners = tournaments
            .iter()
            .map(|tournament| match &tournament.banner {
                Some(banner_name) => {
                    return std::fs::read(self.0.paths.banner(banner_name)).map(Option::Some);
                }
                None => Ok(None),
            })
            .collect::<Result<Vec<_>, _>>()
            .inspect_err(|error| tracing::error!(%error, "could not read banner image"))
            .map_err(InternalServerError)?;

        let iter = izip!(
            tournaments,
            rank_restrictions,
            country_restrictions,
            banners,
        )
        .map(
            |(tournament, rank_restriction, country_restriction, banner)| {
                let rank_restrictions = rank_restriction
                    .iter()
                    .map(|r| RankRange {
                        min: r.min as u32,
                        max: r.max as u32,
                    })
                    .collect();
                let country_restrictions = country_restriction
                    .iter()
                    .map(|c| Country {
                        country_code: c.country_code.clone(),
                    })
                    .collect();

                GetAllTournamentsResponse {
                    tournament: Tournament {
                        id: tournament.id,
                        name: tournament.name,
                        shorthand: tournament.shorthand,
                        format: tournament.format as u32,
                        bws: tournament.bws,
                        mode: tournament.mode.into(),
                        banner,
                        start_date: tournament.start_date.map(Into::into),
                        end_date: tournament.end_date.map(Into::into),
                    },
                    rank_restrictions,
                    country_restrictions,
                }
            },
        );
        Ok(Json(iter.collect()))
    }
}
