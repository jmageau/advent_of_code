use itertools::Itertools;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    combinations(150, &get_containers(&input()))
        .len()
        .to_string()
}

fn answer_two() -> String {
    let containers = get_containers(&input());
    let combinations = combinations(150, &containers);
    let min = combinations.iter().map(Vec::len).min().unwrap();
    combinations
        .iter()
        .filter(|c| c.len() == min)
        .count()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_17").unwrap()
}

fn get_containers(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn combinations(target: usize, containers: &[usize]) -> Vec<Vec<&usize>> {
    (1..=containers.len())
        .flat_map(|c| {
            containers
                .iter()
                .combinations(c)
                .filter(|c| c.iter().cloned().sum::<usize>() == target)
        })
        .collect()
}
