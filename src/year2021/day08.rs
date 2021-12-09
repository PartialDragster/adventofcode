use crate::utils::utils;
use std::collections::{BTreeMap, BTreeSet};

fn get_one(codes: &BTreeSet<BTreeSet<char>>) -> &BTreeSet<char> {
    codes.iter()
        .filter(|set| set.len() == 2)
        .nth(0)
        .unwrap()
}

fn get_four(codes: &BTreeSet<BTreeSet<char>>) -> &BTreeSet<char> {
    codes.iter()
        .filter(|set| set.len() == 4)
        .nth(0)
        .unwrap()
}

fn get_seven(codes: &BTreeSet<BTreeSet<char>>) -> &BTreeSet<char> {
    codes.iter()
        .filter(|set| set.len() == 3)
        .nth(0)
        .unwrap()
}

fn get_eight(codes: &BTreeSet<BTreeSet<char>>) -> &BTreeSet<char> {
    codes.iter()
        .filter(|set| set.len() == 7)
        .nth(0)
        .unwrap()
}

fn get_nine<'a>(codes: &'a BTreeSet<BTreeSet<char>>, four: &BTreeSet<char>, seven: &BTreeSet<char>) -> &'a BTreeSet<char> {
    let union = four.union(seven).clone().map(|c| *c).collect();
    codes.iter()
        .filter(|set| set.is_superset(&union) && set.len() == 6)
        .nth(0)
        .unwrap()
}

fn get_three<'a>(codes: &'a BTreeSet<BTreeSet<char>>, one: &BTreeSet<char>, four:&BTreeSet<char>, nine: &BTreeSet<char>) -> &'a BTreeSet<char> {
    let union = nine.difference(four)
        .map(|c| *c)
        .collect::<BTreeSet<char>>()
        .union(one)
        .clone()
        .map(|c| *c)
        .collect();
    codes.iter()
        .filter(|set| set.is_superset(&union) && set.len() == 5)
        .nth(0)
        .unwrap()
}

fn get_zero<'a>(codes: &'a BTreeSet<BTreeSet<char>>, one: &BTreeSet<char>, four:&BTreeSet<char>, nine: &BTreeSet<char>) -> &'a BTreeSet<char> {
    let union = nine.difference(four)
        .map(|c| *c)
        .collect::<BTreeSet<char>>()
        .union(one)
        .clone()
        .map(|c| *c)
        .collect();
    codes.iter()
        .filter(|set| set.is_superset(&union) && set.len() == 6)
        .nth(0)
        .unwrap()
}

fn get_six<'a>(codes: &'a BTreeSet<BTreeSet<char>>, one: &BTreeSet<char>, eight:&BTreeSet<char>) -> &'a BTreeSet<char> {
    let difference = eight.difference(one)
        .map(|c| *c)
        .collect();
    codes.iter()
        .filter(|set| set.is_superset(&difference))
        .nth(0)
        .unwrap()
}

fn get_five<'a>(codes: &'a BTreeSet<BTreeSet<char>>, six: &BTreeSet<char>) -> &'a BTreeSet<char> {
    codes.iter()
        .filter(|set| six.is_superset(set))
        .nth(0)
        .unwrap()
}

fn calculate_number_map(codes: &BTreeSet<BTreeSet<char>>) -> BTreeMap<BTreeSet<char>, u32> {
    let mut map = BTreeMap::new();
    let mut codes = codes.clone();


    let one = get_one(&codes).clone();
    codes.remove(&one);
    map.insert(one.clone(), 1);

    let four = get_four(&codes).clone();
    codes.remove(&four);
    map.insert(four.clone(), 4);

    let seven = get_seven(&codes).clone();
    codes.remove(&seven);
    map.insert(seven.clone(), 7);

    let eight = get_eight(&codes).clone();
    codes.remove(&eight);
    map.insert(eight.clone(), 8);

    let nine = get_nine(&codes, &four, &seven).clone();
    codes.remove(&nine);
    map.insert(nine.clone(), 9);

    let three = get_three(&codes, &one, &four, &nine).clone();
    codes.remove(&three);
    map.insert(three.clone(), 3);

    let zero = get_zero(&codes, &one, &four, &nine).clone();
    codes.remove(&zero);
    map.insert(zero.clone(), 0);

    let six = get_six(&codes, &one, &eight).clone();
    codes.remove(&six);
    map.insert(six.clone(), 6);

    let five = get_five(&codes, &six).clone();
    codes.remove(&five);
    map.insert(five.clone(), 5);

    let two = codes.iter().last().unwrap().clone();
    map.insert(two.clone(), 2);

    map
}

fn sum_outputs(display_output: &Vec<String>) -> u32 {
    display_output.iter()
        .map(|s| {
            let split_input: Vec<&str> = s.split("|").collect();
            let map = calculate_number_map(&split_input[0]
                .split(" ")
                .filter(|s| s != &"")
                .map(|s| s.chars().collect::<BTreeSet<char>>())
                .collect::<BTreeSet<BTreeSet<char>>>());
            split_input[1]
                .split(" ")
                .filter(|s| s != &"")
                .map(|s| s.chars().collect::<BTreeSet<char>>())
                .fold(0, |sum, n| 10*sum + map[&n] )})
        .sum()
}

fn count_uniques_in_output(display_output: &Vec<String>) -> usize {
    display_output.iter()
        .map(|s| s.split("|").nth(1).unwrap().trim().split(" "))
        .flatten()
        .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
        .count()
}

pub fn run() {
    let display_output: Vec<String> = utils::read_file_to_lines("data/year2021/day08");
    println!("{}", count_uniques_in_output(&display_output));
    println!("{}", sum_outputs(&display_output));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day08() {
        let display_output = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"].iter()
                .map(|s| s.to_string())
                .collect();

        assert_eq!(count_uniques_in_output(&display_output), 26);
        assert_eq!(sum_outputs(&display_output), 61229);
        //assert_eq!(sum_outputs(&display_output), 26);
    }
}
