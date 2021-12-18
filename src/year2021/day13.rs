use crate::utils::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};

fn print_sheet(dots: &HashSet<(u32, u32)>) {
    let (max_x, max_y) = dots.iter()
        .fold( (0, 0), |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)));
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("");
    }
    println!("");
}

fn apply_fold(folds: &mut LinkedList<(char, u32)>, dots: &mut HashSet<(u32, u32)>) {
    let (axis, line) = folds.pop_front().unwrap();
    
    let folded_dots = dots.drain()
        .fold(HashSet::new(), |mut folded_dots, (x, y)| match axis {
            'x' => {
                let x = if x < line { 
                    x 
                } else {
                    2*line - x
                };
                folded_dots.insert((x, y));
                folded_dots
            },
            'y' => {
                let y = if y < line {
                    y
                } else { 
                    2*line - y
                };
                folded_dots.insert((x, y));
                folded_dots
            },
            _ => panic!()
        });
    dots.clear();
    dots.extend(folded_dots);
}

fn parse_instructions(instructions: &Vec<String>) -> (HashSet<(u32, u32)>, LinkedList<(char, u32)>){
    let mut dots = HashSet::new();
    let mut folds = LinkedList::new();

    let mut iter = instructions.iter();

    let loc_re = Regex::new(r"^(\d*),(\d*)$").unwrap();
    while let Some(instruction) = iter.next() {
        if instruction == &"" { break; }

        let caps = loc_re.captures(instruction).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let y = caps.get(2).unwrap().as_str().parse().unwrap();
        dots.insert((x, y));
    }

    let fold_re = Regex::new(r"^fold along ([xy])=(\d*)$").unwrap();
    while let Some(instruction) = iter.next() {
        let caps = fold_re.captures(instruction).unwrap();
        let axis = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let line = caps.get(2).unwrap().as_str().parse().unwrap();
        folds.push_back((axis, line));
    }

    (dots,folds)
}

pub fn run() {
    let instructions: Vec<String> = utils::read_file_to_lines("data/year2021/day13");
    let (mut dots, mut folds) = parse_instructions(&instructions);
    apply_fold(&mut folds, &mut dots);
    println!("{}", dots.len());
    while folds.len() > 0 {
        apply_fold(&mut folds, &mut dots);
    }
    print_sheet(&dots);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day13() {
        let instructions = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
            ].iter().map(|s| s.to_string()).collect();
        let (mut dots, mut folds) = parse_instructions(&instructions);
        print_sheet(&dots);
        apply_fold(&mut folds, &mut dots);
        print_sheet(&dots);
        assert_eq!(dots.len(), 17);
        apply_fold(&mut folds, &mut dots);
        print_sheet(&dots);
    }
}
