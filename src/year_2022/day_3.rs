use std::collections::BTreeSet;

use itertools::Itertools;

aoc_day!(2022, 3);

fn answer_one() -> String {
    input().lines().map(score1).sum::<i32>().to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| score2(&c.collect::<Vec<&str>>()))
        .sum::<i32>()
        .to_string()
}

fn score1(sack: &str) -> i32 {
    let (first, second) = sack.split_at(sack.len() / 2);
    let first: BTreeSet<_> = first.chars().collect();
    let second: BTreeSet<_> = second.chars().collect();

    let common = first.intersection(&second).next().unwrap();

    match *common {
        c @ 'a'..='z' => c as i32 - 96,
        c @ 'A'..='Z' => c as i32 - 38,
        _ => unreachable!(),
    }
}

fn score2(group: &[&str]) -> i32 {
    let sacks: Vec<_> = group
        .iter()
        .map(|g| g.chars().collect::<BTreeSet<_>>())
        .collect();

    let common = (&(&sacks[0] & &sacks[1]) & &sacks[2])
        .into_iter()
        .next()
        .unwrap();

    match common {
        c @ 'a'..='z' => c as i32 - 96,
        c @ 'A'..='Z' => c as i32 - 38,
        _ => unreachable!(),
    }
}
