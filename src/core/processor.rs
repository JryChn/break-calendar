use std::sync::Arc;
use std::time::Duration;

use anyhow::bail;
use anyhow::Result;
use futures::executor::block_on;
use lazy_static::lazy_static;
use tokio::spawn;
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use tokio::time::timeout;
use tracing::{info, warn};

use crate::cache::Cache;
use crate::common::exception::InternalError;
use crate::persistent::Persistent;

lazy_static! {
    static ref PERSISTENT_SYSTEM: Mutex<Persistent> = Mutex::new(Persistent::init());
    static ref CACHE: Arc<RwLock<Cache>> = {
        let cache = block_on(block_on(PERSISTENT_SYSTEM.lock()).load());
        Arc::new(RwLock::new(if cache.is_ok() {
            cache.unwrap()
        } else {
            Cache::init()
        }))
    };
}

pub async fn dynamic_process<F: FnOnce(RwLockWriteGuard<Cache>) + Send + 'static>(
    process_func: F,
) -> Result<()> {
    info!("start dynamic process");
    let result = async {
        info!("try to get cache write lock");
        let cache = CACHE.write().await;
        info!("get cache write lock success");
        async {
            process_func(cache);
        }.await
    };
    match timeout(Duration::from_secs(3), result).await {
        Ok(_) => {
            info!("dynamic process success");
            let cache_replicas = CACHE.clone();
            spawn(async move {
                let cache = cache_replicas.read().await;
                let persistent = PERSISTENT_SYSTEM.lock().await;
                persistent
                    .save(&*cache)
                    .await
                    .expect("save cache failed");
                drop(cache);
            });
            Ok(())
        }
        Err(_) => {
            warn!("dynamic process timeout");
            bail!(InternalError::BusyCache)
        }
    }
}

pub async fn static_process<T: Clone, F: Fn(RwLockReadGuard<Cache>) -> T>(
    read_function: F,
) -> Result<T> {
    info!("start static process");
    let cache_reader = CACHE.read().await;
    Ok(read_function(cache_reader))
}

mod tests {
    use std::thread;
    use std::time::Duration;

    use chrono::{DateTime, Utc};
    use futures::executor::block_on;
    use tokio::join;
    use tokio::time::sleep;

    use crate::core::processor::{CACHE, dynamic_process, static_process};
    use crate::model::event::Event;
    use crate::model::EventCommonTrait;

    #[tokio::test]
    async fn test_dynamic_process() {
        let result = dynamic_process(|mut e| {
            let mut event = Event::init(None);
            event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
            e.insert_events(vec![Box::new(event)]).unwrap()
        });
        assert!(result.await.is_ok());
        assert!(CACHE.read().await.get_all_events::<Event>().len() > 0);
    }

    #[tokio::test]
    async fn test_static_process() {
        let mut event = Event::init(None);
        let id = event.get_id();
        event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
        CACHE
            .write()
            .await
            .insert_events(vec![Box::new(event)])
            .unwrap();
        let result = static_process(move |e| {
            e.get_all_events::<Event>()
                .iter()
                .map(|e| {
                    let e = **e.clone();
                    let e = e.clone();
                    e
                })
                .collect::<Vec<Event>>()
        })
            .await;
        let result = result.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().get_id(), id);
    }

    #[tokio::test]
    async fn test_static_process_with_multiple_read() {
        let mut event = Event::init(None);
        let id = event.get_id();
        event.set_duration(DateTime::from(Utc::now()), DateTime::from(Utc::now()));
        CACHE
            .write()
            .await
            .insert_events(vec![Box::new(event)])
            .unwrap();
        let result = dynamic_process(|_| {
            block_on(sleep(Duration::from_secs(10)));
        })
            .await;
        assert!(result.is_ok());
        let first_result = static_process(|e| {
            e.get_all_events::<Event>()
                .iter()
                .map(|e| {
                    let e = **e.clone();
                    let e = e.clone();
                    e
                })
                .collect::<Vec<Event>>()
        })
            .await;
        assert!(first_result.is_ok());
        let result = static_process(|e| {
            e.get_events_by_id::<Event>(id)
                .map(|e| {
                    let e = **e.clone();
                    let e = e.clone();
                    e
                })
                .unwrap()
        })
            .await
            .unwrap();
        assert_eq!(result.get_id(), id);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_process_conflict() {
        let result_0 = dynamic_process(|_| {
            thread::sleep(Duration::from_secs(10));
        });
        let result_1 = dynamic_process(|_| {});
        let (a, b) = join!(result_0, result_1);
        assert!(a.is_ok());
        assert!(b.is_err());
    }
}
