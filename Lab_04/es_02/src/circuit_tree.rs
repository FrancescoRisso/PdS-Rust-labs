use crate::node::{Node, NodeLink};
use std::marker::PhantomData;

pub struct CircuitTree {
    // choose the right type for root and names
    root: PhantomData<Node>,
    names: PhantomData<Node>,
}

impl CircuitTree {
    pub fn new() -> Self {
        unimplemented!()
    }

    // get a node by name
    pub fn get(&self, name: &str) -> NodeLink {
        unimplemented!()
    }

    // add a new node
    pub fn add(&mut self, parent_name: &str, node: Node) {
        unimplemented!()
    }

    // is the light on? Error if it's not a light
    pub fn light_status(&self, name: &str) -> Result<(), String> {
        unimplemented!();
    }

    pub fn turn_light_on(&self, name: &str) {
        unimplemented!();
    }
}
