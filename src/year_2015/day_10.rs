pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    look_and_say(input()).nth(40).unwrap().len().to_string()
}

fn answer_two() -> String {
    look_and_say(input()).nth(50).unwrap().len().to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_10").unwrap()
}

fn look_and_say(current: String) -> impl Iterator<Item = String> {
    std::iter::successors(Some(current), |current| {
        let current: Vec<_> = current.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut next_digits = vec![1, current[0]];

        for &d in current.iter().skip(1) {
            if d == *next_digits.last().unwrap() {
                let p = next_digits.len() - 2;
                next_digits[p] += 1;
            } else {
                next_digits.extend_from_slice(&[1, d]);
            }
        }

        Some(
            next_digits
                .iter()
                .map(|&d| std::char::from_digit(d, 10).unwrap())
                .collect(),
        )
    })
}
