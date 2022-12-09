use crate::utils::print_answer;
use std::collections::HashSet;
use phf::phf_map;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

static DIRECTION: phf::Map<&'static str, (i32, i32)> = phf_map! {
    "R" => (0, 1),
    "L" => (0, -1),
    "U" => (1, 0),
    "D" => (-1, 0),
};

fn touching(tail: (i32, i32), head: (i32, i32)) -> bool {
    let possible_x = [tail.0, tail.0 - 1, tail.0 + 1];
    let possible_y = [tail.1, tail.1 - 1, tail.1 + 1];

    if possible_x.contains(&head.0) && possible_y.contains(&head.1) {
        return true;
    }

    return false;
}

fn part1(input: &str) -> u32 {
    let mut visited_squares: HashSet<(i32, i32)> = HashSet::new();

    let mut tail: (i32, i32) = (0, 0);
    let mut head: (i32, i32) = (0, 0);

    visited_squares.insert(tail);
    for line in input.lines() {
        let (direction, distance) = line.rsplit_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();

        let direction: (i32, i32) = DIRECTION[direction];

        for _ in 0..distance {
            let new_head_location = (head.0 + direction.0, head.1 + direction.1);

            if !touching(tail, new_head_location) {
                tail = head;
                visited_squares.insert(tail);
            }

            head = new_head_location;
        }
    } 

    return visited_squares.iter().count() as u32;
}

fn part2(input: &str) -> u32 {
    let mut visited_squares: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    const NUM_TAILS: usize = 10;

    let mut snake: Vec<(i32, i32)> = vec![(0, 0); NUM_TAILS];

    for line in input.lines() {
        let (direction, distance) = line.rsplit_once(' ').unwrap();
        let distance = distance.parse::<i32>().unwrap();

        let direction: (i32, i32) = DIRECTION[direction];

        for _ in 0..distance {
            snake[0] = (snake[0].0 + direction.0, snake[0].1 + direction.1);

            for i in 1..NUM_TAILS {
                let dx = snake[i - 1].0 - snake[i].0; 
                let dy = snake[i - 1].1 - snake[i].1; 

                if dx.abs() < 2 && dy.abs() < 2 { 
                    continue;
                } 

                if dx != 0 { 
                    snake[i].0 += dx / dx.abs(); 
                }
                if dy != 0 { 
                    snake[i].1 += dy / dy.abs(); 
                }
            }

            visited_squares.insert(snake[NUM_TAILS - 1]);
        }
    } 

    return visited_squares.iter().count() as u32;
}
