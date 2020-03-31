use itertools::Itertools;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let weights: Vec<_> = input()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    min_entanglement(&weights, 3).to_string()
}

fn answer_two() -> String {
    let weights: Vec<_> = input()
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    min_entanglement(&weights, 4).to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_24").unwrap()
}

fn entanglement(group: &[&usize]) -> usize {
    group.iter().cloned().product()
}

fn min_entanglement(weights: &[usize], group_size: usize) -> usize {
    let group_weight = weights.iter().sum::<usize>() / group_size;
    let is_right_weight = |v: &[&usize]| v.iter().cloned().sum::<usize>() == group_weight;

    let mut min_entanglement = None;
    for i in 1..weights.len() - (group_size - 1) {
        for group in weights
            .iter()
            .combinations(i)
            .filter(|w| is_right_weight(&w))
        {
            if min_entanglement.is_none() {
                min_entanglement = Some(entanglement(&group));
            } else {
                min_entanglement = Some(std::cmp::min(
                    min_entanglement.unwrap(),
                    entanglement(&group),
                ));
            }
        }
        if let Some(e) = min_entanglement {
            return e;
        }
    }

    unreachable!()
}
