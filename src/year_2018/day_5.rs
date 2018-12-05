use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    reduced_length(&input).to_string()
}

fn answer_two() -> String {
    let input = input();

    (65u8..=90)
        .map(|c| c as char)
        .map(|c| {
            reduced_length(
                &input
                    .chars()
                    .filter(|c2| !c.eq_ignore_ascii_case(c2))
                    .collect::<String>(),
            )
        }).min()
        .unwrap()
        .to_string()
}

fn reduced_length(input: &str) -> usize {
    let mut chars: Vec<_> = input.chars().collect();

    loop {
        let index = chars
            .windows(2)
            .enumerate()
            .find(|(_, w)| w[0] != w[1] && w[0].eq_ignore_ascii_case(&w[1]))
            .map(|(i, _)| i);
        if let Some(i) = index {
            chars.remove(i);
            chars.remove(i);
        } else {
            break;
        }
    }

    chars.len()
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_5").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
