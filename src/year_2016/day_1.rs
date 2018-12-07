use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    distance(false)
}

fn answer_two() -> String {
    distance(true)
}

fn distance(stop_at_visited: bool) -> String {
    let input = input();
    let instructions = input.split(", ").collect::<Vec<_>>();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut direction = 0;
    let mut position = (0i32, 0i32);
    let mut visited_positions = vec![position];

    for instruction in instructions {
        let mut chars = instruction.chars();
        let turn = chars.next().unwrap();
        let distance = chars.as_str().parse::<i32>().unwrap();

        if turn == 'R' {
            direction = (direction + 1) % 4;
        } else {
            direction = (direction + 3) % 4;
        }

        for _ in 0..distance {
            position.0 += directions[direction].0;
            position.1 += directions[direction].1;

            if stop_at_visited && visited_positions.contains(&position) {
                return (position.0.abs() + position.1.abs()).to_string();
            } else {
                visited_positions.push(position);
            }
        }
    }

    (position.0.abs() + position.1.abs()).to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_1").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
