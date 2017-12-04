extern crate itertools;

use std::io::prelude::*;
use std::fs::File;
use self::itertools::Itertools;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let passphrases = input.lines();
    passphrases.filter(|p| is_valid(p)).count().to_string()
}

fn answer_two() -> String {
    let input = input();
    let passphrases = input.lines();
    passphrases.filter(|p| is_valid2(p)).count().to_string()
}

fn is_valid(passphrase: &str) -> bool {
    let words: Vec<_> = passphrase.split(" ").collect();
    words.len() == words.iter().unique().count()
}

fn is_valid2(passphrase: &str) -> bool {
    let words: Vec<_> = passphrase.split(" ").collect();
    words.len()
        == words
            .iter()
            .unique_by(|w| {
                let mut sorted_word: Vec<_> = w.chars().collect();
                sorted_word.sort();
                sorted_word
            })
            .count()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_4").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
