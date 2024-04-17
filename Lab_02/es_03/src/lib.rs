use std::ops::Index;

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

    pub fn size(&self) -> usize {
        self.size
    }

    // può essere usata quando il buffer è pieno per forzare una
    // scrittura riscrivendo l’elemento più vecchio
    pub fn overwrite(&mut self, item: T) {
        if self.write(item).is_err() {
            self.read();
            let _ = self.write(item);
        }
    }

    // vedi sotto*
    pub fn make_contiguos(&mut self) {
        if self.head == 0 {
            return;
        }

        let tmp = self.buf[0];
        for i in 1..self.size {
            self.buf[i - 1] = self.buf[i];
        }
        self.buf[self.size - 1] = tmp;

        self.head -= 1;

        self.make_contiguos();
    }

    pub fn get_head(&self) -> usize {
        self.head
    }
}

impl<T> Index<usize> for CircularBuffer<T>
where
    T: Default + Copy,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= (self.size - self.free) {
            panic!()
        }

        &self.buf[(self.head + index) % self.size]
    }
}
