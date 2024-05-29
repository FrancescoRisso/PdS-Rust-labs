use clap::Parser;
use itertools::{iproduct, Itertools};

#[derive(Parser, Debug)]
struct Args {
    /// The array of numbers to be used
    values: Vec<u8>,
}

fn main() {
    let args = Args::parse();
    let values = args.values;
    let num_values = values.len();

    let num_perm: Vec<Vec<u8>> = values.into_iter().permutations(num_values).collect();
    let ops: Vec<Vec<char>> = ['+', '-', '*', '/']
        .into_iter()
        .combinations_with_replacement(num_values - 1)
        .collect();

    let all_possibilities: Vec<(Vec<u8>, Vec<char>)> =
        iproduct!(num_perm.into_iter(), ops.into_iter()).collect();

    check_and_print(&all_possibilities[..])
}

fn check_and_print(vals: &[(Vec<u8>, Vec<char>)]) {
    for val in vals {
        _ = check_and_print_single(&val.0, &val.1);
    }
}

fn check_and_print_single(nums: &Vec<u8>, signs: &Vec<char>) -> Result<(), ()> {
    let mut nums_iter = nums.iter();
    let mut partial: isize = *nums_iter.next().unwrap() as isize;

    for sign in signs {
        let new = *nums_iter.next().unwrap() as isize;
        partial = match sign {
            '+' => partial + new,
            '-' => partial - new,
            '*' => partial * new,
            '/' => {
                if new == 0 {
                    return Err(());
                }

                if (partial / new) * new != partial {
                    return Err(());
                }

                partial / new
            }
            _ => partial,
        }
    }

    if partial == 10 {
        print!("> {}", nums[0]);
        for i in 0..signs.len() {
            print!(" {} {}", signs[i], nums[i + 1]);
        }
        println!("")
    }

    return Ok(());
}
