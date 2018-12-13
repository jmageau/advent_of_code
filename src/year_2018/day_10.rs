use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    "1".to_owned()
}

fn answer_two() -> String {
    let input = input();
    let points = get_points(&input);
    let seconds = run_simulation(points);

    seconds.to_string()
}

#[derive(Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn get_points(input: &str) -> Vec<Point> {
    let line_regex =
        Regex::new(r"^position=<([-\s]?\d+), ([-\s]?\d+)> velocity=<([-\s]?\d+), ([-\s]?\d+)>$")
            .unwrap();

    input
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let numbers: Vec<_> = (1..=4)
                .map(|n| {
                    captures
                        .get(n)
                        .unwrap()
                        .as_str()
                        .trim()
                        .parse::<isize>()
                        .unwrap()
                })
                .collect();
            Point {
                x: numbers[0],
                y: numbers[1],
                vx: numbers[2],
                vy: numbers[3],
            }
        })
        .collect()
}

fn run_simulation(mut points: Vec<Point>) -> usize {
    let mut last_row_count = std::usize::MAX;
    let mut last_points = points.clone();

    for i in 0.. {
        points = points
            .into_iter()
            .map(|p| Point {
                x: p.x + p.vx,
                y: p.y + p.vy,
                vx: p.vx,
                vy: p.vy,
            })
            .collect();

        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        let row_count = (max_y - min_y + 1) as usize;

        if row_count > last_row_count {
            display_points(&last_points);
            return i;
        }

        last_points = points.clone();
        last_row_count = row_count;
    }

    unreachable!()
}

fn display_points(points: &[Point]) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if points.iter().find(|p| p.x == x && p.y == y).is_some() {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_10").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
