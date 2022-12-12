use crate::utils::print_answer;
use std::collections::BinaryHeap;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

struct Node {
    distance: u32,
    pos: (usize, usize),
}

impl Node {
    fn new(distance: u32, pos: (usize, usize)) -> Node {
        Node { distance, pos }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Node {}

fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<u32> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
                row.push(1);
            } else if c == 'E' {
                end = (i, j);
                row.push(26);
            } else {
                row.push(convert_ascii_to_integer(c as u8));
            }
        }

        grid.push(row);
    }

    println!("{:?}", start);
    println!("{:?}", end);
    print_grid(&grid);

    return 0;
}

fn convert_ascii_to_integer(input: u8) -> u32 {
    return 1 + (input - b'a') as u32;
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for cell in row {
            print!("{:02} ", cell);
        }
        println!();
    }
}

fn djikstra(grid: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut distance: Vec<u32> = vec![u32::MAX; grid.len() * grid[0].len()];
    let mut previous: Vec<(usize, usize)> = Vec::new();
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if (i, j) == start {
                distance[i * grid[0].len() + j] = 0;
            } else {
                queue.push(Node::new(u32::MAX, (i, j)));
            }
        }
    }

    return 0;
}