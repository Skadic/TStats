use chrono::{NaiveDate, NaiveTime};
use poem_openapi::Object;

use crate::db::stage;

#[derive(Object, PartialEq, Eq, Debug, Clone)]
#[oai(rename = "Stage", rename_all = "camelCase")]
pub struct StageDto {
    pub tournament_id: usize,
    pub stage_order: usize,
    pub name: String,
    pub best_of: usize,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

impl From<stage::Model> for StageDto {
    fn from(value: stage::Model) -> Self {
        Self {
            tournament_id: value.tournament_id as usize,
            stage_order: value.stage_order as usize,
            name: value.name,
            best_of: value.best_of as usize,
            start_date: value.start_date.map(|dt| dt.date()),
            end_date: value.end_date.map(|dt| dt.date()),
        }
    }
}

impl From<StageDto> for stage::Model {
    fn from(value: StageDto) -> Self {
        Self {
            tournament_id: value.tournament_id as i32,
            stage_order: value.stage_order as i16,
            name: value.name,
            best_of: value.best_of as i16,
            start_date: value.start_date.map(|dt| dt.and_time(NaiveTime::MIN)),
            end_date: value.end_date.map(|dt| dt.and_time(NaiveTime::MIN)),
        }
    }
}
