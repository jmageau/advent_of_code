use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    traverse_grid(&input).0
}

fn answer_two() -> String {
    let input = input();
    traverse_grid(&input).1.to_string()
}

fn traverse_grid(input: &str) -> (String, usize) {
    let grid = get_grid(&input);

    let mut position = starting_position(&grid);
    let mut direction = Direction::Down;
    let mut letters = vec![];
    let mut count = 0;

    loop {
        position = position + direction;
        count += 1;
        let next_char = grid.get(position.0).and_then(|r| r.get(position.1));

        if next_char.is_none() {
            break;
        }

        match next_char.unwrap() {
            ' ' => break,
            '|' | '-' => {}
            '+' => {
                if direction == Direction::Up || direction == Direction::Down {
                    if grid
                        .get(position.0)
                        .and_then(|r| r.get(position.1 + 1))
                        .map(|&c| c != ' ')
                        .unwrap_or(false)
                    {
                        direction = Direction::Right;
                    } else {
                        direction = Direction::Left;
                    }
                } else if grid
                    .get(position.0 + 1)
                    .and_then(|r| r.get(position.1))
                    .map(|&c| c != ' ')
                    .unwrap_or(false)
                {
                    direction = Direction::Down;
                } else {
                    direction = Direction::Up;
                }
            }
            &c => letters.push(c),
        }
    }

    (letters.iter().collect(), count)
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn starting_position(grid: &[Vec<char>]) -> Position {
    Position(0, grid[0].iter().position(|&c| c == '|').unwrap())
}

struct Position(usize, usize);

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, direction: Direction) -> Self {
        #![allow(clippy::suspicious_arithmetic_impl)]
        match direction {
            Direction::Up => Position(self.0 - 1, self.1),
            Direction::Down => Position(self.0 + 1, self.1),
            Direction::Left => Position(self.0, self.1 - 1),
            Direction::Right => Position(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_19").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.to_owned()
}
