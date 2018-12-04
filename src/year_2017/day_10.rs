use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    let lengths: Vec<_> = input.split(",").map(|n| n.parse::<u8>().unwrap()).collect();
    let mut list: Vec<_> = (0..=255).collect();

    let mut current_position = 0;
    let mut skip_size = 0;

    for length in lengths {
        reverse_elements(&mut list, current_position, length);
        current_position = current_position
            .wrapping_add(length)
            .wrapping_add(skip_size);
        skip_size = skip_size.wrapping_add(1);
    }

    (list[0] as usize * list[1] as usize).to_string()
}

fn answer_two() -> String {
    let input = input();

    let mut lengths = input.into_bytes();
    lengths.extend(&[17, 31, 73, 47, 23]);
    let mut list: Vec<_> = (0..=255).collect();

    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for length in lengths.iter() {
            reverse_elements(&mut list, current_position, *length);
            current_position = current_position
                .wrapping_add(*length)
                .wrapping_add(skip_size);
            skip_size = skip_size.wrapping_add(1);
        }
    }

    list.chunks(16)
        .map(|c| c.iter().skip(1).fold(c[0], |acc, x| acc ^ x))
        .map(|c| format!("{:02x}", c))
        .collect()
}

fn reverse_elements(list: &mut Vec<u8>, starting_index: u8, length: u8) {
    for i in 0..length / 2 {
        let first_index = starting_index.wrapping_add(i);
        let second_index = starting_index.wrapping_add(length - 1).wrapping_sub(i);

        let temp = list[first_index as usize];
        list[first_index as usize] = list[second_index as usize];
        list[second_index as usize] = temp;
    }
}

fn input() -> String {
    let mut file = File::open("src/year_2017/input/input_day_10").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
