use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let memory: Vec<_> = input()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    execute(&memory, 12, 2).to_string()
}

fn answer_two() -> String {
    let memory: Vec<_> = input()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..99 {
            if execute(&memory, noun, verb) == 19690720 {
                return (100 * noun + verb).to_string();
            }
        }
    }

    unreachable!()
}

fn input() -> String {
    let mut file = File::open("src/year_2019/input/input_day_2").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}

fn execute(memory: &[usize], noun: usize, verb: usize) -> usize {
    let mut memory = memory.to_vec();
    memory[1] = noun;
    memory[2] = verb;

    for i in (0..memory.len()).step_by(4) {
        if memory[i] == 99 {
            break;
        }

        let operation = memory[i];
        let first_address = memory[i + 1];
        let second_address = memory[i + 2];
        let destination = memory[i + 3];

        match operation {
            1 => memory[destination] = memory[first_address] + memory[second_address],
            2 => memory[destination] = memory[first_address] * memory[second_address],
            _ => unreachable!(),
        }
    }

    memory[0]
}
