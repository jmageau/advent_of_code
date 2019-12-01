use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .map(|m| m / 3 - 2)
        .sum::<u32>()
        .to_string()
}

fn answer_two() -> String {
    let fuel = |mut mass| {
        let mut sum = 0;
        while mass > 6 {
            mass = mass / 3 - 2;
            sum += mass;
        }
        sum
    };

    input()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .map(fuel)
        .sum::<u32>()
        .to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_1").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
