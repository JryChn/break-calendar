use std::collections::HashMap;
use std::fmt::Error;
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::core::model::event::Event;
use crate::core::model::reminder::Reminder;
use crate::persistence::{CommonEvent, DaoOperation};

struct CACHE{
    last_update_time: i64,
    persistent_engine: dyn DaoOperation,
    events: HashMap<i64,Vec<Event>>,
    reminder: Vec<Reminder>,
}


impl CACHE{
    pub fn new(persistent_engine: impl DaoOperation) ->Self{
        CACHE{
            last_update_time:Utc::now().timestamp(),
            persistent_engine,
            events: HashMap::with_capacity(365),
            reminder:Vec::new()
        }
    }

     pub async fn save(&self) -> Result<Ok(_),Err(Error)>{
         let events:Vec<Event> = self.events.iter().flat_map(|events|{events.1}).map(
             |e|{
                 let dao_event = CommonEvent{};
                 super::convertor::event::EventConvertor::convert_from_business_2_dao(&e,dao_event)
             }
         ).collect();
         events.sort_by();
         self.persistent_engine.save_mul_events(events)?
    }
}
