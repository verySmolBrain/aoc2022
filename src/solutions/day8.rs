use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut num_visible: u32 = 0;

    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_visible(&map, (j as i32, i as i32)) {
                num_visible += 1;
            }
        }
    }

    return num_visible;
}

fn is_visible(map: &Vec<Vec<u32>>, node: (i32, i32)) -> bool {
    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    if look_in_direction(map, node, UP)
    || look_in_direction(map, node, DOWN)
    || look_in_direction(map, node, LEFT)
    || look_in_direction(map, node, RIGHT) {
        return true;
    }
    return false;
}

fn look_in_direction(map: &Vec<Vec<u32>>, node: (i32, i32), direction: (i32, i32)) -> bool {
    let mut node = node;
    let node_value = map[node.1 as usize][node.0 as usize];

    loop {
        let next_node = (node.0 + direction.0, node.1 + direction.1);

        if next_node.0 < 0 || next_node.0 >= map[0].len() as i32 
        || next_node.1 < 0 || next_node.1 >= map.len() as i32 {
            return true;
        }

        let next_node_value = map[next_node.1 as usize][next_node.0 as usize];

        if next_node_value >= node_value {
            break;
        }
        
        node = next_node;
    }

    return false;
}

fn part2(input: &str) -> u32 {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut scenic_scores: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        map.push(row);
    }

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            scenic_scores.push(scenic_score(&map, (j as i32, i as i32)));
        }
    }

    return *scenic_scores.iter().max().unwrap();
}

fn scenic_score(map: &Vec<Vec<u32>>, node: (i32, i32)) -> u32 {
    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);

    return num_visible_in_direction(map, node, UP) *
    num_visible_in_direction(map, node, DOWN) *
    num_visible_in_direction(map, node, LEFT) *
    num_visible_in_direction(map, node, RIGHT);
}

fn num_visible_in_direction(map: &Vec<Vec<u32>>, node: (i32, i32), direction: (i32, i32)) -> u32 {
    let mut node = node;
    let node_value = map[node.1 as usize][node.0 as usize];
    let mut num_nodes: u32 = 0;

    loop {
        let next_node = (node.0 + direction.0, node.1 + direction.1);

        if next_node.0 < 0 || next_node.0 >= map[0].len() as i32 
        || next_node.1 < 0 || next_node.1 >= map.len() as i32 {
            break;
        }

        let next_node_value = map[next_node.1 as usize][next_node.0 as usize];
        num_nodes += 1;

        if next_node_value >= node_value {
            break;
        }
        
        node = next_node;
    }

    return num_nodes;
}