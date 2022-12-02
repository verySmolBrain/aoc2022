use std::fmt::Display;

pub fn print_answer<T: Display>(f: fn(&str) -> T, input: &str) {
    println!("Answer: {}", f(input));
}