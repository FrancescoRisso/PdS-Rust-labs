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
        match self
            .children
            .iter()
            .filter(|child| child.get(path).is_ok())
            .next()
        {
            None => Err(FSError::NotFound),
            Some(node) => Ok(node),
        }
    }

    pub fn get_mut<'a, 'b>(&'a mut self, path: &'b str) -> Result<&'a mut Node, FSError> {
        match self
            .children
            .iter_mut()
            .filter(|child| child.get(path).is_ok())
            .next()
        {
            None => Err(FSError::NotFound),
            Some(node) => Ok(node),
        }
    }

    pub fn mkdir(&mut self, name: String) -> &mut Node {
        self.children.push(Node::Dir(Dir::new(name)));
        self.children.last_mut().unwrap()
    }
}
