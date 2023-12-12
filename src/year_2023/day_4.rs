use std::collections::{BTreeMap, BTreeSet};

use parse_display::FromStr;

aoc_day!(2023, 4);

#[derive(FromStr, Debug)]
#[display("Card {id}: {winning} | {numbers}")]
struct Card {
    id: String,
    winning: String,
    numbers: String,
}

impl Card {
    fn winning_count(&self) -> usize {
        let winning: BTreeSet<i32> = self
            .winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers: BTreeSet<i32> = self
            .numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        numbers.iter().filter(|n| winning.contains(n)).count()
    }

    fn score(&self) -> i32 {
        let count = self.winning_count();
        if count > 0 {
            2i32.pow(count as u32 - 1)
        } else {
            0
        }
    }

    fn id(&self) -> i32 {
        self.id.trim().parse::<i32>().unwrap()
    }
}

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.parse::<Card>().unwrap().score())
        .sum::<i32>()
        .to_string()
}

fn answer_two() -> String {
    let cards: Vec<_> = input()
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect();

    let mut card_counts: BTreeMap<_, _> = cards.iter().map(|c| (c.id(), 1)).collect();

    for card in cards {
        let winning_count = card.winning_count();
        let card_count = card_counts[&card.id()];

        for i in card.id() + 1..card.id() + 1 + winning_count as i32 {
            let count = card_counts.get_mut(&i).unwrap();
            *count += card_count;
        }
    }

    card_counts.values().sum::<i32>().to_string()
}
