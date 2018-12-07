use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = input();

    let grid = get_grid(&input);
    size_of_largest_area(&grid).to_string()
}

fn answer_two() -> String {
    let input = input();

    let coordinates = get_coordinates(&input);
    size_of_good_area(&coordinates, 10000).to_string()
}

fn get_coordinates(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(", ").map(|p| p.parse::<usize>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn get_grid(input: &str) -> Vec<Vec<Cell>> {
    let coordinates = get_coordinates(&input);

    let dimensions = coordinates.iter().fold((0, 0), |(acc_x, acc_y), &(x, y)| {
        (acc_x.max(x + 1), acc_y.max(y + 1))
    });

    let mut grid = vec![vec![Cell::Empty(None); dimensions.0]; dimensions.1];

    for (i, &(x, y)) in coordinates.iter().enumerate() {
        grid[y][x] = Cell::Coordinate(i);
    }

    grid.iter_mut().enumerate().for_each(|(r, row)| {
        row.iter_mut().enumerate().for_each(|(c, cell)| {
            if let Cell::Empty(_) = cell {
                let closest = closest_coordinate((c, r), &coordinates);
                *cell = Cell::Empty(closest);
            }
        })
    });

    grid
}

#[derive(Debug, Copy, Clone)]
enum Cell {
    Coordinate(usize),
    Empty(Option<usize>),
}

fn closest_coordinate(p: (usize, usize), coordinates: &[(usize, usize)]) -> Option<usize> {
    let mut closest_coordinate = None;
    let mut min_distance = std::usize::MAX;
    let mut multiple = false;

    for (i, &c) in coordinates.iter().enumerate() {
        let distance = distance(p, c);
        if distance < min_distance {
            closest_coordinate = Some(i);
            multiple = false;
            min_distance = distance;
        } else if distance == min_distance {
            multiple = true;
        }
    }

    closest_coordinate.filter(|_| !multiple)
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let x = if p1.0 > p2.0 {
        p1.0 - p2.0
    } else {
        p2.0 - p1.0
    };
    let y = if p1.1 > p2.1 {
        p1.1 - p2.1
    } else {
        p2.1 - p1.1
    };
    x + y
}

fn size_of_largest_area(grid: &[Vec<Cell>]) -> usize {
    let mut areas = HashMap::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if let Some(coordinate) = match cell {
                Cell::Coordinate(c) => Some(c),
                Cell::Empty(Some(c)) => Some(c),
                Cell::Empty(None) => None,
            } {
                let area = areas.entry(coordinate).or_insert(Some(0));
                if r == 0 || r == grid.len() - 1 || c == 0 || c == grid[0].len() - 1 {
                    *area = None;
                } else if let Some(a) = area {
                    *a += 1;
                }
            }
        }
    }

    areas.values().filter_map(|&a| a).max().unwrap()
}

fn size_of_good_area(coordinates: &[(usize, usize)], max_distance: usize) -> usize {
    let dimensions = coordinates.iter().fold((0, 0), |(acc_x, acc_y), &(x, y)| {
        (acc_x.max(x + 1), acc_y.max(y + 1))
    });

    (0..dimensions.1)
        .flat_map(|r| (0..dimensions.0).map(move |c| total_distance((r, c), &coordinates)))
        .filter(|&d| d < max_distance)
        .count()
}

fn total_distance(p: (usize, usize), coordinates: &[(usize, usize)]) -> usize {
    coordinates.iter().map(|&c| distance(p, c)).sum()
}

fn input() -> String {
    let mut file = File::open("src/year_2018/input/input_day_6").unwrap();
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string.trim().to_owned()
}
