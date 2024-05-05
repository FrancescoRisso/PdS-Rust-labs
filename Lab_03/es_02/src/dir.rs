use crate::fs_error::FSError;
use crate::node::Node;
use std::time::SystemTime;

#[derive(PartialEq)]
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

    pub fn mkfile(&mut self, f: Node) -> &mut Node {
        self.children.push(f);
        self.children.last_mut().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.children.len() == 0
    }

    pub fn rm(&mut self, node_name: String) -> Result<Node, FSError> {
        let index = match self
            .children
            .iter()
            .position(|node| node.name() == node_name)
        {
            Some(index) => index,
            None => return Err(FSError::NotFound),
        };

        if (&self.children[index])
            .try_into()
            .is_ok_and(|dir: &Dir| !dir.is_empty())
        {
            return Err(FSError::DirNotEmpty);
        }

        Ok(self.children.swap_remove(index))
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
