use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let range = input()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    (range[0]..=range[1])
        .filter(|&n| match_criteria(n))
        .count()
        .to_string()
}

fn answer_two() -> String {
    let range = input()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    (range[0]..=range[1])
        .filter(|&n| match_criteria2(n))
        .count()
        .to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_4").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}

fn match_criteria(n: usize) -> bool {
    let digits: Vec<_> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits.windows(2).any(|w| w[0] == w[1]) && digits.windows(2).all(|w| w[0] <= w[1])
}

fn match_criteria2(n: usize) -> bool {
    let digits: Vec<_> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    (0..digits.len()).any(|i| {
        let current = digits.get(i);
        let next = digits.get(i + 1);
        let previous = i.checked_sub(1).and_then(|p| digits.get(p));
        let after_next = digits.get(i + 2);

        current == next && previous != current && after_next != current
    }) && digits.windows(2).all(|w| w[0] <= w[1])
}
