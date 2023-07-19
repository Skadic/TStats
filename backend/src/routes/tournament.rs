use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::model::{tournament::Tournament, TableType};

#[derive(Debug, Deserialize, Serialize)]
pub struct ById {
    id: String,
}

pub async fn get_tournament(
    State(db): State<Surreal<Client>>,
    Query(param): Query<ById>,
) -> String {
    dbg!(&param);
    let tournament: Option<Tournament> = db
        .select((Tournament::table_name(), param.id))
        .await
        .unwrap();

    format!("Tournament: {:?}", tournament)
}
