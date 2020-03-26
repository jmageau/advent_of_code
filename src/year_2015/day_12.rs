use serde_json::Value;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    get_numbers_sum(&serde_json::from_str(&input()).unwrap(), false).to_string()
}

fn answer_two() -> String {
    get_numbers_sum(&serde_json::from_str(&input()).unwrap(), true).to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_12").unwrap()
}

fn get_numbers_sum(value: &Value, filter: bool) -> i64 {
    match value {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| get_numbers_sum(v, filter)).sum(),
        Value::Object(o) => {
            if filter && o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(|v| get_numbers_sum(v, filter)).sum()
            }
        }
    }
}
