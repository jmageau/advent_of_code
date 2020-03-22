use regex::Regex;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.chars().count() - (decode(l).chars().count() - 2))
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|l| encode(l).chars().count() + 2 - l.chars().count())
        .sum::<usize>()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_8").unwrap()
}

fn decode(string: &str) -> String {
    let string = string.replace("\\\"", "\"");
    let string = string.replace("\\\\", "/");

    let re = Regex::new(r"\\x(..)").unwrap();
    let string = re.replace_all(&string, |captures: &regex::Captures| {
        std::char::from_u32(u32::from_str_radix(&captures[1].to_string(), 16).unwrap())
            .unwrap()
            .to_string()
    });
    string.replace("/", "\\")
}

fn encode(string: &str) -> String {
    string.replace("\\", "\\\\").replace("\"", "\\\"")
}
