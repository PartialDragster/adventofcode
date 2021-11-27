use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn read_input() -> Vec<(u32, u32, u32)> {
    let file = File::open("data/year2015/day02")
        .expect("Couldn't open data file for year 2015 day 02");
    io::BufReader::new(file)
        .lines()
        .map(|string| string.unwrap()
             .split('x')
             .map(|c| c.parse()
                  .unwrap())
             .collect_tuple()
             .unwrap())
        .collect()
}

fn calculate_paper((l,w,h): &(u32, u32, u32)) -> u32 {
    let lw = l*w;
    let wh = w*h;
    let hl = h*l;
    2*lw + 2*wh + 2*hl + lw.min(wh).min(hl)
}

fn calculate_ribbon((l,w,h): &(u32, u32, u32)) -> u32 {
    let lw = l + w;
    let wh = w + h;
    let hl = h + l;
    (2*lw).min(2*wh).min(2*hl) + l*w*h
}

pub fn run() {
    let input = read_input();
    let r: u32 = input.iter()
        .map(|x| calculate_paper(x))
        .fold(0, |a, b| a + b);
    println!("{}", r);
    let r: u32 = input.iter()
        .map(|x| calculate_ribbon(x))
        .fold(0, |a, b| a + b);
    println!("{}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day02_test_calculate_paper() {
        assert_eq!(calculate_paper(&(2,3,4)), 58);
        assert_eq!(calculate_paper(&(1,1,10)), 43);
    }

    #[test]
    fn year2015_day02_test_calculate_ribbon() {
        assert_eq!(calculate_ribbon(&(2,3,4)), 34);
        assert_eq!(calculate_ribbon(&(1,1,10)), 14);
    }
}
