use anyhow::bail;
use chrono::{DateTime, Utc};

use crate::common::exception::InternalError;
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

impl Event {
    fn id(&self) -> u128 {
        self.id
    }
    pub fn series_id(&self) -> Option<u128> {
        self.series_id
    }
    pub fn author_id(&self) -> u128 {
        self.author_id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn duration_time(&self) -> &DurationTime {
        &self.duration_time
    }
    pub fn restriction(&self) -> &Vec<AllocateRestriction> {
        &self.restriction
    }
}


pub struct EventBuilder {
    event: Event,
}

impl EventBuilder {
    pub fn new() -> EventBuilder {
        EventBuilder {
            event: Event {
                id: uuid::Uuid::new_v4().as_u128(),
                series_id: None,
                author_id: 0,
                title: "".to_string(),
                description: "".to_string(),
                location: None,
                duration_time: DurationTime { start_time: Default::default(), end_time: Default::default() },
                participants: vec![],
                importance: Default::default(),
                restriction: vec![],
            }
        }
    }
    pub fn id(&mut self, id: u128) {
        self.event.id = id;
    }
    pub fn series_id(&mut self, id: u128) {
        self.event.series_id = Some(id);
    }
    pub fn author_id(&mut self, id: u128) {
        self.event.author_id = id;
    }

    pub fn title(&mut self, title: &str) {
        self.event.title = title.to_string();
    }

    pub fn start_time(&mut self, start_time: DateTime<Utc>) {
        self.event.duration_time.start_time = start_time;
    }
    pub fn end_time(&mut self, end_time: DateTime<Utc>) {
        self.event.duration_time.end_time = end_time;
    }

    pub fn add_restriction(&mut self, restriction: AllocateRestriction) {
        self.event.restriction.push(restriction);
    }

    pub fn remove_restriction(&mut self, restriction: AllocateRestriction) {}

    pub fn build(self) -> anyhow::Result<Event> {
        if self.event.duration_time.start_time.gt(&self.event.duration_time.end_time) {
            bail!(InternalError::DataPersistentError)
        }
        Ok(self.event)
    }
}


#[derive(PartialEq,Clone)]
pub enum AllocateRestriction {
    AvoidWeekendOrOnly(bool),
    AvoidHolidayOrOnly(bool),
    StartEndDateTimes(i64,i64,u8),
}