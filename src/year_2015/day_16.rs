pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    info(&input())
        .iter()
        .enumerate()
        .find(|(_, i)| {
            i.children.unwrap_or(3) == 3
                && i.cats.unwrap_or(7) == 7
                && i.samoyeds.unwrap_or(2) == 2
                && i.pomeranians.unwrap_or(3) == 3
                && i.akitas.unwrap_or(0) == 0
                && i.vizslas.unwrap_or(0) == 0
                && i.goldfish.unwrap_or(5) == 5
                && i.trees.unwrap_or(3) == 3
                && i.cars.unwrap_or(2) == 2
                && i.perfumes.unwrap_or(1) == 1
        })
        .map(|(i, _)| i + 1)
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    info(&input())
        .iter()
        .enumerate()
        .find(|(_, i)| {
            i.children.unwrap_or(3) == 3
                && i.cats.unwrap_or(8) > 7
                && i.samoyeds.unwrap_or(2) == 2
                && i.pomeranians.unwrap_or(2) < 3
                && i.akitas.unwrap_or(0) == 0
                && i.vizslas.unwrap_or(0) == 0
                && i.goldfish.unwrap_or(4) < 5
                && i.trees.unwrap_or(4) > 3
                && i.cars.unwrap_or(2) == 2
                && i.perfumes.unwrap_or(1) == 1
        })
        .map(|(i, _)| i + 1)
        .unwrap()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_16").unwrap()
}

#[derive(Debug)]
struct Info {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn info(input: &str) -> Vec<Info> {
    input
        .lines()
        .map(|l| {
            let words: Vec<_> = l.split(' ').collect();
            let f = |s| {
                words.iter().position(|&w| w == s).and_then(|i| {
                    let mut v = words[i + 1].to_owned();
                    if v.ends_with(',') {
                        v.pop();
                    }
                    v.parse().ok()
                })
            };

            Info {
                children: f("children:"),
                cats: f("cats:"),
                samoyeds: f("samoyeds:"),
                pomeranians: f("pomeranians:"),
                akitas: f("akitas:"),
                vizslas: f("vizslas:"),
                goldfish: f("goldfish:"),
                trees: f("trees:"),
                cars: f("cars:"),
                perfumes: f("perfumes:"),
            }
        })
        .collect()
}
