use std::ops::{Add, AddAssign};
use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = 312051;
    let odd_square = (input as f64).sqrt() as i32 - 1;
    let max_coordinate = odd_square / 2 + 1;

    let mut position = Pos::new(odd_square / 2, odd_square / 2);
    let mut direction = Pos::RIGHT;

    for _ in odd_square * odd_square..input {
        position += direction;

        if position.x == max_coordinate {
            direction = Pos::UP;
        }
        if position.y == -max_coordinate {
            direction = Pos::LEFT;
        }
        if position.x == -max_coordinate {
            direction = Pos::DOWN;
        }
        if position.y == max_coordinate {
            direction = Pos::RIGHT;
        }
    }

    position.distance(Pos::new(0, 0)).to_string()
}

fn answer_two() -> String {
    let input = 312051;

    let mut grid = HashMap::new();
    let mut position = Pos::new(0, 0);
    let mut direction = Pos::RIGHT;
    let mut max_coordinate = 1;

    grid.insert(position, 1);

    loop {
        position += direction;

        let neighbours = vec![
            Pos::new(-1, -1),
            Pos::new(0, -1),
            Pos::new(1, -1),
            Pos::new(-1, 0),
            Pos::new(1, 0),
            Pos::new(-1, 1),
            Pos::new(0, 1),
            Pos::new(1, 1),
        ];

        let value: u32 = neighbours
            .iter()
            .filter_map(|&n| grid.get(&(position + n)))
            .sum();

        if value > input {
            return value.to_string();
        }

        grid.insert(position, value);

        if position.x == max_coordinate {
            direction = Pos::UP;
        }
        if position.y == -max_coordinate {
            direction = Pos::LEFT;
        }
        if position.x == -max_coordinate {
            direction = Pos::DOWN;
        }
        if position.y == max_coordinate {
            direction = Pos::RIGHT;
            max_coordinate += 1;
        }
    }
}


#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const UP: Self = Pos { x: 0, y: -1 };
    const DOWN: Self = Pos { x: 0, y: 1 };
    const LEFT: Self = Pos { x: -1, y: 0 };
    const RIGHT: Self = Pos { x: 1, y: 0 };

    fn new(x: i32, y: i32) -> Pos {
        Pos { x: x, y: y }
    }

    fn distance(&self, other: Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Pos) {
        *self = Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
