use crate::utils::print_answer;
use std::collections::HashSet;
use std::cmp::{ min, max };

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    let rocks = parse_map(input);
    let (left, right, height) = rocks.iter()
        .fold((usize::MAX, 0, 0), |(left, right, height), (x, y)| {
            (min(left, *x), max(right, *x), max(height, *y))
        });

    drop_sand(rocks, left, right, height)
}

fn parse_map(input: &str) -> HashSet<(usize, usize)> {
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    for line in input.lines() {
        for (c1, c2) in line.split(" -> ").zip(line.split(" -> ").skip(1)) {
            let (c1x, c1y) = parse_corners(c1);
            let (c2x, c2y) = parse_corners(c2);

            if c1x == c2x {
                for y in min(c1y, c2y)..max(c1y, c2y) + 1 {
                    rocks.insert((c1x, y));
                }
            } else {
                for x in min(c1x, c2x)..max(c1x, c2x) + 1 {
                    rocks.insert((x, c1y));
                }
            }
        }
    }
    rocks
}

fn parse_corners(input: &str) -> (usize, usize) {
    let (x, y) = input.rsplit_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn drop_sand(mut rocks: HashSet<(usize, usize)>, left: usize, right: usize, height: usize) -> u32 {
    const SAND_SOURCE: (usize, usize) = (500, 0);
    let mut num_sands: u32 = 0;

    loop { // Dropping sand
        let mut cur = SAND_SOURCE;
        loop { // Falling down
            if cur.0 < left || cur.0 > right || cur.1 > height { // Falling in abyss
                return num_sands;
            }

            if !(rocks.contains(&(cur.0, cur.1 + 1))) { // Falling down
                cur = (cur.0, cur.1 + 1);
            } else if !(rocks.contains(&(cur.0 - 1, cur.1 + 1))) { // if left is free go left down
                cur = (cur.0 - 1, cur.1);
            } else if !(rocks.contains(&(cur.0 + 1, cur.1 + 1))) { // if right is free go right down
                cur = (cur.0 + 1, cur.1);
            } else { // Nowhere to go
                num_sands += 1;
                rocks.insert(cur);
                break;
            }
        }
    }
}

fn part2(input: &str) -> u32 {
    let rocks = parse_map(input);
    let (left, right, height) = rocks.iter()
        .fold((usize::MAX, 0, 0), |(left, right, height), (x, y)| {
            (min(left, *x), max(right, *x), max(height, *y))
        });

    drop_sand_two(rocks, left, right, height)
}

fn drop_sand_two(mut rocks: HashSet<(usize, usize)>, _left: usize, _right: usize, height: usize) -> u32 {
    const SAND_SOURCE: (usize, usize) = (500, 0);
    let max_depth: usize = height + 2;
    let mut num_sands: u32 = 0;

    loop { // Dropping sand
        let mut cur = SAND_SOURCE;
        loop { // Falling down
            if !(rocks.contains(&(cur.0, cur.1 + 1))) && cur.1 + 1 != max_depth { // Falling down
                cur = (cur.0, cur.1 + 1);
            } else if !(rocks.contains(&(cur.0 - 1, cur.1 + 1))) && cur.1 + 1 != max_depth { // if left is free go left down
                cur = (cur.0 - 1, cur.1);
            } else if !(rocks.contains(&(cur.0 + 1, cur.1 + 1))) && cur.1 + 1 != max_depth { // if right is free go right down
                cur = (cur.0 + 1, cur.1);
            } else { // Nowhere to go
                if cur == SAND_SOURCE { // Blocked
                    return num_sands + 1;
                }

                num_sands += 1;
                rocks.insert(cur);
                break;
            }
        }
    }
}