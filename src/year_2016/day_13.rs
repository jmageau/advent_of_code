use std::collections::{HashSet, VecDeque};

aoc_day!(2016, 13);

fn answer_one() -> String {
    let number = input().trim().parse().unwrap();
    min_steps(Point::new(1, 1), Point::new(31, 39), number)
        .0
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let number = input().trim().parse().unwrap();
    min_steps(Point::new(1, 1), Point::new(31, 39), number)
        .1
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

fn min_steps(from: Point, to: Point, number: isize) -> (Option<usize>, usize) {
    let mut visited = HashSet::new();
    let mut visited_under_50 = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, from));

    while let Some((steps, current)) = queue.pop_front() {
        for neighbour in free_neighbours(current, number) {
            if visited.contains(&neighbour) {
                continue;
            }
            if neighbour == to {
                return (Some(steps + 1), visited_under_50.len());
            }
            queue.push_back((steps + 1, neighbour));
            visited.insert(neighbour);
            if steps + 1 <= 50 {
                visited_under_50.insert(neighbour);
            }
        }
    }

    (None, 0)
}

fn free_neighbours(point: Point, number: isize) -> impl Iterator<Item = Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(move |(dx, dy)| Point::new(point.x + dx, point.y + dy))
        .filter(|p| p.x >= 0 && p.y >= 0)
        .filter(move |&p| is_open(p, number))
}

fn is_open(point: Point, number: isize) -> bool {
    (point.x * point.x + 3 * point.x + 2 * point.x * point.y + point.y + point.y * point.y + number)
        .count_ones()
        % 2
        == 0
}
