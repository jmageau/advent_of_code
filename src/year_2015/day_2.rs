use std::io::prelude::*;
use std::fs::File;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();
    let lines = input.lines();

    let mut total_area = 0;
    for line in lines {
        let dimensions: Vec<u32> = line.split('x').map(|d| d.parse::<u32>().unwrap()).collect();
        let smallest_side_area = dimensions.iter().product::<u32>() / dimensions.iter().max().unwrap();
        let area = 2 * (dimensions[0] * dimensions[1] + dimensions[1] * dimensions[2] + dimensions[2] * dimensions[0]) + smallest_side_area;
        total_area += area;
    }
    total_area.to_string()
}

fn answer_two() -> String {
    let input = input();
    let lines = input.lines();

    let mut total_length = 0;
    for line in lines {
        let dimensions: Vec<u32> = line.split('x').map(|d| d.parse::<u32>().unwrap()).collect();
        let wrap_length = 2 * (dimensions.iter().sum::<u32>() - dimensions.iter().max().unwrap());
        let ribbon_length = dimensions.iter().product::<u32>();
        total_length += wrap_length + ribbon_length;
    }
    total_length.to_string()
}

fn input() -> String {
    let mut file = File::open("src/year_2015/input/input_day_2").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string
}
