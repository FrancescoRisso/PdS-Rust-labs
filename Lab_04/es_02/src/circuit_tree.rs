use crate::node::{Node, NodeLink};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct CircuitTree {
    // choose the right type for root and names
    root: NodeLink,
    names: HashMap<String, NodeLink>,
}

impl From<&str> for CircuitTree {
    fn from(value: &str) -> Self {
        _ = value;
        unimplemented!()
    }
}

impl CircuitTree {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn with_values(root: NodeLink, names: HashMap<String, NodeLink>) -> Self {
        CircuitTree { root, names }
    }

    // get a node by name
    pub fn get(&self, name: &str) -> NodeLink {
        _ = self.root;
        _ = self.names;
        _ = name;
        unimplemented!()
    }

    // add a new node
    pub fn add(&mut self, parent_name: &str, node: Node) {
        _ = parent_name;
        _ = node;
        unimplemented!()
    }

    // is the light on? Error if it's not a light
    pub fn light_status(&self, name: &str) -> Result<bool, String> {
        _ = name;
        unimplemented!();
    }

    pub fn turn_light_on(&self, name: &str) {
        _ = name;
        unimplemented!();
    }

    pub fn get_node_names(&self) -> Vec<String> {
        self.names.keys().cloned().collect()
    }
}
