use std::collections::HashMap;

aoc_day!(2016, 10);

fn answer_one() -> String {
    let (instructions, mut values) = parse(&input());
    let answer1 = run(&instructions, &mut values);
    answer1.to_string()
}

fn answer_two() -> String {
    let (instructions, mut values) = parse(&input());
    run(&instructions, &mut values);
    (values[&Target::Output(Output(0))].first().unwrap()
        * values[&Target::Output(Output(1))].first().unwrap()
        * values[&Target::Output(Output(2))].first().unwrap())
    .to_string()
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Bot(usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Output(usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Value(usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Target {
    Bot(Bot),
    Output(Output),
}

fn parse(input: &str) -> (HashMap<Bot, (Target, Target)>, HashMap<Target, Vec<usize>>) {
    let mut instructions = HashMap::new();
    let mut values = HashMap::new();

    for line in input.lines() {
        let mut words = line.split(' ');
        if line.starts_with("value") {
            let v = words.nth(1).unwrap().parse::<usize>().unwrap();
            let b = words.last().unwrap().parse::<usize>().unwrap();
            let val = values.entry(Target::Bot(Bot(b))).or_insert_with(Vec::new);
            val.push(v);
            val.sort();
        } else {
            let b = words.nth(1).unwrap().parse::<usize>().unwrap();
            let t1 = words.nth(3).unwrap();
            let l = words.nth(0).unwrap().parse::<usize>().unwrap();
            let l = if t1 == "output" {
                Target::Output(Output(l))
            } else {
                Target::Bot(Bot(l))
            };
            let t2 = words.nth(3).unwrap();
            let h = words.nth(0).unwrap().parse::<usize>().unwrap();
            let h = if t2 == "output" {
                Target::Output(Output(h))
            } else {
                Target::Bot(Bot(h))
            };

            instructions.insert(Bot(b), (l, h));
        }
    }

    (instructions, values)
}

fn run(
    instructions: &HashMap<Bot, (Target, Target)>,
    values: &mut HashMap<Target, Vec<usize>>,
) -> usize {
    let mut answer1 = None;

    while let Some((target, chips)) = values.iter().find(|(_, c)| c.len() == 2) {
        let target = target.clone();
        let chips = chips.clone();
        if let Target::Bot(b) = target {
            let (low_target, high_target) = instructions[&b];
            let low = values.entry(low_target).or_insert_with(Vec::new);
            low.push(chips[0]);
            low.sort();
            let high = values.entry(high_target).or_insert_with(Vec::new);
            high.push(chips[1]);
            high.sort();
            if chips[0] == 17 && chips[1] == 61 {
                answer1 = Some(b.0);
            }
            values.get_mut(&target).unwrap().clear();
        }
    }

    answer1.unwrap()
}
