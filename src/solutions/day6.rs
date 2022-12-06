use crate::utils::print_answer;
use std::collections:: { VecDeque, HashSet };
use std::iter::FromIterator;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut marker: VecDeque<char> = VecDeque::new();
    let mut marker_location: i32 = 0;

    const NUM_UNIQUE: usize = 4;

    for (i, c) in input.chars().enumerate() {
        if marker.len() != NUM_UNIQUE {
            marker.push_back(c);
        } else {
            marker.pop_front();
            marker.push_back(c);
        }

        if HashSet::<char>::from_iter(marker.iter().cloned()).len() == NUM_UNIQUE {
            marker_location = i as i32 + 1;
            break;
        }
    }

    return marker_location;
}

fn part2(input: &str) -> i32 {
    let mut marker: VecDeque<char> = VecDeque::new();
    let mut marker_location: i32 = 0;

    const NUM_UNIQUE: usize = 14;

    for (i, c) in input.chars().enumerate() {
        if marker.len() != NUM_UNIQUE {
            marker.push_back(c);
        } else {
            marker.pop_front();
            marker.push_back(c);
        }

        if HashSet::<char>::from_iter(marker.iter().cloned()).len() == NUM_UNIQUE {
            marker_location = i as i32 + 1;
            break;
        }
    }

    return marker_location;
}