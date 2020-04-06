use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

aoc_day!(2016, 22);

fn answer_one() -> String {
    let disk_usage = parse(&input());

    disk_usage
        .iter()
        .permutations(2)
        .filter(|c| c[0].1.used != 0 && c[0].1.used <= c[1].1.avail())
        .count()
        .to_string()
}

fn answer_two() -> String {
    let disk_usage = parse(&input());

    let max_x = disk_usage.keys().map(|p| p.x).max().unwrap();
    let empty_node = disk_usage.iter().find(|(_, n)| n.used == 0).unwrap().0;
    let next_to_goal_node = Point::new(max_x - 1, 0);
    let wall_edge = disk_usage
        .iter()
        .find(|(p, n)| {
            p.x < max_x
                && n.size <= disk_usage[empty_node].size
                && disk_usage[&Point::new(p.x + 1, p.y)].size > disk_usage[empty_node].size
        })
        .unwrap()
        .0;

    let steps = empty_node.distance(wall_edge)
        + wall_edge.distance(&next_to_goal_node)
        + (max_x - 1) * 5
        + 1;
    steps.to_string()
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

    fn distance(&self, other: &Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Node {
    size: usize,
    used: usize,
}

impl Node {
    fn new(size: usize, used: usize) -> Node {
        Node { size, used }
    }

    fn avail(&self) -> usize {
        self.size - self.used
    }
}

fn parse(input: &str) -> HashMap<Point, Node> {
    let line_regex =
        Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$").unwrap();

    input
        .lines()
        .skip(2)
        .map(|l| {
            let captures = line_regex.captures(l).unwrap();
            let x = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let y = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let size = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let used = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();

            (Point::new(x, y), Node::new(size, used))
        })
        .collect()
}
