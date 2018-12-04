use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    get_squares(&input)
        .iter()
        .map(|r| r.iter().filter(|&&c| c).count())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let input = input();

    let squares = get_squares(&input);

    let mut groups = HashMap::new();
    let mut current_group_id = 1;

    for (row, cols) in squares.iter().enumerate() {
        for (col, s) in cols.iter().enumerate() {
            if !s || groups.contains_key(&(row, col)) {
                continue;
            }
            add_to_group(&squares, (row, col), current_group_id, &mut groups);
            current_group_id += 1;
        }
    }

    (current_group_id - 1).to_string()
}

fn add_to_group(
    squares: &Vec<Vec<bool>>,
    position: (usize, usize),
    group_id: usize,
    mut groups: &mut HashMap<(usize, usize), usize>,
) {
    if !squares[position.0][position.1] || groups.contains_key(&position) {
        return;
    }

    groups.insert(position, group_id);

    if position.0 > 0 {
        add_to_group(
            &squares,
            (position.0 - 1, position.1),
            group_id,
            &mut groups,
        )
    }
    if position.0 < 127 {
        add_to_group(
            &squares,
            (position.0 + 1, position.1),
            group_id,
            &mut groups,
        )
    }
    if position.1 > 0 {
        add_to_group(
            &squares,
            (position.0, position.1 - 1),
            group_id,
            &mut groups,
        )
    }
    if position.1 < 127 {
        add_to_group(
            &squares,
            (position.0, position.1 + 1),
            group_id,
            &mut groups,
        )
    }
}

fn get_squares(input: &str) -> Vec<Vec<bool>> {
    (0..128)
        .map(|i| {
            let hash = knot_hash(&format!("{}-{}", input, i));
            hash.chars()
                .flat_map(|c| {
                    let n = u32::from_str_radix(&c.to_string(), 16).unwrap();
                    format!("{:04b}", n)
                        .chars()
                        .map(|c| c == '1')
                        .collect::<Vec<_>>()
                }).collect()
        }).collect()
}

fn knot_hash(input: &str) -> String {
    let mut lengths = input.to_owned().into_bytes();
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
    let mut file = File::open("src/year_2017/input/input_day_14").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
