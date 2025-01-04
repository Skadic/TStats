use std::sync::Arc;

use miette::{Context, IntoDiagnostic};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use model::db::{prelude::*, stage, tournament};

#[derive(Clone)]
pub struct StageService {
    db: DatabaseConnection,
}

impl StageService {
    pub fn new(db: DatabaseConnection) -> Arc<Self> {
        Arc::new(Self { db })
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
        tournament_id: i32,
        stage_order: i32,
        db: &DatabaseConnection,
    ) -> miette::Result<Option<(tournament::Model, Option<stage::Model>)>> {
        Tournament::find_by_id(tournament_id)
            .find_also_related(Stage)
            .filter(stage::Column::StageOrder.eq(stage_order))
            .one(db)
            .await
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("could not fetch stage '{stage_order}' for tournament '{tournament_id}'")
            })
    }
}
