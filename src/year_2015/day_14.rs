use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let reindeer = get_reindeer(&input());
    positions_after_seconds(2503, &reindeer)
        .values()
        .map(|v| v.0)
        .max()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let reindeer = get_reindeer(&input());
    positions_after_seconds(2503, &reindeer)
        .values()
        .map(|v| v.1)
        .max()
        .unwrap()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_14").unwrap()
}

fn get_reindeer(input: &str) -> HashMap<String, (i32, i32, i32)> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(' ');
            let name = words.nth(0).unwrap().to_owned();
            let speed = words.nth(2).unwrap().parse::<i32>().unwrap();
            let duration = words.nth(2).unwrap().parse::<i32>().unwrap();
            let rest = words.nth(6).unwrap().parse::<i32>().unwrap();

            (name, (speed, duration, rest))
        })
        .collect()
}

fn positions_after_seconds(
    seconds: i32,
    reindeer: &HashMap<String, (i32, i32, i32)>,
) -> HashMap<String, (i32, i32)> {
    let mut state: HashMap<_, _> = reindeer
        .iter()
        .map(|(name, &(speed, duration, rest))| {
            (name.to_owned(), (speed, duration, rest, duration, 0, 0))
        })
        .collect();

    for _ in 0..seconds {
        for (speed, duration, rest, timer, distance, _) in state.values_mut() {
            *timer -= 1;
            if *timer >= 0 {
                *distance += *speed;
            } else if *timer == -*rest {
                *timer = *duration;
            }
        }
        state
            .values_mut()
            .max_by_key(|(_, _, _, _, distance, _)| *distance)
            .unwrap()
            .5 += 1;
    }

    state
        .into_iter()
        .map(|(name, (_, _, _, _, distance, points))| (name, (distance, points)))
        .collect()
}
