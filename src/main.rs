use std::env;
use std::fs;
use solutions::*;

mod solutions;
mod utils;

fn main() {
    let day = env::args().nth(1).expect("Day number not provided");

    match day.as_str() {
        "1" => day1::run(read_input(&day)),
        "2" => day2::run(read_input(&day)),
        "3" => day3::run(read_input(&day)),
        "4" => day4::run(read_input(&day)),
        "5" => day5::run(read_input(&day)),
        "6" => day6::run(read_input(&day)),
        "7" => day7::run(read_input(&day)),
        "8" => day8::run(read_input(&day)),
        "9" => day9::run(read_input(&day)),
        "10" => day10::run(read_input(&day)),
        "11" => day11::run(read_input(&day)),
        "12" => day12::run(read_input(&day)),
        "13" => day13::run(read_input(&day)),
        "14" => day14::run(read_input(&day)),
        // "15" => day15::run(read_input(&day)),
        // "16" => day16::run(read_input(&day)),
        // "17" => day17::run(read_input(&day)),
        // "18" => day18::run(read_input(&day)),
        // "19" => day19::run(read_input(&day)),
        // "20" => day20::run(read_input(&day)),
        // "21" => day21::run(read_input(&day)),
        // "22" => day22::run(read_input(&day)),
        // "23" => day23::run(read_input(&day)),
        // "24" => day24::run(read_input(&day)),
        // "25" => day25::run(read_input(&day)),
        _ => panic!("Day {} not implemented", day),
    }
}

fn read_input(day: &str) -> String {
    let path = format!("src/data/day{}.txt", day);
    fs::read_to_string(path).expect("Unable to read file")
}
