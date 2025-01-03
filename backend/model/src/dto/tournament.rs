use crate::db::sea_orm_active_enums::OsuMode;
use crate::db::{country_restriction, rank_restriction};
use chrono::NaiveDate;
use poem_openapi::Object;
use std::borrow::Borrow;

#[derive(Object, PartialEq, Eq, Debug)]
#[oai(rename = "Tournament", rename_all = "camelCase")]
pub struct TournamentDto {
    pub id: i32,
    pub name: String,
    pub shorthand: String,
    pub format: u32,
    pub bws: bool,
    pub mode: OsuMode,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub banner: Option<Vec<u8>>,
    pub rank_restrictions: Vec<RankRangeDto>,
    pub country_restrictions: Vec<CountryDto>,
}

#[derive(Object, PartialEq, Eq, Debug, Clone, Copy)]
#[oai(rename = "RankRange", rename_all = "camelCase")]
pub struct RankRangeDto {
    pub min: u32,
    pub max: u32,
}

#[derive(Object, PartialEq, Eq, Debug)]
#[oai(rename = "Country", rename_all = "camelCase")]
pub struct CountryDto {
    pub country_code: String,
}

impl<T: Borrow<rank_restriction::Model>> From<T> for RankRangeDto {
    fn from(t: T) -> Self {
        let t: &rank_restriction::Model = t.borrow();
        Self {
            min: t.min as u32,
            max: t.max as u32,
        }
    }
}

impl<T: Borrow<country_restriction::Model>> From<T> for CountryDto {
    fn from(t: T) -> Self {
        let t: &country_restriction::Model = t.borrow();
        Self {
            country_code: t.country_code.clone(),
        }
    }
}
