use std::{collections::HashMap, str::FromStr};

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    get_value(&Value::Wire(Wire("a".to_owned())), &circuit(&input())).to_string()
}

fn answer_two() -> String {
    let mut circuit = circuit(&input());
    *circuit.get_mut(&Wire("b".to_owned())).unwrap() = Expression::Assign(Value::Signal(Signal(
        get_value(&Value::Wire(Wire("a".to_owned())), &circuit),
    )));
    get_value(&Value::Wire(Wire("a".to_owned())), &circuit).to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_7").unwrap()
}

#[derive(Debug)]
struct Signal(u16);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Wire(String);

#[derive(Debug)]
enum Value {
    Signal(Signal),
    Wire(Wire),
}

impl FromStr for Value {
    type Err = ();
    fn from_str(s: &str) -> Result<Value, Self::Err> {
        if let Ok(n) = s.parse::<u16>() {
            Ok(Value::Signal(Signal(n)))
        } else {
            Ok(Value::Wire(Wire(s.to_owned())))
        }
    }
}

#[derive(Debug)]
enum Expression {
    Assign(Value),
    And(Value, Value),
    Or(Value, Value),
    LeftShift(Value, Value),
    RightShift(Value, Value),
    Not(Value),
}

fn circuit(input: &str) -> HashMap<Wire, Expression> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(" ").collect();
            let wire = Wire((*parts.last().unwrap()).to_owned());
            let expression = if parts[1] == "->" {
                Expression::Assign(parts[0].parse().unwrap())
            } else if parts[1] == "AND" {
                Expression::And(parts[0].parse().unwrap(), parts[2].parse().unwrap())
            } else if parts[1] == "OR" {
                Expression::Or(parts[0].parse().unwrap(), parts[2].parse().unwrap())
            } else if parts[1] == "LSHIFT" {
                Expression::LeftShift(parts[0].parse().unwrap(), parts[2].parse().unwrap())
            } else if parts[1] == "RSHIFT" {
                Expression::RightShift(parts[0].parse().unwrap(), parts[2].parse().unwrap())
            } else if parts[0] == "NOT" {
                Expression::Not(parts[1].parse().unwrap())
            } else {
                unreachable!()
            };

            (wire, expression)
        })
        .collect()
}

fn get_value(value: &Value, circuit: &HashMap<Wire, Expression>) -> u16 {
    get_value_impl(value, circuit, &mut HashMap::new())
}

fn get_value_impl(
    value: &Value,
    circuit: &HashMap<Wire, Expression>,
    known_values: &mut HashMap<Wire, u16>,
) -> u16 {
    let expression = match value {
        Value::Signal(s) => return s.0,
        Value::Wire(w) => {
            if let Some(v) = known_values.get(w) {
                return *v;
            } else {
                circuit.get(&w).unwrap()
            }
        }
    };

    let result = match expression {
        Expression::Assign(v) => get_value_impl(v, circuit, known_values),
        Expression::And(v1, v2) => {
            get_value_impl(v1, circuit, known_values) & get_value_impl(v2, circuit, known_values)
        }
        Expression::Or(v1, v2) => {
            get_value_impl(v1, circuit, known_values) | get_value_impl(v2, circuit, known_values)
        }
        Expression::LeftShift(v1, v2) => {
            get_value_impl(v1, circuit, known_values) << get_value_impl(v2, circuit, known_values)
        }
        Expression::RightShift(v1, v2) => {
            get_value_impl(v1, circuit, known_values) >> get_value_impl(v2, circuit, known_values)
        }
        Expression::Not(v) => !get_value_impl(v, circuit, known_values),
    };

    if let Value::Wire(w) = value {
        known_values.insert(w.clone(), result);
    }

    result
}
