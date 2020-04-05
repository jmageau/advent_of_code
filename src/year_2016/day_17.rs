use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::VecDeque;

aoc_day!(2016, 17);

fn answer_one() -> String {
    let passcode = input().trim().to_owned();

    shortest_path(Point::new(0, 0), Point::new(3, 3), &passcode)
}

fn answer_two() -> String {
    let passcode = input().trim().to_owned();

    longest_path(Point::new(0, 0), Point::new(3, 3), &passcode)
        .len()
        .to_string()
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

fn shortest_path(starting: Point, target: Point, passcode: &str) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((starting, String::new()));

    while let Some((current, path)) = queue.pop_front() {
        if current == target {
            return path;
        }

        for (next, letter) in next_rooms(current, passcode, &path) {
            queue.push_back((next, format!("{}{}", path, letter)));
        }
    }

    unreachable!()
}

fn longest_path(starting: Point, target: Point, passcode: &str) -> String {
    let mut queue = VecDeque::new();
    queue.push_back((starting, String::new()));

    let mut longest_path = String::new();
    while let Some((current, path)) = queue.pop_front() {
        if current == target {
            if path.len() > longest_path.len() {
                longest_path = path;
            }
            continue;
        }

        for (next, letter) in next_rooms(current, passcode, &path) {
            queue.push_back((next, format!("{}{}", path, letter)));
        }
    }

    longest_path
}

fn next_rooms(current: Point, passcode: &str, path: &str) -> Vec<(Point, char)> {
    let mut md5 = Md5::new();
    md5.input_str(&format!("{}{}", passcode, path));
    let hash_chars: Vec<_> = md5.result_str().chars().collect();

    let mut rooms = vec![];
    let open_chars = "bcdef";

    if open_chars.contains(hash_chars[0]) && current.y > 0 {
        rooms.push((Point::new(current.x, current.y - 1), 'U'));
    }
    if open_chars.contains(hash_chars[1]) && current.y < 3 {
        rooms.push((Point::new(current.x, current.y + 1), 'D'));
    }
    if open_chars.contains(hash_chars[2]) && current.x > 0 {
        rooms.push((Point::new(current.x - 1, current.y), 'L'));
    }
    if open_chars.contains(hash_chars[3]) && current.x < 3 {
        rooms.push((Point::new(current.x + 1, current.y), 'R'));
    }

    rooms
}
