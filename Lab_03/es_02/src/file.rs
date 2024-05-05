use std::time::SystemTime;

pub struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

impl File {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
