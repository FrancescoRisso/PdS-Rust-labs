use std::time::SystemTime;

pub struct File {
    name: String,
    modified: SystemTime,
    content: Vec<u8>,
}

impl File {
    pub fn new(name: String) -> Self {
        File {
            name: name,
            modified: SystemTime::now(),
            content: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn test_file() -> Self {
        File::new("testFile".to_string())
    }
}
