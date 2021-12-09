use crate::utils::utils;
use std::collections::HashSet;

fn is_basin((i, j): (usize, usize), floor_map: &Vec<Vec<u32>>) -> bool {
    (i == 0 || floor_map[i-1][j] > floor_map[i][j])
        && (i == floor_map.len() - 1 || floor_map[i+1][j] > floor_map[i][j])
        && (j == 0 || floor_map[i][j-1] > floor_map[i][j])
        && (j == floor_map[i].len() - 1 || floor_map[i][j+1] > floor_map[i][j])
}

fn calculate_basin_size(basin_sink: &(usize, usize), floor_map: &Vec<Vec<u32>>) -> usize {
    let mut to_check: Vec<(usize, usize)> = vec![*basin_sink];
    let mut in_basin: HashSet<(usize, usize)> = HashSet::new();

    while !to_check.is_empty() {
        let el = to_check.pop().unwrap();
        if floor_map[el.0][el.1] == 9 { continue; }
        if in_basin.contains(&el) { continue; }

        in_basin.insert(el);

        if el.0 != 0 { to_check.push((el.0 - 1, el.1)); }
        if el.0 != floor_map.len()-1 { to_check.push((el.0 + 1, el.1)); }
        if el.1 != 0 { to_check.push((el.0, el.1 - 1)); }
        if el.1 != floor_map[el.0].len() - 1 { to_check.push((el.0, el.1 + 1)); }
    }
    in_basin.len()
}

fn three_basin_mult(floor_map: &Vec<Vec<u32>>) -> usize {
    let mut basin_sizes = vec![];
    for i in 0..floor_map.len() {
        for j in 0..floor_map[i].len() {
            if is_basin((i, j), floor_map) {
                let basin_size = calculate_basin_size(&(i,j), floor_map);
                if basin_sizes.len() < 3 {
                    basin_sizes.push(basin_size);
                } else {
                    let basin_sizes_min = *basin_sizes.iter().min().unwrap();
                    if basin_size > basin_sizes_min {
                        basin_sizes.swap_remove(basin_sizes.iter().position(|e| e == &basin_sizes_min).unwrap());
                        basin_sizes.push(basin_size);
                    }
                }
            }
        }
    }

   basin_sizes.iter().product()
}

fn low_level_risk_sum(floor_map: &Vec<Vec<u32>>) -> u32 {
    let mut total_risk = 0;
    for i in 0..floor_map.len() {
        for j in 0..floor_map[i].len() {
            if is_basin((i, j), floor_map) {
                   total_risk += floor_map[i][j] + 1;
               }
        }
    }
    total_risk
}

pub fn run() {
    let floor_map: Vec<Vec<u32>> = utils::read_file_to_lines("data/year2021/day09")
        .iter()
        .filter(|s| s != &"")
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("{}", low_level_risk_sum(&floor_map));
    println!("{}", three_basin_mult(&floor_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day09() {
        let floor_map: Vec<Vec<u32>> = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"]
                .iter()
                .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect();
        assert_eq!(low_level_risk_sum(&floor_map), 15);
        assert_eq!(three_basin_mult(&floor_map), 1134);
    }
}
