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
    pub fn switch(&mut self) -> Result<(), String> {
        match self.function {
            NodeFunction::Generator(true) => self.function = NodeFunction::Generator(false),
            NodeFunction::Generator(false) => self.function = NodeFunction::Generator(true),
            NodeFunction::Switch(true) => self.function = NodeFunction::Switch(false),
            NodeFunction::Switch(false) => self.function = NodeFunction::Switch(true),
            NodeFunction::Light => return Err("Cannot switch a light".to_string()),
        }

        Ok(())
    }

    pub fn add_out(&mut self, out: NodeLink) {
        let index = match self.outs {
            [None, None] => 0,
            [Some(_), None] => 1,
            _ => return,
        };

        self.outs[index] = out;
    }

    pub fn update_parent(&mut self, tree: &CircuitTree) {
        match &self.parent_name {
            None => {}
            Some(str) => match str.as_str() {
                "-" => {}
                name => {
                    self.parent = match tree.get(name) {
                        Some(node) => Some(Rc::downgrade(&node)),
                        None => None,
                    }
                }
            },
        };
    }

    pub fn get_status(&mut self) -> Option<bool> {
        match self.function {
            NodeFunction::Generator(state) => Some(state),
            NodeFunction::Switch(state) => Some(state),
            NodeFunction::Light => Some(self.get_chain_status()?),
        }
    }

    pub fn get_chain_status(&self) -> Option<bool> {
        match self.function {
            NodeFunction::Generator(true) => Some(true),
            NodeFunction::Generator(false) => Some(false),
            NodeFunction::Switch(false) => Some(false),
            _ => {
                // this is either a light or an "onS switch, that simply relay the parent's state
                match &self.parent {
                    None => None,
                    Some(parent_weak) => match parent_weak.upgrade() {
                        None => None,
                        Some(parent) => parent.as_ref().borrow().get_chain_status(),
                    },
                }
            }
        }
    }

    pub fn get_parent(&self) -> NodeLink {
        self.parent.clone()?.upgrade()
    }

    pub fn get_out(&self, index: usize) -> NodeLink {
        match index {
            0..=1 => self.outs[index].clone(),
            _ => None,
        }
    }

    pub fn encapsulate(self) -> NodeLink {
        Some(Rc::new(RefCell::new(self)))
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_parent_name(&self) -> String {
        self.parent_name.clone().unwrap_or("".to_string())
    }

    pub fn is_light(&self) -> bool {
        self.function == NodeFunction::Light
    }
}
