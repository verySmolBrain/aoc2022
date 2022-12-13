use std::str::FromStr;

use crate::utils::print_answer;
use itertools::Itertools;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(u32),
    Packet(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
            (Packet::Value(a), Packet::Packet(_b)) 
                => Packet::Packet(vec![Packet::Value(*a)]).cmp(other),
            (Packet::Packet(_a), Packet::Value(b)) 
                => self.cmp(&Packet::Packet(vec![Packet::Value(*b)])),
            (Packet::Packet(a), Packet::Packet(b)) => {
                for (i, j) in a.iter().zip(b.iter()) {
                    if *i != *j {
                        return i.cmp(j);
                    }
                }
                a.len().cmp(&b.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn process(s: &mut Vec<&str>) -> Vec<Packet> {
            let mut result: Vec<Packet> = Vec::new();
            while let Some(c) = s.pop() {
                match c {
                    "]" => return result,
                    "[" => result.push(Packet::Packet(process(s))),
                    "" => (),
                    _ => result.push(Packet::Value(c.parse::<u32>().unwrap()))
                }
            }
            result
        }
        
        Ok(Packet::Packet(process(&mut s
            .replace('[', "[,")
            .replace(']', ",]")
            .split(',')
            .rev()
            .collect()
        )))
    }

    type Err = ();
}

fn part1(input: &str) -> u32 {
    let mut num_matches: u32 = 0;
    let mut index: u32 = 0;

    let mut lines = input.lines();
    while let Some((left, right)) = lines.next_tuple() {
        index += 1;
        if left.parse::<Packet>().unwrap() < right.parse::<Packet>().unwrap() {
            num_matches += index;
        }

        lines.next(); // removes white line
    }

    return num_matches;
}

fn part2(input: &str) -> u32 {
    let mut packets = input.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<Packet>>();
    
    let packet_two = "[[2]]".parse::<Packet>().unwrap();
    let packet_six = "[[6]]".parse::<Packet>().unwrap();

    packets.push(packet_two.clone());
    packets.push(packet_six.clone());

    packets.sort();

    let pos_1 = packets.iter().position(|p| *p == packet_two).unwrap();
    let pos_2 = packets.iter().position(|p| *p == packet_six).unwrap();

    return (pos_1 + 1) as u32 * (pos_2 + 1) as u32;
}