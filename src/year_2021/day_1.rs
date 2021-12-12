use itertools::Itertools;

aoc_day!(2021, 1);

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(a, b, c, d)| a + b + c < b + c + d)
        .count()
        .to_string()
}
