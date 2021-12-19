use crate::utils::utils;
use std::collections::{ HashSet, HashMap };

fn taxicab(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.max(p2.0) - p1.0.min(p2.0)
        + p1.1.max(p2.1) - p1.1.min(p1.0)
}

fn neighbour_checker(
    current: &(usize, usize), 
    neighbour: &(usize, usize), 
    goal: &(usize, usize), 
    open_set: &mut HashSet<(usize, usize)>, 
    f_score: &mut HashMap<(usize, usize), usize>, 
    g_score: &mut HashMap<(usize, usize), u32>, 
    map: &Vec<Vec<u32>>) {
    let tentative_g_score = g_score[current] + map[neighbour.0][neighbour.1];
    if tentative_g_score < *g_score.entry(*neighbour).or_insert(u32::MAX) {
        g_score.insert(*neighbour, tentative_g_score);
        f_score.insert(*neighbour, tentative_g_score as usize + taxicab(neighbour, goal));
        open_set.insert(*neighbour);
    }
}

fn a_star(start: &(usize, usize), goal: &(usize, usize), map: &Vec<Vec<u32>>) -> u32 {
    let mut open_set = HashSet::new();
    open_set.insert(*start);

    let mut g_score = HashMap::new();
    g_score.insert(*start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(*start, taxicab(start, goal));

    while !open_set.is_empty() {
        // couldn't seem to find reduce on HashSet's iter??
        let mut open_set_iter = open_set.iter();
        let mut current = *open_set_iter.next().unwrap();
        while let Some(open_set_element) = open_set_iter.next() {
            if f_score[open_set_element] < f_score[&current] {
                current = *open_set_element;
            }
        }

        if current == *goal {
            return g_score[&current];
        }

        open_set.remove(&current);
        if current.0 > 0 { 
            neighbour_checker(&current, &(current.0 - 1, current.1), goal, &mut open_set, &mut f_score, &mut g_score, map);        
        }
        if current.0 < map.len() - 1 { 
            neighbour_checker(&current, &(current.0 + 1, current.1), goal, &mut open_set, &mut f_score, &mut g_score, map);        
        }
        if current.1 > 0 { 
            neighbour_checker(&current, &(current.0, current.1 - 1), goal, &mut open_set, &mut f_score, &mut g_score, map);        
        }
        if current.1 < map[0].len() - 1 { 
            neighbour_checker(&current, &(current.0, current.1 + 1), goal, &mut open_set, &mut f_score, &mut g_score, map);        
        }
    }
    panic!();
}

fn augment(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let y_len = map.len();
    let x_len = map[0].len();
    let mut augmented_map = vec![vec![0; 5*x_len]; 5*y_len];
    for i in 0..(5*y_len) {
        for j in 0..(5*x_len) {
            augmented_map[i][j] = 1 + (map[i % y_len][j % x_len] + (i/y_len + j/x_len) as u32 - 1) % 9;
        }
    }
    augmented_map
}

pub fn run() {
    let map: Vec<Vec<u32>> = utils::read_file_to_lines("data/year2021/day15")
        .iter()
        .map(|s| s.chars()
             .map(|c| c.to_digit(10).unwrap())
             .collect())
        .collect();
    println!("{}", a_star(&(0,0), &(map.len() - 1, map[0].len() - 1), &map));
        let augmented_map = augment(&map);
        let score = a_star(&(0,0), &(augmented_map.len()-1, augmented_map[0].len()-1), &augmented_map);
        println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day15() {
        let map: Vec<Vec<u32>> = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ].iter()
            .map(|s| s.chars()
                 .map(|c| c.to_digit(10).unwrap())
                 .collect())
            .collect();
        let score = a_star(&(0,0), &(map.len()-1, map[0].len()-1), &map);
        assert_eq!(score, 40);
        let augmented_map = augment(&map);
        for row in &augmented_map {
            for v in row {
                print!("{}", v);
            }
            println!("");
        }

        let score = a_star(&(0,0), &(augmented_map.len()-1, augmented_map[0].len()-1), &augmented_map);
        assert_eq!(score, 315);

    }
}
