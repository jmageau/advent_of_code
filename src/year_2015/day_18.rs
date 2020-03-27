pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let mut lights = get_lights(&input());
    for _ in 0..100 {
        lights = next(&lights);
    }

    lights
        .into_iter()
        .map(|r| r.into_iter().filter(|&c| c).count())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let mut lights = get_lights(&input());
    for _ in 0..100 {
        lights = next(&lights);
        *lights.first_mut().unwrap().first_mut().unwrap() = true;
        *lights.first_mut().unwrap().last_mut().unwrap() = true;
        *lights.last_mut().unwrap().first_mut().unwrap() = true;
        *lights.last_mut().unwrap().last_mut().unwrap() = true;
    }

    lights
        .into_iter()
        .map(|r| r.into_iter().filter(|&c| c).count())
        .sum::<usize>()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_18").unwrap()
}

fn get_lights(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn checked_add(a: usize, b: i32) -> Option<usize> {
    if b >= 0 {
        a.checked_add(b as usize)
    } else {
        a.checked_sub(b.wrapping_abs() as usize)
    }
}

fn next(lights: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let neighbours_delta = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let neighbours_count = |r: usize, c| {
        neighbours_delta
            .iter()
            .filter_map(|&(rd, cd)| {
                if let (Some(r), Some(c)) = (checked_add(r, rd), checked_add(c, cd)) {
                    lights.get(r).and_then(|col| col.get(c))
                } else {
                    None
                }
            })
            .filter(|&&c| c)
            .count()
    };
    lights
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, &cell)| {
                    let n = neighbours_count(r, c);
                    if cell && n != 2 && n != 3 {
                        false
                    } else if !cell && n == 3 {
                        true
                    } else {
                        cell
                    }
                })
                .collect()
        })
        .collect()
}
