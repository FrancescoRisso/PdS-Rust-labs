use std::{
    fs::{read_to_string, write},
    time::SystemTime,
};

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn print_error(e: Error) {
    use Error::*;

    match e {
        Simple(time) => println!("[{:?}] => Generic error", time),
        Complex(time, message) => println!("[{:?}] => {}", time, message),
    }
}

fn copy_repeat_file(file: &str) {
    match read_to_string(file) {
        Ok(string) => {
            let mut res = String::new();
            for _ in 0..10 {
                res.push_str(&string);
            }

            match write(file, res) {
                Err(error) => println!("{}", error),
                _ => {}
            }
        }
        Err(error) => println!("{}", error),
    }
}

struct Node {
    name: String,
    size: u32,
    count: u32,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            name,
            size: 0,
            count: 0,
        }
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    pub fn to_string(&self) -> String {
        let mut tmp = self.name.clone();
        tmp.push_str(format!(" size:{} count:{}", self.size, self.count).as_str());
        tmp
    }
}

fn main() {
    // use Error::*;
    // copy_repeat_file("../test.txt");
    // print_error(Simple(SystemTime::now()));
    // print_error(Complex(
    //     SystemTime::now(),
    //     "Boh I am testing stuff".to_string(),
    // ));

    let node = Node::new("nodo".to_string()).size(10).count(5);
    println!("{}", node.to_string())
}
