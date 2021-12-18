use std::collections::BTreeMap;

use itertools::Itertools;

aoc_day!(2021, 8);

fn answer_one() -> String {
    let values: Vec<_> = input()
        .lines()
        .map(|l| {
            let (input, output) = l.split_once('|').unwrap();
            let input: Vec<_> = input.split_whitespace().map(|s| s.to_owned()).collect();
            let output: Vec<_> = output.split_whitespace().map(|s| s.to_owned()).collect();
            (input, output)
        })
        .collect();

    values
        .iter()
        .flat_map(|(_, output)| output)
        .filter(|output| matches!(output.len(), 2 | 4 | 3 | 7))
        .count()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|l| {
            let (input, output) = l.split_once('|').unwrap();
            let input: Vec<_> = input.split_whitespace().map(|s| s.to_owned()).collect();
            let output: Vec<_> = output.split_whitespace().map(|s| s.to_owned()).collect();
            (input, output)
        })
        .map(|(input, output)| output_value(input, output))
        .sum::<i32>()
        .to_string()
}

fn output_value(mut input: Vec<String>, mut output: Vec<String>) -> i32 {
    for v in &mut input {
        *v = v.chars().sorted().collect()
    }
    for v in &mut output {
        *v = v.chars().sorted().collect()
    }

    let segments = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];

    let permutation = "abcdefg"
        .chars()
        .permutations(7)
        .find(|permutation| {
            let mapping: BTreeMap<_, _> = "abcdefg".chars().zip(permutation).collect();
            let permutation_segments: Vec<_> = segments
                .iter()
                .map(|segment| {
                    segment
                        .chars()
                        .map(|c| **mapping.get(&c).unwrap())
                        .sorted()
                        .collect()
                })
                .collect();

            input.iter().all(|s| permutation_segments.contains(s))
                && output.iter().all(|s| permutation_segments.contains(s))
        })
        .unwrap();

    let mapping: BTreeMap<_, _> = permutation.iter().zip("abcdefg".chars()).collect();

    output
        .iter()
        .map(|number| {
            number
                .chars()
                .map(|c| mapping.get(&c).unwrap())
                .sorted()
                .collect::<String>()
        })
        .map(|s| {
            segments
                .iter()
                .position(|&segment| segment == s)
                .unwrap()
                .to_string()
        })
        .collect::<String>()
        .parse()
        .unwrap()
}
