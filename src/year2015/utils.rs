use std::fs;

pub fn read_file_to_string(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect(&format!("Something went wrong reading file {}", filename))
}
