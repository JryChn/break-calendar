use chrono::{DateTime, FixedOffset};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("invalid start time {start_time:?} and end time {end_time:?}")]
    InvalidStartEndTimeError {
        start_time: DateTime<FixedOffset>,
        end_time: DateTime<FixedOffset>,
    },
    #[error("invalid timezone {time_zone:?}")]
    InvalidTimeZoneError { time_zone: String },
    #[error("conflict between time: {start_time:?} and end time:{end_time:?}")]
    ConflictEventError {
        start_time: DateTime<FixedOffset>,
        end_time: DateTime<FixedOffset>,
    },
    #[error("the event with id: {event_id} is already exist")]
    EventsAlreadyExistError { event_id: u128 },
    #[error("data persistence error")]
    DataPersistenceError,
    #[error("unknown data error")]
    UnknownError,
    #[error("Event not found error")]
    EventNotFoundError,
    #[error("Busy Cache")]
    BusyCache,
}
