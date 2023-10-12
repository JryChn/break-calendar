use std::rc::Rc;
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;

use crate::api::model::{ApiCustomInformation, ApiEvent};
use crate::core::cache::CACHE;
use crate::core::model::custom_info::CustomInfo;
use crate::persistence::file_system::FileSystemPersistence;

const DEFAULT_MAX_ALLOCATE_TIMES: u8 = 30;

lazy_static!(
    static ref buffer : Mutex= Mutex::new(Rc::new(CACHE::new(FileSystemPersistence::init())));
);

fn create_new_event(api_event: ApiEvent) -> Vec<u128> {
    // create new business event
    // convert api event to business event
    // check business event if conflict
    // creat repeat event if needed
    // save to dao
    // return result
    todo!()
}


fn fetch_event_by_id(id: u128) -> Vec<ApiEvent> {
    // find by id
    // return
    todo!()
}

fn fetch_event_by_series_id(id: u128) -> Vec<ApiEvent> {
    // find by id
    // return
    todo!()
}

fn fetch_events_in_day(day: DateTime<Utc>) -> Vec<ApiEvent> {
    // find by day
    //return
    todo!()
}

fn fetch_events_in_week(week: DateTime<Utc>) -> Vec<ApiEvent> {
    // find by week
    // return
    todo!()
}

fn fetch_events_in_month(month: DateTime<Utc>) -> Vec<ApiEvent> {
    // find by month
    // return
    todo!()
}

fn fetch_events_in_year(year: DateTime<Utc>) -> Vec<ApiEvent> {
    // find by year
    // return
    todo!()
}


fn update_event(api_event: ApiEvent) -> Vec<u128> {
    // find by id
    // update mut field
    // save
    // return
    todo!()
}

fn delete_event(id: u128) {

    // find by id
    // delete
    // save
    todo!()
}

fn delete_series_events(series_id: u128) {
    // find by series id
    // delete multiple
    // save
    todo!()
}

fn delete_series_events_after_one_event(series_id: u128, id: u128) {
    // find by series id
    // filter after one day
    // delete multiple
    // save
}


fn create_new_custom_info(api_info: ApiCustomInformation) -> u128 {
    todo!()
}

fn fetch_custom_info(id: u128) -> CustomInfo {
    todo!()
}

fn fetch_all_custom_info() -> Vec<CustomInfo> {
    todo!()
}

fn delete_custom_info(id: u128) {
    todo!()
}