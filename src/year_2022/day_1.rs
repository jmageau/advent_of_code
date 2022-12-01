use itertools::Itertools;

aoc_day!(2022, 1);

fn answer_one() -> String {
    let input = input();
    input
        .split("\r\n\r\n")
        .map(|e| {
            e.trim()
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let input = input();
    input
        .split("\r\n\r\n")
        .map(|e| {
            e.trim()
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>()
        .to_string()
}
