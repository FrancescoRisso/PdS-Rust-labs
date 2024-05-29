use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The array of numbers to be used
    values: Vec<u8>,
}

fn main() {
    let args = Args::parse();
    let nums = args.values;

    println!("{:?}", nums);
}
