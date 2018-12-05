use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut registers = HashMap::new();

    let mut current_instruction_index = 0;
    let mut last_sound = 0;

    // Wrapping add will keep index positive
    while current_instruction_index < instructions.len() {
        match &instructions[current_instruction_index] {
            Instruction::Snd(v) => last_sound = get_current_value(v, &registers),
            Instruction::Set(r, v) => {
                let v = get_current_value(v, &registers);
                let e = registers.entry(r.to_owned()).or_insert(0);
                *e = v;
            }
            Instruction::Add(r, v) => {
                let v = get_current_value(v, &registers);
                let e = registers.entry(r.to_owned()).or_insert(0);
                *e += v;
            }
            Instruction::Mul(r, v) => {
                let v = get_current_value(v, &registers);
                let e = registers.entry(r.to_owned()).or_insert(0);
                *e *= v;
            }
            Instruction::Mod(r, v) => {
                let v = get_current_value(v, &registers);
                let e = registers.entry(r.to_owned()).or_insert(0);
                *e %= v;
            }
            Instruction::Rcv(v) => {
                let v = registers.get(v).unwrap();
                if *v != 0 {
                    break;
                }
            }
            Instruction::Jgz(v1, v2) => {
                let v1 = get_current_value(v1, &registers);
                let v2 = get_current_value(v2, &registers);

                if v1 > 0 {
                    current_instruction_index = current_instruction_index
                        .wrapping_add(v2 as usize)
                        .wrapping_sub(1);
                }
            }
        }

        current_instruction_index = current_instruction_index.wrapping_add(1);
    }

    last_sound.to_string()
}

fn answer_two() -> String {
    let input = input();

    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut program_0_registers = HashMap::new();
    let mut program_1_registers = HashMap::new();
    program_1_registers.insert("p".to_owned(), 1);

    let mut i0 = 0;
    let mut i1 = 0;

    let mut program_0_buffer = VecDeque::new();
    let mut program_1_buffer = VecDeque::new();

    let mut program_1_send_count = 0;

    // Wrapping add will keep index positive
    while i0 < instructions.len() || i1 < instructions.len() {
        let instruction0 = instructions.get(i0);
        let instruction1 = instructions.get(i1);

        if program_0_buffer.is_empty() && program_1_buffer.is_empty() {
            if let Some(Instruction::Rcv(_)) = instruction0 {
                if let Some(Instruction::Rcv(_)) = instruction1 {
                    break;
                }
            }
        }

        if let Some(instruction) = instruction0 {
            i0 = run_instruction(
                instruction,
                &mut program_0_registers,
                &mut program_0_buffer,
                &mut program_1_buffer,
                i0,
            );
        }
        if let Some(instruction) = instruction1 {
            if let Instruction::Snd(_) = instruction {
                program_1_send_count += 1;
            }

            i1 = run_instruction(
                instruction,
                &mut program_1_registers,
                &mut program_1_buffer,
                &mut program_0_buffer,
                i1,
            );
        }
    }

    program_1_send_count.to_string()
}

fn run_instruction(
    instruction: &Instruction,
    registers: &mut HashMap<String, isize>,
    buffer: &mut VecDeque<isize>,
    other_buffer: &mut VecDeque<isize>,
    mut current_instruction_index: usize,
) -> usize {
    match instruction {
        Instruction::Snd(v) => {
            let v = get_current_value(v, &registers);
            buffer.push_back(v);
        }
        Instruction::Set(r, v) => {
            let v = get_current_value(v, &registers);
            let e = registers.entry(r.to_owned()).or_insert(0);
            *e = v;
        }
        Instruction::Add(r, v) => {
            let v = get_current_value(v, &registers);
            let e = registers.entry(r.to_owned()).or_insert(0);
            *e += v;
        }
        Instruction::Mul(r, v) => {
            let v = get_current_value(v, &registers);
            let e = registers.entry(r.to_owned()).or_insert(0);
            *e *= v;
        }
        Instruction::Mod(r, v) => {
            let v = get_current_value(v, &registers);
            let e = registers.entry(r.to_owned()).or_insert(0);
            *e %= v;
        }
        Instruction::Rcv(r) => {
            if let Some(received_value) = other_buffer.pop_front() {
                let e = registers.entry(r.to_owned()).or_insert(0);
                *e = received_value;
            } else {
                return current_instruction_index;
            }
        }
        Instruction::Jgz(v1, v2) => {
            let v1 = get_current_value(v1, &registers);
            let v2 = get_current_value(v2, &registers);

            if v1 > 0 {
                current_instruction_index = current_instruction_index
                    .wrapping_add(v2 as usize)
                    .wrapping_sub(1);
            }
        }
    }

    current_instruction_index.wrapping_add(1)
}

#[derive(Debug)]
enum Instruction {
    Snd(Value),
    Set(String, Value),
    Add(String, Value),
    Mul(String, Value),
    Mod(String, Value),
    Rcv(String),
    Jgz(Value, Value),
}

#[derive(Debug)]
enum Value {
    Register(String),
    Number(isize),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<isize>() {
            Ok(Value::Number(n))
        } else {
            Ok(Value::Register(s.to_owned()))
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let instruction = parts.next().unwrap();
        let first_value = parts.next().unwrap();
        let second_value = parts.next();

        match instruction {
            "snd" => Ok(Instruction::Snd(first_value.parse().unwrap())),
            "set" => Ok(Instruction::Set(
                first_value.to_owned(),
                second_value.unwrap().parse().unwrap(),
            )),
            "add" => Ok(Instruction::Add(
                first_value.to_owned(),
                second_value.unwrap().parse().unwrap(),
            )),
            "mul" => Ok(Instruction::Mul(
                first_value.to_owned(),
                second_value.unwrap().parse().unwrap(),
            )),
            "mod" => Ok(Instruction::Mod(
                first_value.to_owned(),
                second_value.unwrap().parse().unwrap(),
            )),
            "rcv" => Ok(Instruction::Rcv(first_value.parse().unwrap())),
            "jgz" => Ok(Instruction::Jgz(
                first_value.parse().unwrap(),
                second_value.unwrap().parse().unwrap(),
            )),
            _ => unreachable!(),
        }
    }
}

fn get_current_value(value: &Value, registers: &HashMap<String, isize>) -> isize {
    match value {
        Value::Register(r) => *registers.get(r).unwrap(),
        Value::Number(n) => *n,
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_18").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
