use crate::waiter::Waiter;

pub struct CyclicBarrier {}

impl CyclicBarrier {
    pub fn new(n: isize) -> Self {
        Self {}
    }

    pub fn get_waiter(&self) -> Waiter {
        Waiter {}
    }
}
