use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use miette::{Context, IntoDiagnostic};
use model::{
    db::{
        pool_bracket, pool_map,
        prelude::{PoolBracket, PoolMap},
    },
    dto::pool::{PoolBracketDto, PoolDto},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use super::OsuService;

#[derive(Clone)]
pub struct PoolService {
    db: DatabaseConnection,
    osu: Arc<OsuService>,
}

impl PoolService {
    pub fn new(db: DatabaseConnection, osu: Arc<OsuService>) -> Arc<Self> {
        Arc::new(Self { db, osu })
    }

    /// Fetch all stages for a tournament.
    ///
    /// Returns `None`, if the tournament does not exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an error in the communication with the
    /// database.
    pub async fn get_all(
        &self,
        tournament_id: usize,
        stage_order: usize,
    ) -> miette::Result<PoolDto> {
        let brackets = self.fetch_pool_brackets(tournament_id, stage_order).await?;

        let brackets = futures::stream::iter(brackets)
            .enumerate()
            .then(|(order, (bracket, maps))| async move {
                let maps = futures::stream::iter(maps)
                    .then(|map| self.osu.get_beatmap(map.map_id as u32))
                    .try_collect::<Vec<_>>()
                    .await
                    .wrap_err("could not fetch osu beatmaps")?;
                Ok::<PoolBracketDto, miette::Report>(PoolBracketDto {
                    order,
                    name: bracket.name,
                    maps,
                })
            })
            .try_collect::<Vec<_>>()
            .await
            .wrap_err("could not create pool brackets")?;

        Ok(PoolDto {
            tournament_id,
            stage_order,
            brackets,
        })
    }

    async fn fetch_pool_brackets(
        &self,
        tournament_id: usize,
        stage_order: usize,
    ) -> miette::Result<Vec<(pool_bracket::Model, Vec<pool_map::Model>)>> {
        PoolBracket::find()
            .filter(pool_bracket::Column::TournamentId.eq(tournament_id as i32))
            .find_with_related(PoolMap)
            .order_by_asc(pool_bracket::Column::BracketOrder)
            .order_by_asc(pool_map::Column::MapOrder)
            .all(&self.db)
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("could not fetch pool brackets for tournament '{tournament_id}' and stage '{stage_order}'"))
    }
}
