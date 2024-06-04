use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLockWriteGuard};

use anyhow::Result;
use lazy_static::lazy_static;

use crate::cache::Cache;

lazy_static! {
    pub static ref PROCESS_QUEUE: Mutex<ProcessQueue> = Mutex::new(ProcessQueue::new());
}

pub struct ProcessQueue {
    dynamic_process:
    Mutex<VecDeque<Arc<Mutex<dyn FnOnce(RwLockWriteGuard<Cache>) + Send + 'static>>>>,
}

impl ProcessQueue {
    fn new() -> Self {
        ProcessQueue {
            dynamic_process: Mutex::new(VecDeque::new()),
        }
    }
    pub fn add_to_queue<F: FnOnce(RwLockWriteGuard<Cache>) + Send + Copy + 'static>(
        &mut self,
        func: Arc<Mutex<F>>,
    ) -> Result<()> {
        self.dynamic_process.lock().unwrap().push_back(func);
        Ok(())
    }
}
