use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let program = parse(&input());

    let mut registers = HashMap::new();
    registers.insert("a".to_owned(), 0);
    registers.insert("b".to_owned(), 0);
    run(&program, &mut registers);

    registers["b"].to_string()
}

fn answer_two() -> String {
    let program = parse(&input());

    let mut registers = HashMap::new();
    registers.insert("a".to_owned(), 1);
    registers.insert("b".to_owned(), 0);
    run(&program, &mut registers);

    registers["b"].to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_23").unwrap()
}

#[derive(Debug)]
enum Instruction {
    Hlf(String),
    Tpl(String),
    Inc(String),
    Jmp(isize),
    Jie(String, isize),
    Jio(String, isize),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let instruction = words.next().unwrap();
        let mut param1 = words.next().unwrap().to_owned();
        if param1.ends_with(',') {
            param1.pop();
        }
        let param2 = words.next();

        Ok(match instruction {
            "hlf" => Instruction::Hlf(param1),
            "tpl" => Instruction::Tpl(param1),
            "inc" => Instruction::Inc(param1),
            "jmp" => Instruction::Jmp(param1.parse::<isize>().unwrap()),
            "jie" => Instruction::Jie(param1, param2.unwrap().parse::<isize>().unwrap()),
            "jio" => Instruction::Jio(param1, param2.unwrap().parse::<isize>().unwrap()),
            _ => unreachable!(),
        })
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn run(program: &[Instruction], registers: &mut HashMap<String, isize>) {
    let mut i = 0isize;
    while let Some(instruction) = usize::try_from(i).ok().and_then(|i| program.get(i)) {
        match instruction {
            Instruction::Hlf(r) => {
                *registers.get_mut(r).unwrap() /= 2;
                i += 1;
            }
            Instruction::Tpl(r) => {
                *registers.get_mut(r).unwrap() *= 3;
                i += 1;
            }
            Instruction::Inc(r) => {
                *registers.get_mut(r).unwrap() += 1;
                i += 1;
            }
            Instruction::Jmp(o) => {
                i += o;
            }
            Instruction::Jie(r, o) => {
                if registers[r] % 2 == 0 {
                    i += o;
                } else {
                    i += 1;
                }
            }
            Instruction::Jio(r, o) => {
                if registers[r] == 1 {
                    i += o;
                } else {
                    i += 1;
                }
            }
        }
    }
}
