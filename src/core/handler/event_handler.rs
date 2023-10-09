use std::error::Error;

use chrono::{DateTime, Utc};

use crate::core::model::event::Event;

pub fn create_new_event(title: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Event {
    todo!()
}

pub fn create_repeated_events(title: String, start_time: DateTime<Utc>, end_time: DateTime<Utc>, days: u8, times: u8) -> Vec<Event> {
    todo!()
}

pub fn find_event_by_id(id: u128) -> Option<Event> {
    todo!()
}

pub fn fetch_events_by_duration(start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Vec<Event> {
    todo!()
}

pub fn check_if_conflict(start:DateTime<Utc>,end: DateTime<Utc>) -> bool {
    todo!()
}

pub fn delete_event(id: u128) -> Result<Ok(_), Err(dyn Error)> {
    todo!()
}

pub fn delete_series_events(series_id: u128) -> Result<Ok(_), Err(dyn Error)> {
    todo!()
}


pub fn delete_series_events_after_current(id: u128, series_id: u128) -> Result<Ok(_), Err(dyn Error)> {
    todo!()
}

pub fn check_if_occupy_weekend(event:Event) -> bool{
    todo!()
}
pub fn check_if_occupy_holiday(event:Event) -> bool{
    todo!()
}
