use itertools::Itertools;

aoc_day!(2021, 7);

fn answer_one() -> String {
    let positions: Vec<i32> = input()
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let (min, max) = match positions.iter().minmax() {
        itertools::MinMaxResult::MinMax(&a, &b) => (a, b),
        _ => unreachable!(),
    };

    (min..=max)
        .map(|target| positions.iter().map(|p| (target - p).abs()).sum::<i32>())
        .min()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let positions: Vec<i32> = input()
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let (min, max) = match positions.iter().minmax() {
        itertools::MinMaxResult::MinMax(&a, &b) => (a, b),
        _ => unreachable!(),
    };

    (min..=max)
        .map(|target| {
            positions
                .iter()
                .map(|p| {
                    let n = (target - p).abs();
                    n * (n + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
        .to_string()
}
