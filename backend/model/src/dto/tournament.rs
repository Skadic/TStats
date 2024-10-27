use crate::db::sea_orm_active_enums::OsuMode;
use chrono::NaiveDate;
use poem_openapi::Object;

#[derive(Object, PartialEq, Eq, Debug)]
pub struct GetAllTournamentsResponse {
    pub tournament: Tournament,
    pub rank_restrictions: Vec<RankRange>,
    pub country_restrictions: Vec<Country>,
}

#[derive(Object, PartialEq, Eq, Debug)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
    pub shorthand: String,
    pub format: u32,
    pub bws: bool,
    pub mode: OsuMode,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub banner: Option<Vec<u8>>,
}

#[derive(Object, PartialEq, Eq, Debug)]
pub struct RankRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Object, PartialEq, Eq, Debug)]
pub struct Country {
    pub country_code: String,
}
