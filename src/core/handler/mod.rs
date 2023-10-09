use chrono::{DateTime, Utc};

pub mod reminder_handler;
pub mod event_handler;
pub mod custom_info_handler;

pub fn filter_weekend(days: Vec<DateTime<Utc>>) -> Vec<DateTime<Utc>> {
    todo!()
}

pub fn filter_holiday(days: Vec<DateTime<Utc>>) -> Vec<DateTime<Utc>> {
    todo!()
}