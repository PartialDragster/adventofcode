use std::collections::{HashSet};

use crate::utils::utils;

fn contains_at_least_three_vowels(string: &str) -> bool {
    3 <= string.chars()
        .filter(|&c| "aeiou".contains(c))
        .take(3)
        .count()
}

fn contains_doubled_letter(string: &str) -> bool {
    None == string.chars()
        .try_fold('\0', |prev, x| if prev == x { None } else { Some(x) })
}

fn doesnt_contain_restricted(string: &str) -> bool {
    !string.contains("ab")
        && !string.contains("cd")
        && !string.contains("pq")
        && !string.contains("xy")
}

fn is_nice(string: &str) -> bool {
    contains_at_least_three_vowels(string)
        && contains_doubled_letter(string)
        && doesnt_contain_restricted(string)
}

fn has_repeated_two_letters(string: &str) -> bool {
    let mut chars = string.chars();
    let first_char = chars.next().unwrap();
    let second_char = chars.next().unwrap();
    None == chars.try_fold((second_char, HashSet::new(), format!("{}{}",first_char,second_char)), 
                   |(previous_char, mut hash_set, delayed_add), new_char|  
                   {
                       let pair = format!("{}{}",previous_char, new_char);
                       if hash_set.contains(&pair) {
                           None 
                       } else {
                           hash_set.insert(delayed_add);
                           Some((new_char, hash_set, pair))
                       }
                   })
}

fn has_repeated_letters_around_one(string: &str) -> bool {
    let mut chars = string.chars();
    let first_char = chars.next().unwrap();
    let second_char = chars.next().unwrap();
    None == chars.try_fold((first_char, second_char),
                    |(two_before_char, previous_char), new_char|
                    if new_char == two_before_char {
                        None
                    } else {
                        Some((previous_char, new_char))
                    })
}

fn is_nice_fixed(string: &str) -> bool {
    has_repeated_two_letters(string) 
        && has_repeated_letters_around_one(string)
}

pub fn run() {
    let string = utils::read_file_to_lines("data/year2015/day05");
    println!("{}",string.iter().filter(|s| is_nice(s)).count());
    println!("{}",string.iter().filter(|s| is_nice_fixed(s)).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day05_test_is_nice() {
        assert_eq!(contains_at_least_three_vowels("aei"), true);
        assert_eq!(contains_at_least_three_vowels("xazegov"), true);
        assert_eq!(contains_at_least_three_vowels("aeiouaeiouaeiou"), true);
        assert_eq!(contains_doubled_letter("xx"), true);
        assert_eq!(contains_doubled_letter("abcdde"), true);
        assert_eq!(contains_doubled_letter("aabbccdd"), true);
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn year2015_day05_test_is_nice_fixed() {
        assert_eq!(has_repeated_two_letters("xyxy"), true);
        assert_eq!(has_repeated_two_letters("aabcdefgaa"), true);
        assert_eq!(has_repeated_two_letters("aaa"), false);
        assert_eq!(has_repeated_letters_around_one("xyx"), true);
        assert_eq!(has_repeated_letters_around_one("abcdefeghi"), true);
        assert_eq!(has_repeated_letters_around_one("aaa"), true);
        assert_eq!(is_nice_fixed("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_fixed("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_fixed("ieodomkazucvgmuy"), false);
    }
}
