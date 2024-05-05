use crate::fs_error::FSError;
use crate::node::Node;
use std::time::SystemTime;

pub struct Dir {
    name: String,
    modified: SystemTime,
    children: Vec<Node>,
}

impl Dir {
    pub fn new(name: String) -> Self {
        Dir {
            name: name,
            modified: SystemTime::now(),
            children: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get<'a, 'b>(&'a self, path: &'b str) -> Result<&'a Node, FSError> {
        for child in &self.children {
            match child.get(path) {
                Ok(node) => return Ok(node),
                _ => {}
            };
        }

        Err(FSError::NotFound)
    }

    pub fn get_mut<'a, 'b>(&'a mut self, path: &'b str) -> Result<&'a mut Node, FSError> {
        for child in &mut self.children {
            match child.get_mut(path) {
                Ok(node) => return Ok(node),
                _ => {}
            };
        }

        Err(FSError::NotFound)
    }

    pub fn mkdir(&mut self, name: String) -> &mut Node {
        self.children.push(Node::Dir(Dir::new(name)));
        self.children.last_mut().unwrap()
    }

    pub fn test_root_dir_with_subdir() -> Self {
        let mut tmp = Dir::new("".to_string());
        tmp.children.push(Node::test_dir_node());
        tmp.children.push(Node::test_file_node());
        tmp
    }

    pub fn test_dir_with_file() -> Self {
        let mut tmp = Dir::new("testDir".to_string());
        tmp.children.push(Node::test_file_node());
        tmp
    }
}