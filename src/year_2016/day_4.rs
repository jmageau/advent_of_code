extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use self::regex::Regex;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let real_rooms_count: u32 = real_rooms().iter().map(|r| r.1).sum();
    real_rooms_count.to_string()
}

fn answer_two() -> String {
    let real_rooms = real_rooms();

    for room in real_rooms {
        let decrypted_name = decrypt_name(room.0, room.1);
        if decrypted_name == "northpoleobjectstorage" {
            return room.1.to_string();
        }
    }
    unreachable!()
}

fn real_rooms() -> Vec<(String, u32)> {
    let mut rooms = vec![];

    let input = input();
    let lines = input.lines();
    let re = Regex::new(r"^([\w-]+)-(\d+)\[(\w+)\]$").unwrap();

    for line in lines {
        let captures = re.captures(line).unwrap();
        let name: String = captures.get(1).unwrap().as_str().chars().filter(|&c| c != '-').collect();
        let id = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let checksum = captures.get(3).unwrap();

        let mut char_counts = name.chars()
            .map(|c1| (c1, name.chars()
                .filter(|&c2| c2 == c1).count()))
            .collect::<Vec<_>>();
        char_counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        char_counts.dedup();

        let correct_checksum: String = char_counts.iter().map(|c| c.0).take(5).collect();

        if checksum.as_str() == correct_checksum {
            rooms.push((name, id));
        }
    }
    rooms
}

fn decrypt_name(encrypted_name: String, id: u32) -> String {
    let chars = encrypted_name.chars();
    let mut decrypted_chars = vec![];

    for char in chars {
        let char_code = char as u8 - 97;
        let amount_to_shift = (id % 26) as u8;
        let decrypted_char = (97 + (char_code + amount_to_shift) % 26) as char;
        decrypted_chars.push(decrypted_char);
    }
    decrypted_chars.into_iter().collect()
}


fn input() -> String {
    let mut file = File::open("src/year_2016/input/input_day_4").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
