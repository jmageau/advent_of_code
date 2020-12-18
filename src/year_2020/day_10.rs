use std::collections::HashMap;

aoc_day!(2020, 10);

fn answer_one() -> String {
    let mut ratings: Vec<_> = input().lines().map(|l| l.parse().unwrap()).collect();
    ratings.sort();
    ratings.insert(0, 0);
    ratings.push(ratings.last().unwrap() + 3);

    ratings
        .windows(2)
        .fold([0, 0], |mut acc, w| {
            if abs_difference(w[0], w[1]) == 1 {
                acc[0] += 1;
            } else if abs_difference(w[0], w[1]) == 3 {
                acc[1] += 1;
            }
            acc
        })
        .iter()
        .product::<u32>()
        .to_string()
}

fn answer_two() -> String {
    let mut ratings: Vec<_> = input().lines().map(|l| l.parse().unwrap()).collect();
    ratings.sort();
    ratings.insert(0, 0);
    ratings.push(ratings.last().unwrap() + 3);

    let mut paths = HashMap::new();

    path_count(&ratings, 0, ratings.len() - 1, &mut paths).to_string()
}

fn path_count(
    ratings: &[u32],
    from_index: usize,
    to_index: usize,
    paths: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if from_index == to_index {
        return 1;
    }
    if let Some(&count) = paths.get(&(from_index, to_index)) {
        return count;
    }

    let prev = to_index.saturating_sub(3)..to_index;
    let count = prev
        .filter(|&p| ratings[to_index] - ratings[p] <= 3)
        .map(|p| path_count(&ratings, from_index, p, paths))
        .sum();
    paths.insert((from_index, to_index), count);
    count
}

fn abs_difference(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
