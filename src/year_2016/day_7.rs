use std::io::prelude::*;
use std::fs::File;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    input
        .lines()
        .filter(|a| supports_tls(a))
        .count()
        .to_string()
}

fn answer_two() -> String {
    let input = input();
    input
        .lines()
        .filter(|a| supports_ssl(a))
        .count()
        .to_string()
}

fn supports_tls(address: &str) -> bool {
    let chars: Vec<_> = address.chars().collect();
    let mut in_brackets = false;
    let mut is_valid = false;
    for (i, &c) in chars.iter().enumerate() {
        if i + 3 >= chars.len() {
            break;
        }
        if c == '[' {
            in_brackets = true;
        } else if c == ']' {
            in_brackets = false;
        } else if chars[i] != chars[i + 1] && chars[i] == chars[i + 3]
            && chars[i + 1] == chars[i + 2]
        {
            if in_brackets {
                return false;
            }
            is_valid = true;
        }
    }
    is_valid
}

fn supports_ssl(address: &str) -> bool {
    let chars: Vec<_> = address.chars().collect();
    let mut in_brackets = false;
    let mut in_brackets_words = vec![];
    let mut out_brackets_words = vec![];

    for (i, &c) in chars.iter().enumerate() {
        if i + 2 >= chars.len() {
            break;
        }
        if c == '[' {
            in_brackets = true;
        } else if c == ']' {
            in_brackets = false;
        } else {
            let word = vec![chars[i], chars[i + 1], chars[i + 2]];
            if word[0] == word[2] {
                if in_brackets {
                    in_brackets_words.push(word);
                } else {
                    out_brackets_words.push(word);
                }
            }
        }
    }

    in_brackets_words
        .iter()
        .any(|w| out_brackets_words.contains(&vec![w[1], w[0], w[1]]))
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_7").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
