use std::fmt::format;

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

    let res = check_and_print(&all_possibilities[..]);
    println!("{:?}", res);
}

fn check_and_print(vals: &[(Vec<u8>, Vec<char>)]) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for val in vals {
        match check_and_print_single(&val.0, &val.1) {
            Err(_) => {}
            Ok(res) => result.push(res),
        }
    }

    result
}

fn check_and_print_single(nums: &Vec<u8>, signs: &Vec<char>) -> Result<String, ()> {
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
        let mut res = format!("{}", nums[0]);
        for i in 0..signs.len() {
            res = format!("{}{}{}", res, signs[i], nums[i + 1]);
        }
        return Ok(format!("{}=10", res));
    }

    return Err(());
}
