use std::sync::mpsc::{Receiver, Sender};

pub struct Waiter {
    txs: Vec<Sender<bool>>,
    rx: Receiver<bool>,
}

impl Waiter {
    pub fn new(rx: Receiver<bool>, txs: Vec<Sender<bool>>) -> Self {
        Self { rx, txs }
    }

    pub fn wait(&self) {
        for tx in &self.txs {
            _ = tx.send(true);
        }

        for _ in &self.txs {
            _ = self.rx.recv();
        }
    }
}
