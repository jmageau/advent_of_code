extern crate crypto;

use std::io::prelude::*;
use std::fs::File;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    get_suffix(5)
}

fn answer_two() -> String {
    get_suffix(6)
}

fn get_suffix(target_leading_zeroes_count: usize) -> String {
    let input = input();
    let mut hash = Md5::new();

    for i in 0.. {
        let s = input.clone() + &i.to_string();
        hash.input_str(&s);
        let output = hash.result_str();
        let leading_zeroes_count = output.chars().take_while(|&c| c == '0').count();
        if leading_zeroes_count == target_leading_zeroes_count {
            return i.to_string();
        }
        hash.reset();
    }
    unreachable!()
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_4").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
