use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    get_lights_count(false).to_string()
}

fn answer_two() -> String {
    get_lights_count(true).to_string()
}

fn get_lights_count(use_brightness: bool) -> u32 {
    let input = input();
    let lines = input.lines();
    let re = Regex::new(r"^(turn on|turn off|toggle) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)$").unwrap();

    let mut lights = HashMap::new();
    for i in 0..1000 {
        for j in 0..1000 {
            lights.insert((i, j), 0);
        }
    }

    for line in lines {
        let captures = re.captures(line).unwrap();
        let instruction = captures.get(1).unwrap();
        let start_x = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let start_y = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let end_x = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let end_y = captures.get(5).unwrap().as_str().parse::<u32>().unwrap();

        for x in start_x..end_x + 1 {
            for y in start_y..end_y + 1 {
                let light = lights.get_mut(&(x, y)).unwrap();
                if use_brightness {
                    match instruction.as_str() {
                        "turn on" => *light += 1,
                        "turn off" => *light = if *light == 0 { 0 } else { *light - 1 },
                        "toggle" => *light += 2,
                        _ => unreachable!()
                    }
                } else {
                    match instruction.as_str() {
                        "turn on" => *light = 1,
                        "turn off" => *light = 0,
                        "toggle" => *light = 1 - *light,
                        _ => unreachable!()
                    }
                }
            }
        }
    }
    lights.iter().map(|(_, &v)| v).sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_6").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
