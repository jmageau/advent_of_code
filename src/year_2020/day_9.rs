use itertools::Itertools;

aoc_day!(2020, 9);

fn answer_one() -> String {
    let numbers: Vec<_> = input().lines().map(|n| n.parse().unwrap()).collect();
    first_invalid(&numbers, 25).to_string()
}

fn answer_two() -> String {
    let numbers: Vec<_> = input().lines().map(|n| n.parse().unwrap()).collect();
    let first_invalid = first_invalid(&numbers, 25);
    let (min, max) = find_min_max(&numbers, first_invalid);

    (min + max).to_string()
}

fn first_invalid(numbers: &[usize], preamble_size: usize) -> usize {
    *numbers
        .windows(preamble_size + 1)
        .find(|w| !is_valid(w))
        .unwrap()
        .last()
        .unwrap()
}

fn is_valid(window: &[usize]) -> bool {
    let (numbers, target) = window.split_at(window.len() - 1);
    let target = target[0];

    numbers
        .iter()
        .combinations(2)
        .any(|c| c[0] + c[1] == target)
}

fn find_min_max(numbers: &[usize], target: usize) -> (usize, usize) {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let sum = numbers[i..j].iter().sum::<usize>();
            if sum == target {
                let min = numbers[i..j].iter().min().unwrap();
                let max = numbers[i..j].iter().max().unwrap();
                return (*min, *max);
            }
        }
    }

    unreachable!()
}
