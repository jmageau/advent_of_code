use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    collisions(&input())
        .iter()
        .map(|(p, _)| p.manhattan_distance(Point::ZERO))
        .min()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    collisions(&input()).values().min().unwrap().to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_3").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };

    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: Point) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

fn collisions(input: &str) -> HashMap<Point, usize> {
    let paths: Vec<_> = input
        .lines()
        .map(|l| l.split(',').collect::<Vec<_>>())
        .collect();

    let mut first_wire = HashMap::new();
    let mut second_wire = HashMap::new();

    let add_points = |path: &[&str], wire: &mut HashMap<_, _>| {
        let mut current_position = Point::new(0, 0);
        let mut steps = 0;

        for direction in path {
            let delta = match direction.chars().next().unwrap() {
                'U' => Point::new(0, -1),
                'D' => Point::new(0, 1),
                'L' => Point::new(-1, 0),
                'R' => Point::new(1, 0),
                _ => unreachable!(),
            };
            let distance: i32 = direction
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();

            for _ in 0..distance {
                current_position = current_position + delta;
                steps += 1;

                wire.entry(current_position).or_insert(steps);
            }
        }
    };

    add_points(&paths[0], &mut first_wire);
    add_points(&paths[1], &mut second_wire);

    let mut collisions = HashMap::new();

    for (p, d) in first_wire {
        if let Some(&d2) = second_wire.get(&p) {
            collisions.insert(p, d + d2);
        }
    }

    collisions
}
