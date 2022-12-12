use crate::utils::print_answer;
use std::collections::VecDeque;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
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

    let distance = bfs(&grid, end);

    return distance[start.0][start.1];
}

fn convert_ascii_to_integer(input: u8) -> u32 {
    return 1 + (input - b'a') as u32;
}

fn bfs(grid: &Vec<Vec<u32>>, end: (usize, usize)) -> Vec<Vec<u32>> {
    let mut queue = VecDeque::new();
    let mut distances: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];
    queue.push_front(end);

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for (nx, ny) in get_valid_neighbours(&grid, (x, y)) {
            if distances[nx][ny] == 0 {
                distances[nx][ny] = distances[x][y] + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    distances
}

fn get_valid_neighbours(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    if pos.0 > 0 {
        neighbours.push((pos.0 - 1, pos.1));
    }
    if pos.0 < grid.len() - 1 {
        neighbours.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        neighbours.push((pos.0, pos.1 - 1));
    }
    if pos.1 < grid[0].len() - 1 {
        neighbours.push((pos.0, pos.1 + 1));
    }

    neighbours.iter()
        .filter(|(x, y)| grid[*x][*y] + 1 >= grid[pos.0][pos.1])
        .map(|(x, y)| (*x, *y))
        .collect()
}

fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut end = (0, 0);

    input.lines().enumerate()
        .for_each(|(i, line)| {
            let mut row: Vec<u32> = Vec::new();
            line.chars().enumerate()
                .for_each(|(j, c)| {
                    if c == 'E' {
                        end = (i, j);
                        row.push(26);
                    } else if c == 'S' {
                        row.push(1);
                    } else {
                        row.push(convert_ascii_to_integer(c as u8));
                    }
                }
            );
            grid.push(row);
        }
    );

    let mut distances = Vec::new();
    let distance = bfs(&grid, end);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 && distance[i][j] != 0 {
                distances.push(distance[i][j]);
            }
        }
    }

    return *distances.iter().min().unwrap();
}