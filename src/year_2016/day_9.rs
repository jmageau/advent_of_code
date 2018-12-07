use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    decompress(&input).len().to_string()
}

fn answer_two() -> String {
    let input = input();
    Sequence::new(&input, 1).length().to_string()
}

fn decompress(input: &str) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mut output = vec![];
    let mut i = 0;

    let marker_regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
    while i < input.len() {
        if chars[i] != '(' {
            output.push(chars[i]);
            i += 1;
        } else {
            let substring: String = chars.iter().skip(i).collect();
            let captures = marker_regex.captures(&substring).unwrap();
            let length = captures.get(0).unwrap().as_str().len();
            let character_count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let repeat_count = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

            let string_to_repeat: Vec<_> = chars
                .iter()
                .skip(i + length)
                .take(character_count)
                .cloned()
                .collect();

            (0..repeat_count).for_each(|_| string_to_repeat.iter().for_each(|&c| output.push(c)));

            i += length + character_count;
        }
    }

    output.iter().collect()
}

#[derive(Debug)]
struct Sequence {
    chars: Vec<char>,
    repeats: usize,
}

impl Sequence {
    fn new(string: &str, repeats: usize) -> Sequence {
        Sequence {
            chars: string.chars().collect(),
            repeats,
        }
    }

    fn length(&self) -> usize {
        let mut result = 0;
        let mut i = 0;

        while i < self.chars.len() {
            if self.chars[i] != '(' {
                result += 1;
                i += 1;
            } else {
                let marker_regex = Regex::new(r"^\((\d+)x(\d+)\)").unwrap();
                let substring: String = self.chars.iter().skip(i).collect();
                let captures = marker_regex.captures(&substring).unwrap();
                let length = captures.get(0).unwrap().as_str().len();
                let character_count = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let repeat_count = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

                let s: String = substring
                    .chars()
                    .skip(length)
                    .take(character_count)
                    .collect();

                result += Sequence::new(&s, repeat_count).length();
                i += length + character_count;
            }
        }

        result * self.repeats
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_9").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
