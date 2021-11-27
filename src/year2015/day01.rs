use std::fs;

fn char_to_floor_change<'a>(instructions: &'a str) -> impl Iterator<Item = i32> + 'a {
    instructions.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0})
}

fn calculate_floor(instructions: &str) -> i32 {
    char_to_floor_change(instructions)
        .fold(0, |a, b| a + b)
}

fn calculate_first_basement(instructions: &str) -> usize {
    char_to_floor_change(instructions)
        .try_fold((0, 0), |a, b| {
            let floor = a.0 + b;
            let position = a.1 + 1;
            if floor != -1 { Ok((floor, position)) } else { Err(position) }
        })
        .err()
        .unwrap()
}

fn read_input() -> String {
    fs::read_to_string("data/year2015/day01")
        .expect("Something went wrong reading the file")
}

pub fn run() {
    let input: String = read_input();
    println!("{}", calculate_floor(&input));
    println!("{}", calculate_first_basement(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day01_test_calculate_floor() {
        assert_eq!(calculate_floor("(())"), 0);
        assert_eq!(calculate_floor("()()"), 0);
        assert_eq!(calculate_floor("((("), 3);
        assert_eq!(calculate_floor("(()(()("), 3);
        assert_eq!(calculate_floor("))((((("), 3);
        assert_eq!(calculate_floor("())"), -1);
        assert_eq!(calculate_floor("))("), -1);
        assert_eq!(calculate_floor(")))"), -3);
        assert_eq!(calculate_floor(")())())"), -3);
    }

    #[test]
    fn year2015_day01_test_calculate_first_basement() {
        assert_eq!(calculate_first_basement(")"), 1);
        assert_eq!(calculate_first_basement("()())"), 5);
    }
}
