use std::io::IsTerminal;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use log::{debug, error};
use surrealdb::sql::{Id, Thing};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tracing::field::debug;

use crate::model::relations::is_stage::IsStage;
use crate::model::stage::Stage;
use crate::model::TableType;
use crate::Record;

#[derive(Debug, serde::Deserialize)]
pub struct ByTournamentId {
    tournament_id: String,
}

#[derive(Debug, serde::Deserialize)]
struct Stages {
    stages: Vec<Stage<'static>>,
}

/// Get all tournaments
pub async fn get_all_stages(
    State(db): State<Surreal<Client>>,
    Query(param): Query<ByTournamentId>,
) -> Result<Json<Vec<Stage<'static>>>, (StatusCode, String)> {
    let mut response = db
        .query(
            r#"SELECT ->is_stage->stage as stages FROM type::thing("tournament", $id) FETCH stages"#,
        )
        .bind(("id", param.tournament_id))
        .await
        .map_err(|e| {
            error!("failed to query stages: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    debug!("{:?}", &response);

    let stage_opt: Option<Stages> = response.take(0).map_err(|e| {
        error!("failed to parse stages: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    return match stage_opt {
        Some(stages) if stages.stages.is_empty() => Ok(Json(Vec::new())),
        None => Ok(Json(Vec::new())),
        Some(stages) => Ok(Json(stages.stages)),
    };
}

pub async fn create_stage(
    State(db): State<Surreal<Client>>,
    Query(params): Query<ByTournamentId>,
    Json(stage): Json<Stage<'static>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let stage: Stage = db
        .create(Stage::table_name())
        .content(stage)
        .await
        .map_err(|e| {
            error!("failed to create stage: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let tournament: Thing = ("tournament".to_string(), Id::from(params.tournament_id)).into();
    debug!("tournament: {:?}", &tournament);
    let _: Record = db
        .create(IsStage::table_name())
        .content(IsStage::new(&tournament, stage.id.as_ref().unwrap()))
        .await
        .map_err(|e| {
            error!("failed to create is_stage relation: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::OK, stage.id.as_ref().unwrap().to_string()))
}
