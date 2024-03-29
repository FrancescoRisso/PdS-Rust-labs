use std::fs::{read_to_string, write};

pub fn copy_repeat_file(file: &str) {
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
