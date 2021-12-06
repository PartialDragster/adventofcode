use crate::utils::utils;
use regex::Regex;
use std::cmp::{min, max};
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    p1: (u32, u32),
    p2: (u32, u32),
}

impl Line {
    fn new(x1: u32, x2: u32, y1: u32, y2: u32) -> Self {
        Line {
            p1: (x1, x2),
            p2: (y1, y2),
        }
    }

    fn is_straight(&self) -> bool {
        self.p1.0 == self.p2.0 || self.p1.1 == self.p2.1
    }
}
        

fn parse_input(input: &Vec<String>) -> Vec<Line> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    input.iter()
        .map(|str| {
            let caps = re.captures(str)
                .unwrap();
            Line::new(caps.get(1).unwrap().as_str().parse().unwrap(), 
                      caps.get(2).unwrap().as_str().parse().unwrap(),
                      caps.get(3).unwrap().as_str().parse().unwrap(),
                      caps.get(4).unwrap().as_str().parse().unwrap())})
        .collect()
}

fn crossings(lines: &Vec<Line>) -> HashMap<(u32, u32), u32> {
    lines.iter()
        .fold(HashMap::new(), |mut map, line| {
            let max_x = max(line.p1.0, line.p2.0);
            let min_x = min(line.p1.0, line.p2.0);
            let max_y = max(line.p1.1, line.p2.1);
            let min_y = min(line.p1.1, line.p2.1);
            let steps = max(max_x - min_x, max_y - min_y) as i32;
            for t in 0..=steps {
                let x = (t*(line.p2.0 as i32 - line.p1.0 as i32)/steps + line.p1.0 as i32) as u32;
                let y = (t*(line.p2.1 as i32 - line.p1.1 as i32)/steps + line.p1.1 as i32) as u32;
                *map.entry((x, y)).or_insert(0) += 1;
            }
            map})
}

fn at_least_two_overlap(map: HashMap<(u32, u32), u32>) -> usize {
    map.iter()
        .filter(|(_, v)| *v >= &2)
        .count()
}

pub fn run() {
    let input: Vec<String> = utils::read_file_to_lines("data/year2021/day05");

    let mut lines: Vec<Line> = parse_input(&input);
    lines.retain(|l| l.is_straight());
    let crossings_map = crossings(&lines);
    let count = at_least_two_overlap(crossings_map);
    println!("{}", count);

    let lines: Vec<Line> = parse_input(&input);
    let crossings_map = crossings(&lines);
    let count = at_least_two_overlap(crossings_map);
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day05_test() {
        let input = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"];
        let input = input.iter()
            .map(|s| s.to_string())
            .collect();
        let mut lines: Vec<Line> = parse_input(&input);
        lines.retain(|l| l.is_straight());
        let crossings_map = crossings(&lines);
        let count = at_least_two_overlap(crossings_map);
        assert_eq!(count, 5);

        let input = input.iter()
            .map(|s| s.to_string())
            .collect();
        let lines: Vec<Line> = parse_input(&input);
        let crossings_map = crossings(&lines);
        let count = at_least_two_overlap(crossings_map);
        assert_eq!(count, 12);

//        assert_eq!(get_winning_board_score(&draws, &boards), 4512);
    }
}
