use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    execute_instructions(&input).0.to_string()
}

fn answer_two() -> String {
    let input = input();
    execute_instructions(&input).1.to_string()
}

fn execute_instructions(input: &str) -> (i32, i32) {
    let mut overall_max = 0;

    let mut registers = HashMap::new();
    let register_regex =
        Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) (==|!=|<=|>=|<|>) (-?\d+)$").unwrap();

    for line in input.lines() {
        let captures = register_regex.captures(line).unwrap();
        let register = captures.get(1).unwrap().as_str();
        let operation = captures.get(2).unwrap().as_str();
        let amount = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let condition_register = captures.get(4).unwrap().as_str();
        let condition_operator = captures.get(5).unwrap().as_str();
        let condition_value = captures.get(6).unwrap().as_str().parse::<i32>().unwrap();

        let condition_result = {
            let condition_register_value = registers.get(condition_register).unwrap_or(&0);
            match condition_operator {
                "==" => *condition_register_value == condition_value,
                "!=" => *condition_register_value != condition_value,
                "<=" => *condition_register_value <= condition_value,
                ">=" => *condition_register_value >= condition_value,
                "<" => *condition_register_value < condition_value,
                ">" => *condition_register_value > condition_value,
                _ => unreachable!(),
            }
        };

        if condition_result {
            let r = registers.entry(register).or_insert(0);
            *r += if operation == "inc" { amount } else { -amount };
            if *r > overall_max {
                overall_max = *r;
            }
        }
    }

    let max = registers.values().max().unwrap();
    (*max, overall_max)
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_8").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
