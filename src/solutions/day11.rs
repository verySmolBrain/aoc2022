use crate::utils::print_answer;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    if_true: Option<usize>,
    if_false: Option<usize>,
    inspected: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: Box::new(|a| a),
            test: Box::new(|a| a == 0),
            if_true: None,
            if_false: None,
            inspected: 0,
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn run_round(&mut self) -> Vec<(usize, u64)> {
        let mut throw_to: Vec<(usize, u64)> = Vec::new();

        for item in &self.items {
            self.inspected += 1;
            let new = (self.operation)(*item) / 3;

            if (self.test)(new) {
                throw_to.push((self.if_true.unwrap(), new));
            } else {
                throw_to.push((self.if_false.unwrap(), new));
            }
        }

        self.items.clear();

        return throw_to;
    }
}


fn part1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        
        match &line[..] {
            ["Monkey", _monkey_pos] => {
                monkeys.push(Monkey::new());
            },
            ["Starting", "items:", items @ ..] => {
                items.iter()
                .map(|item| item.trim_end_matches(','))
                .for_each(|item| {
                    monkeys.last_mut().unwrap().add_item(item.parse::<u64>().unwrap());
                });
            },
            ["Operation:", "new", "=", "old", operation, val] => {
                if *operation == "+".to_string() && val.to_string() == "old".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a + a);
                } else if *operation == "*".to_string() && val.to_string() == "old".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a * a);
                } 

                if !val.parse::<u64>().is_ok() {
                    continue;
                }

                let val = val.parse::<u64>().unwrap();
                if *operation == "+".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a + val.to_owned());
                } else {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a * val.to_owned());
                }
            },
            ["Test:", "divisible", "by", val] => {
                let val = val.parse::<u64>().unwrap();

                monkeys.last_mut().unwrap().test = Box::new(
                    move |a| a % val.to_owned() == 0
                );
            },
            ["If", "true:", "throw", "to", "monkey", monkey_pos] => {
                monkeys.last_mut().unwrap().if_true = Some(monkey_pos.parse::<usize>().unwrap());
            },
            ["If", "false:", "throw", "to", "monkey", monkey_pos] => {
                monkeys.last_mut().unwrap().if_false = Some(monkey_pos.parse::<usize>().unwrap());
            },
            _ => (),
        }
    }

    const NUM_ROUNDS: u64 = 20;

    for _ in 0..NUM_ROUNDS {
        for i in 0..monkeys.len() {
            let throw_to = monkeys[i].run_round();
            for (monkey, item) in throw_to {
                monkeys[monkey].add_item(item);
            }
        }
    }

    let mut inspected: Vec<u64> = Vec::from_iter(monkeys.iter().map(|monkey| monkey.inspected));
    inspected.sort();

    return inspected[inspected.len() - 1] * inspected[inspected.len() - 2];
}


//
//


struct BeegMonkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    if_true: Option<usize>,
    if_false: Option<usize>,
    inspected: u64,
    factors: u64
}

impl BeegMonkey {
    fn new() -> BeegMonkey {
        BeegMonkey {
            items: Vec::new(),
            operation: Box::new(|a| a),
            test: Box::new(|a| a == 0),
            if_true: None,
            if_false: None,
            inspected: 0,
            factors: 1
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn run_round(&mut self) -> Vec<(usize, u64)> {
        let mut throw_to: Vec<(usize, u64)> = Vec::new();

        for item in &self.items {
            self.inspected += 1;
            let new = (self.operation)(*item) % self.factors;

            if (self.test)(new) {
                throw_to.push((self.if_true.unwrap(), new));
            } else {
                throw_to.push((self.if_false.unwrap(), new));
            }
        }

        self.items.clear();

        return throw_to;
    }
}

fn part2(input: &str) -> u64 {
    let mut monkeys: Vec<BeegMonkey> = Vec::new();
    let mut factors = 1;

    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        
        match &line[..] {
            ["Monkey", _monkey_pos] => {
                monkeys.push(BeegMonkey::new());
            },
            ["Starting", "items:", items @ ..] => {
                items.iter()
                .map(|item| item.trim_end_matches(','))
                .for_each(|item| {
                    monkeys.last_mut().unwrap().add_item(item.parse::<u64>().unwrap());
                });
            },
            ["Operation:", "new", "=", "old", operation, val] => {
                if *operation == "+".to_string() && val.to_string() == "old".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a + a);
                } else if *operation == "*".to_string() && val.to_string() == "old".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a * a);
                } 

                if !val.parse::<u64>().is_ok() {
                    continue;
                }

                let val = val.parse::<u64>().unwrap();
                if *operation == "+".to_string() {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a + val.to_owned());
                } else {
                    monkeys.last_mut().unwrap().operation = Box::new(move |a| a * val.to_owned());
                }
            },
            ["Test:", "divisible", "by", val] => {
                let val = val.parse::<u64>().unwrap();

                factors *= val;

                monkeys.last_mut().unwrap().test = Box::new(
                    move |a| a % val.to_owned() == 0
                );
            },
            ["If", "true:", "throw", "to", "monkey", monkey_pos] => {
                monkeys.last_mut().unwrap().if_true = Some(monkey_pos.parse::<usize>().unwrap());
            },
            ["If", "false:", "throw", "to", "monkey", monkey_pos] => {
                monkeys.last_mut().unwrap().if_false = Some(monkey_pos.parse::<usize>().unwrap());
            },
            _ => (),
        }
    }

    const NUM_ROUNDS: u64 = 10000;

    for monkey in &mut monkeys {
        monkey.factors = factors;
    }

    for _ in 0..NUM_ROUNDS {
        for i in 0..monkeys.len() {
            let throw_to = monkeys[i].run_round();
            for (monkey, item) in throw_to {
                monkeys[monkey].add_item(item);
            }
        }
    }

    let mut inspected: Vec<u64> = Vec::from_iter(monkeys.iter().map(|monkey| monkey.inspected));
    inspected.sort();

    return inspected[inspected.len() - 1] * inspected[inspected.len() - 2];
}

// while let Some(..) = input.try_next::<(W, W, Ws)>()? {
//     let (_, _, Split(items)) = input.line::<(W, W, Split<',', ArrayRingBuffer<_, QUEUE>>)>()?;
//     let (_, _, _, _, op, operand) = input.next::<(W, W, W, W, Op, Operand)>()?;
//     let (_, _, _, div) = input.next::<(W, W, W, _)>()?;
//     let (_, _, _, _, _, if_true) = input.next::<(W, W, W, W, W, usize)>()?;
//     let (_, _, _, _, _, if_false) = input.next::<(W, W, W, W, W, usize)>()?;

//     factors *= div;

//     monkeys.push(Monkey {
//         items,
//         op,
//         operand,
//         div,
//         if_true,
//         if_false,
//     });
// }