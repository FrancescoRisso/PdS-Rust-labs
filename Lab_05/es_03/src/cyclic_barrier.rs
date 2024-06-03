use std::sync::{Condvar, Mutex};

pub struct CyclicBarrier {
    total: i32,
    arrived: Mutex<i32>,
    cond_var: Condvar,
}

impl CyclicBarrier {
    pub fn new(n: i32) -> Self {
        CyclicBarrier {
            total: n,
            arrived: Mutex::new(0),
            cond_var: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut arrived_lock = self.arrived.lock().unwrap();
        *arrived_lock += 1;

        if *arrived_lock == self.total {
            *arrived_lock = 0;
            self.cond_var.notify_all();
        } else {
            _ = self
                .cond_var
                .wait_while(arrived_lock, |arrived| *arrived != 0);
        }
    }
}
