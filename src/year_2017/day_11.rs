use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    execute_steps(&input).0.to_string()
}

fn answer_two() -> String {
    let input = input();
    execute_steps(&input).1.to_string()
}

fn execute_steps(input: &str) -> (usize, usize) {
    let path: Vec<_> = input.split(",").collect();

    let mut position = Position(0, 0, 0);
    let mut max_distance = 0;

    for step in path {
        let m = match step {
            "n" => Position(0, 1, -1),
            "ne" => Position(1, 0, -1),
            "se" => Position(1, -1, 0),
            "s" => Position(0, -1, 1),
            "sw" => Position(-1, 0, 1),
            "nw" => Position(-1, 1, 0),
            _ => unreachable!(),
        };

        position = position + m;
        max_distance = max(max_distance, position.distance_to_origin())
    }

    (position.distance_to_origin(), max_distance)
}

struct Position(isize, isize, isize);

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Position {
    fn distance_to_origin(&self) -> usize {
        max(max(self.0.abs(), self.1.abs()), self.2.abs()) as usize
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_11").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
