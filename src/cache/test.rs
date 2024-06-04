#[cfg(test)]
mod tests {
    use chrono::{DateTime, Days, Duration, Utc};

    use crate::cache::Cache;
    use crate::model::event::Event;
    use crate::model::EventCommonTrait;
    use crate::model::generator_instance::{GeneratorInstance, Repeat, RepeatStrategy};
    use crate::model::reminder::Reminder;

// use crate::model::reminder::Reminder;

    #[test]
    fn get_events_returns_error_when_no_matching_event() {
        let cache = Cache::init();
        let events = cache.get_events_by_id::<Event>(1);
        assert!(events.is_err());
    }

    #[test]
    fn set_events_adds_event_to_cache() {
        let mut cache = Cache::init();
        let mut event = Event::init(None);
        let start_time = DateTime::from(Utc::now());
        let end_time = DateTime::from(Utc::now())
            .checked_add_days(Days::new(1))
            .unwrap();
        event.set_duration(start_time, end_time);
        let id = event.get_id();
        cache.insert_events(vec![Box::new(event)]).unwrap();
        let events = cache.get_all_events::<Event>();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].get_id(), id);
        assert_eq!(events[0].get_start_time(), start_time);
        assert_eq!(events[0].get_end_time(), end_time);
        let events = cache.get_events_by_day::<Event>(start_time.date_naive());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].get_id(), id);
        assert_eq!(events[0].get_start_time(), start_time);
        assert_eq!(events[0].get_end_time(), end_time);
        let events = cache.get_events_by_day::<Event>(end_time.date_naive());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].get_id(), id);
        assert_eq!(events[0].get_start_time(), start_time);
        assert_eq!(events[0].get_end_time(), end_time);
        let events = cache.get_events_by_id::<Event>(id);
        assert!(events.is_ok());
        let events = events.unwrap();
        assert_eq!(events.get_id(), id);
        assert_eq!(events.get_start_time(), start_time);
        assert_eq!(events.get_end_time(), end_time);
    }

    #[test]
    fn set_events_returns_error_when_events_conflict() {
        let mut cache = Cache::init();
        let mut event1 = Event::init(None);
        let mut event2 = Event::init(None);
        let event1_start_time = DateTime::from(Utc::now());
        let event1_end_time = DateTime::from(Utc::now())
            .checked_add_days(Days::new(1))
            .unwrap();
        event1.set_duration(event1_start_time, event1_end_time);
        let event2_start_time = DateTime::from(Utc::now().checked_sub_days(Days::new(1)).unwrap());
        let event2_end_time = DateTime::from(Utc::now())
            .checked_add_signed(Duration::hours(12))
            .unwrap();
        event2.set_duration(event2_start_time, event2_end_time);
        cache.insert_events(vec![Box::new(event1)]).unwrap();
        let result = cache.insert_events(vec![Box::new(event2)]);
        assert!(result.is_err());
        assert_eq!(cache.get_all_events::<Event>().len(), 1);
        let mut generated_instance = GeneratorInstance::new();
        generated_instance.repeat = Some(Repeat {
            repeat_strategy: RepeatStrategy {
                repeat_gap_day: 0,
                repeat_gap_month: 0,
                repeat_gap_year: 0,
                skip_weekday: false,
                skip_weekend: false,
                skip_holiday: false,
                start_day: 0,
                until_day: 0,
            },
            event_queue: vec![],
            throw_error_when_conflict: true,
        });
        cache.add_or_update_instances(vec![generated_instance.clone()]);
        let mut event2 = Event::init(None);
        event2.set_duration(event2_start_time, event2_end_time);
        event2.set_generator_instance(generated_instance.get_id());
        let result = cache.insert_events(vec![Box::new(event2)]);
        assert!(result.is_err());
        assert_eq!(cache.get_all_events::<Event>().len(), 1);

        let mut generated_instance = GeneratorInstance::new();
        generated_instance.repeat = Some(Repeat {
            repeat_strategy: RepeatStrategy {
                repeat_gap_day: 0,
                repeat_gap_month: 0,
                repeat_gap_year: 0,
                skip_weekday: false,
                skip_weekend: false,
                skip_holiday: false,
                start_day: 0,
                until_day: 0,
            },
            event_queue: vec![],
            throw_error_when_conflict: false,
        });
        cache.add_or_update_instances(vec![generated_instance.clone()]);
        let mut event2 = Event::init(None);
        event2.set_duration(event2_start_time, event2_end_time);
        event2.set_generator_instance(generated_instance.get_id());
        let result = cache.insert_events(vec![Box::new(event2)]);
        assert!(result.is_ok());
        assert_eq!(cache.get_all_events::<Event>().len(), 1);
    }

    //
    #[test]
    fn delete_event_removes_event_from_cache() {
        let mut cache = Cache::init();
        let mut event = Event::init(None);
        let start_time = DateTime::from(Utc::now());
        let end_time = DateTime::from(Utc::now())
            .checked_add_days(Days::new(2))
            .unwrap();
        event.set_duration(start_time, end_time);
        let id = event.get_id();
        cache.insert_events(vec![Box::new(event)]).unwrap();
        assert_eq!(cache.get_events_by_day::<Event>(start_time.date_naive()).len(), 1);
        assert_eq!(
            cache
                .get_events_by_day::<Event>(
                    start_time
                        .checked_add_days(Days::new(1))
                        .unwrap()
                        .date_naive()
                )
                .len(),
            1
        );
        assert_eq!(cache.get_events_by_day::<Event>(end_time.date_naive()).len(), 1);
        assert!(cache.get_events_by_id::<Event>(id).is_ok());
        assert_eq!(cache.get_all_events::<Event>().len(), 1);
        assert_eq!(
            cache.events_all.first().unwrap().get_start_time(),
            start_time
        );
        assert_eq!(cache.get_events_by_id::<Event>(id).unwrap().get_end_time(), end_time);
        cache.delete_event(id).unwrap();
        let events = cache.get_all_events::<Event>();
        assert!(events.is_empty());
        assert!(cache.get_events_by_id::<Event>(id).is_err());
        assert!(cache.get_events_by_day::<Event>(start_time.date_naive()).is_empty());
    }


    #[test]
    fn set_reminders_adds_reminder_to_cache() {
        let mut cache = Cache::init();
        let mut reminder = Reminder::init(None);
        reminder.set_duration(
            DateTime::from(Utc::now()),
            DateTime::from(Utc::now().checked_add_signed(Duration::hours(1)).unwrap()),
        );
        let id = reminder.get_id();
        cache.insert_events(vec![Box::new(reminder)]).unwrap();
        let reminders =
            cache.get_events_by_id::<Reminder>(id);
        assert_eq!(reminders.is_ok(), true);
        assert_eq!(reminders.unwrap().get_kind().to_string(), "Reminder");
    }
}
