use crate::node::{Node, NodeLink};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
pub struct CircuitTree {
    // choose the right type for root and names
    root: NodeLink,
    names: HashMap<String, NodeLink>,
}

impl From<&str> for CircuitTree {
    fn from(value: &str) -> Self {
        let mut res = CircuitTree::new();

        for line in value.lines() {
            let node: Node = line.into();
            let parent_name = node.get_parent_name();

            res.add(parent_name.as_str(), node);
        }

        res
    }
}

impl CircuitTree {
    pub fn new() -> Self {
        CircuitTree {
            root: None,
            names: HashMap::new(),
        }
    }

    pub fn with_values(root: NodeLink, names: HashMap<String, NodeLink>) -> Self {
        CircuitTree { root, names }
    }

    // get a node by name
    pub fn get(&self, name: &str) -> NodeLink {
        match self.names.get(name) {
            None => None,
            Some(res) => res.clone(),
        }
    }

    // add a new node
    pub fn add(&mut self, parent_name: &str, mut node: Node) {
        node.update_parent(&self);

        let node_name = node.get_name();
        let node_link = node.encapsulate();

        if self.root.is_none() {
            self.root = node_link.clone();
        }

        match self.get(&parent_name) {
            None => {}
            Some(parent) => parent.as_ref().borrow_mut().add_out(node_link.clone()),
        }

        self.names.insert(node_name, node_link);
    }

    // is the light on? Error if it's not a light
    pub fn light_status(&self, name: &str) -> Result<bool, String> {
        match self.get(name) {
            None => Err("Node not found".to_string()),
            Some(node_link) => {
                let node_ptr = node_link.as_ref().borrow();
                match node_ptr.is_light() {
                    false => Err("Node is not light".to_string()),
                    true => Ok(node_ptr
                        .get_chain_status()
                        .expect("Problems in the circuit")),
                }
            }
        }
    }

    pub fn turn_light_on(&self, name: &str) {
        match self.get(name) {
            None => {}
            Some(node_link) => {
                let mut node_ptr = node_link.as_ref().borrow_mut();
                match node_ptr.is_light() {
                    false => {}
                    true => node_ptr.turn_on_chain(),
                }
            }
        };
    }

    pub fn get_node_names(&self) -> HashSet<String> {
        self.names.keys().cloned().collect()
    }
}
