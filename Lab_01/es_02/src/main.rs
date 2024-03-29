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

use board::{Board, Boat};
use clap::{Args, Parser, Subcommand};
use std::fs::{read_to_string, write};

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

    /// adds a new ship to an existing file
    Add(AddArgs),
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

#[derive(Args)]
struct AddArgs {
    /// the file where to save the grid
    fname: String,

    /// the size and orientation of the ship, in the format [1234][HV]
    /// 1/2/3/4 ==> length of the ship
    /// H/V ==> horizontal or vertical
    // maybe better?
    size_orientation: String,

    /// comma-separated list of 2 numbers, representing the row and column of the top-left corner of the ship
    top_left_corner: String,
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

        Commands::Add(AddArgs {
            fname,
            size_orientation,
            top_left_corner,
        }) => {
            // Remove from here if you fix the clap thing
            let tmp: Vec<&str> = top_left_corner.split(",").collect();
            let mut corner_vec: Vec<usize> = Vec::with_capacity(2);
            for n in tmp {
                corner_vec.push(n.parse().unwrap());
            }
            // remove up to here

            let mut board: Board;

            match read_to_string(&fname) {
                Err(error) => {
                    println!("{}", error);
                    return;
                }
                Ok(string) => board = Board::from(string),
            };

            let corner = (corner_vec[0], corner_vec[1]);
            let len = size_orientation.chars().nth(0).unwrap_or(' ') as usize - '0' as usize;
            let horiz_char = size_orientation.chars().nth(1).unwrap_or(' ');

            if horiz_char != 'H' && horiz_char != 'V' {
                println!("{} is neither H nor V", horiz_char);
                return;
            }

            if len != 1 && len != 2 && len != 3 && len != 4 {
                println!("{} is not within 1, 2, 3, 4", len);
                return;
            }

            let boat = if horiz_char == 'H' {
                Boat::Horizontal(len)
            } else {
                Boat::Vertical(len)
            };

            match board.add_boat(boat, corner) {
                Err(error) => {
                    println!("{}", error);
                    return;
                }
                Ok(b) => board = b,
            }

            match write(&fname, board.to_string()) {
                Err(error) => println!("{}", error),
                _ => println!("Updated file {}", fname),
            };
        }
    }
}
