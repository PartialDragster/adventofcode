use crate::utils::utils;

fn calc_position(course: &Vec<String>) -> (i32, i32) {
    course.iter()
        .map(|str| {
            let mut split = str.split(" ");
            let command = split.next().unwrap();
            let num = split.next().unwrap().parse::<i32>().unwrap();
            match command as &str {
                "forward" => (num, 0),
                "down" => (0, num),
                "up" => (0, -num),
                _ => panic!()
            }})
    .fold((0, 0), | (old_x, old_y), (del_x, del_y)| (old_x + del_x, old_y + del_y))
}

fn calc_position_by_aim(course: &Vec<String>) -> (i32, i32) {
    let pos_and_aim = course.iter()
        .fold((0, 0, 0), |(x, y, aim), str| {
            let mut split = str.split(" ");
            let command = split.next().unwrap();
            let num = split.next().unwrap().parse::<i32>().unwrap();
            match command as &str {
                "forward" => (x + num, y + aim * num, aim),
                "down" => (x, y, aim + num),
                "up" => (x, y, aim - num),
                _ => panic!()
            }
        });
    (pos_and_aim.0, pos_and_aim.1)
}

pub fn run() {
       let input: Vec<String> = utils::read_file_to_lines("data/year2021/day02");
       let pos = calc_position(&input);
       println!("{}", pos.0 * pos.1);
       let pos = calc_position_by_aim(&input);
       println!("{}", pos.0 * pos.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year2021_day02_test_calc_position() {
        let course = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(calc_position(&course), (15, 10))
    }

    #[test]
    fn year2021_day02_test_calc_position_by_aim() {
        let course = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"]
                .iter()
                .map(|str| str.to_string())
                .collect();
        assert_eq!(calc_position_by_aim(&course), (15, 60))
    }
}
