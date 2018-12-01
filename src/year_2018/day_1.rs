use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

fn answer_two() -> String {
    let input = input();

    let changes: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut seen_frequencies = HashSet::new();
    let mut frequency = 0;

    for change in changes.iter().cycle() {
        frequency += change;
        if !seen_frequencies.insert(frequency) {
            break;
        }
    }

    frequency.to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_1").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
