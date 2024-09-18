use proto::utils::DateMillis;
use sqlx::types::chrono::NaiveDateTime;
use tonic::Status;
use tracing::error;

#[allow(unused)]
pub mod debug;
pub mod osu_auth;
pub mod osu_user;
pub mod pool;
pub mod score;
pub mod stage;
pub mod tournament;

fn convert_start_end(
    start_date: Option<DateMillis>,
    end_date: Option<DateMillis>,
) -> Result<(Option<NaiveDateTime>, Option<NaiveDateTime>), Status> {
    let start_date: Option<NaiveDateTime> = match start_date {
        Some(date) => Some(date.try_into().map_err(|error| {
            error!(%error, "could not convert timestamp into date millis");
            Status::internal("error creating tournament")
        })?),
        None => None,
    };
    let end_date: Option<NaiveDateTime> = match end_date {
        Some(date) => Some(date.try_into().map_err(|error| {
            error!(%error, "could not convert timestamp into date millis");
            Status::internal("error creating tournament")
        })?),
        None => None,
    };

    if let (Some(ref start), Some(ref end)) = (start_date, end_date) {
        if start.and_utc().timestamp_millis() > end.and_utc().timestamp_millis() {
            return Err(Status::invalid_argument("start date is after end date"));
        }
    }

    Ok((start_date, end_date))
}
