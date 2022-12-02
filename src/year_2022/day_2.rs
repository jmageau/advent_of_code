use itertools::Itertools;

aoc_day!(2022, 2);

fn answer_one() -> String {
    input().lines().map(score_one).sum::<i32>().to_string()
}

fn answer_two() -> String {
    input().lines().map(score_two).sum::<i32>().to_string()
}

fn score_one(round: &str) -> i32 {
    let (first, second) = round.split_whitespace().collect_tuple().unwrap();

    let score = match second {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    };

    let outcome = match (first, second) {
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        _ => unreachable!(),
    };

    score + outcome
}

fn score_two(round: &str) -> i32 {
    let (first, second) = round.split_whitespace().collect_tuple().unwrap();

    let outcome = match second {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => unreachable!(),
    };

    let score = match (first, second) {
        ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
        ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
        ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
        _ => unreachable!(),
    };

    score + outcome
}
