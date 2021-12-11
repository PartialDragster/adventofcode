use crate::utils::utils;

#[derive(PartialEq)]
enum Score {
    Complete,
    Incomplete(u64),
    Corrupted(u64),
}

fn get_score(input: &str) -> Score {
    let mut stack = vec![];
    match input.chars()
        .find_map(|ch| match ch {
            '(' | '[' | '{' | '<' => {
                //println!("pushing {}", ch);
                stack.push(ch);
                None
            },
            ')' => match stack.pop() {
                Some('(') => None,
                _ => Some(Score::Corrupted(3)),
            }
            ']' => match stack.pop() {
                Some('[') => None,
                _ => Some(Score::Corrupted(57)),
            }
            '}' => match stack.pop() {
                Some('{') => None,
                _ => Some(Score::Corrupted(1197)),
            }
            '>' => match stack.pop() {
                Some('<') => None,
                _ => Some(Score::Corrupted(25137)),
            }
            _ => None,
        }) {
            Some(n) => n,
            None => {
                if stack.len() == 0 {
                    Score::Complete
                } else {
                    Score::Incomplete(stack.iter()
                        .rev()
                        .fold(0, |sum, ch| 5*sum + match ch {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        }))
                }
            }, 
        }
}

pub fn run() {
    let subsystem: Vec<String> = utils::read_file_to_lines("data/year2021/day10");
    let score = subsystem.iter()
        .fold(0, |sum, line| sum + match get_score(&line) {
            Score::Corrupted(n) => n,
            _ => 0,
        });
    println!("{}", score);

    let mut score: Vec<u64> = subsystem.iter()
        .filter_map(|line| match get_score(&line) {
            Score::Incomplete(n) => Some(n),
            _ => None})
        .collect();
    score.sort();
    println!("{}", score[score.len() / 2]);
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn year2021_day10() {
        assert!(get_score(&"()") == Score::Complete);
        assert!(get_score(&"[]") == Score::Complete);
        assert!(get_score(&"([])") == Score::Complete);
        assert!(get_score(&"{()()()}") == Score::Complete);
        assert!(get_score(&"<([{}])>") == Score::Complete);
        assert!(get_score(&"[<>({}){}[([])<>]]") == Score::Complete);
        assert!(get_score(&"(((((((((())))))))))") == Score::Complete);

        let subsystem: Vec<String> = vec! [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ].iter()
            .map(|s| s.to_string())
            .collect();
        let score = subsystem.iter()
            .fold(0, |sum, line| sum + match get_score(&line) {
                Score::Corrupted(n) => n,
                _ => 0,
            });
        assert_eq!(score, 26397);

    }
}
