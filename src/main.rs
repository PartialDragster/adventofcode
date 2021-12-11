use std::env;

mod year2015;
mod year2021;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "year2015_day01" => year2015::day01::run(),
        "year2015_day02" => year2015::day02::run(),
        "year2015_day03" => year2015::day03::run(),
        "year2015_day04" => year2015::day04::run(),
        "year2015_day05" => year2015::day05::run(),
        "year2015_day06" => year2015::day06::run(),
        "year2015_day07" => year2015::day07::run(),
        "year2015_day08" => year2015::day08::run(),
        "year2015_day09" => year2015::day09::run(),
        "year2015_day10" => year2015::day10::run(),
        "year2015_day11" => year2015::day11::run(),
        "year2021_day01" => year2021::day01::run(),
        "year2021_day02" => year2021::day02::run(),
        "year2021_day03" => year2021::day03::run(),
        "year2021_day04" => year2021::day04::run(),
        "year2021_day05" => year2021::day05::run(),
        "year2021_day06" => year2021::day06::run(),
        "year2021_day07" => year2021::day07::run(),
        "year2021_day08" => year2021::day08::run(),
        "year2021_day09" => year2021::day09::run(),
        "year2021_day10" => year2021::day10::run(),
        "year2021_day11" => year2021::day11::run(),
        puzzle => panic!("{} not recognised puzzle", puzzle),
    }
}
