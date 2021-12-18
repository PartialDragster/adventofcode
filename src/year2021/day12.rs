use crate::utils::utils;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn is_big_cave(s: &str) -> bool {
    Regex::new(r"^[A-Z]+$").unwrap().is_match(s)
}

fn count_paths(system: &Vec<String>) -> u32 {
    let system = parse(system); 
    count_paths_given(&"start".to_owned(), &"end".to_owned(), &mut Vec::new(), &system, false)
}

fn count_paths_twice_small_visit(system: &Vec<String>) -> u32 {
    let system = parse(system); 
    count_paths_given(&"start".to_owned(), &"end".to_owned(), &mut Vec::new(), &system, true)
}

fn count_paths_given<'a>(current: &'a String, goal: &'a String, path: &mut Vec<&'a String>, system: &'a HashMap<String, HashSet<String>>, allow_twice_small_visit: bool) -> u32 {
    if current == goal { return 1; }

    let mut count = 0;
    path.push(current);
    for connected_cave in system.get(current).unwrap() {
        if connected_cave == "start" { continue; }

        if is_big_cave(&connected_cave) 
            || !path.contains(&&connected_cave) 
            || allow_twice_small_visit {
                count += count_paths_given(&connected_cave, goal, path, system, allow_twice_small_visit && (is_big_cave(&connected_cave) || !path.contains(&&connected_cave)));
        }
    }
    path.pop();
    count
}

fn parse(system: &Vec<String>) -> HashMap<String, HashSet<String>> {
    system.iter()
        .map(|l| {
            let mut split = l.split("-");
            (split.next().unwrap().to_string(), split.next().unwrap().to_string())})
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.entry(key.clone())
              .or_insert(HashSet::new())
              .insert(value.clone());
            map.entry(value)
                .or_insert(HashSet::new())
                .insert(key);
            map})
}

pub fn run() {
    let system: Vec<String> = utils::read_file_to_lines("data/year2021/day12");
    println!("{}", count_paths(&system));
    println!("{}", count_paths_twice_small_visit(&system));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day12() {
        let system = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(is_big_cave("start"), false);
        assert_eq!(is_big_cave("A"), true);
        assert_eq!(is_big_cave("b"), false);
        assert_eq!(is_big_cave("c"), false);
        assert_eq!(is_big_cave("d"), false);
        assert_eq!(is_big_cave("end"), false);
        assert_eq!(count_paths(&system), 10);
        assert_eq!(count_paths_twice_small_visit(&system), 36);

        let system=vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc",
        ].iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(count_paths(&system), 19);
        assert_eq!(count_paths_twice_small_visit(&system), 103);

        let system=vec![
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW",
        ].iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(count_paths(&system), 226);
        assert_eq!(count_paths_twice_small_visit(&system), 3509);
    }
}
