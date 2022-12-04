use std::fmt::Display;


pub enum AOCError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for AOCError {
    fn from(e: std::io::Error) -> Self {
        AOCError::IoError(e)
    }
}

pub fn print_answer<T: Display>(f: fn(&str) -> T, input: &str) {
    println!("Answer: {}", f(input));
}