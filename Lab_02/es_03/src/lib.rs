pub struct CircularBuffer<T>
where
    T: Default + Copy,
{
    buf: Vec<T>,
    head: usize,
    tail: usize,
    free: usize,
    size: usize,
}

impl<T> CircularBuffer<T>
where
    T: Default + Copy,
{
    pub fn new(capacity: usize) -> Self {
        let mut buf: Vec<T> = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buf.push(T::default());
        }

        CircularBuffer {
            buf: buf,
            head: 0,
            tail: 0,
            free: capacity,
            size: capacity,
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), ()> {
        if self.free == 0 {
            return Err(());
        }

        self.buf[self.tail] = item;
        self.tail = (self.tail + 1) % self.size;
        self.free -= 1;

        Ok(())
    }

    pub fn read(&mut self) -> Option<T> {
        if self.free == self.size {
            return None;
        }

        let val = self.buf[self.head];
        self.head = (self.head + 1) % self.size;
        self.free += 1;

        Some(val)
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.free = self.size;
    }

    // pub dn size(&self) -> usize;
    // // può essere usata quando il buffer è pieno per forzare una
    // // scrittura riscrivendo l’elemento più vecchio
    // pub fn overwrite(&mut self, item: ...) {};
    // // vedi sotto*
    // pub fn make_contiguos(&mut self) {};
}
