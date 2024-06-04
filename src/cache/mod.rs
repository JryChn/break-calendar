use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;

use anyhow::bail;
use anyhow::Result;
use chrono::{Datelike, Days, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use crate::common::exception::InternalError;
use crate::common::utils::{check_conflict, MAX_EVENT_TIMESTAMP, MIN_EVENT_TIMESTAMP};
use crate::model::EventCommonTrait;
use crate::model::generator_instance::GeneratorInstance;

mod test;

pub struct Cache {
    // cache only care about conflict, event valid and other self check not here
    properties: Properties,
    events_all: Vec<Rc<Box<dyn EventCommonTrait>>>,
    events_by_date: HashMap<NaiveDate, Vec<Rc<Box<dyn EventCommonTrait>>>>,
    events_by_id: HashMap<u128, Rc<Box<dyn EventCommonTrait>>>,
    instance: HashMap<u128, GeneratorInstance>,
}

#[derive(Deserialize, Serialize)]
struct Properties {
    last_modified: i64,
}

impl Cache {
    pub fn init() -> Self {
        let mut init_hashmap = HashMap::new();
        let mut day = MIN_EVENT_TIMESTAMP.date();
        while day.le(&MAX_EVENT_TIMESTAMP.date()) {
            init_hashmap.insert(day, Vec::new());
            day = day.checked_add_days(Days::new(1)).unwrap();
        }
        Cache {
            properties: Properties {
                last_modified: Utc::now().timestamp_millis(),
            },
            events_all: vec![],
            events_by_date: init_hashmap,
            events_by_id: Default::default(),
            instance: Default::default(),
        }
    }

    pub fn insert_events(&mut self, events: Vec<Box<dyn EventCommonTrait>>) -> Result<()> {
        for event in events {
            let event = Rc::new(event);
            let mut date_event_vec = Vec::new();
            date_event_vec.push(event.clone());
            let start_date = event.get_start_time().naive_utc().date();
            let mut pointer_date = start_date.clone();
            let end_date = event.get_end_time().naive_utc().date();
            while pointer_date.le(&end_date) {
                for e in self.events_by_date.get(&pointer_date).unwrap() {
                    date_event_vec.push(e.clone());
                }
                pointer_date = pointer_date.checked_add_days(Days::new(1)).unwrap();
            }
            // check conflict
            let conflict_events = check_conflict(&date_event_vec);
            if !conflict_events.is_empty() {
                let ignore_conflict = event.get_generator_instance().map_or(false, |id| {
                    self.instance.get(&id).map_or(false, |instance| {
                        instance
                            .repeat
                            .as_ref()
                            .map_or(false, |repeat| !repeat.throw_error_when_conflict)
                    })
                });
                if ignore_conflict {
                    continue;
                }
                bail!(InternalError::ConflictEventError {
                    start_time: event.get_start_time(),
                    end_time: event.get_end_time()
                });
            }
            if self.events_by_id.get(&event.get_id()).is_some() {
                // clean for update first
                self.delete_event(event.get_id())
                    .expect("Unexpected run here when create for update");
            }
            self.events_all.push(event.clone());
            let mut pointer_date = start_date.clone();
            while pointer_date.le(&end_date) {
                self.events_by_date
                    .get_mut(&pointer_date)
                    .unwrap()
                    .push(event.clone());
                pointer_date = pointer_date.checked_add_days(Days::new(1)).unwrap();
            }
            self.events_by_id.insert(event.get_id(), event.clone());
        }
        self.properties.last_modified = Utc::now().timestamp_millis();
        Ok(())
    }

    pub fn delete_event(&mut self, event_id: u128) -> Result<()> {
        let search_result = self.events_by_id.get(&event_id);
        if search_result.is_none() {
            bail!(InternalError::EventNotFoundError)
        }
        self.events_all.retain(|event| event.get_id() != event_id);
        let start_date = search_result.unwrap().get_start_time().naive_utc().date();
        let end_date = search_result.unwrap().get_end_time().naive_utc().date();
        let mut pointer_date = start_date.clone();
        while pointer_date.le(&end_date) {
            self.events_by_date
                .get_mut(&pointer_date)
                .unwrap()
                .retain(|event| event.get_id() != event_id);
            pointer_date = pointer_date.checked_add_days(Days::new(1)).unwrap();
        }
        self.events_by_id.remove(&event_id);
        self.properties.last_modified = Utc::now().timestamp_millis();
        Ok(())
    }

    pub fn get_events_by_day<E: EventCommonTrait>(&self, day: NaiveDate) -> Vec<Rc<Box<&E>>> {
        self.events_by_date
            .get(&day)
            .unwrap()
            .iter()
            .filter_map(|e| {
                let e = e.as_ref().downcast_ref::<E>();
                if e.is_none() {
                    return None;
                } else {
                    Some(Rc::new(Box::new(e.unwrap())))
                }
            })
            .collect()
    }
    pub fn get_events_by_id<E: EventCommonTrait>(&self, id: u128) -> Result<Rc<Box<&E>>> {
        let result = self.events_by_id.get(&id);
        if result.is_none() {
            bail!(InternalError::EventNotFoundError)
        }
        let result = result.unwrap().as_ref();
        let result = result.downcast_ref::<E>();
        if result.is_none() {
            bail!(InternalError::EventNotFoundError)
        }
        Ok(Rc::new(Box::new(result.unwrap())))
    }

    pub fn get_all_events<E: EventCommonTrait>(&self) -> Vec<Rc<Box<&E>>> {
        self.events_all
            .iter()
            .filter_map(|e| {
                let e = e.as_ref().downcast_ref::<E>();
                if e.is_none() {
                    return None;
                } else {
                    Some(Rc::new(Box::new(e.unwrap())))
                }
            })
            .collect()
    }
    pub fn get_all_raw_events(&self) -> Vec<Rc<Box<dyn EventCommonTrait>>> {
        self.events_all.clone()
    }

    pub fn get_instances(&self, id: u128) -> Option<GeneratorInstance> {
        self.instance.clone().get(&id).cloned()
    }

    pub fn add_or_update_instances(&mut self, instances: Vec<GeneratorInstance>) {
        for instance in instances {
            self.instance.insert(instance.get_id(), instance);
        }
    }
    pub fn get_all_instances(&self) -> Vec<GeneratorInstance> {
        self.instance.values().map(|i| i.clone()).collect()
    }
}
