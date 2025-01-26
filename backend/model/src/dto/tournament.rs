use crate::db::sea_orm_active_enums::OsuMode;
use crate::db::{country_restriction, rank_restriction, tournament};
use chrono::NaiveDate;
use poem_openapi::Object;
use std::borrow::Borrow;

#[derive(Object, PartialEq, Eq, Debug)]
#[oai(rename = "Tournament", rename_all = "camelCase")]
pub struct TournamentDto {
    pub id: usize,
    pub name: String,
    pub shorthand: String,
    pub format: usize,
    pub bws: bool,
    pub mode: OsuMode,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub banner: Option<Vec<u8>>,
    pub rank_restrictions: Vec<RankRangeDto>,
    pub country_restrictions: Vec<CountryDto>,
}

impl<R, C> From<(tournament::Model, R, C, Option<Vec<u8>>)> for TournamentDto
where
    R: IntoIterator<Item = rank_restriction::Model>,
    C: IntoIterator<Item = country_restriction::Model>,
{
    fn from(
        (tournament, rank_restrictions, country_restrictions, banner): (
            tournament::Model,
            R,
            C,
            Option<Vec<u8>>,
        ),
    ) -> Self {
        Self {
            id: tournament.id as usize,
            name: tournament.name,
            shorthand: tournament.shorthand,
            format: tournament.format as usize,
            bws: tournament.bws,
            mode: tournament.mode,
            start_date: tournament.start_date.map(|dt| dt.date()),
            end_date: tournament.end_date.map(|dt| dt.date()),
            banner,
            rank_restrictions: rank_restrictions.into_iter().map(Into::into).collect(),
            country_restrictions: country_restrictions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Object, PartialEq, Eq, Debug, Clone, Copy)]
#[oai(rename = "RankRange", rename_all = "camelCase")]
pub struct RankRangeDto {
    pub min: usize,
    pub max: usize,
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
            min: t.min as usize,
            max: t.max as usize,
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
