use crate::utils::print_answer;
use std::collections::HashSet;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i64 {
    parse_input(input)
}

fn parse_input(input: &str) -> i64 {
    let mut sensor_coverage: Vec<(i64, i64)> = Vec::new();
    let mut beacon_map: HashSet<(i64, i64)> = HashSet::new();
    let mut covered: HashSet<(i64, i64)> = HashSet::new();

    const SEARCH: i64 = 10;

    for line in input.lines() {
        let line = line.replace(',', "").replace(':', "");
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let (sx, sy) = parse_x_y(line.get(2).unwrap(), line.get(3).unwrap());
        let (bx, by) = parse_x_y(line.get(8).unwrap(), line.get(9).unwrap());

        beacon_map.insert((bx, by));
        let magnitude = i64::abs(sx - bx) + i64::abs(sy - by);

        let y_diff = i64::abs(sy - SEARCH); // How far y is from the search row
        // Bounds on the SEARCH row (Since magnitude - y_diff gives how wide the coverage is in the row)
        sensor_coverage.push((sx - (magnitude - y_diff), (sx + (magnitude - y_diff)))); 
    }

    for range in sensor_coverage {
        for n in range.0..range.1 + 1 {
            covered.insert((n, SEARCH));
        }
    }

    return covered.iter().filter(|x| x.1 == SEARCH).count() as i64 - 
        beacon_map.iter().filter(|x| x.1 == SEARCH).count() as i64;
}

fn parse_x_y(x: &str, y:&str) -> (i64, i64) {
    let x = x.split('=').collect::<Vec<&str>>().get(1).unwrap().parse::<i64>().unwrap();
    let y = y.split('=').collect::<Vec<&str>>().get(1).unwrap().parse::<i64>().unwrap();

    (x, y)
}

fn part2(input: &str) -> i64 {
    let (x, y) = parse_input_2(input);
    println!("{} {}", x, y);
    return x * 4000000 + y;
}

struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64)
}

impl Sensor {
    fn new(pos: (i64, i64), beacon: (i64, i64)) -> Sensor {
        Sensor {
            pos,
            beacon
        }
    }
}

fn parse_input_2(input: &str) -> (i64, i64) {
    let mut sensors: Vec<Sensor> = Vec::new();

    const BOUND: i64 = 4000000;

    for line in input.lines() {
        let line = line.replace(',', "").replace(':', "");
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let (sx, sy) = parse_x_y(line.get(2).unwrap(), line.get(3).unwrap());
        let (bx, by) = parse_x_y(line.get(8).unwrap(), line.get(9).unwrap());

        sensors.push(Sensor::new((sx, sy), (bx, by)));
    }

    for y in 0..BOUND + 1 {
        let mut ranges: Vec<(i64, i64)> = Vec::new();

        for sensor in &sensors {
            let (sx, sy) = sensor.pos;
            let (bx, by) = sensor.beacon;

            let magnitude = i64::abs(sx - bx) + i64::abs(sy - by);
            let y_diff = i64::abs(sy - y); // How far y is from the search row

            if y_diff == magnitude {
                ranges.push((sx, sx));
            } else if y_diff < magnitude {
                ranges.push((sx - (magnitude - y_diff), (sx + (magnitude - y_diff))));
            }
        }

        ranges.sort();

        let mut end_range = ranges.get(0).unwrap().1;
        let mut uncovered: HashSet<(i64, i64)> = HashSet::new();
        for range in ranges {
            uncovered.retain(|u| {
                if range.0 <= u.0 && range.1 >= u.1 {
                    return false;
                }
                return true;
            });
            if range.0 > end_range + 1 && range.0 < BOUND - 1{
                uncovered.insert((end_range + 1, range.0 - 1));
            }
            if range.1 > end_range {
                end_range = range.1;
            }
        }

        if !uncovered.is_empty() {
            return (uncovered.drain().next().unwrap().0, y);
        }
    }

    return (0, 0);
}