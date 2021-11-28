extern crate regex;

use regex::Regex;
use unordered_pair::UnorderedPair;
use std::collections::{HashMap, HashSet};

use crate::year2015::utils;

fn parse_line(line: &str, cities: &mut HashSet<String>, distances: &mut HashMap<UnorderedPair<String>, u32>) -> (String, String, u32) {
    let re = Regex::new(r"(.*) to (.*) = (.*)").unwrap();
    let caps = re.captures(line).unwrap();
    cities.insert(caps[1].to_string());
    cities.insert(caps[2].to_string());
    distances.insert(UnorderedPair::from((caps[1].to_string(), caps[2].to_string())), caps[3].parse::<u32>().unwrap());
    (caps[1].to_string(), caps[2].to_string(), caps[3].parse::<u32>().unwrap())
}

fn calculate_distance(cities: &Vec<&String>, distances: &HashMap<UnorderedPair<String>, u32>) -> u32 {
    let mut total_len = 0;
    for i in 1..cities.len() {
        let city_pair = UnorderedPair::from((cities[i - 1].to_string(), cities[i].to_string()));
        total_len += match distances.get(&city_pair) {
            Some(distance) => distance,
            None => panic!(),
        };
    }
    total_len
}

// uses heaps algorithm
fn traveling_salesman(cities: &HashSet<String>, distances: &HashMap<UnorderedPair<String>, u32>) -> u32 {
    let mut min_distance = u32::MAX;
    let mut cities: Vec<_> = cities.into_iter().collect();
    let mut c = vec![0; cities.len()];

    min_distance = min_distance.min(calculate_distance(&cities, &distances));

    let mut i = 0;
    while i < cities.len() {
        if c[i] < i {
            if i % 2 == 0 {
                cities.swap(0, i);
            } else {
                cities.swap(c[i], i);
            }
            min_distance = min_distance.min(calculate_distance(&cities, &distances));
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    min_distance
}

// uses heaps algorithm
fn traveling_salesman_maxxed(cities: &HashSet<String>, distances: &HashMap<UnorderedPair<String>, u32>) -> u32 {
    let mut max_distance = 0;
    let mut cities: Vec<_> = cities.into_iter().collect();
    let mut c = vec![0; cities.len()];

    max_distance = max_distance.max(calculate_distance(&cities, &distances));

    let mut i = 0;
    while i < cities.len() {
        if c[i] < i {
            if i % 2 == 0 {
                cities.swap(0, i);
            } else {
                cities.swap(c[i], i);
            }
            max_distance = max_distance.max(calculate_distance(&cities, &distances));
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    max_distance
}

pub fn run() {
    let input = utils::read_file_to_lines("data/year2015/day09");
    let mut cities = HashSet::new();
    let mut distances = HashMap::new();

    for line in input {
        parse_line(&line, &mut cities, &mut distances);
    }
    println!("{}", traveling_salesman(&cities, &distances));
    println!("{}", traveling_salesman_maxxed(&cities, &distances));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day09_test_provided_example() {
        let input = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];
        let mut cities = HashSet::new();
        let mut distances = HashMap::new();

        for line in input {
            parse_line(&line, &mut cities, &mut distances);
        }
        assert_eq!(traveling_salesman(&cities, &distances), 605);
    }
}
