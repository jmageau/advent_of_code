extern crate itertools;

use std::io::prelude::*;
use std::fs::File;
use self::itertools::Itertools;


pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let numbers = input
        .lines()
        .map(|l| l.split('\t').filter_map(|s| s.parse().ok()).collect())
        .collect();
    checksum1(numbers).to_string()
}

fn answer_two() -> String {
    let input = input();
    let numbers = input
        .lines()
        .map(|l| l.split('\t').filter_map(|s| s.parse().ok()).collect())
        .collect();
    checksum2(numbers).to_string()
}

fn checksum1(numbers: Vec<Vec<u32>>) -> u32 {
    numbers
        .iter()
        .map(|r| r.iter().max().unwrap() - r.iter().min().unwrap())
        .sum()
}

fn checksum2(numbers: Vec<Vec<u32>>) -> u32 {
    numbers
        .iter()
        .map(|r| {
            r.iter()
                .tuple_combinations()
                .filter(|&(a, b)| a % b == 0 || b % a == 0)
                .map(|(a, b)| ::std::cmp::max(a / b, b / a))
                .nth(0)
                .unwrap()
        })
        .sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_2").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
