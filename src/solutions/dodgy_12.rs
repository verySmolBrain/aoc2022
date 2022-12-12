use crate::utils::print_answer;
use std::collections::BinaryHeap;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    distance: u32,
    pos: (usize, usize),
    neighbours: Vec<(usize, usize)>,
}

impl Node {
    fn new(distance: u32, pos: (usize, usize), neighbours: Vec<(usize, usize)>) -> Node {
        Node { 
            distance, 
            pos,
            neighbours,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance).reverse())
    }
}

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

    let _distance = djikstra(&grid, start, end);

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
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut visited: Vec<bool> = vec![false; grid[0].len() * grid.len()];
    let mut distances: Vec<u32> = vec![u32::MAX; grid[0].len() * grid.len()];

    let neighbours = get_neighbors(grid.len(), grid[0].len(), start);
    queue.push(Node::new(0, start, neighbours));
    distances[start.0 * grid[0].len() + start.1] = 0;
    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        visited[u.pos.0 * grid[0].len() + u.pos.1] = true;

        if u.distance == u32::MAX {
            continue;
        }
        if u.pos == end {
            println!("Found end: {}", u.distance);
            return u.distance;
        }

        for neighbour in u.neighbours {
            if visited[neighbour.0 * grid[0].len() + neighbour.1] {
                continue;
            }

            let neighbours = get_neighbors(grid.len(), grid[0].len(), neighbour);
            let candidate = Node::new(u.distance + grid[neighbour.0][neighbour.1], neighbour, neighbours);

            if candidate.distance < distances[candidate.pos.0 * grid[0].len() + candidate.pos.1] {
                distances[candidate.pos.0 * grid[0].len() + candidate.pos.1] = candidate.distance;
                queue.push(candidate);
            }
        }
    }

    return 0;
}

fn get_neighbors(grid_x: usize, grid_y: usize, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.0 < grid_x - 1 {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.1 < grid_y - 1 {
        neighbors.push((pos.0, pos.1 + 1));
    }

    return neighbors;
}
