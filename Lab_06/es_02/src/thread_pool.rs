use std::{
    sync::{Arc, Mutex},
    thread, vec,
};

use crate::semaphore::Semaphore;

pub struct ThreadPool {
    tasks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
    queue_sem: Arc<Semaphore>,
}

impl ThreadPool {
    pub fn new(num_threads: isize) -> Self {
        let tasks = Arc::new(Mutex::new(vec![]));
        let queue_sem = Arc::new(Semaphore::new(0));

        for _ in 0..num_threads {
            let tasks = tasks.clone();
            let queue_sem = queue_sem.clone();

            thread::spawn(move || loop {
                queue_sem.wait();

                let task: Box<dyn FnOnce() + Send> = {
                    let mut tasks = tasks.lock().unwrap();
                    tasks.pop().unwrap()
                };

                task();
            });
        }

        Self { tasks, queue_sem }
    }

    pub fn execute(&self, func: Box<dyn FnOnce() + Send>) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(func);
        self.queue_sem.signal();
    }
}
