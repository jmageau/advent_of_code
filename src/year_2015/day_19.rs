use std::collections::{HashMap, HashSet};

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let (replacements, molecule) = parse_input(&input());
    possibilities(&molecule, &replacements).len().to_string()
}

fn answer_two() -> String {
    let (_, molecule) = parse_input(&input());

    let count = molecule.chars().filter(|c| c.is_uppercase()).count();
    let rn_count = molecule.matches("Rn").count();
    let ar_count = molecule.matches("Ar").count();
    let y_count = molecule.matches("Y").count();

    (count - rn_count - ar_count - 2 * y_count - 1).to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_19").unwrap()
}

fn parse_input(input: &str) -> (HashMap<String, HashSet<String>>, String) {
    let lines: Vec<_> = input.lines().collect();
    let mut replacements = HashMap::new();
    lines.iter().take_while(|l| !l.is_empty()).for_each(|l| {
        let mut words = l.split(' ');
        let from = words.nth(0).unwrap().to_owned();
        let to = words.nth(1).unwrap().to_owned();
        replacements
            .entry(from.clone())
            .or_insert_with(HashSet::new);
        replacements.get_mut(&from).unwrap().insert(to);
    });
    let molecule = lines.last().unwrap().to_string();
    (replacements, molecule)
}

fn possibilities(
    molecule: &str,
    replacements: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    (0..molecule.len())
        .flat_map(|i| {
            replacements
                .into_iter()
                .filter(move |&(from, _)| molecule[i..].starts_with(from))
                .flat_map(move |(from, to)| {
                    to.iter()
                        .map(move |t| [&molecule[..i], t, &molecule[i + from.len()..]].concat())
                })
        })
        .collect()
}
