use std::collections::BTreeMap;

use parse_display::{Display, FromStr};

aoc_day!(2021, 5);

fn answer_one() -> String {
    let lines: Vec<Line> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut points = BTreeMap::new();

    for line in lines {
        if line.x1 == line.x2 {
            let (start, end) = min_max(line.y1, line.y2);
            for y in start..=end {
                *points.entry((line.x1, y)).or_insert(0) += 1;
            }
        } else if line.y1 == line.y2 {
            let (start, end) = min_max(line.x1, line.x2);
            for x in start..=end {
                *points.entry((x, line.y1)).or_insert(0) += 1;
            }
        }
    }

    points
        .into_iter()
        .filter(|&(_, v)| v >= 2)
        .count()
        .to_string()
}

fn answer_two() -> String {
    let lines: Vec<Line> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut points = BTreeMap::new();

    for line in lines {
        let dx = match line.x1.cmp(&line.x2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let dy = match line.y1.cmp(&line.y2) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };

        let mut point = (line.x1, line.y1);
        while point != (line.x2, line.y2) {
            *points.entry((point.0, point.1)).or_insert(0) += 1;
            point.0 += dx;
            point.1 += dy;
        }
        *points.entry((point.0, point.1)).or_insert(0) += 1;
    }

    points
        .into_iter()
        .filter(|&(_, v)| v >= 2)
        .count()
        .to_string()
}

#[derive(Display, FromStr, Debug)]
#[display("{x1},{y1} -> {x2},{y2}")]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
