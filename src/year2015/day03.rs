use std::collections::HashSet;

use crate::year2015::utils;

fn calculate_house_deliveries(instructions: &str) -> usize {
    let init = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(init);
    instructions.chars() 
        .fold((init, visited), |(old_coord, mut visited), dir|
              {
                  let new_coord = match dir {
                      '^' => (old_coord.0, old_coord.1 + 1),
                      '>' => (old_coord.0 + 1, old_coord.1),
                      'v' => (old_coord.0, old_coord.1 - 1),
                      '<' => (old_coord.0 - 1, old_coord.1),
                      _ => old_coord
                  };
                  visited.insert(new_coord);
                  (new_coord, visited)
              }
             )
        .1
        .len()
}

fn calculate_house_deliveries_with_robosanta(instructions: &str) -> usize {
    let init = (0, 0);
    let santa_init = init;
    let robo_santa_init = init;
    let mut visited = HashSet::new();
    visited.insert(init);
    instructions.chars()
        .fold((santa_init, robo_santa_init, visited),
              |(old_l_coord, old_r_coord, mut visited), dir|
              {
                  let new_l_coord = match dir {
                      '^' => (old_l_coord.0, old_l_coord.1 + 1),
                      '>' => (old_l_coord.0 + 1, old_l_coord.1),
                      'v' => (old_l_coord.0, old_l_coord.1 - 1),
                      '<' => (old_l_coord.0 - 1, old_l_coord.1),
                      _ => old_l_coord
                  };
                  visited.insert(new_l_coord);
                  (old_r_coord, new_l_coord, visited)
              }
             )
        .2
        .len()
}

pub fn run() {
    let input = utils::read_file_to_string("data/year2015/day03");
    println!("{}", calculate_house_deliveries(&input));
    println!("{}", calculate_house_deliveries_with_robosanta(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2015_day03_test_calculate_house_deliveries() {
        assert_eq!(calculate_house_deliveries(">"), 2);
        assert_eq!(calculate_house_deliveries("^>v<"), 4);
        assert_eq!(calculate_house_deliveries("^v^v^v^v^v"), 2);
    }

    #[test]
    fn year2015_day03_test_calculate_house_deliveries_with_robosanta() {
        assert_eq!(calculate_house_deliveries_with_robosanta("^v"), 3);
        assert_eq!(calculate_house_deliveries_with_robosanta("^>v<"), 3);
        assert_eq!(calculate_house_deliveries_with_robosanta("^v^v^v^v^v"), 11);
    }
}
