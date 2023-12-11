use std::collections::BTreeMap;

aoc_day!(2023, 1);

fn calibration_value(s: &str, names: bool) -> i32 {
    let mut first = None;
    let mut last = None;

    for i in 0..s.len() {
        if let Some(n) = get_digit(&s[i..], names) {
            first.get_or_insert(n);
            let _ = last.insert(n);
        }
    }

    let a = first.unwrap();
    let b = last.unwrap();

    format!("{a}{b}").parse::<i32>().unwrap()
}

fn get_digit(s: &str, names: bool) -> Option<u32> {
    if let Some(d) = s.chars().next().and_then(|c| c.to_digit(10)) {
        return Some(d);
    }

    let numbers = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    if names {
        for (name, n) in numbers {
            if s.starts_with(name) {
                return Some(n);
            }
        }
    }

    None
}

fn answer_one() -> String {
    input()
        .lines()
        .map(|s| calibration_value(&s, false))
        .sum::<i32>()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|s| calibration_value(&s, true))
        .sum::<i32>()
        .to_string()
}
