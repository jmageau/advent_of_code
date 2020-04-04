use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

aoc_day!(2016, 12);

fn answer_one() -> String {
    let instructions = parse(&input());
    let mut registers = HashMap::new();

    run(&instructions, &mut registers);

    registers[&'a'].to_string()
}

fn answer_two() -> String {
    let instructions = parse(&input());
    let mut registers = HashMap::new();
    registers.insert('c', 1);

    run(&instructions, &mut registers);

    registers[&'a'].to_string()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Value {
    Register(char),
    Integer(isize),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Instruction {
    Cpy(Value, char),
    Inc(char),
    Dec(char),
    Jnz(Value, isize),
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
                Instruction::Cpy(value, param2.unwrap().chars().next().unwrap())
            }
            "inc" => Instruction::Inc(param1.chars().next().unwrap()),
            "dec" => Instruction::Dec(param1.chars().next().unwrap()),
            "jnz" => {
                let value = if let Ok(v) = param1.parse() {
                    Value::Integer(v)
                } else {
                    Value::Register(param1.chars().next().unwrap())
                };
                Instruction::Jnz(value, param2.unwrap().parse().unwrap())
            }
            _ => unreachable!(),
        })
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn run(instructions: &[Instruction], registers: &mut HashMap<char, isize>) {
    let mut i = 0isize;
    while let Some(instruction) = usize::try_from(i).ok().and_then(|i| instructions.get(i)) {
        match instruction {
            Instruction::Cpy(v, r) => {
                let val = match v {
                    Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                    Value::Integer(int) => *int,
                };
                *registers.entry(*r).or_insert(0) = val;
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
            Instruction::Jnz(v, int) => {
                let val = match v {
                    Value::Register(r2) => *registers.entry(*r2).or_insert(0),
                    Value::Integer(int) => *int,
                };
                if val != 0 {
                    i += int;
                } else {
                    i += 1;
                }
            }
        }
    }
}
