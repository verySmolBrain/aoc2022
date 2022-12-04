use crate::utils::print_answer;
use std::collections::HashSet;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut priority: i32 = 0;

    for line in input.lines() {
        let mut contains: HashSet<char> = HashSet::new();

        let half: usize = line.len() / 2;
        let (first, second) = line.split_at(half);

        for c in first.chars() {
            contains.insert(c);
        }

        for c in second.chars() {
            if contains.contains(&c) {
                priority += priority_score(c as u8);
                contains.remove(&c);
            }
        }
    }

    return priority;
}

fn priority_score(item: u8) -> i32 {
    if item.is_ascii_lowercase() {
        return 1 + (item - b'a') as i32;
    } else {
        return 27 + (item - b'A') as i32;
    }
}

fn unique_items(groups: &Vec<&str>) -> char {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    for group in groups {
        let mut set: HashSet<char> = HashSet::new();
        for c in group.chars() {
            set.insert(c);
        }

        sets.push(set);
    }

    let (intersection, others) = sets.split_first_mut().unwrap();
    for other in others {
        intersection.retain(| e | other.contains(e));
    }

    return *intersection.iter().next().unwrap();
}

fn part2(input: &str) -> i32 {
    let mut priority: i32 = 0;
    const GROUP_SIZE: i32 = 3;

    let mut groups: Vec<&str> = Vec::new();

    for line in input.lines() {
        groups.push(line);
        if groups.len() != GROUP_SIZE as usize {
            continue;
        }

        let badge = unique_items(&groups);
        priority += priority_score(badge as u8);

        groups.clear();
    }

    return priority;
}