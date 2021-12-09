use crate::utils::utils;

fn triangle_number(n: u32) -> u32 {
    return n*(n+1) / 2;
}

fn minimum_fuel_crab_metric(positions: &Vec<u32>) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut m_sum = u32::MAX;
    for i in min..=max {
        let sum = positions.iter()
            .map(|position| triangle_number(if position > &i { position - i } else { i - position }))
            .sum();
        if sum < m_sum {
            m_sum = sum;
        }
    }
    m_sum
}

fn minimum_fuel_amount(positions: &Vec<u32>) -> u32 {
    let mut positions = positions.clone();
    positions.sort();
    let median = positions[positions.len() / 2];
    positions.iter()
        .map(|position| if position > &median { position - median } else { median - position })
        .sum()
}

pub fn run() {
    let initial_positions: Vec<u32> = utils::read_file_to_string("data/year2021/day07")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    println!("{}", minimum_fuel_amount(&initial_positions));
    println!("{}", minimum_fuel_crab_metric(&initial_positions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day07() {
        let initial_positions = "16,1,2,0,4,2,7,1,2,14".split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(minimum_fuel_amount(&initial_positions), 37);
        assert_eq!(minimum_fuel_crab_metric(&initial_positions), 168);
    }
}
