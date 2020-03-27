pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let ingredients = get_ingredients(&input());

    let mut max = 0;
    for i0 in 0..=100 {
        for i1 in 0..=(100 - i0) {
            for i2 in 0..(100 - i0 - i1) {
                let (score, _) = get_score(i0, i1, i2, &ingredients);
                if score > max {
                    max = score;
                }
            }
        }
    }

    max.to_string()
}

fn answer_two() -> String {
    let ingredients = get_ingredients(&input());

    let mut max = 0;
    for i0 in 0..=100 {
        for i1 in 0..=(100 - i0) {
            for i2 in 0..(100 - i0 - i1) {
                let (score, calories) = get_score(i0, i1, i2, &ingredients);
                if score > max && calories == 500 {
                    max = score;
                }
            }
        }
    }

    max.to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_15").unwrap()
}

fn get_ingredients(input: &str) -> Vec<(i32, i32, i32, i32, i32)> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split(' ');
            let mut name = words.nth(0).unwrap().to_owned();
            name.pop();
            let mut capacity = words.nth(1).unwrap().to_owned();
            capacity.pop();
            let capacity = capacity.parse::<i32>().unwrap();
            let mut durability = words.nth(1).unwrap().to_owned();
            durability.pop();
            let durability = durability.parse::<i32>().unwrap();
            let mut flavor = words.nth(1).unwrap().to_owned();
            flavor.pop();
            let flavor = flavor.parse::<i32>().unwrap();
            let mut texture = words.nth(1).unwrap().to_owned();
            texture.pop();
            let texture = texture.parse::<i32>().unwrap();
            let calories = words.nth(1).unwrap().to_owned().parse::<i32>().unwrap();
            (capacity, durability, flavor, texture, calories)
        })
        .collect()
}

fn get_score(
    i0: i32,
    i1: i32,
    i2: i32,
    ingredients: &Vec<(i32, i32, i32, i32, i32)>,
) -> (i32, i32) {
    let i3 = 100 - i0 - i1 - i2;
    let capacity = i0 * ingredients[0].0
        + i1 * ingredients[1].0
        + i2 * ingredients[2].0
        + i3 * ingredients[3].0;
    let durability = i0 * ingredients[0].1
        + i1 * ingredients[1].1
        + i2 * ingredients[2].1
        + i3 * ingredients[3].1;
    let flavor = i0 * ingredients[0].2
        + i1 * ingredients[1].2
        + i2 * ingredients[2].2
        + i3 * ingredients[3].2;
    let texture = i0 * ingredients[0].3
        + i1 * ingredients[1].3
        + i2 * ingredients[2].3
        + i3 * ingredients[3].3;
    let calories = i0 * ingredients[0].4
        + i1 * ingredients[1].4
        + i2 * ingredients[2].4
        + i3 * ingredients[3].4;

    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        (0, calories)
    } else {
        (capacity * durability * flavor * texture, calories)
    }
}
