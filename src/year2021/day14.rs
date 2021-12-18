use crate::utils::utils;
use regex::Regex;
use std::collections::HashMap;

fn get_max_min_diff(pairs: &HashMap<String, u64>) -> u64 {
    let char_counts = pairs.iter()
        .fold(HashMap::new(), |mut counts, (pair, count)| {
            let first_ch = pair.as_bytes()[0] as char;
            if first_ch != '^' {
                *counts.entry(first_ch).or_insert(0) += count;
            }
            counts
        });
    let mut min_count = u64::MAX;
    let mut max_count = 0;
    for count in char_counts.values() {
        if count < &min_count {
            min_count = *count;
        }
        if count > &max_count {
            max_count = *count;
        }
    }
    max_count - min_count
}

fn do_insertion(pairs: &mut HashMap<String, u64>, instructions: &HashMap<String, String>) {
    let mut new_pairs = HashMap::new();
    pairs.drain()
        .for_each(|(k, v)| {
            if instructions.contains_key(&k) {
                let production = &instructions[&k];
                *new_pairs.entry(format!("{}{}", k.as_bytes()[0] as char, production)).or_insert(0) += v;
                *new_pairs.entry(format!("{}{}", production, k.as_bytes()[1] as char)).or_insert(0) += v;
            } else {
                *new_pairs.entry(k).or_insert(0) += v;
            }
        });
    pairs.clear();
    pairs.extend(new_pairs);
}

fn parse_instructions(instructions: &Vec<String>) -> (HashMap<String, u64>, HashMap<String, String>){
    let mut iter = instructions.iter();
    
    let init_template = "^".to_owned() + iter.next().unwrap() + "$";
    let pairs = (1..init_template.len())
        .fold(HashMap::new(), |mut pairs, i| {
            *pairs.entry(init_template[(i-1)..=i].to_string()).or_insert(0) += 1;
            pairs
        });

    iter.next(); // get rid of newline

    let mut productions: HashMap<String, String> = HashMap::new();
    let fold_re = Regex::new(r"^(..) -> (.)$").unwrap();
    while let Some(instruction) = iter.next() {
        let caps = fold_re.captures(instruction).unwrap();
        let lhs = caps.get(1).unwrap().as_str().to_string();
        let rhs = caps.get(2).unwrap().as_str().to_string();
        productions.insert(lhs, rhs);
    }

    (pairs, productions)
}

pub fn run() {
    let instructions: Vec<String> = utils::read_file_to_lines("data/year2021/day14");
    let (mut pairs, productions) = parse_instructions(&instructions);
    for _ in 0..10 {
        do_insertion(&mut pairs, &productions);
    }
    println!("{}", get_max_min_diff(&pairs));
    for _ in 0..30 {
        do_insertion(&mut pairs, &productions);
    }
    println!("{}", get_max_min_diff(&pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day14() {
        let instructions = vec![
                "NNCB",
                "",
                "CH -> B",
                "HH -> N",
                "CB -> H",
                "NH -> C",
                "HB -> C",
                "HC -> B",
                "HN -> C",
                "NN -> C",
                "BH -> H",
                "NC -> B",
                "NB -> B",
                "BN -> B",
                "BB -> N",
                "BC -> B",
                "CC -> N",
                "CN -> C",
            ].iter().map(|s| s.to_string()).collect();
        let (mut pairs, productions) = parse_instructions(&instructions);
        for _ in 0..10 {
            do_insertion(&mut pairs, &productions);
        }
        assert_eq!(get_max_min_diff(&pairs), 1588);
        for _ in 0..30 {
            do_insertion(&mut pairs, &productions);
        }
        assert_eq!(get_max_min_diff(&pairs), 2188189693529);
    }
}
