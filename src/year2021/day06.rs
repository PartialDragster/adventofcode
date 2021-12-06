use crate::utils::utils;
use std::collections::HashMap;

fn run_lantern_fish_simulation(timers: &Vec<u64>, days: &u64) -> u64 {
    let mut countdowns = timers.iter()
        .fold(HashMap::new(), |mut pop, timer| {
            *pop.entry(*timer).or_insert(0) += 1;
            pop
        });
    for _ in 1..=*days {
        countdowns = countdowns.iter()
            .fold(HashMap::new(), |mut pop, (remain, count)| {
                if remain > &0 {
                    *pop.entry(remain - 1).or_insert(0) += count;
                } else {
                    *pop.entry(6).or_insert(0) += count;
                    *pop.entry(8).or_insert(0) += count;
                }
                pop
            });
    }
    countdowns.values().sum()
}

pub fn run() {
    let initial_timers: Vec<u64> = utils::read_file_to_string("data/year2021/day06")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    println!("{}", run_lantern_fish_simulation(&initial_timers, &80));
    println!("{}", run_lantern_fish_simulation(&initial_timers, &256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day06() {
        let initial_timers = "3,4,3,1,2".split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(run_lantern_fish_simulation(&initial_timers, &18), 26);
    }
}
