use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let mut screen = vec![vec![false; 6]; 50];
    input.lines().for_each(|l| apply_operation(&mut screen, l));
    screen
        .iter()
        .map(|c| c.iter().filter(|&&p| p).count())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let input = input();
    let mut screen = vec![vec![false; 6]; 50];
    input.lines().for_each(|l| apply_operation(&mut screen, l));
    format_screen(&screen)
}

fn apply_operation(screen: &mut Vec<Vec<bool>>, operation: &str) {
    let rect_regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let row_regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    let col_regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();

    if let Some(captures) = rect_regex.captures(operation) {
        let width = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        for i in 0..width {
            for j in 0..height {
                screen[i][j] = true;
            }
        }
    } else if let Some(captures) = row_regex.captures(operation) {
        let row = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let amount = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let old_row: Vec<_> = screen.iter().map(|c| c[row]).collect();
        for i in 0..screen.len() {
            screen[i][row] = old_row[(old_row.len() + i - amount) % old_row.len()];
        }
    } else if let Some(captures) = col_regex.captures(operation) {
        let col = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let amount = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let old_col = screen[col].clone();
        for i in 0..screen[col].len() {
            screen[col][i] = old_col[(old_col.len() + i - amount) % old_col.len()];
        }
    }
}

fn format_screen(screen: &Vec<Vec<bool>>) -> String {
    let mut result = String::new();
    for j in 0..screen[0].len() {
        result.push('\n');
        for i in 0..screen.len() {
            let c = if screen[i][j] { '#' } else { ' ' };
            result.push(c);
        }
    }
    result
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_8").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
