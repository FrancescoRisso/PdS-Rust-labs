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
    // which type for parent?
    // PhantomData is just a placeholder to let it compile
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
        _ = value;
        unimplemented!()
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
