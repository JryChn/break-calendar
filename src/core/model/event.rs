use crate::core::model::{DurationTime, ImportanceLevel, Location};

pub struct Event {
    id: u128,
    // the unique identifier fot the event
    series_id: Option<u128>,
    // not None if the event has relate event or one of the repeat events
    // ID of the first event in a recurring series
    author_id: u128,
    title: String,
    pub description: String,
    pub location: Option<Location>,
    duration_time: DurationTime,
    pub participants: Vec<String>,
    pub importance: ImportanceLevel,
    restriction: Vec<AllocateRestriction>,
}
#[derive(PartialEq,Clone)]
pub enum AllocateRestriction {
    AvoidWeekendOrOnly(bool),
    AvoidHolidayOrOnly(bool),
    StartEndDateTimes(i64,i64,u8),
}