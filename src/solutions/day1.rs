use crate::utils::print_answer;
use std::collections::BinaryHeap;
use std::cmp:: { max, Reverse};

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut greatest_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    for line in input.lines() {
        if line == "" {
            current_calories = 0;
            continue;
        }

        current_calories += line.parse::<i32>().unwrap();

        if current_calories > greatest_calories {
            greatest_calories = current_calories;
        }
    }

    return greatest_calories;
}

fn part2(input: &str) -> i32 {
    let num_elves = 3;
    let mut greatest_calories: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut current_calories: i32 = 0;

    for line in input.lines() {
        if line == "" {
            current_calories = 0;
            continue;
        }

        current_calories += line.parse::<i32>().unwrap();

        if greatest_calories.len() != num_elves {
            greatest_calories.push(Reverse(current_calories));
            continue;
        }

        let previous_calories = greatest_calories.pop()
            .unwrap_or(Reverse(0)).0;
        greatest_calories.push(
            Reverse(
                max(current_calories, previous_calories)
            )
        );
    }

    return greatest_calories.iter().map(|Reverse(x)| x).sum();
}