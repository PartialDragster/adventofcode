use std::fs::{self, File};
use std::io::{self, BufRead};

pub fn read_file_to_string(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect(&format!("Something went wrong reading file {}", filename))
}

pub fn read_file_to_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename)
        .expect(&format!("Something went wrong reading file {}", filename));
    return io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
