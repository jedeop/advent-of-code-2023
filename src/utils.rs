use std::fs;

pub fn read_input(day: usize) -> String {
    fs::read_to_string(format!("./input/day{}.txt", day)).expect("input file doesn't exist")
}
