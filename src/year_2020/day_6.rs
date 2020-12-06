use std::collections::HashSet;

aoc_day!(2020, 6);

fn answer_one() -> String {
    input()
        .split("\n\n")
        .map(|g| {
            g.lines()
                .flat_map(|p| p.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    input()
        .split("\n\n")
        .map(|g| {
            let questions = g.lines().flat_map(|p| p.chars()).collect::<HashSet<_>>();
            questions
                .iter()
                .filter(|&&q| g.lines().all(|l| l.contains(q)))
                .count()
        })
        .sum::<usize>()
        .to_string()
}
