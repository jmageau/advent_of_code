use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    let steps = get_steps(&input);
    traverse_steps(&steps)
}

fn answer_two() -> String {
    let input = input();

    let steps = get_steps(&input);
    let order = traverse_steps(&steps);

    run_steps(&steps, &order, 5).to_string()
}

fn get_steps(input: &str) -> HashMap<char, Vec<char>> {
    let mut steps = HashMap::new();

    for line in input.lines() {
        let mut chars = line.chars();
        let step = chars.nth(5).unwrap();
        let dependent_step = chars.nth(30).unwrap();

        steps.entry(step).or_insert_with(Vec::new);
        let dependent_steps = steps.entry(dependent_step).or_insert_with(Vec::new);
        dependent_steps.push(step);
    }

    steps
}

fn traverse_steps(steps: &HashMap<char, Vec<char>>) -> String {
    let mut ordered_steps = vec![];

    while ordered_steps.len() != steps.len() {
        let mut ready_steps: Vec<_> = steps
            .iter()
            .filter(|(&k, _)| !ordered_steps.contains(&k))
            .filter(|(_, v)| v.iter().all(|s| ordered_steps.contains(s)))
            .map(|(&k, _)| k)
            .collect();
        ready_steps.sort();
        ordered_steps.extend(ready_steps);
    }

    ordered_steps.iter().collect()
}

fn run_steps(steps: &HashMap<char, Vec<char>>, order: &str, worker_count: usize) -> usize {
    let mut total_duration = 0;
    let mut done_steps = HashSet::new();
    let ordered_steps: Vec<_> = order.chars().collect();
    let mut workers = HashMap::new();

    while done_steps.len() != steps.len() {
        for _ in 0..(worker_count - workers.len()) {
            if let Some(next_step) = ordered_steps.iter().find(|&s| {
                !done_steps.contains(s)
                    && !workers.contains_key(s)
                    && steps[s].iter().all(|s| done_steps.contains(s))
            }) {
                workers.insert(*next_step, *next_step as u8 - 4);
            }
        }

        for v in workers.values_mut() {
            *v -= 1;
        }

        let done_workers: Vec<_> = workers
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(k, _)| k)
            .cloned()
            .collect();
        for w in done_workers {
            workers.remove(&w);
            done_steps.insert(w);
        }

        total_duration += 1;
    }

    total_duration
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_7").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
