use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let digits: Vec<_> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    sum(&digits, 1).to_string()
}

fn answer_two() -> String {
    let input = input();
    let digits: Vec<_> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    sum(&digits, input.len() / 2).to_string()
}

fn sum(digits: &[u32], offset: usize) -> u32 {
    digits
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d == digits[(i + offset) % digits.len()])
        .map(|(_, d)| d)
        .sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_1").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
