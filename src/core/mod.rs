use chrono::TimeZone;

use crate::model::EventCommonTrait;

mod executorPool;
mod processor;

//
// pub fn create_events(
//     event: Event,
//     ignore_conflict: Option<bool>,
//     repeat_strategy: Option<RepeatStrategy>,
// ) -> Result<Vec<Event>> {
//     dynamic_process(|mut e| {
//         let mut instance = GeneratorInstance::new();
//         // generate repeat logic
//         let mut events = Vec::new();
//         if repeat_strategy.is_some() {
//             let repeat_strategy = repeat_strategy.unwrap();
//             if RepeatStrategy::check_valid(&repeat_strategy) {
//                 // bail!(InternalError::InvalidStartEndTimeError {start_time: Default::default(),end_time: Default::default()})
//             }
//             let repeat = Repeat {
//                 repeat_strategy: repeat_strategy.clone(),
//                 event_queue: vec![],
//                 throw_error_when_conflict: ignore_conflict.unwrap_or(true),
//             };
//             instance.repeat = Some(repeat);
//             // generate events by repeat logic
//             let mut current_day_pointer = DateTime::from_timestamp_millis(repeat_strategy.start_day).unwrap();
//             let start_offset = event.get_start_time().signed_duration_since(event.get_start_time().date().and_hms_opt(0, 0, 0).unwrap());
//             let end_offset = event.get_end_time().signed_duration_since(event.get_end_time().date().and_hms_opt(0, 0, 0).unwrap());
//             while current_day_pointer.le(&DateTime::from_timestamp_millis(repeat_strategy.until_day).unwrap()) {
//                 let mut new_event = event.self_clone();
//                 new_event.set_duration(DateTime::from(current_day_pointer.checked_add_signed(start_offset).unwrap()), DateTime::from(current_day_pointer.checked_add_signed(end_offset).unwrap()));
//                 new_event.set_generator_instance(instance.get_id());
//                 events.push(new_event);
//                 current_day_pointer = current_day_pointer.checked_add_signed(Duration::days(repeat_strategy.repeat_gap_day as i64)).unwrap();
//             }
//         }
//         if e.set_instances(vec![instance.clone()]).is_ok() {
//             e.set_events(events).unwrap()
//         }
//     }
//     ).and(
//         Ok
//         (Vec::new())
//     )
// }
//
// pub fn get_events_by_day(date: DateTime<FixedOffset>) -> Vec<Arc<Box<Event>>> {
//     static_process(|e| {
//         e.get_events_by_day::<Event>(date.date_naive())
//             .into_iter()
//             .map(|event| {
//                 let e = **event.clone();
//                 let e = e.clone();
//                 Arc::new(Box::new(e))
//             })
//             .collect::<Vec<Arc<Box<Event>>>>()
//     }).unwrap()
// }
//
// pub fn get_all_events() -> Vec<Event> {
//     todo!()
// }
//
// pub fn get_events_by_id(id: u128) -> Vec<Event> {
//     static_process(|e| e.get_events::<Utc>(None, Some(id))).unwrap()
// }
//
// pub fn update_events(events: Vec<Event>) -> Result<()> {
//     todo!()
// }
//
// pub fn delete_events_by_id(events_id: Vec<u128>) -> Result<()> {
//     todo!()
// }
// pub fn delete_events_by_day(dates: Vec<NaiveDate>) -> Result<()> {
//     todo!()
// }
//
// pub fn create_new_reminder(events: Vec<Reminder>, ignore_conflict: bool) -> Vec<Reminder> {
//     todo!()
// }
//
// pub fn get_reminder_by_day(date: NaiveDate) -> Vec<Reminder> {
//     todo!()
// }
//
// pub fn get_reminder_by_id(id: u128) -> Vec<Reminder> {
//     todo!()
// }
//
// pub fn update_reminders(reminders: Vec<Reminder>) -> Result<()> {
//     todo!()
// }
//
// pub fn delete_reminders_by_id(reminders_id: Vec<u128>) -> Result<()> {
//     todo!()
// }
// pub fn delete_reminders_by_day(dates: Vec<NaiveDate>) -> Result<()> {
//     todo!()
// }
