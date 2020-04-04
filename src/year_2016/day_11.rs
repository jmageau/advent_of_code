use itertools::Itertools;
use std::collections::{BTreeSet, HashSet, VecDeque};

aoc_day!(2016, 11);

fn answer_one() -> String {
    let floors = parse(&input());

    minimum_steps(State {
        elevator: 0,
        floors,
    })
    .to_string()
}

fn answer_two() -> String {
    let mut floors = parse(&input());
    floors[0].insert(Item::Generator(Generator("elerium".to_owned())));
    floors[0].insert(Item::Microchip(Microchip("elerium".to_owned())));
    floors[0].insert(Item::Generator(Generator("dilithium ".to_owned())));
    floors[0].insert(Item::Microchip(Microchip("dilithium ".to_owned())));

    minimum_steps(State {
        elevator: 0,
        floors,
    })
    .to_string()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Generator(String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Microchip(String);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Item {
    Generator(Generator),
    Microchip(Microchip),
}

fn parse(input: &str) -> Vec<BTreeSet<Item>> {
    input
        .lines()
        .map(|l| {
            let mut items = BTreeSet::new();
            let words: Vec<_> = l.split(' ').collect();
            for i in 0..words.len() {
                let mut word = words[i].to_owned();
                if word.ends_with(',') || word.ends_with('.') {
                    word.pop();
                }
                if word == "generator" {
                    items.insert(Item::Generator(Generator(words[i - 1].to_owned())));
                } else if word == "microchip" {
                    let chip = words[i - 1].split('-').next().unwrap().to_owned();
                    items.insert(Item::Microchip(Microchip(chip)));
                }
            }
            items
        })
        .collect()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct State {
    elevator: usize,
    floors: Vec<BTreeSet<Item>>,
}

fn minimum_steps(state: State) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, state.clone()));

    let mut seen_states = HashSet::new();
    seen_states.insert(state);

    loop {
        let (steps, state) = queue.pop_front().unwrap();

        for c in 1..=state.floors[state.elevator].len().min(2) {
            for options in state.floors[state.elevator].iter().combinations(c) {
                if state.elevator < 3 {
                    let mut new_state = state.clone();
                    new_state.elevator += 1;
                    for &o in options.iter() {
                        new_state.floors[state.elevator].remove(o);
                        new_state.floors[state.elevator + 1].insert(o.clone());
                    }
                    if !seen_states.contains(&new_state) && is_valid(&new_state) {
                        if is_done(&new_state) {
                            return steps + 1;
                        }
                        queue.push_back((steps + 1, new_state.clone()));
                        seen_states.insert(new_state);
                    }
                }
                if state.elevator > 0 {
                    let mut new_state = state.clone();
                    new_state.elevator -= 1;
                    for &o in options.iter() {
                        new_state.floors[state.elevator].remove(o);
                        new_state.floors[state.elevator - 1].insert(o.clone());
                    }
                    if !seen_states.contains(&new_state) && is_valid(&new_state) {
                        if is_done(&new_state) {
                            return steps + 1;
                        }
                        queue.push_back((steps + 1, new_state.clone()));
                        seen_states.insert(new_state);
                    }
                }
            }
        }
    }
}

fn is_valid(state: &State) -> bool {
    state.floors.iter().all(|f| {
        for i in f {
            if let Item::Microchip(Microchip(m)) = i {
                if f.contains(&Item::Generator(Generator(m.clone()))) {
                    continue;
                }
                if f.iter().any(|i2| matches!(i2, Item::Generator(_))) {
                    return false;
                }
            }
        }
        true
    })
}

fn is_done(state: &State) -> bool {
    state.floors.iter().rev().skip(1).all(BTreeSet::is_empty)
}
