// use copy_repeat_file::copy_repeat_file;
// use node::Node;
// use print_error::{print_error, Error::*};
// use std::time::SystemTime;

// mod copy_repeat_file;
// mod node;
// mod print_error;

// fn main() {
//     copy_repeat_file("../test.txt");

//     print_error(Simple(SystemTime::now()));
//     print_error(Complex(
//         SystemTime::now(),
//         "Boh I am testing stuff".to_string(),
//     ));

//     let node = Node::new("nodo".to_string()).size(10).count(5);
//     println!("{}", node.to_string())
// }

use board::Board;
use clap::{
    error::{Error, ErrorKind},
    Args, Command, Parser, Subcommand,
};
use std::fs::write;

mod board;

#[derive(Parser)]
struct MyArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// creates a new file with a given set of boats
    New(NewArgs),
}

#[derive(Args)]
struct NewArgs {
    /// the file where to save the grid
    fname: String,

    /// comma-separated list of 4 numbers, representing the number of 1-, 2-, 3-, 4-long ships
    // #[arg(num_args = 4, value_delimiter = ',')]
    // ships: Vec<u8>,
    ships: String,
}

fn main() {
    match MyArgs::parse().command {
        Commands::New(NewArgs { fname, ships }) => {
            // Remove from here if you fix the clap thing
            let tmp: Vec<&str> = ships.split(",").collect();
            let mut ships: Vec<u8> = Vec::with_capacity(4);
            for n in tmp {
                ships.push(n.chars().nth(0).unwrap_or('0') as u8 - '0' as u8)
            }
            // remove up to here

            match write(&fname, Board::new(&ships[0..4]).to_string()) {
                Err(error) => println!("{}", error),
                _ => println!("Created file {}", fname),
            };
        }
    }
}
