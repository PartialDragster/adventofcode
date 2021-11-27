use std::env;

mod year2015;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "year2015_day01" => year2015::day01::run(),
        "year2015_day02" => year2015::day02::run(),
        puzzle => panic!("{} not recognised puzzle", puzzle),
    }
}
