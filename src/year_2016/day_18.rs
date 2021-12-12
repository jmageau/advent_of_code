aoc_day!(2016, 18);

fn answer_one() -> String {
    let first_row: Vec<_> = input().trim().chars().map(|c| c == '^').collect();
    rows(first_row)
        .take(40)
        .map(|r| r.iter().filter(|&&r| !r).count())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let first_row: Vec<_> = input().trim().chars().map(|c| c == '^').collect();
    rows(first_row)
        .take(400000)
        .map(|r| r.iter().filter(|&&r| !r).count())
        .sum::<usize>()
        .to_string()
}

fn rows(first_row: Vec<bool>) -> impl Iterator<Item = Vec<bool>> {
    std::iter::successors(Some(first_row), |prev| {
        Some(
            (0..prev.len())
                .map(|i| {
                    let left = i > 0 && prev[i - 1];
                    let right = i < prev.len() - 1 && prev[i + 1];
                    !left && right || left && !right
                })
                .collect(),
        )
    })
}
