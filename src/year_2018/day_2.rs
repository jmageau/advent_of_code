use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    let (two_count, three_count) = input.lines().fold((0, 0), |(acc_2, acc_3), l| {
        let has_2 = has_char_with_count(l, 2);
        let has_3 = has_char_with_count(l, 3);

        (
            acc_2 + if has_2 { 1 } else { 0 },
            acc_3 + if has_3 { 1 } else { 0 },
        )
    });

    (two_count * three_count).to_string()
}

fn has_char_with_count(string: &str, count: usize) -> bool {
    let chars: Vec<_> = string.chars().collect();

    for c in chars.iter() {
        if chars.iter().filter(|&c2| c2 == c).count() == count {
            return true;
        }
    }

    false
}

fn answer_two() -> String {
    let input = input();

    let ids: Vec<_> = input.lines().collect();

    for id in ids.iter() {
        if let Some(other_id) = ids
            .iter()
            .find(|&other_id| other_id != id && string_equal_but_one_char(id, other_id))
        {
            return id
                .chars()
                .zip(other_id.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect();
        }
    }

    unreachable!()
}

fn string_equal_but_one_char(first: &str, second: &str) -> bool {
    first
        .chars()
        .zip(second.chars())
        .filter(|(a, b)| a != b)
        .count()
        == 1
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_2").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
