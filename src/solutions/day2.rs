use crate::utils::print_answer;
use phf::phf_map;

pub fn run(input: String) {
    print_answer(part1, &input);
    print_answer(part2, &input);
}

/*
    Lose
    P1 R0 
    S2 P1 
    R0 S2 

    Win
    S2 R0 
    R0 P1 
    P1 S2 
 */

static SCORE: phf::Map<&'static str, i32> = phf_map! {
    "A" => 0,
    "X" => 0,
    "B" => 1,
    "Y" => 1,
    "C" => 2,
    "Z" => 2,
};

static SCORE_P2: phf::Map<&'static str, i32> = phf_map! {
    "A" => 0,
    "X" => 0, // lose
    "B" => 1,
    "Y" => 3, // draw
    "C" => 2,
    "Z" => 6, // win
};

fn part1(input: &str) -> i32 {
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    let mut score: i32 = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let p1 = parts.next().unwrap();
        let p2 = parts.next().unwrap();

        let p1_score = *SCORE.get(p1).unwrap();
        let p2_score = *SCORE.get(p2).unwrap();

        if p1_score == p2_score {
            score += DRAW;
        } else if (p1_score + 1) % 3 == p2_score {
            score += WIN;
        }

        score += p2_score + 1;
    }

    return score;
}

fn part2(input: &str) -> i32 {
    const LOSE: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    let mut score: i32 = 0;

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let p1 = parts.next().unwrap();
        let outcome = parts.next().unwrap();

        let p1_score = *SCORE_P2.get(p1).unwrap();
        let outcome_score = *SCORE_P2.get(outcome).unwrap();

        if outcome_score == DRAW {
            score += p1_score;
        } else if outcome_score == WIN {
            score += (p1_score + 1) % 3;
        } else if outcome_score == LOSE {
            score += (p1_score + 2) % 3;
        }

        score += outcome_score + 1;
    }

    return score;
}