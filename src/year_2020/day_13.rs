aoc_day!(2020, 13);

fn answer_one() -> String {
    let current_time = input().lines().next().unwrap().parse::<usize>().unwrap();
    let buses: Vec<_> = input()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect();

    let bus = buses
        .iter()
        .filter_map(|b| b.parse::<usize>().ok())
        .min_by_key(|&b| next_factor(current_time, b))
        .unwrap();
    let wait_time = (current_time / bus + 1) * bus - current_time;

    (bus * wait_time).to_string()
}

fn answer_two() -> String {
    let buses: Vec<_> = input()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, b)| b.parse::<usize>().ok().map(|b| (i, b)))
        .collect();

    let (i, step) = buses.iter().max_by_key(|&(_, b)| b).unwrap();
    let first = step - i;

    (first..)
        .step_by(*step)
        .find(|t| buses.iter().all(|(i, b)| (t + i) % b == 0))
        .unwrap()
        .to_string()
}

fn next_factor(n: usize, factor: usize) -> usize {
    if n % factor == 0 {
        n
    } else {
        (n / factor + 1) * factor
    }
}
