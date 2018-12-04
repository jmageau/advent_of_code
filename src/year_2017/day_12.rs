use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let connections = get_connections(&input);

    let mut connected_programs = HashSet::new();
    get_connected_programs(0, &connections, &mut connected_programs);
    connected_programs.len().to_string()
}

fn answer_two() -> String {
    let input = input();
    let connections = get_connections(&input);

    let mut counted_ids = HashSet::new();
    let mut group_count = 0;

    for (id, _) in connections.iter() {
        if counted_ids.contains(id) {
            continue;
        }

        let mut connected_program_ids = HashSet::new();
        get_connected_programs(*id, &connections, &mut connected_program_ids);

        counted_ids.extend(connected_program_ids);
        group_count += 1;
    }

    group_count.to_string()
}

fn get_connections(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut connections = HashMap::new();
    let line_regex = Regex::new(r"^(\d+) <-> ([\d, ]+)$").unwrap();

    for line in input.lines() {
        let captures = line_regex.captures(line).unwrap();
        let id = captures
            .get(1)
            .unwrap()
            .as_str()
            .trim()
            .parse::<usize>()
            .unwrap();
        let connected_ids: Vec<_> = captures
            .get(2)
            .unwrap()
            .as_str()
            .trim()
            .split(", ")
            .map(|id| id.parse::<usize>().unwrap())
            .collect();

        connections.insert(id, connected_ids);
    }

    connections
}

fn get_connected_programs(
    id: usize,
    connections: &HashMap<usize, Vec<usize>>,
    connected_programs: &mut HashSet<usize>,
) {
    connected_programs.insert(id);
    for connected_program_id in connections.get(&id).unwrap() {
        if connected_programs.insert(*connected_program_id) {
            get_connected_programs(*connected_program_id, connections, connected_programs);
        }
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_12").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
