use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    mutex: Mutex<isize>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(initial_value: isize) -> Self {
        Self {
            mutex: Mutex::new(initial_value),
            condvar: Condvar::new(),
        }
    }

    pub fn signal(&self) {
        let mut counter = self.mutex.lock().unwrap();
        *counter += 1;

        self.condvar.notify_one();
    }

    pub fn wait(&self) {
        let mut counter = self.mutex.lock().unwrap();
        if *counter == 0 {
            counter = self.condvar.wait(counter).unwrap();
        }

        *counter -= 1;
    }
}
