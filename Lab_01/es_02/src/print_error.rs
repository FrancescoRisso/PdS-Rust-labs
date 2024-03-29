use std::time::SystemTime;

pub enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

pub fn print_error(e: Error) {
    use Error::*;

    match e {
        Simple(time) => println!("[{:?}] => Generic error", time),
        Complex(time, message) => println!("[{:?}] => {}", time, message),
    }
}
