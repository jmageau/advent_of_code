use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let mut instructions: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
    let mut current_index = 0i32;
    let mut steps = 0;

    while current_index >= 0 && current_index < instructions.len() as i32 {
        instructions[current_index as usize] += 1;
        current_index += instructions[current_index as usize] - 1;
        steps += 1;
    }

    steps.to_string()
}

fn answer_two() -> String {
    let input = input();
    let mut instructions: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
    let mut current_index = 0i32;
    let mut steps = 0;

    while current_index >= 0 && current_index < instructions.len() as i32 {
        let change = if instructions[current_index as usize] >= 3 {
            -1
        } else {
            1
        };
        instructions[current_index as usize] += change;
        current_index += instructions[current_index as usize] - change;
        steps += 1;
    }

    steps.to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_5").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
