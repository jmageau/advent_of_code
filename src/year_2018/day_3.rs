use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let squares = get_squares(&input);

    squares
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .count()
        .to_string()
}

fn answer_two() -> String {
    let input = input();
    let squares = get_squares(&input);

    let mut good_ids = HashSet::new();
    let mut bad_ids = HashSet::new();

    for (_, ids) in squares {
        if ids.len() > 1 {
            for id in ids {
                bad_ids.insert(id);
                good_ids.remove(&id);
            }
        } else {
            assert!(ids.len() == 1);
            let id = ids[0];
            if !bad_ids.contains(&id) {
                good_ids.insert(id);
            }
        }
    }
    assert!(good_ids.len() == 1);
    good_ids.iter().next().unwrap().to_string()
}

fn get_squares(input: &str) -> HashMap<(usize, usize), Vec<usize>> {
    let mut squares = HashMap::new();
    let line_regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    for line in input.lines() {
        let captures = line_regex.captures(line).unwrap();
        let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let width = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let height = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        for i in x..x + width {
            for j in y..y + height {
                let ids = squares.entry((i, j)).or_insert_with(Vec::new);
                ids.push(id);
            }
        }
    }

    squares
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_3").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
