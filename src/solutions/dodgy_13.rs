use crate::utils::print_answer;
use itertools::Itertools;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    let mut num_matches: u32 = 0;

    let mut lines = input.lines();
    while let Some((left, right)) = lines.next_tuple() {
        if right_order(left, right) {
            num_matches += 1;
        }

        lines.next(); // removes white line
    }

    return num_matches;
}

fn right_order(left: &str, right: &str) -> bool {
    println!("{:?}, {:?}", left, right);

    let mut left_iter = left.chars().enumerate();
    let mut right_iter = right.chars().enumerate();

    loop {
        let left_char = left_iter.next();
        let right_char = right_iter.next();

        if !left_char.is_none() && right_char.is_none() {
            return false;
        }
        if left_char.is_none() || right_char.is_none() {
            return true;
        }
        
        let (i, left_char) = left_char.unwrap();
        let (j, right_char) = right_char.unwrap();

        if left_char.is_numeric() && right_char.is_numeric() {
            if left_char > right_char {
                return false;
            } else {
                continue;
            }
        } else if left_char == '[' && right_char == '[' {
            if !right_order(&left[i + 1..], &right[j + 1..]) {
                return false;
            } else {
                left_iter.next();
                right_iter.next();
                continue;
            }
        } else if left_char == '[' && right_char.is_numeric() {
            let new_right = &mut right[j..].to_owned();
            String::insert(new_right, j + 1, ']');
            if !right_order(&left[i + 1..], &new_right) {
                return false;
            } else {
                left_iter.next();
                right_iter.next();
                continue;
            }
        } else if left_char.is_numeric() && right_char == '[' {
            let new_left = &mut left[i..].to_owned();
            String::insert(new_left, i + 1, ']');
            if !right_order(new_left, &right[j + 1..]) {
                return false;
            } else {
                left_iter.next();
                right_iter.next();
                continue;
            }
        } else if left_char == ']' && right_char == ']' {
            println!("{} {}", left_char, i);
            return true;
        }
    }
}