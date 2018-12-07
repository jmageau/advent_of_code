use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let lines = input.lines();
    let mut directions = HashMap::new();
    directions.insert('U', (0, 1));
    directions.insert('D', (0, -1));
    directions.insert('L', (-1, 0));
    directions.insert('R', (1, 0));

    let mut position = (0i8, 0i8);
    let mut buttons = vec![];
    for line in lines {
        let chars = line.chars();
        for char in chars {
            let direction = directions[&char];
            position.0 += direction.0;
            position.1 += direction.1;
            if position.0.abs() > 1 || position.1.abs() > 1 {
                position.0 -= direction.0;
                position.1 -= direction.1;
            }
        }
        let button = 5 + position.0 - 3 * position.1;
        buttons.push(button.to_string());
    }
    buttons.join("")
}

fn answer_two() -> String {
    let input = input();
    let lines = input.lines();
    let mut directions = HashMap::new();
    directions.insert('U', -5);
    directions.insert('D', 5);
    directions.insert('L', -1);
    directions.insert('R', 1);
    let grid = "  1   234 56789 ABC   D  ";

    let mut position = 10i8;
    let mut buttons = vec![];
    for line in lines {
        let chars = line.chars();
        for char in chars {
            let offset = directions[&char];
            position += offset;
            if position < 0
                || position > grid.len() as i8
                || grid.chars().nth(position as usize).unwrap() == ' '
            {
                position -= offset;
            }
        }
        let button = grid.chars().nth(position as usize).unwrap();
        buttons.push(button.to_string());
    }
    buttons.join("")
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_2").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
