use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let memory: Vec<_> = input()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    execute(1, &memory).last().unwrap().to_string()
}

fn answer_two() -> String {
    let memory: Vec<_> = input()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    execute(5, &memory).last().unwrap().to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_5").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}

fn execute(input_value: isize, memory: &[isize]) -> Vec<isize> {
    let mut memory = memory.to_vec();

    let mut i = 0;
    let mut outputs = vec![];
    while i < memory.len() {
        let (opcode, parameters_modes) = split_instruction(memory[i]);
        match opcode {
            1 => {
                let first_value = get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                );
                let second_value = get_value(
                    memory[i + 2],
                    *parameters_modes.get(1).unwrap_or(&0),
                    &memory,
                );
                let destination = memory[i + 3];
                memory[destination as usize] = first_value + second_value;
                i += 4;
            }
            2 => {
                let first_value = get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                );
                let second_value = get_value(
                    memory[i + 2],
                    *parameters_modes.get(1).unwrap_or(&0),
                    &memory,
                );
                let destination = memory[i + 3];
                memory[destination as usize] = first_value * second_value;
                i += 4;
            }
            3 => {
                let address = memory[i + 1];
                memory[address as usize] = input_value;
                i += 2;
            }
            4 => {
                let output = get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                );
                outputs.push(output);
                i += 2;
            }
            5 => {
                if get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                ) != 0
                {
                    i = get_value(
                        memory[i + 2],
                        *parameters_modes.get(1).unwrap_or(&0),
                        &memory,
                    ) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                if get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                ) == 0
                {
                    i = get_value(
                        memory[i + 2],
                        *parameters_modes.get(1).unwrap_or(&0),
                        &memory,
                    ) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let first = get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                );
                let second = get_value(
                    memory[i + 2],
                    *parameters_modes.get(1).unwrap_or(&0),
                    &memory,
                );
                let destination = memory[i + 3];
                memory[destination as usize] = if first < second { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let first = get_value(
                    memory[i + 1],
                    *parameters_modes.get(0).unwrap_or(&0),
                    &memory,
                );
                let second = get_value(
                    memory[i + 2],
                    *parameters_modes.get(1).unwrap_or(&0),
                    &memory,
                );
                let destination = memory[i + 3];
                memory[destination as usize] = if first == second { 1 } else { 0 };
                i += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }

    outputs
}

fn split_instruction(instruction: isize) -> (isize, Vec<isize>) {
    let opcode = instruction % 100;
    let mut parameter_modes = digits(instruction / 100);
    parameter_modes.reverse();

    (opcode, parameter_modes)
}

fn get_value(v: isize, parameter_mode: isize, memory: &[isize]) -> isize {
    match parameter_mode {
        0 => memory[v as usize],
        1 => v,
        _ => unreachable!(),
    }
}

fn digits(n: isize) -> Vec<isize> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}
