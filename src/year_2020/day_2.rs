aoc_day!(2020, 2);

fn answer_one() -> String {
    input()
        .lines()
        .filter(|l| is_valid1(&l))
        .count()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .filter(|l| is_valid2(&l))
        .count()
        .to_string()
}

fn is_valid1(line: &str) -> bool {
    let parts: Vec<_> = line.split(" ").collect();
    let min_max: Vec<_> = parts[0]
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let letter = parts[1].chars().next().unwrap();
    let password = parts[2];

    let letter_count = password.chars().filter(|&c| c == letter).count();

    letter_count >= min_max[0] && letter_count <= min_max[1]
}

fn is_valid2(line: &str) -> bool {
    let parts: Vec<_> = line.split(" ").collect();
    let positions: Vec<_> = parts[0]
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let letter = parts[1].chars().next().unwrap();
    let password = parts[2];

    password
        .chars()
        .enumerate()
        .filter(|&(i, c)| positions.contains(&(i + 1)) && c == letter)
        .count()
        == 1
}
