use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let lines = input.lines();

    lines.filter(|&l| is_nice_1(l)).count().to_string()
}

fn answer_two() -> String {
    let input = input();
    let lines = input.lines();

    lines.filter(|&l| is_nice_2(l)).count().to_string()
}

fn is_nice_1(string: &str) -> bool {
    let vowels: Vec<_> = "aeiou".chars().collect();
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    let chars: Vec<_> = string.chars().collect();
    let mut unique_chars = chars.clone();
    unique_chars.dedup();

    let three_vowels = chars.iter().filter(|c| vowels.contains(c)).count() >= 3;
    let letter_twice_in_a_row = chars.len() != unique_chars.len();
    let contains_bad_string = bad_strings.iter().any(|s| string.contains(s));

    three_vowels && letter_twice_in_a_row && !contains_bad_string
}

fn is_nice_2(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    let mut two_pairs = false;
    for i in 0..s.len() - 1 {
        let parts = s.split_at(i + 2);
        let pair = parts.0.split_at(i).1;
        let tail = parts.1;
        if tail.contains(pair) {
            two_pairs = true;
            break;
        }
    }
    let chars: Vec<_> = s.chars().collect();
    let mut repeating_letter = false;
    for i in 0..s.len() - 2 {
        let c1 = chars[i];
        let c3 = chars[i + 2];
        if c1 == c3 {
            repeating_letter = true;
            break;
        }
    }

    two_pairs && repeating_letter
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_5").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
