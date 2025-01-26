use std::sync::Arc;

use futures::{TryFutureExt, TryStreamExt};
use miette::{Context, IntoDiagnostic};
use model::{
    db::{prelude::*, stage},
    dto::{stage::StageDto},
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Clone)]
pub struct StageService {
    db: DatabaseConnection,
}

impl StageService {
    pub fn new(db: DatabaseConnection) -> Arc<Self> {
        Arc::new(Self { db })
    }

    /// Fetch all stages for a tournament.
    ///
    /// Returns `None`, if the tournament does not exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an error in the communication with the
    /// database.
    pub async fn get_all(&self, tournament_id: usize) -> miette::Result<Vec<StageDto>> {
        Stage::find()
            .filter(stage::Column::TournamentId.eq(tournament_id as i32))
            .stream(&self.db)
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("could not stream db results fetching stages"))?
            .map_ok(StageDto::from)
            .try_collect::<Vec<_>>()
            .await
            .into_diagnostic()
            .wrap_err_with(|| format!("could not fetch stages for tournament '{tournament_id}'"))
    }

    /// Fetch a stage for a tournament.
    ///
    /// Returns `None`, if the tournament does not exist, and returns `Some((_, None))` if the
    /// associated stage does not exist.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an error in the communication with the
    /// database.
    pub async fn get_by_id(
        &self,
        tournament_id: usize,
        stage_order: usize,
    ) -> miette::Result<Option<StageDto>> {
        Stage::find_by_id((tournament_id as i32, stage_order as i16))
            .one(&self.db)
            .map_ok(|opt| opt.map(StageDto::from))
            .await
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("could not fetch stage '{stage_order}' for tournament '{tournament_id}'")
            })
    }
}
