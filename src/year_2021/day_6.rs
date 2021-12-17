use std::collections::BTreeMap;

aoc_day!(2021, 6);

fn answer_one() -> String {
    let input: Vec<i64> = input()
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut numbers = BTreeMap::new();
    for n in input {
        *numbers.entry(n).or_default() += 1;
    }

    simulate(&mut numbers, 80);

    numbers.values().sum::<i64>().to_string()
}

fn answer_two() -> String {
    let input: Vec<i64> = input()
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut numbers = BTreeMap::new();
    for n in input {
        *numbers.entry(n).or_default() += 1;
    }

    simulate(&mut numbers, 256);

    numbers.values().sum::<i64>().to_string()
}

fn simulate(numbers: &mut BTreeMap<i64, i64>, days: i64) {
    for _ in 0..days {
        let mut new_numbers = BTreeMap::new();
        for (&d, &s) in numbers.iter() {
            if d == 0 {
                *new_numbers.entry(6).or_default() += s;
                *new_numbers.entry(8).or_default() += s;
            } else {
                *new_numbers.entry(d - 1).or_default() += s;
            }
        }

        *numbers = new_numbers;
    }
}
