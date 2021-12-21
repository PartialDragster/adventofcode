use crate::utils::utils;
use regex::Regex;

fn inv_triangle(t: i32) -> i32 {
    (2f64*t as f64).sqrt().floor() as i32
}

fn largest_step_from_origin(lower: i32, upper: i32) -> i32 {
    if upper < 0 {
        (-lower) as i32
    } else if lower > 0 {
        (upper - lower) as i32
    } else {
        upper.max(-lower) as i32
    }
}

fn f_y(velocity: i32, step: i32) -> i32 {
    step*velocity - (step*step - step) / 2
}

fn parse_area(area: &str) -> ((i32, i32), (i32, i32)) {
    let re = Regex::new(r"^target area: x=([-]?\d+)\.\.([-]?\d+), y=([-]?\d+)\.\.([-]?\d+)$").unwrap();
    let caps = re.captures(area).unwrap();
    let x_min = caps.get(1).unwrap().as_str().parse().unwrap();
    let x_max = caps.get(2).unwrap().as_str().parse().unwrap();
    let y_min = caps.get(3).unwrap().as_str().parse().unwrap();
    let y_max = caps.get(4).unwrap().as_str().parse().unwrap();
    ( (x_min, x_max), (y_min, y_max) )
}

fn ends_inside_area(vx: i32, vy: i32, ( (x_min, x_max), (y_min, y_max) ): ( (i32, i32), (i32, i32) )) -> bool {
    let mut vx = vx;
    let mut vy = vy;

    let mut x = 0;
    let mut y = 0;
    loop {
        x += vx;
        y += vy;
        if vx > 0 { vx -= 1; }
        vy -= 1;
        if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
            return true;
        }
        if x > x_max { return false; }
        if y < y_min { return false; }
    }
}

// only works if x_min > 0 and y_max < 0
fn initial_velocity_count_that_hits(area: &str) -> i32 {
    let ( (x_min, x_max), (y_min, y_max) ) = parse_area(area);
    let min_vx = inv_triangle(x_min as i32);
    let max_vx = x_max;
    let min_vy = y_min;
    let max_vy = -y_min;
    
    let mut count = 0;
    for vx in min_vx..=max_vx {
        for vy in min_vy..=max_vy {
            if ends_inside_area(vx, vy, ( (x_min, x_max), (y_min, y_max) )) {
                count += 1;
            }
        }
    }
    count
}

fn calculate_highest_point(area: &str) -> i32 {
    let ( (_, _), (y_min, y_max) ) = parse_area(area);

    let vel = largest_step_from_origin(y_min, y_max)-1;
    f_y(vel, vel)

}

pub fn run() {
    let target_area = utils::read_file_to_string("data/year2021/day17").trim_end().to_string();
    println!("{}", calculate_highest_point(&target_area));
    println!("{}", initial_velocity_count_that_hits(&target_area));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day17() {
        let target_area = "target area: x=20..30, y=-10..-5";
        assert_eq!(calculate_highest_point(target_area), 45);
        assert_eq!(initial_velocity_count_that_hits(target_area), 112);
    }
}
