aoc_day!(2020, 3);

fn answer_one() -> String {
    let map: Vec<Vec<_>> = input()
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    tree_count(&map, (3, 1)).to_string()
}

fn answer_two() -> String {
    let map: Vec<Vec<_>> = input()
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    (tree_count(&map, (1, 1))
        * tree_count(&map, (3, 1))
        * tree_count(&map, (5, 1))
        * tree_count(&map, (7, 1))
        * tree_count(&map, (1, 2)))
    .to_string()
}

fn tree_count(map: &Vec<Vec<bool>>, step: (usize, usize)) -> usize {
    (0..)
        .map(|i| (i * step.0, i * step.1))
        .take_while(|&(_, y)| y < map.len())
        .filter(|&(x, y)| map[y][x % map[0].len()])
        .count()
}
