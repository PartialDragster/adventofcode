use crate::year2015::utils;

fn count_characters(sample: &str) -> (usize, usize) {
    let mut code_character_count = 0;
    let mut string_character_count = 0;

    let mut char_stream = sample.chars();
    if char_stream.next().unwrap() == '"' {
        code_character_count += 1;
    } else {
        panic!();
    }
    loop {
        match char_stream.next() {
            Some('\\') => {
                code_character_count += 1;
                match char_stream.next() {
                    Some('\\') => {
                        code_character_count += 1;
                        string_character_count += 1;
                    }
                    Some('"') => {
                        code_character_count += 1;
                        string_character_count += 1;
                    }
                    Some('x') => {
                        code_character_count += 1;
                        if char_stream.next().unwrap().is_digit(16) && char_stream.next().unwrap().is_digit(16) {
                            code_character_count += 2;
                            string_character_count += 1;
                        } else {
                            panic!();
                        }
                    }
                    _ => {
                        panic!();
                    }

                }
            },
            Some('"') => {
                code_character_count += 1;
                return (code_character_count, string_character_count);
            }
            Some(_) => {
                code_character_count += 1;
                string_character_count += 1;
            }
            _ => {
                panic!();
            }
        }
    }
}

pub fn run() {
    let input = utils::read_file_to_lines("data/year2015/day08");
    let r1 = input.iter()
        .fold(0, |a, b| {
            let (ccc, scc) = count_characters(b);
            a + (ccc - scc)
        });
    println!("{}", r1);
    let r2 = input.iter()
        .map(|s| s.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#))
        .map(|s| format!(r#""{}""#, s))
        .fold(0, |a, b| {
            let (ccc, scc) = count_characters(&b);
            a + (ccc - scc)
        });
    println!("{}", r2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn year2015_day08_test_provided_example() {
        assert_eq!(count_characters(r#""""#), (2,0));
        assert_eq!(count_characters(r#""abc""#), (5,3));
        assert_eq!(count_characters(r#""aaa\"aaa""#), (10,7));
        assert_eq!(count_characters(r#""\x27""#), (6,1));
    }
}
