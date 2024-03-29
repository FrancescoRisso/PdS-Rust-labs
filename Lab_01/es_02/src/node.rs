pub struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            name,
            size: 0,
            count: 0,
        }
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn to_string(&self) -> String {
        let mut tmp = self.name.clone();
        tmp.push_str(format!(" size:{} count:{}", self.size, self.count).as_str());
        tmp
    }
}
