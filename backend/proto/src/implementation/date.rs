use prost_types::Timestamp;
use sqlx::types::chrono::{DateTime, NaiveDateTime};

use crate::utils::DateMillis;

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DateConversionError {
    #[error("milliseconds out of range: {0}")]
    MillisOutOfRange(i64),
    #[error("seconds too large to be converted to millis: {0}")]
    SecondsOutOfRange(i64),
}

impl TryFrom<Timestamp> for DateMillis {
    type Error = DateConversionError;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        match value.seconds.checked_mul(1000) {
            Some(res) => Ok(DateMillis {
                millis: res + value.nanos as i64 / 1_000_000,
            }),
            None => Err(DateConversionError::MillisOutOfRange(value.seconds)),
        }
    }
}

impl From<NaiveDateTime> for DateMillis {
    fn from(value: NaiveDateTime) -> Self {
        DateMillis {
            millis: value.and_utc().timestamp_millis(),
        }
    }
}

impl TryFrom<DateMillis> for NaiveDateTime {
    type Error = DateConversionError;

    fn try_from(value: DateMillis) -> Result<Self, Self::Error> {
        DateTime::from_timestamp_millis(value.millis)
            .ok_or(DateConversionError::MillisOutOfRange(value.millis))
            .map(|v| v.naive_utc())
    }
}
