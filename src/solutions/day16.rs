use crate::utils::print_answer;
use std::collections::HashSet;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    // Max value for valves n - 1 -> n such that n has the max flow rate
    const TIME_LIMIT: usize = 30;
    let dp: Vec<Vec<Option<usize>>> = vec![vec![None; TIME_LIMIT]; TIME_LIMIT];
    let graph: HashSet<(Vec<&str>, u32)> = HashSet::new();

    for line in input.lines() {
        
    }

    0
}