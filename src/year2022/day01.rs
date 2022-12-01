use crate::utils::utils;

fn most_calories_carried_by_one_elf(entries: &Vec<String>) -> u32 {
    most_calories_carried_by_top_n_elves(entries, 1)[0]
}

fn sum_of_calories_carried_by_top_three_elves(entries: &Vec<String>) -> u32 {
    most_calories_carried_by_top_n_elves(entries, 3)
        .iter()
        .sum()
}

fn insert_into(mut v: Vec<u32>, entry: u32) -> Vec<u32> {
    if entry > v[v.len() - 1] {
        for i in (0..v.len()).rev() {
            if entry > v[i] {
                if i < v.len()-1 { v[i+1] = v[i]; }
                if i == 0 { v[i] = entry; }
            } else {
                v[i+1] = entry; 
                break;
            }
        }
    }
    v
}

fn most_calories_carried_by_top_n_elves(entries: &Vec<String>, n: usize) -> Vec<u32> {
    let (sum, largest) = entries.iter()
        .fold((0, vec![u32::MIN; n]), |(sum, largest), entry| {
            if "".eq(entry) {
                (0, insert_into(largest, sum))
            } else {
                (sum + entry.parse::<u32>().unwrap(), largest)
            }
        });
    insert_into(largest, sum)
}

pub fn run() {
    let input: Vec<String> = utils::read_file_to_lines("data/year2022/day01");
    println!("{}", most_calories_carried_by_one_elf(&input));
    println!("{}", sum_of_calories_carried_by_top_three_elves(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2022_day01_part1() {
        let report = vec![
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
            ]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(most_calories_carried_by_one_elf(&report), 24000);
    }

    #[test]
    fn year2022_day01_part2() {
        let report = vec![
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
            ]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(sum_of_calories_carried_by_top_three_elves(&report), 45000);
    }
}
