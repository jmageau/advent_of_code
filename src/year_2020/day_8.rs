use std::collections::HashSet;
use std::str::FromStr;

aoc_day!(2020, 8);

fn answer_one() -> String {
    let instructions: Vec<_> = input().lines().map(|l| l.parse().unwrap()).collect();
    run(&instructions).0.to_string()
}

fn answer_two() -> String {
    let instructions: Vec<_> = input().lines().map(|l| l.parse().unwrap()).collect();

    for i in 0..instructions.len() {
        let mut new_instructions = instructions.clone();
        match instructions[i] {
            Instruction::Acc(_) => {}
            Instruction::Jmp(offset) => new_instructions[i] = Instruction::Nop(offset),
            Instruction::Nop(n) => new_instructions[i] = Instruction::Jmp(n),
        }
        let (acc, r) = run(&new_instructions);
        if r {
            return acc.to_string();
        }
    }

    unreachable!()
}

#[derive(Clone, Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        match parts[0] {
            "acc" => Ok(Instruction::Acc(parts[1].parse().unwrap())),
            "jmp" => Ok(Instruction::Jmp(parts[1].parse().unwrap())),
            "nop" => Ok(Instruction::Nop(parts[1].parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

fn run(instructions: &[Instruction]) -> (isize, bool) {
    let mut seen_instructions = HashSet::new();
    let mut acc = 0;

    let mut i = 0isize;
    while i < instructions.len() as isize {
        if seen_instructions.contains(&i) {
            return (acc, false);
        }
        seen_instructions.insert(i);

        match instructions[i as usize] {
            Instruction::Acc(amount) => {
                acc += amount;
                i += 1;
            }
            Instruction::Jmp(offset) => {
                i += offset;
            }
            Instruction::Nop(_) => i += 1,
        }
    }

    (acc, true)
}
