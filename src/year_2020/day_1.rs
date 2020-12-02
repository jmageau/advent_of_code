use itertools::Itertools;

aoc_day!(2020, 1);

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .combinations(2)
        .find(|c| c.iter().sum::<u32>() == 2020)
        .map(|c| c.iter().product::<u32>())
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .combinations(3)
        .find(|c| c.iter().sum::<u32>() == 2020)
        .map(|c| c.iter().product::<u32>())
        .unwrap()
        .to_string()
}
