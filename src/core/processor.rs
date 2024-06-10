use std::sync::{Arc, mpsc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::Duration;

use anyhow::bail;
use anyhow::Result;
use lazy_static::lazy_static;
use tracing::info;

use crate::cache::Cache;
use crate::common::exception::InternalError;
use crate::core::delayQueue::PROCESS_QUEUE;
use crate::core::executorPool::EXECUTOR_POOL;
use crate::model::EventCommonTrait;
use crate::persistent::Persistent;

lazy_static! {
    static ref PERSISTENT_SYSTEM: Persistent = Persistent::init();
    static ref CACHE: Arc<RwLock<Cache>> = {
        let cache = PERSISTENT_SYSTEM.load();
        Arc::new(RwLock::new(if cache.is_ok() {
            cache.unwrap()
        } else {
            Cache::init()
        }))
    };
}

pub fn dynamic_process<F: FnOnce(RwLockWriteGuard<Cache>) + Send + Copy + 'static>(
    process_func: F,
) -> Result<()> {
    info!("start dynamic process");
    let (tx, rx) = mpsc::channel();
    EXECUTOR_POOL.execute(move || {
        let dynamic_func = Arc::new(Mutex::new(process_func));
        let mut cache_lock = CACHE.try_write();
        if cache_lock.is_err() {
            thread::sleep(Duration::from_secs(3));
            cache_lock = CACHE.try_write();
            if cache_lock.is_err() {
                let dynamic_func = Arc::clone(&dynamic_func);
                PROCESS_QUEUE
                    .lock()
                    .unwrap()
                    .add_to_queue(dynamic_func)
                    .expect("Error when add to queue");
                tx.send(false).unwrap();
                return;
            }
        }
        tx.send(true).unwrap();
        let cache_writer = cache_lock.unwrap();
        let dynamic_func = Arc::clone(&dynamic_func);
        dynamic_func.lock().unwrap()(cache_writer);
    });
    if rx.recv().unwrap() {
        Ok(())
    } else {
        bail!(InternalError::BusyCache)
    }
}

pub fn static_process<T: Clone, F: Fn(RwLockReadGuard<Cache>) -> T>(read_function: F) -> T {
    info!("start static process");
    let mut cache_reader = CACHE.try_read();
    if cache_reader.is_err() {
        thread::sleep(Duration::from_secs(3));
        cache_reader = CACHE.try_read();
    }
    let cache_reader = cache_reader.unwrap();
    read_function(cache_reader)
}

mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use chrono::{DateTime, Utc};

    use crate::core::processor::{CACHE, dynamic_process, static_process};
    use crate::model::event::Event;
    use crate::model::EventCommonTrait;

    #[test]
    fn test_dynamic_process() {
        let result = dynamic_process(|mut e| {
            let mut event = Event::init(None);
            event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
            e.insert_events(vec![Box::new(event)]).unwrap()
        });
        assert!(result.is_ok());
        sleep(Duration::from_secs(3));
        assert!(CACHE.read().unwrap().get_all_events::<Event>().len() > 0);
    }

    #[test]
    fn test_static_process() {
        let mut event = Event::init(None);
        let id = event.get_id();
        event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
        CACHE
            .write()
            .unwrap()
            .insert_events(vec![Box::new(event)])
            .unwrap();
        let result = static_process(move |e| e.get_all_events::<Event>());
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().get_id(), id);
    }
    //
    // #[test]
    // fn test_static_process_with_multiple_read() {
    //     let mut event = Event::init(None);
    //     let id = event.get_id();
    //     event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
    //     CACHE
    //         .write()
    //         .unwrap()
    //         .insert_events(vec![Box::new(event)])
    //         .unwrap();
    //     let first_result = CACHE.read().unwrap().get_events_by_id::<Event>(id);
    //     let result = static_process(|e| e.get_events_by_id::<Event>(id).unwrap().clone()).unwrap();
    //     assert_eq!(result.get_id(), id);
    // }
    //
    // #[test]
    // fn test_process_conflict() {
    //     let result = dynamic_process(|mut e| {
    //         sleep(Duration::from_secs(10));
    //     });
    //     assert!(result.is_ok());
    //     let result = dynamic_process(|mut e| {});
    //     assert!(result.is_err())
    // }
}
