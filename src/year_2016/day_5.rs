extern crate crypto;

use std::io::prelude::*;
use std::fs::File;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    get_password(false)
}

fn answer_two() -> String {
    get_password(true)
}

fn get_password(use_position: bool) -> String {
    let input = input();
    let mut hash = Md5::new();

    let mut password_chars = vec![];
    for i in 0.. {
        let s = input.clone() + &i.to_string();
        hash.input_str(&s);
        let output = hash.result_str();
        hash.reset();
        let leading_zeroes_count = output.chars().take(5).filter(|&c| c == '0').count();
        if leading_zeroes_count == 5 {
            if use_position {
                let position = output.chars().nth(5).unwrap().to_digit(16).unwrap();
                let char = output.chars().nth(6).unwrap();
                if position < 8 && !password_chars.iter().any(|&(_, p)| p == position) {
                    password_chars.push((char, position));
                }
            } else {
                let char = output.chars().nth(5).unwrap();
                password_chars.push((char, i));
            }
            if password_chars.len() == 8 {
                break;
            }
        }
    }
    password_chars.sort_by_key(|&(_, p)| p);
    password_chars.iter().map(|&(c, _)| c).collect()
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_5").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
