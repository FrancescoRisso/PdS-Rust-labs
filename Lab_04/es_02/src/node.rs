use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

pub enum NodeFunction {
    Generator(bool),
    Switch(bool),
    Light,
}

pub type NodeLink = Option<Rc<RefCell<Node>>>;

pub struct Node {
    name: String,
    function: NodeFunction,
    // which type for parent?
    // PhantomData is just a placeholder to let it compile
    parent: PhantomData<Node>,
    outs: [NodeLink; 2],
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        unimplemented!()
    }
}

impl Node {
    // turn on or off the switch or the generator, if it's a light return an error
    pub fn switch(&mut self) /*add return */
    {
        unimplemented!()
    }
}
