use std::io::prelude::*;
use std::fs::File;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let lines = input.lines();

    let mut valid_triangles = 0;
    for line in lines {
        let dimensions: Vec<_> = line.split(" ").map(|s| s.parse::<u16>().unwrap()).collect();
        if is_valid_triangle(&dimensions) {
            valid_triangles += 1;
        }
    }
    valid_triangles.to_string()
}

fn answer_two() -> String {
    let input = input();
    let lines: Vec<_> = input.lines().collect();

    let mut valid_triangles = 0;
    for i in 0..(lines.len()) / 3 {
        let numbers_1: Vec<_> = lines[i * 3].split(" ").map(|s| s.parse::<u16>().unwrap()).collect();
        let numbers_2: Vec<_> = lines[i * 3 + 1].split(" ").map(|s| s.parse::<u16>().unwrap()).collect();
        let numbers_3: Vec<_> = lines[i * 3 + 2].split(" ").map(|s| s.parse::<u16>().unwrap()).collect();

        let dimensions = vec![
            vec![numbers_1[0], numbers_2[0], numbers_3[0]],
            vec![numbers_1[1], numbers_2[1], numbers_3[1]],
            vec![numbers_1[2], numbers_2[2], numbers_3[2]]];

        valid_triangles += dimensions.iter().filter(|d| is_valid_triangle(d)).count();
    }
    valid_triangles.to_string()
}

fn is_valid_triangle(dimensions: &Vec<u16>) -> bool {
    let sum: u16 = dimensions.iter().sum();
    let max: u16 = dimensions.iter().max().unwrap().clone();
    sum > max * 2
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_3").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
