use itertools::Itertools;
use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let seating_values = seating_values(&input());
    let seats: Vec<_> = seating_values.keys().cloned().collect();

    seats
        .iter()
        .permutations(seats.len())
        .map(|s| happiness(&s, &seating_values))
        .max()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let seating_values = seating_values(&input());
    let seats: Vec<_> = seating_values
        .keys()
        .cloned()
        .chain(std::iter::once("Me".to_owned()))
        .collect();

    seats
        .iter()
        .permutations(seats.len())
        .map(|s| happiness(&s, &seating_values))
        .max()
        .unwrap()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_13").unwrap()
}

fn seating_values(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut words = line.split(' ');
        let name = words.nth(0).unwrap().to_owned();
        let gain = words.nth(1).unwrap() == "gain";
        let value = words.nth(0).unwrap().parse::<i32>().unwrap() * if gain { 1 } else { -1 };
        let mut neighbour = words.nth(6).unwrap().to_owned();
        neighbour.pop();
        let e = result.entry(name).or_insert_with(HashMap::new);
        e.insert(neighbour, value);
    }
    result
}

fn happiness(seats: &[&String], seating_values: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let h = |seats: &[&String]| {
        seating_values
            .get(seats[0])
            .and_then(|v| v.get(seats[1]))
            .unwrap_or(&0)
            + seating_values
                .get(seats[1])
                .and_then(|v| v.get(seats[0]))
                .unwrap_or(&0)
    };
    h(&[seats.last().unwrap(), seats.first().unwrap()]) + seats.windows(2).map(h).sum::<i32>()
}
