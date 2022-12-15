use crate::utils::print_answer;
use std::collections::HashSet;

pub fn run(input: String) {
    print_answer(part1, &input);
    //print_answer(part2, &input);
}

fn part1(input: &str) -> u32 {
    let sensor_map = parse_input(input);

    const SEARCH: i32 = 10;
    let count = sensor_map.iter().fold(0, |acc, (_x, y)| {
        if *y == SEARCH {
            return acc + 1;
        } else {
            return acc;
        }
    });

    count
}

fn parse_input(input: &str) -> HashSet<(i32, i32)> {
    let mut sensor_map: HashSet<(i32, i32)> = HashSet::new();
    let mut beacon_map: HashSet<(i32, i32)> = HashSet::new();

    for line in input.lines() {
        let line = line.replace(',', "").replace(':', "");
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let (sx, sy) = parse_x_y(line.get(2).unwrap(), line.get(3).unwrap());
        let (bx, by) = parse_x_y(line.get(8).unwrap(), line.get(9).unwrap());

        let magnitude = i32::abs(sx - bx) + i32::abs(sy - by);
        insert_sensor(&mut sensor_map, sx, sy, magnitude);
        beacon_map.insert((bx, by));
    }

    for (x, y) in beacon_map {
        sensor_map.remove(&(x, y));
    }

    return sensor_map;
}

fn parse_x_y(x: &str, y:&str) -> (i32, i32) {
    let x = x.split('=').collect::<Vec<&str>>().get(1).unwrap().parse::<i32>().unwrap();
    let y = y.split('=').collect::<Vec<&str>>().get(1).unwrap().parse::<i32>().unwrap();

    (x, y)
}

fn insert_sensor(sensor_map: &mut HashSet<(i32, i32)>, x: i32, y: i32, magnitude: i32) {
    let mut width = 1;
    for i in (y - magnitude)..(y + magnitude) + 1 {
        for j in 0..width {
            sensor_map.insert((i, x + j));
            sensor_map.insert((i, x - j));
        }
        sensor_map.insert((i, x));
        
        if i < y {
            width += 1;
        } else {
            width -= 1;
        }
    }
}