use itertools::Itertools;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    next_passwords(input())
        .find(|p| password_is_valid(p))
        .unwrap()
}

fn answer_two() -> String {
    next_passwords(answer_one())
        .find(|p| password_is_valid(p))
        .unwrap()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_11").unwrap()
}

fn next_passwords(current: String) -> impl Iterator<Item = String> {
    fn next_password(current: &str) -> String {
        let mut chars: Vec<_> = current.chars().collect();
        let new_last = std::char::from_u32(*chars.last().unwrap() as u32 + 1).unwrap();
        if chars.len() == 1 {
            new_last.to_string()
        } else if new_last <= 'z' {
            *chars.last_mut().unwrap() = new_last;
            chars.iter().collect()
        } else {
            format!(
                "{}a",
                next_password(&chars.iter().take(chars.len() - 1).collect::<String>())
            )
        }
    }

    std::iter::successors(Some(current), |current| Some(next_password(current))).skip(1)
}

fn password_is_valid(password: &str) -> bool {
    let chars: Vec<_> = password.chars().collect();
    chars
        .windows(3)
        .any(|w| w[2] as i32 - w[1] as i32 == 1 && w[1] as i32 - w[0] as i32 == 1)
        && !chars.iter().any(|&c| c == 'i' || c == 'o' || c == 'l')
        && chars
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == w[1])
            .map(|(i, _)| i)
            .combinations(2)
            .any(|c| c[1] - c[0] > 1)
}
