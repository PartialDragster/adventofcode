use crate::utils::utils;

fn gamma(report: &Vec<String>) -> u32 {
    let len = report[0].len();
    report.iter()
        .map(|str| str.to_string().into_bytes())
        .fold(vec![0; len], |mut sum_array, bit_array| {
            for (rsa, rba) in sum_array.iter_mut().zip(&bit_array) {
                *rsa = *rsa + (2 * (*rba as i32 - 48) - 1);
            }
            sum_array
        })
        .iter()
        .fold(0, |r, n| 2*r + if n > &0 { 1 } else { 0 })
}

fn epsilon(report: &Vec<String>) -> u32 {
    let len = report[0].len();
    report.iter()
        .map(|str| str.to_string().into_bytes())
        .fold(vec![0; len], |mut sum_array, bit_array| {
            for (rsa, rba) in sum_array.iter_mut().zip(&bit_array) {
                *rsa = *rsa + (2 * (*rba as i32 - 48) - 1);
            }
            sum_array
        })
        .iter()
        .fold(0, |r, n| 2*r + if n < &0 { 1 } else { 0 })
}

fn get_most_common_byte_at_position(report: &Vec<String>, position: &usize) -> u8 {
    let count = report.iter()
        .fold(0, |sum, str| sum + 2*(str.as_bytes()[*position] as i32 - 48) - 1);
    if count >= 0 { '1' as u8} else { '0' as u8}
}

fn oxygen_generator(report: &Vec<String>) -> u32 {
    let len = report[0].len();
    let mut report = report.clone();
    for i in 0..len {
        let most_common_byte = get_most_common_byte_at_position(&report, &i);
        report = report.iter()
            .filter(|str| str.as_bytes()[i] == most_common_byte)
            .map(|str| str.to_string())
            .collect();
        if report.len() == 0 { break; }
    }
    report[0].chars()
        .fold(0, |sum, ch| 2 * sum + if ch == '1' { 1 } else { 0 })
}

fn get_least_common_byte_at_position(report: &Vec<String>, position: &usize) -> u8 {
    let count = report.iter()
        .fold(0, |sum, str| sum + 2*(str.as_bytes()[*position] as i32 - 48) - 1);
    if count < 0 { '1' as u8 } else { '0' as u8}
}


fn co2_scrubber(report: &Vec<String>) -> u32 {
    let len = report[0].len();
    let mut report = report.clone();
    for i in 0..len {
        let least_common_byte = get_least_common_byte_at_position(&report, &i);
        report = report.iter()
            .filter(|str| str.as_bytes()[i] == least_common_byte)
            .map(|str| str.to_string())
            .collect();
        if report.len() == 1 { break; }
    }
    report[0].chars()
        .fold(0, |sum, ch| 2 * sum + if ch == '1' { 1 } else { 0 })
}

pub fn run() {
       let input: Vec<String> = utils::read_file_to_lines("data/year2021/day03");
       println!("{}", gamma(&input)*epsilon(&input));
       println!("{}", oxygen_generator(&input)*co2_scrubber(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day03_test_gamma_epislon() {
        let report: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010" ]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(gamma(&report), 22);
        assert_eq!(epsilon(&report), 9);
    }

    #[test]
    fn year2021_day03_test_oxygen() {
        let report: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010" ]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(oxygen_generator(&report), 23);
        assert_eq!(co2_scrubber(&report), 10);
    }
}
