use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

fn part1(input: &str) -> i32 {
    let mut register_x: i32 = 1;
    let mut cycle_strength: Vec<i32> = Vec::new();
    let mut cycles: i32 = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line[..] {
            ["noop"] => {
                iter_cycle(&mut cycles, &mut cycle_strength, register_x);
            },
            ["addx", val] => {
                iter_cycle(&mut cycles, &mut cycle_strength, register_x);
                iter_cycle(&mut cycles, &mut cycle_strength, register_x);
                register_x += val.parse::<i32>().unwrap();
            }
            _ => (),
        }

        if cycles > 220 {
            break;
        }
    }

    return cycle_strength.iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, x)| x)
        .sum();
}

fn iter_cycle(cycles: &mut i32, cycle_strength: &mut Vec<i32>, register_x: i32) {
    *cycles += 1;
    if *cycles % 20 == 0 {
        cycle_strength.push(*cycles * register_x);
    }
}

fn iter_cycle_message(cycles: &mut i32, message: &mut String, register_x: i32) {
    if *cycles % 40 == 0 { // new line every 40 cycles
        message.push('\n');
    }

    let sprite_positions = vec![register_x - 1, register_x, register_x + 1];

    if sprite_positions.contains(&(*cycles % 40)) {
        message.push('#');
    } else {
        message.push('.');
    }

    *cycles += 1;
}

fn part2(input: &str) -> String {
    let mut register_x: i32 = 1;
    let mut message = String::new();
    let mut cycles: i32 = 0;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        match line[..] {
            ["noop"] => {
                iter_cycle_message(&mut cycles, &mut message, register_x);
            },
            ["addx", val] => {
                iter_cycle_message(&mut cycles, &mut message, register_x);
                iter_cycle_message(&mut cycles, &mut message, register_x);
                register_x += val.parse::<i32>().unwrap();
            }
            _ => (),
        }
    }

    return message;
}