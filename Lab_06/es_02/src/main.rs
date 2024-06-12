mod semaphore;
mod thread_pool;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use thread_pool::ThreadPool;

fn main() {
    // alloca i worker
    let threadpool = ThreadPool::new(10);
    let total = Arc::new(Mutex::new(0));

    for x in 0..100 {
        let total = total.clone();
        threadpool.execute(Box::new(move || {
            {
                let mut total = total.lock().unwrap();
                *total += x;
                println!("long running task {} (sum up to now: {})", x, total);
            }
            thread::sleep(Duration::from_millis(1000))
        }))
    }

    threadpool.stop();
}
