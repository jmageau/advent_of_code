extern crate chrono;

use self::chrono::prelude::*;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let guard_minutes_counts = get_guard_minutes_counts(&input);

    let best_guard = guard_minutes_counts
        .iter()
        .max_by_key(|(_, minutes_count)| minutes_count.values().sum::<usize>())
        .unwrap()
        .0;
    let best_minute = *guard_minutes_counts
        .get(best_guard)
        .unwrap()
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0 as usize;

    (best_guard * best_minute).to_string()
}

fn answer_two() -> String {
    let input = input();
    let guard_minutes_counts = get_guard_minutes_counts(&input);

    let best_guard = guard_minutes_counts
        .iter()
        .max_by_key(|(_, minutes_count)| minutes_count.values().max().unwrap())
        .unwrap()
        .0;
    let best_minute = *guard_minutes_counts
        .get(best_guard)
        .unwrap()
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0 as usize;

    (best_guard * best_minute).to_string()
}

fn get_guard_minutes_counts(input: &str) -> HashMap<usize, HashMap<u32, usize>> {
    let line_regex = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (?:Guard #(\d+) begins shift|(falls asleep)|(wakes up))$").unwrap();

    let mut events = BTreeMap::new();

    for line in input.lines() {
        let captures = line_regex.captures(line).unwrap();

        let date_time = Utc
            .datetime_from_str(captures.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M")
            .unwrap();

        let event = if let Some(id) = captures.get(2) {
            Event::BeginShift(id.as_str().parse::<usize>().unwrap())
        } else if let Some(_) = captures.get(3) {
            Event::FallAsleep
        } else if let Some(_) = captures.get(4) {
            Event::WakeUp
        } else {
            unreachable!()
        };

        events.insert(date_time, event);
    }

    let mut guard_minutes_counts = HashMap::new();
    let mut current_guard = 0;
    let mut fall_asleep_minute = 0;

    for (date_time, event) in events {
        match event {
            Event::BeginShift(id) => current_guard = id,
            Event::FallAsleep => fall_asleep_minute = date_time.minute(),
            Event::WakeUp => {
                let wake_up_minute = date_time.minute();
                let minute_counts = guard_minutes_counts
                    .entry(current_guard)
                    .or_insert(HashMap::new());
                for minute in fall_asleep_minute..wake_up_minute {
                    let count = minute_counts.entry(minute).or_insert(0usize);
                    *count += 1;
                }
            }
        }
    }
    guard_minutes_counts
}

#[derive(Debug)]
enum Event {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_4").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
