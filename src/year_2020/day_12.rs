use std::ops::Add;
use std::str::FromStr;

aoc_day!(2020, 12);

fn answer_one() -> String {
    let instructions: Vec<Instruction> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut ship_position = Point::new(0, 0);
    let mut ship_direction = 0;
    for instruction in instructions {
        run_instruction1(instruction, &mut ship_position, &mut ship_direction);
    }

    ship_position.manhattan_distance(Point::ZERO).to_string()
}

fn answer_two() -> String {
    let instructions: Vec<Instruction> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut ship_position = Point::new(0, 0);
    let mut waypoint_position = Point::new(10, -1);
    for instruction in instructions {
        run_instruction2(instruction, &mut ship_position, &mut waypoint_position);
    }

    ship_position.manhattan_distance(Point::ZERO).to_string()
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

#[derive(Debug)]
enum Instruction {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let value = value.parse::<isize>().unwrap();
        match action {
            "N" => Ok(Instruction::N(value)),
            "S" => Ok(Instruction::S(value)),
            "E" => Ok(Instruction::E(value)),
            "W" => Ok(Instruction::W(value)),
            "L" => Ok(Instruction::L(value)),
            "R" => Ok(Instruction::R(value)),
            "F" => Ok(Instruction::F(value)),
            _ => unreachable!(),
        }
    }
}

fn normalize_angle(angle: isize) -> isize {
    if angle >= 0 {
        angle % 360
    } else {
        (360 - (-angle % 360)) % 360
    }
}

fn run_instruction1(
    instruction: Instruction,
    ship_position: &mut Point,
    ship_direction: &mut isize,
) {
    match instruction {
        Instruction::N(value) => ship_position.y -= value,
        Instruction::S(value) => ship_position.y += value,
        Instruction::E(value) => ship_position.x += value,
        Instruction::W(value) => ship_position.x -= value,
        Instruction::L(value) => *ship_direction -= value,
        Instruction::R(value) => *ship_direction += value,
        Instruction::F(value) => {
            *ship_direction = normalize_angle(*ship_direction);
            match ship_direction {
                0 => ship_position.x += value,
                90 => ship_position.y += value,
                180 => ship_position.x -= value,
                270 => ship_position.y -= value,
                _ => unreachable!(),
            }
        }
    }
}

fn run_instruction2(
    instruction: Instruction,
    ship_position: &mut Point,
    waypoint_position: &mut Point,
) {
    match instruction {
        Instruction::N(value) => waypoint_position.y -= value,
        Instruction::S(value) => waypoint_position.y += value,
        Instruction::E(value) => waypoint_position.x += value,
        Instruction::W(value) => waypoint_position.x -= value,
        Instruction::L(value) => *waypoint_position = rotate(*waypoint_position, -value),
        Instruction::R(value) => *waypoint_position = rotate(*waypoint_position, value),
        Instruction::F(value) => {
            ship_position.x += waypoint_position.x * value;
            ship_position.y += waypoint_position.y * value;
        }
    }
}

fn rotate(mut point: Point, angle: isize) -> Point {
    let angle = normalize_angle(angle);
    let times = angle / 90;
    for _ in 0..times {
        point = Point::new(-point.y, point.x);
    }
    point
}
