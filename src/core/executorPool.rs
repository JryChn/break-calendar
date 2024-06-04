use lazy_static::lazy_static;
use threadpool::ThreadPool;

lazy_static! {
    pub static ref EXECUTOR_POOL: ThreadPool =
        ThreadPool::with_name("core executor pool".to_string(), num_cpus::get());
}
