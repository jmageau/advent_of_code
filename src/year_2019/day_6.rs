use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let orbits = get_orbits(&input());
    orbits
        .keys()
        .map(|o| orbits.get(o).map(|_| 1).unwrap_or(0) + indirect_orbits(o, &orbits).len())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let orbits = get_orbits(&input());
    let first = indirect_orbits("YOU", &orbits);
    let second = indirect_orbits("SAN", &orbits);

    let first_common = first.iter().find(|f| second.contains(f)).unwrap();

    (first.iter().position(|f| f == first_common).unwrap()
        + second.iter().position(|f| f == first_common).unwrap()
        + 2)
    .to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_6").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}

fn get_orbits(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(')').collect();
            (parts[1].to_owned(), parts[0].to_owned())
        })
        .collect()
}

fn indirect_orbits(object: &str, orbits: &HashMap<String, String>) -> Vec<String> {
    let mut result = vec![];
    let mut current = &orbits[object];
    while let Some(next) = orbits.get(current) {
        result.push(next.clone());
        current = &next;
    }
    result
}
