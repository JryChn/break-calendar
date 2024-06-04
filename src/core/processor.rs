use std::sync::{Arc, mpsc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::Duration;

use actix_web::rt::time::sleep;
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
    static ref CACHE: Mutex<RwLock<Cache>> = {
        let cache = PERSISTENT_SYSTEM.load();
        Mutex::new(RwLock::new(if cache.is_ok() {
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
        let mut cache_lock = CACHE.try_lock();
        if cache_lock.is_err() {
            thread::sleep(Duration::from_secs(3));
            cache_lock = CACHE.try_lock();
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
        let cache_lock = cache_lock.unwrap();
        let mut cache_writer = cache_lock.try_write();
        if cache_writer.is_err() {
            thread::sleep(Duration::from_secs(3));
            cache_writer = cache_lock.try_write();
            if cache_writer.is_err() {
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
        let cache_writer = cache_writer.unwrap();
        let dynamic_func = Arc::clone(&dynamic_func);
        dynamic_func.lock().unwrap()(cache_writer);
    });
    if rx.recv().unwrap() {
        Ok(())
    } else {
        bail!(InternalError::BusyCache)
    }
}

pub fn static_process<T, F: FnOnce(RwLockReadGuard<Cache>) -> T>(read_function: F) -> Result<T> {
    info!("start static process");
    let cache_lock = CACHE.lock().unwrap();
    let cache_reader = cache_lock.read().unwrap();
    Ok(read_function(cache_reader))
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
            let mut event = Event::init();
            event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
            e.set_events(vec![event.clone()]).unwrap()
        });
        assert!(result.is_ok());
        sleep(std::time::Duration::from_secs(3));
        assert!(
            CACHE
                .lock()
                .unwrap()
                .read()
                .unwrap()
                .get_events::<Utc>(Some(Utc::now()), None)
                .len()
                > 0
        );
    }

    #[test]
    fn test_static_process() {
        let mut event = Event::init();
        event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
        CACHE
            .lock()
            .unwrap()
            .write()
            .unwrap()
            .set_events(vec![event.clone()])
            .unwrap();
        let result = static_process(|e| e.get_events::<Utc>(None, Some(event.get_id()))).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().get_id(), event.get_id());
    }

    #[test]
    fn test_static_process_with_multiple_read() {
        let mut event = Event::init();
        event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
        CACHE
            .lock()
            .unwrap()
            .write()
            .unwrap()
            .set_events(vec![event.clone()])
            .unwrap();
        let first_result = CACHE
            .lock()
            .unwrap()
            .read()
            .unwrap()
            .get_events::<Utc>(None, Some(event.clone().get_id()));
        let result = static_process(|e| e.get_events::<Utc>(None, Some(event.get_id()))).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().get_id(), event.get_id());
        println!("{}", first_result.first().unwrap().get_id());
    }

    #[test]
    fn test_process_conflict() {
        let result = dynamic_process(|mut e| {
            sleep(Duration::from_secs(10));
        });
        assert!(result.is_ok());
        let result = dynamic_process(|mut e| {});
        assert!(result.is_err())
    }
}
