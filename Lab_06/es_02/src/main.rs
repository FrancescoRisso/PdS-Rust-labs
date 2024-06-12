mod semaphore;
// mod thread_pool;

use semaphore::Semaphore;
use std::{sync::Arc, thread, time::Duration};
// use thread_pool::ThreadPool;

// fn main() {
//     // alloca i worker
//     let threadpool = ThreadPool::new(10);
//     for x in 0..100 {
//         threadpool.execute(Box::new(move || {
//             println!("long running task {}", x);
//             thread::sleep(Duration::from_millis(1000))
//         }))
//     }
//     // just to keep the main thread alive
//     loop {
//         thread::sleep(Duration::from_millis(1000))
//     }
// }

fn main() {
    let sem = Arc::new(Semaphore::new(1));
    let sem2 = sem.clone();

    let t1 = std::thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(2000));
            sem.signal();
        }
    });

    let t2 = std::thread::spawn(move || {
        for i in 0..10 {
			sem2.wait();
            println!("{i}");
        }
    });

    _ = t1.join();
    _ = t2.join();
}
