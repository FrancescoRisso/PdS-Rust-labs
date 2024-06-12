use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    vec,
};

use crate::semaphore::Semaphore;

type Task = Option<Box<dyn FnOnce() + Send>>;

pub struct ThreadPool {
    tasks: Arc<Mutex<VecDeque<Task>>>,
    queue_sem: Arc<Semaphore>,
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: isize) -> Self {
        let tasks: Arc<Mutex<VecDeque<Task>>> = Arc::new(Mutex::new(VecDeque::new()));
        let queue_sem = Arc::new(Semaphore::new(0));
        let mut threads = vec![];

        for _ in 0..num_threads {
            let tasks = tasks.clone();
            let queue_sem = queue_sem.clone();

            let join_handle = thread::spawn(move || loop {
                queue_sem.wait();

                let task: Box<dyn FnOnce() + Send> = {
                    let mut tasks = tasks.lock().unwrap();

                    match tasks.pop_front().unwrap() {
                        None => break,
                        Some(task) => task,
                    }
                };

                task();
            });

            threads.push(join_handle);
        }

        Self {
            tasks,
            queue_sem,
            threads,
        }
    }

    pub fn execute(&self, func: Box<dyn FnOnce() + Send>) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push_back(Some(func));
        self.queue_sem.signal();
    }

    pub fn stop(self) {
        {
            let mut tasks = self.tasks.lock().unwrap();

            for _ in &self.threads {
                tasks.push_back(None);
                self.queue_sem.signal();
            }
        }

        for thread in self.threads {
            _ = thread.join();
        }
    }
}
