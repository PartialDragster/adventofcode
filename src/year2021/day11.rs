use crate::utils::utils;

struct Octopus {
    last_flash_step: usize,
    current_energy_level: u32,
}

fn run_simulation(oct_levels: &mut Vec<Vec<Octopus>>, current_day: &mut usize, days: &usize) -> u32 {
    let mut total_flashes = 0;
    let final_day = *current_day + days;
    while *current_day < final_day {
        *current_day += 1;
        let mut octopuses_to_flash : Vec<(usize, usize)> = vec![];
        for i in 0..oct_levels.len() {
            for j in 0..oct_levels[i].len() {
                oct_levels[i][j].current_energy_level += 1;
                if oct_levels[i][j].current_energy_level > 9 {
                    octopuses_to_flash.push((i, j));
                }
            }
        }

        while octopuses_to_flash.len() > 0 {
            let oct_loc = octopuses_to_flash.pop().unwrap();
            if oct_levels[oct_loc.0][oct_loc.1].last_flash_step == *current_day {
                continue;
            }
            total_flashes += 1;
            oct_levels[oct_loc.0][oct_loc.1].current_energy_level = 0;
            oct_levels[oct_loc.0][oct_loc.1].last_flash_step = *current_day;
            for i in (1.max(oct_loc.0)-1)..=(oct_levels.len()-1).min(oct_loc.0 + 1) {
                for j in (1.max(oct_loc.1)-1)..=(oct_levels[i].len()-1).min(oct_loc.1 + 1) {
                    if i == oct_loc.0 && j == oct_loc.1 {
                        continue;
                    }
                    if oct_levels[i][j].last_flash_step == *current_day {
                        continue;
                    }
                    oct_levels[i][j].current_energy_level += 1;
                    if oct_levels[i][j].current_energy_level > 9 && !octopuses_to_flash.contains(&(i, j)) {
                        octopuses_to_flash.push((i, j));
                    }
                }
            }
        }
    }

    total_flashes
}

fn find_all_flash_day(oct_levels: &mut Vec<Vec<Octopus>>) -> usize {
    let total_octopuses: u32 = oct_levels.iter().fold(0, |sum, row| sum + row.len()) as u32;
    let mut day = 0;
    loop {
        if run_simulation(oct_levels, &mut day, &1) == total_octopuses {
            return day;
        }
    }
}

pub fn run() {
    let mut oct_levels: Vec<Vec<Octopus>>= utils::read_file_to_lines("data/year2021/day11")
        .iter()
        .map(|s| s.chars().map(|c| Octopus {
            last_flash_step : usize::MIN,
            current_energy_level : c.to_digit(10).unwrap()})
             .collect())
        .collect();
    println!("{}", run_simulation(&mut oct_levels, &mut 0, &100));
    let mut oct_levels: Vec<Vec<Octopus>>= utils::read_file_to_lines("data/year2021/day11")
        .iter()
        .map(|s| s.chars().map(|c| Octopus {
            last_flash_step : usize::MIN,
            current_energy_level : c.to_digit(10).unwrap()})
             .collect())
        .collect();
    println!("{}", find_all_flash_day(&mut oct_levels));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day11_part1() {
        let mut oct_levels: Vec<Vec<Octopus>> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ].iter()
            .map(|s| s.chars().map(|c| Octopus { 
                last_flash_step : usize::MIN, 
                current_energy_level: c.to_digit(10).unwrap()
            }).collect())
            .collect();
        assert_eq!(run_simulation(&mut oct_levels, &mut 0, &100), 1656);
    }

    #[test]
    fn year2021_day11_part2() {
        let mut oct_levels: Vec<Vec<Octopus>> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ].iter()
            .map(|s| s.chars().map(|c| Octopus { 
                last_flash_step : usize::MIN, 
                current_energy_level: c.to_digit(10).unwrap()
            }).collect())
            .collect();
        assert_eq!(find_all_flash_day(&mut oct_levels), 195);
    }
}
