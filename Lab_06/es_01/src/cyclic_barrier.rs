use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex,
};

use crate::waiter::Waiter;

pub struct CyclicBarrier {
    waiters: Mutex<Vec<Waiter>>,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        let mut txs: Vec<Sender<bool>> = vec![];
        let mut rxs: Vec<Receiver<bool>> = vec![];

        for _ in 0..n {
            let (tx, rx) = channel::<bool>();
            txs.push(tx);
            rxs.push(rx);
        }

        Self {
            waiters: Mutex::new(
                rxs.into_iter()
                    .map(|rx| Waiter::new(rx, txs.clone()))
                    .collect(),
            ),
        }
    }

    pub fn get_waiter(&self) -> Waiter {
        self.waiters.lock().unwrap().pop().unwrap()
    }
}
