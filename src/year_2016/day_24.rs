use std::collections::{BTreeSet, HashSet, VecDeque};

aoc_day!(2016, 24);

fn answer_one() -> String {
    let cells = parse(&input());
    min_steps(&cells, false).to_string()
}

fn answer_two() -> String {
    let cells = parse(&input());
    min_steps(&cells, true).to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cell {
    Wall,
    Empty,
    Val(usize),
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            c => Cell::Val(c.to_string().parse().unwrap()),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|l| l.chars().map(Cell::from_char).collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    start: Point,
    current: Point,
    seen: BTreeSet<usize>,
}

impl State {
    fn new(cells: &[Vec<Cell>]) -> State {
        let start = cells
            .iter()
            .enumerate()
            .filter_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, cell)| {
                        if let Cell::Val(v) = cell {
                            *v == 0
                        } else {
                            false
                        }
                    })
                    .map(|(c, _)| Point::new(c as isize, r as isize))
            })
            .next()
            .unwrap();

        State {
            start,
            current: start,
            seen: std::iter::once(0).collect(),
        }
    }

    fn next_states(&self, cells: &[Vec<Cell>]) -> Vec<State> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dx, dy)| Point::new(self.current.x + dx, self.current.y + dy))
            .filter(|p| cells[p.y as usize][p.x as usize] != Cell::Wall)
            .map(|new_current| {
                let mut new_state = self.clone();
                new_state.current = new_current;
                if let Cell::Val(v) = cells[new_current.y as usize][new_current.x as usize] {
                    new_state.seen.insert(v);
                }
                new_state
            })
            .collect()
    }
}

fn min_steps(cells: &[Vec<Cell>], return_to_zero: bool) -> usize {
    let max_val = cells
        .iter()
        .map(|r| {
            r.iter()
                .map(|c| if let Cell::Val(v) = c { *v } else { 0 })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    let mut queue = VecDeque::new();
    queue.push_back((0, State::new(cells)));

    let mut seen_states = HashSet::new();
    seen_states.insert(State::new(cells));

    while let Some((steps, state)) = queue.pop_front() {
        for new_state in state.next_states(cells) {
            if seen_states.contains(&new_state) {
                continue;
            }
            if new_state.seen.len() == max_val + 1 {
                if !return_to_zero || new_state.current == new_state.start {
                    return steps + 1;
                }
            }
            queue.push_back((steps + 1, new_state.clone()));
            seen_states.insert(new_state);
        }
    }

    unreachable!()
}
