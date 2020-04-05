aoc_day!(2016, 15);

fn answer_one() -> String {
    let discs = parse(&input());

    (0..)
        .find(|&t| simulate(t, discs.clone()))
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let mut discs = parse(&input());
    discs.push((11, 0));

    (0..)
        .find(|&t| simulate(t, discs.clone()))
        .unwrap()
        .to_string()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let positions = l.split(' ').nth(3).unwrap().parse().unwrap();
            let mut initial = l.split(' ').last().unwrap().to_owned();
            initial.pop();
            let initial = initial.parse().unwrap();
            (positions, initial)
        })
        .collect()
}

fn simulate(button_time: usize, mut discs: Vec<(usize, usize)>) -> bool {
    for (positions, current) in discs.iter_mut() {
        *current = (*current + button_time) % *positions;
    }

    for capsule in 0..=discs.len() {
        if capsule > 0 && discs[capsule - 1].1 != 0 {
            return false;
        }
        for (positions, current) in discs.iter_mut() {
            *current = (*current + 1) % *positions;
        }
    }

    true
}
