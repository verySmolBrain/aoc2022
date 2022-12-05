use crate::utils::print_answer;
use std::ops::Range;
use itertools::Itertools;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part1_functional, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut num_overlap: i32 = 0;

    for line in input.lines() {
        let (l1, l2) = line.rsplit_once(',').unwrap();

        let (s1, e1) = l1.rsplit_once('-').unwrap();
        let (s2, e2) = l2.rsplit_once('-').unwrap();

        let r1 = s1.parse::<i32>().unwrap()..e1.parse::<i32>().unwrap();
        let r2 = s2.parse::<i32>().unwrap()..e2.parse::<i32>().unwrap();

        if contains(&r1, &r2) || contains(&r2, &r1) {
            num_overlap += 1;
        }
    }

    return num_overlap;
}

fn contains(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    r1.start <= r2.start && r1.end >= r2.end
}

fn overlap(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    if (r1.start <= r2.start && r1.end <= r2.end) && (r1.end >= r2.start) {
        return true;
    }

    return false;
}

fn part2(input: &str) -> i32 {
    let mut num_overlap: i32 = 0;

    for line in input.lines() {
        let (l1, l2) = line.rsplit_once(',').unwrap();

        let (s1, e1) = l1.rsplit_once('-').unwrap();
        let (s2, e2) = l2.rsplit_once('-').unwrap();

        let r1 = s1.parse::<i32>().unwrap()..e1.parse::<i32>().unwrap();
        let r2 = s2.parse::<i32>().unwrap()..e2.parse::<i32>().unwrap();

        if overlap(&r1, &r2) || overlap(&r2, &r1) || contains(&r1, &r2) || contains(&r2, &r1) {
            num_overlap += 1;
        }
    }

    return num_overlap;
}

fn part1_functional(input: &str) -> usize {
    let a = input
        .lines()
        .filter_map(|s| s.split(|c: char| !c.is_numeric()).map(|n| n.parse::<i32>().unwrap()).next_tuple())
        .filter(|(a1, a2, b1, b2)| a1 <= b1 && b2 <= a2 || b1 <= a1 && a2 <= b2)
        .count();

    return a;
}