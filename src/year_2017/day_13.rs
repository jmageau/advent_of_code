use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let layers = get_layers(&input);
    severity(layers_hit(&layers, 0)).to_string()
}

fn answer_two() -> String {
    let input = input();
    let layers = get_layers(&input);

    (0..)
        .find(|&delay| layers_hit(&layers, delay).is_empty())
        .unwrap()
        .to_string()
}

fn get_layers(input: &str) -> HashMap<usize, usize> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(": ").map(|n| n.parse::<usize>().unwrap()).collect();
            (parts[0], parts[1])
        }).collect()
}

fn layers_hit(layers: &HashMap<usize, usize>, delay: usize) -> Vec<(&usize, &usize)> {
    layers
        .iter()
        .filter(|(&depth, &range)| layer_hit(depth, range, delay))
        .collect()
}

fn layer_hit(depth: usize, range: usize, delay: usize) -> bool {
    if range == 1 {
        return true;
    }

    (depth + delay) % ((range - 1) * 2) == 0
}

fn severity(hit_layers: Vec<(&usize, &usize)>) -> usize {
    hit_layers
        .iter()
        .map(|(&depth, &range)| depth * range)
        .sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_13").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
