use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

const STARTING_PROGRAMS: [char; 16] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
];

fn answer_one() -> String {
    let input = input();

    let moves = get_moves(&input);
    let mut programs = STARTING_PROGRAMS;

    dance(&moves, &mut programs);
    programs.iter().collect()
}

fn answer_two() -> String {
    let input = input();

    let moves = get_moves(&input);
    let mut programs = STARTING_PROGRAMS;

    let multiple = (1..)
        .find(|_| {
            dance(&moves, &mut programs);
            programs == STARTING_PROGRAMS
        })
        .unwrap();

    programs = STARTING_PROGRAMS;

    for _ in 0..(1_000_000_000 % multiple) {
        dance(&moves, &mut programs);
    }

    programs.iter().collect()
}

fn get_moves(input: &str) -> Vec<Move> {
    input
        .split(',')
        .map(|m| {
            let mut chars = m.chars();
            let move_type = chars.next().unwrap();
            let rest: String = chars.collect();
            match move_type {
                's' => Move::Spin(rest.parse::<usize>().unwrap()),
                'x' => {
                    let positions: Vec<_> = rest
                        .split('/')
                        .map(|p| p.parse::<usize>().unwrap())
                        .collect();
                    Move::Exchange(positions[0], positions[1])
                }
                'p' => {
                    let mut rest_chars = rest.chars();
                    let first_program_name = rest_chars.next().unwrap();
                    rest_chars.next();
                    let second_program_name = rest_chars.next().unwrap();
                    Move::Partner(first_program_name, second_program_name)
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn dance(moves: &[Move], programs: &mut [char; 16]) {
    for dance_move in moves {
        match dance_move {
            Move::Spin(n) => programs.rotate_right(*n),
            Move::Exchange(i, j) => programs.swap(*i, *j),
            Move::Partner(a, b) => {
                let i = programs.iter().position(|&p| p == *a).unwrap();
                let j = programs.iter().position(|&p| p == *b).unwrap();
                programs.swap(i, j)
            }
        }
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_16").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
