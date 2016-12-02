use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let chars = input.chars();
    let mut directions = HashMap::new();
    directions.insert('^', (0,1));
    directions.insert('v', (0,-1));
    directions.insert('>', (1,0));
    directions.insert('<', (-1,0));

    let mut position = (0, 0);
    let mut visited_houses = vec![position];
    for char in chars {
        let direction = directions[&char];
        position.0 += direction.0;
        position.1 += direction.1;
        if !visited_houses.contains(&position) {
            visited_houses.push(position);
        }
    }
    visited_houses.len().to_string()
}

fn answer_two() -> String {
    let input = input();
    let chars = input.chars();
    let mut directions = HashMap::new();
    directions.insert('^', (0,1));
    directions.insert('v', (0,-1));
    directions.insert('>', (1,0));
    directions.insert('<', (-1,0));

    let mut position = (0, 0);
    let mut position2 = (0, 0);
    let mut first_moving = true;
    let mut visited_houses = vec![position];
    for char in chars {
        let direction = directions[&char];
        if first_moving {
            position.0 += direction.0;
            position.1 += direction.1;
            if !visited_houses.contains(&position) {
                visited_houses.push(position);
            }
        } else {
            position2.0 += direction.0;
            position2.1 += direction.1;
            if !visited_houses.contains(&position2) {
                visited_houses.push(position2);
            }
        }
        first_moving = !first_moving;
    }
    visited_houses.len().to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_3").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
