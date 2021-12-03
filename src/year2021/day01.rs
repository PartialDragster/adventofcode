use crate::utils::utils;

fn increase_count(report: &Vec<u32>) -> u32 {
    report[1..].iter()
        .fold((0, report[0]), |(count, previous_value), current_value| (if current_value > &previous_value { count + 1 } else { count }, *current_value))
        .0
}

fn increase_three_window(report: &Vec<u32>) -> u32 {
    let window_c = report[2];
    let window_b = report[1] + window_c;
    let window_a = report[0] + window_b;
    report[3..].iter()
        .fold((0, window_a, window_b, window_c), |(count, w0, w1, w2), current_value| (if w1 + current_value > w0 { count + 1} else { count }, w1 + *current_value, w2 + *current_value, *current_value))
        .0
}

pub fn run() {
    let input: Vec<u32> = utils::read_file_to_lines("data/year2021/day01")
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect();
    println!("{}", increase_count(&input));
    println!("{}", increase_three_window(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day01_test_increase_count() {
        let report = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263];
        assert_eq!(increase_count(&report), 7);
    }

    #[test]
    fn year2021_day01_test_increase_three_window() {
        let report = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263];
        assert_eq!(increase_three_window(&report), 5);
    }
}
