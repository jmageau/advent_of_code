use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

pub fn answers() -> String {
    answer_one()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2016/input/input_day_25").unwrap()
}

fn answer_one() -> String {
    let instructions = parse(&input());
    let mut registers = HashMap::new();
    registers.insert('a', 0);

    for i in 0.. {
        *registers.get_mut(&'a').unwrap() = i;
        let output = run(instructions.clone(), &mut registers);

        if output.take(1000).eq((0..=1).cycle().take(1000)) {
            return i.to_string();
        }
    }

    unreachable!()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Value {
    Register(char),
    Integer(isize),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Instruction {
    Cpy(Value, Value),
    Inc(char),
    Dec(char),
    Jnz(Value, Value),
    Tgl(char),
    Out(Value),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let instruction = words.next().unwrap();
        let param1 = words.next().unwrap().to_owned();
        let param2 = words.next();

        Ok(match instruction {
            "cpy" => {
                let value = if let Ok(v) = param1.parse() {
                    Value::Integer(v)
                } else {
                    Value::Register(param1.chars().next().unwrap())
                };
                Instruction::Cpy(
                    value,
                    Value::Register(param2.unwrap().chars().next().unwrap()),
                )
            }
            "inc" => Instruction::Inc(param1.chars().next().unwrap()),
            "dec" => Instruction::Dec(param1.chars().next().unwrap()),
            "jnz" => {
                let value = if let Ok(v) = param1.parse() {
                    Value::Integer(v)
                } else {
                    Value::Register(param1.chars().next().unwrap())
                };
                let offset = if let Ok(v) = param2.unwrap().parse() {
                    Value::Integer(v)
                } else {
                    Value::Register(param2.unwrap().chars().next().unwrap())
                };
                Instruction::Jnz(value, offset)
            }
            "tgl" => Instruction::Tgl(param1.chars().next().unwrap()),
            "out" => {
                let value = if let Ok(v) = param1.parse() {
                    Value::Integer(v)
                } else {
                    Value::Register(param1.chars().next().unwrap())
                };
                Instruction::Out(value)
            }

            _ => unreachable!(),
        })
    }
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match self {
            Instruction::Cpy(v, c) => Instruction::Jnz(*v, *c),
            Instruction::Inc(c) => Instruction::Dec(*c),
            Instruction::Dec(c) | Instruction::Tgl(c) => Instruction::Inc(*c),
            Instruction::Jnz(v, int) => Instruction::Cpy(*v, *int),
            Instruction::Out(v) => Instruction::Out(*v),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn run<'a>(
    mut instructions: Vec<Instruction>,
    registers: &'a mut HashMap<char, isize>,
) -> impl Iterator<Item = isize> + 'a {
    let mut i = 0isize;
    std::iter::from_fn(move || {
        while let Some(instruction) = usize::try_from(i).ok().and_then(|i| instructions.get(i)) {
            match instruction {
                Instruction::Cpy(v, r) => {
                    if let Value::Register(r) = r {
                        let val = match v {
                            Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                            Value::Integer(int) => *int,
                        };
                        *registers.entry(*r).or_insert(0) = val;
                    }
                    i += 1;
                }
                Instruction::Inc(r) => {
                    *registers.entry(*r).or_insert(0) += 1;
                    i += 1;
                }
                Instruction::Dec(r) => {
                    *registers.entry(*r).or_insert(0) -= 1;
                    i += 1;
                }
                Instruction::Jnz(v, offset) => {
                    let val = match v {
                        Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                        Value::Integer(int) => *int,
                    };
                    let offset = match offset {
                        Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                        Value::Integer(int) => *int,
                    };

                    if val != 0 {
                        i += offset;
                    } else {
                        i += 1;
                    }
                }
                Instruction::Tgl(c) => {
                    let offset = *registers.entry(*c).or_insert(0);

                    if let Some(i) = instructions.get_mut((i + offset) as usize) {
                        i.toggle();
                    }
                    i += 1;
                }
                Instruction::Out(v) => {
                    let val = match v {
                        Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                        Value::Integer(int) => *int,
                    };
                    i += 1;
                    return Some(val);
                }
            }
        }
        unreachable!()
    })
}
