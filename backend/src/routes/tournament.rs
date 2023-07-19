use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use log::{debug, error, info, warn};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::model::{tournament::Tournament, TableType};
use crate::Record;
use crate::routes::ById;

/// Get all tournaments
pub async fn get_all_tournaments(
    State(db): State<Surreal<Client>>,
) -> Result<Json<Vec<Tournament<'static>>>, (StatusCode, String)> {
    db.select(Tournament::table_name())
        .await
        .map(Json)
        .map_err(|e| {
            error!("failed to get tournaments: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}

/// Get a tournament by its ID
pub async fn get_tournament(
    State(db): State<Surreal<Client>>,
    Query(param): Query<ById>,
) -> Result<Json<Option<Tournament<'static>>>, (StatusCode, String)> {
    // Find the tournament with the given ID
    db.select((Tournament::table_name(), &param.id))
        .await
        .map(|opt: Option<_>| {
            if opt.is_none() {
                info!("tournament with id \"{}\" not found", &param.id);
            }
            opt
        })
        .map(Json)
        .map_err(|e| {
            error!("error fetching tournament with id \"{}\": {e}", &param.id);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}

/// Create a new tournament
pub async fn create_tournament(
    State(db): State<Surreal<Client>>,
    Json(tournament): Json<Tournament<'_>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let name = tournament.name.clone();
    db.create(Tournament::table_name())
        .content(tournament)
        .await
        // Hint to the compiler that we want to create a record
        .map(|r: Record| {
            debug!("creating tournament \"{}\" with id {}", &name, r.id.id);
            (StatusCode::OK, r.id.id.to_string())
        })
        .map_err(|e| {
            warn!("tournament \"{}\" already exists", &name);
            (StatusCode::FORBIDDEN, e.to_string())
        })
}
