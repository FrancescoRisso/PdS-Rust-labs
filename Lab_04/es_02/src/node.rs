use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::circuit_tree::CircuitTree;

#[derive(PartialEq, Debug)]
pub enum NodeFunction {
    Generator(bool),
    Switch(bool),
    Light,
}

pub type NodeLink = Option<Rc<RefCell<Node>>>;
pub type NodeBackLink = Option<Weak<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    name: String,
    function: NodeFunction,
    parent_name: Option<String>,
    parent: NodeBackLink,
    outs: [NodeLink; 2],
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.function == other.function
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut iter = value.split(" ");

        let node_type = iter.next().unwrap_or("G");
        let name = iter.next().unwrap_or("").to_string();
        let parent = iter.next().unwrap_or("-").to_string();
        let state_str = iter.next().unwrap_or("off");

        let state_bool = match state_str {
            "on" => true,
            _ => false,
        };

        let function = match node_type {
            "G" => NodeFunction::Generator(state_bool),
            "S" => NodeFunction::Switch(state_bool),
            _ => NodeFunction::Light,
        };

        let mut res = Node::without_links(name, function);
        res.parent_name = Some(parent);

        res
    }
}

impl Node {
    pub fn new(
        name: String,
        function: NodeFunction,
        parent: NodeBackLink,
        outs: [NodeLink; 2],
    ) -> Self {
        Node {
            name,
            function,
            parent_name: None,
            parent,
            outs,
        }
    }

    pub fn without_links(name: String, function: NodeFunction) -> Self {
        Node::new(name, function, None, [None, None])
    }

    // turn on or off the switch or the generator, if it's a light return an error
    pub fn switch(&mut self) /*add return */
    {
        unimplemented!()
    }

    pub fn update_parent(&mut self, tree: CircuitTree) {
        _ = tree;
        unimplemented!()
    }

    pub fn get_status(&mut self) -> Option<bool> {
        _ = self.parent;
        _ = self.outs;
        unimplemented!()
    }

    pub fn get_parent(&self) -> NodeLink {
        self.parent.clone()?.upgrade()
    }
}
