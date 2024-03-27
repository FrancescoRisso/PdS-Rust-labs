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

fn main() {
    use Error::*;
    copy_repeat_file("../test.txt");
    print_error(Simple(SystemTime::now()));
    print_error(Complex(
        SystemTime::now(),
        "Boh I am testing stuff".to_string(),
    ));
}
