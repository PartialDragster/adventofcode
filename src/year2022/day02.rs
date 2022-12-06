use crate::utils::utils;
use regex::Regex;
use std::collections::HashMap;

enum Janken {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn parse_line(s: &String) -> Option<(char, char)> {
    let regex = Regex::new(r"([ABC]) ([XYZ])").unwrap();
    let caps = regex.captures(s)?;
    let lhs = caps.get(1)?.as_str().chars().nth(0)?;
    let rhs = caps.get(2)?.as_str().chars().nth(0)?;
    Some((lhs, rhs))
}

fn uniqueify(strategy_guide: Vec<(char, char)>) -> HashMap<(char, char), u32> {
    let mut map = HashMap::new();
    for pair in strategy_guide {
        map.entry(pair)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    map
}

fn shape_score(shape: &Janken) -> u32 {
    match shape {
        Janken::Rock => 1,
        Janken::Paper => 2,
        Janken::Scissors => 3,
    }
}

fn their_code_to_janken(code: &char) -> Janken {
    match code {
        'A' => Janken::Rock,
        'B' => Janken::Paper,
        'C' => Janken::Scissors,
        _ => panic!(),
    }
}

fn my_code_to_janken(code: &char) -> Janken {
    match code {
        'X' => Janken::Rock,
        'Y' => Janken::Paper,
        'Z' => Janken::Scissors,
        _ => panic!(),
    }
}

fn code_to_outcome(code: &char) -> Outcome {
    match code {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn move_score(their_move: &Janken, my_move: &Janken) -> u32 {
    match (their_move, my_move) {
        (Janken::Rock, Janken::Scissors) 
            | (Janken::Paper, Janken::Rock) 
            | (Janken::Scissors, Janken::Paper) 
            => 0,
        (Janken::Rock, Janken::Rock) 
            | (Janken::Paper, Janken::Paper) 
            | (Janken::Scissors, Janken::Scissors) 
            => 3,
        (Janken::Rock, Janken::Paper) 
            | (Janken::Paper, Janken::Scissors) 
            | (Janken::Scissors, Janken::Rock) 
            => 6,
    }
}

fn required_move(their_move: &Janken, outcome: &Outcome) -> Janken {
    match (their_move, outcome) {
        (Janken::Rock, Outcome::Draw) 
            | (Janken::Paper, Outcome::Loss) 
            | (Janken::Scissors, Outcome::Win) 
            => Janken::Rock,
        (Janken::Rock, Outcome::Win) 
            | (Janken::Paper, Outcome::Draw) 
            | (Janken::Scissors, Outcome::Loss) 
            => Janken::Paper,
        (Janken::Rock, Outcome::Loss) 
            | (Janken::Paper, Outcome::Win) 
            | (Janken::Scissors, Outcome::Draw) 
            => Janken::Scissors,
    }
}

fn round_score_part1(their_move: &char, my_move: &char) -> u32 {
    let their_move = their_code_to_janken(their_move);
    let my_move = my_code_to_janken(my_move);

    move_score(&their_move, &my_move) + shape_score(&my_move)
}

fn round_score_part2(their_move: &char, outcome: &char) -> u32 {
    let their_move = their_code_to_janken(their_move);
    let outcome = code_to_outcome(outcome);
    let my_move = required_move(&their_move, &outcome);
    
    move_score(&their_move, &my_move) + shape_score(&my_move)
}

fn calculate_score_part1(strategy_guide: &HashMap<(char, char), u32>) -> u32 {
    strategy_guide.iter()
        .map(|((lhs, rhs), occur)| round_score_part1(lhs, rhs) * occur)
        .sum()
}

fn calculate_score_part2(strategy_guide: &HashMap<(char, char), u32>) -> u32 {
    strategy_guide.iter()
        .map(|((lhs, rhs), occur)| round_score_part2(lhs, rhs) * occur)
        .sum()
}

pub fn run() {
    let input = utils::read_file_to_lines("data/year2022/day02")
        .iter()
        .map(|str| parse_line(&str).unwrap())
        .collect();
    let input = uniqueify(input);
    println!("{}", calculate_score_part1(&input));
    println!("{}", calculate_score_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2022_day02_part1() {
        let strategy_guide = vec![
            "A Y",
            "B X",
            "C Z",
            ]
                .iter()
                .map(|str| str.to_string())
                .map(|str| parse_line(&str).unwrap())
                .collect();
        let strategy_guide = uniqueify(strategy_guide);
        assert_eq!(calculate_score_part1(&strategy_guide), 15);
    }

    #[test]
    fn year2022_day02_part2() {
        let strategy_guide = vec![
            "A Y",
            "B X",
            "C Z",
            ]
                .iter()
                .map(|str| str.to_string())
                .map(|str| parse_line(&str).unwrap())
                .collect();
        let strategy_guide = uniqueify(strategy_guide);
        assert_eq!(calculate_score_part2(&strategy_guide), 12);
    }
}
