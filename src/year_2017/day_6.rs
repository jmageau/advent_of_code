use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let mut banks: Vec<u32> = input.split("\t").map(|n| n.parse().unwrap()).collect();
    let mut seen_banks = HashSet::new();
    let mut cycles = 0;

    while seen_banks.insert(banks.clone()) {
        let mut i = banks
            .iter()
            .enumerate()
            .rev()  // If, multiple maxes, last is returned
            .max_by_key(|&(_, bank)| bank)
            .unwrap()
            .0;
        let mut blocks = banks[i];
        banks[i] = 0;
        while blocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            blocks -= 1;
        }
        cycles += 1;
    }

    cycles.to_string()
}

fn answer_two() -> String {
    let input = input();
    let mut banks: Vec<u32> = input.split("\t").map(|n| n.parse().unwrap()).collect();
    let mut seen_banks = HashSet::new();
    let mut seen_banks_ordered = vec![];

    while seen_banks.insert(banks.clone()) {
        seen_banks_ordered.push(banks.clone());
        let mut i = banks
            .iter()
            .enumerate()
            .rev()  // If, multiple maxes, last is returned
            .max_by_key(|&(_, bank)| bank)
            .unwrap()
            .0;
        let mut blocks = banks[i];
        banks[i] = 0;
        while blocks > 0 {
            i = (i + 1) % banks.len();
            banks[i] += 1;
            blocks -= 1;
        }
    }
    let length =
        seen_banks_ordered.len() - seen_banks_ordered.iter().position(|b| b == &banks).unwrap();

    length.to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_6").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
