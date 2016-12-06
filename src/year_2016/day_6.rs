use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    decrypt_message(true)
}

fn answer_two() -> String {
    decrypt_message(false)
}

fn decrypt_message(use_max: bool) -> String {
    let input = input();
    let lines = input.lines();
    let mut position_letters = HashMap::new();

    for line in lines {
        let chars = line.chars();
        for (i, char) in chars.enumerate() {
            let mut position_letter = position_letters.entry(i).or_insert(HashMap::new());
            let mut char_count = (*position_letter).entry(char).or_insert(0);
            *char_count += 1;
        }
    }
    let mut position_letters_vec: Vec<_> = position_letters.iter().collect();
    position_letters_vec.sort_by_key(|&(&i, _)| i);

    if use_max {
        position_letters_vec.iter()
            .map(|&(_, ref char_counts)| *char_counts.iter()
                .max_by_key(|&(_, c)| c).unwrap().0)
            .collect()
    } else {
        position_letters_vec.iter()
            .map(|&(_, ref char_counts)| *char_counts.iter()
                .min_by_key(|&(_, c)| c).unwrap().0)
            .collect()
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_6").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
