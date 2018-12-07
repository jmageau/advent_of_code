use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let chars = input.chars();
    let floor = chars.fold(0, |acc, c| acc + if c == '(' { 1 } else { -1 });
    floor.to_string()
}

fn answer_two() -> String {
    let input = input();
    let chars = input.chars();
    let mut floor = 0;
    for (i, char) in chars.enumerate() {
        if floor == -1 {
            return i.to_string();
        }
        floor += if char == '(' { 1 } else { -1 };
    }
    unreachable!()
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_1").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
