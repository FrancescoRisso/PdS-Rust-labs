use std::{collections::HashSet, sync::Mutex, time::Instant};

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

    let mut now = Instant::now();
    let res = check_and_print(&all_possibilities[..]);
    println!("No thread took {:?}", now.elapsed());

    for num_threads in 1..10 {
        let slice_len = all_possibilities.len() / num_threads;
        let mut res_threads = HashSet::<String>::new();
        let res_threads_mut = Mutex::new(&mut res_threads);

        let mut slices = vec![];
        for i in 0..num_threads {
            slices.push(&all_possibilities[(i * slice_len)..((i + 1) * slice_len)]);
        }

        now = Instant::now();

        std::thread::scope(|s| {
            for slice in slices {
                s.spawn(|| {
                    let res_single = check_and_print(slice);
                    res_threads_mut.lock().unwrap().extend(res_single);
                });
            }
        });

        print!("{} threads took {:?} ", num_threads, now.elapsed());
        match res == res_threads {
            true => println!("and provided a correct result"),
            false => println!("but provided a wrong result"),
        }
    }
}

fn check_and_print(vals: &[(Vec<u8>, Vec<char>)]) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();

    for val in vals {
        match check_and_print_single(&val.0, &val.1) {
            Err(_) => {}
            Ok(res) => {
                result.insert(res);
            }
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
