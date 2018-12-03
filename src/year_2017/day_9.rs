use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    process_sequence(&input).0.to_string()
}

fn answer_two() -> String {
    let input = input();
    process_sequence(&input).1.to_string()
}

fn process_sequence(input: &str) -> (usize, usize) {
    let mut total_score = 0;
    let mut current_group_score = 1;
    let mut skip_next = false;
    let mut inside_garbage = false;
    let mut garbage_count = 0;

    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if inside_garbage && c == '!' {
            skip_next = true;
            continue;
        }

        if !inside_garbage && c == '<' {
            inside_garbage = true;
            continue;
        }

        if inside_garbage && c == '>' {
            inside_garbage = false;
            continue;
        }

        if !inside_garbage && c == '{' {
            total_score += current_group_score;
            current_group_score += 1;
            continue;
        }

        if !inside_garbage && c == '}' {
            current_group_score -= 1;
            continue;
        }

        if inside_garbage {
            garbage_count += 1;
        }
    }

    (total_score, garbage_count)
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_9").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
