use itertools::Itertools;
use std::collections::HashMap;

aoc_day!(2020, 7);

fn answer_one() -> String {
    let rules: HashMap<String, HashMap<String, usize>> = input().lines().map(parse_rule).collect();
    rules
        .keys()
        .filter(|r| contains(r, "shiny gold", &rules))
        .count()
        .to_string()
}

fn answer_two() -> String {
    let rules: HashMap<String, HashMap<String, usize>> = input().lines().map(parse_rule).collect();
    (count("shiny gold", &rules) - 1).to_string()
}

fn parse_rule(line: &str) -> (String, HashMap<String, usize>) {
    let mut parts = line.split(" bags contain ");
    let color = parts.next().unwrap().to_owned();

    let contents = parts.next().unwrap();
    if contents.starts_with("no") {
        return (color, HashMap::new());
    }

    let contents = contents
        .split(", ")
        .map(|c| {
            let mut parts = c.split_whitespace();
            let count = parts.next().unwrap().parse().unwrap();
            let color = parts.take(2).join(" ");
            (color, count)
        })
        .collect();

    (color, contents)
}

fn contains(first: &str, second: &str, rules: &HashMap<String, HashMap<String, usize>>) -> bool {
    rules
        .get(first)
        .unwrap()
        .keys()
        .any(|r| r == second || contains(r, second, &rules))
}

fn count(bag: &str, rules: &HashMap<String, HashMap<String, usize>>) -> usize {
    let contents = rules.get(bag).unwrap();
    if contents.is_empty() {
        return 1;
    }
    1 + contents
        .iter()
        .map(|(b, c)| count(b, &rules) * c)
        .sum::<usize>()
}
