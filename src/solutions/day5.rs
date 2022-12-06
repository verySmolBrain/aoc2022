use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> String {
    let mut arrangement: Vec<Vec<String>> = Vec::new();

    const NUM_BLOCKS: usize = 0;
    const FROM: usize = 1;
    const TO: usize = 2;

    for line in input.lines() {
        if !line.chars().any(|c| c.is_digit(10)) && !line.is_empty() {
            let row = line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|s| s.iter().collect::<String>())
                .collect::<Vec<String>>();
            
            for (i, r) in row.iter().enumerate() {
                if arrangement.len() == i {
                    arrangement.push(Vec::new());
                }
                
                let s = r.trim();
                if !s.is_empty() {
                    arrangement[i].insert(0, s.to_string());
                }
            }
        }

        if line.starts_with("move") {
            let commands = line.split_whitespace()
                .filter(|s| s.parse::<u32>().is_ok())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            
            for _ in 0..commands[NUM_BLOCKS] {
                let from = arrangement[commands[FROM] as usize - 1].pop().unwrap();
                arrangement[commands[TO] as usize - 1].push(from);
            }
        }
    }

    return arrangement.iter()
        .fold(String::new(), |acc, row| acc + row.last().unwrap());
}

fn part2(input: &str) -> String {
    let mut arrangement: Vec<Vec<String>> = Vec::new();

    const NUM_BLOCKS: usize = 0;
    const FROM: usize = 1;
    const TO: usize = 2;

    for line in input.lines() {
        if !line.chars().any(|c| c.is_digit(10)) && !line.is_empty() {
            let row = line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|s| s.iter().collect::<String>())
                .collect::<Vec<String>>();
            
            for (i, r) in row.iter().enumerate() {
                if arrangement.len() == i {
                    arrangement.push(Vec::new());
                }
                
                let s = r.trim();
                if !s.is_empty() {
                    arrangement[i].insert(0, s.to_string());
                }
            }
        }

        if line.starts_with("move") {
            let commands = line.split_whitespace()
                .filter(|s| s.parse::<u32>().is_ok())
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            
            let new_len = arrangement[commands[FROM] as usize - 1].len() - commands[NUM_BLOCKS] as usize;
            let from = arrangement[commands[FROM] as usize - 1].drain(new_len..).collect::<Vec<String>>();

            for c in from {
                arrangement[commands[TO] as usize - 1].push(c);
            }
        }
    }

    return arrangement.iter()
        .fold(String::new(), |acc, row| acc + row.last().unwrap());
}
