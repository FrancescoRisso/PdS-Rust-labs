use copy_repeat_file::copy_repeat_file;
use node::Node;
use print_error::{print_error, Error::*};
use std::time::SystemTime;

mod copy_repeat_file;
mod node;
mod print_error;

fn main() {
    copy_repeat_file("../test.txt");

    print_error(Simple(SystemTime::now()));
    print_error(Complex(
        SystemTime::now(),
        "Boh I am testing stuff".to_string(),
    ));

    let node = Node::new("nodo".to_string()).size(10).count(5);
    println!("{}", node.to_string())
}
