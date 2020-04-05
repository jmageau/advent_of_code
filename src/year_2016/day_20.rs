aoc_day!(2016, 20);

fn answer_one() -> String {
    let ranges = parse(&input());
    allowed(ranges).next().unwrap().to_string()
}

fn answer_two() -> String {
    let ranges = parse(&input());
    allowed(ranges).count().to_string()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<_> = input
        .lines()
        .map(|l| {
            (
                l.split('-').nth(0).unwrap().parse().unwrap(),
                l.split('-').nth(1).unwrap().parse().unwrap(),
            )
        })
        .collect();
    result.sort();
    result
}

fn allowed(mut ranges: Vec<(usize, usize)>) -> impl Iterator<Item = usize> {
    (0..=4294967295).filter(move |&ip| {
        if ip > ranges[0].1 {
            ranges.remove(0);
        }
        ranges.iter().all(|&(low, high)| ip < low || ip > high)
    })
}
