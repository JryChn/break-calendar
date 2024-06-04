use std::rc::Rc;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use lazy_static::lazy_static;

use crate::model::EventCommonTrait;

lazy_static! {
    pub static ref MAX_EVENT_TIMESTAMP: NaiveDateTime = NaiveDate::from_ymd_opt(2100, 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    pub static ref MIN_EVENT_TIMESTAMP: NaiveDateTime = NaiveDate::from_ymd_opt(1970, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
}

pub fn get_cache_day_number_from_time(time: DateTime<FixedOffset>) -> i64 {
    time.naive_utc()
        .signed_duration_since(*MIN_EVENT_TIMESTAMP)
        .num_days()
}

pub fn get_all_day_numbers() -> i64 {
    MAX_EVENT_TIMESTAMP
        .signed_duration_since(*MIN_EVENT_TIMESTAMP)
        .num_days()
}

pub fn check_conflict(
    events: &Vec<Rc<Box<dyn EventCommonTrait>>>,
) -> Vec<(Rc<Box<dyn EventCommonTrait>>, Rc<Box<dyn EventCommonTrait>>)> {
    let mut result = Vec::new();
    for i in 0..events.len() {
        for j in 0..events.len() {
            if i != j {
                {
                    let first_event = events[i].clone();
                    let second_event = events[j].clone();
                    if first_event
                        .get_start_time()
                        .le(&second_event.get_end_time())
                        || first_event
                        .get_end_time()
                        .ge(&second_event.get_start_time())
                        || (first_event
                        .get_start_time()
                        .lt(&second_event.get_start_time())
                        && first_event.get_end_time().gt(&second_event.get_end_time()))
                        || (first_event
                        .get_start_time()
                        .gt(&second_event.get_start_time())
                        && first_event.get_end_time().lt(&second_event.get_end_time()))
                    {
                        result.push((events[i].clone(), events[j].clone()));
                    }
                }
            }
        }
    }
    result
}

pub fn convert_from_string_to_datetime(naive_time: i64, offset: String) -> DateTime<FixedOffset> {
    DateTime::from_timestamp_millis(naive_time)
        .unwrap()
        .with_timezone(&FixedOffset::from_str(&offset).unwrap())
}
