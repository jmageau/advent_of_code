aoc_day!(2022, 4);

fn answer_one() -> String {
    input()
        .lines()
        .filter(|l| fully_contains(l))
        .count()
        .to_string()
}

fn answer_two() -> String {
    input().lines().filter(|l| overlaps(l)).count().to_string()
}

fn fully_contains(line: &str) -> bool {
    let (first, second) = line.split_once(',').unwrap();
    let first = first.split_once('-').unwrap();
    let second = second.split_once('-').unwrap();
    let first: (i32, i32) = (first.0.parse().unwrap(), first.1.parse().unwrap());
    let second: (i32, i32) = (second.0.parse().unwrap(), second.1.parse().unwrap());

    first.0 <= second.0 && first.1 >= second.1 || second.0 <= first.0 && second.1 >= first.1
}

fn overlaps(line: &str) -> bool {
    let (first, second) = line.split_once(',').unwrap();
    let first = first.split_once('-').unwrap();
    let second = second.split_once('-').unwrap();
    let first: (i32, i32) = (first.0.parse().unwrap(), first.1.parse().unwrap());
    let second: (i32, i32) = (second.0.parse().unwrap(), second.1.parse().unwrap());

    first.0 <= second.1 && first.1 >= second.0
}
