use std::collections::BTreeMap;

aoc_day!(2016, 19);

fn answer_one() -> String {
    let count = input().trim().parse().unwrap();
    simulate_next(count, false).to_string()
}

fn answer_two() -> String {
    let count = input().trim().parse().unwrap();
    simulate_next(count, true).to_string()
}

fn simulate_next(count: usize, across: bool) -> usize {
    let mut elves: BTreeMap<_, _> = (1..=count).map(|e| (e, 1)).collect();

    let mut current = 1;
    let mut next = if !across {
        2
    } else {
        current + elves.len() / 2
    };
    while elves.len() > 1 {
        *elves.get_mut(&current).unwrap() += elves[&next];
        let new_next = *elves
            .range(next..)
            .chain(elves.range(..next))
            .cycle()
            .nth(if !across {
                2
            } else {
                if elves.len() % 2 == 0 {
                    1
                } else {
                    2
                }
            })
            .unwrap()
            .0;
        elves.remove(&next);
        next = new_next;
        current = *elves
            .range(current..)
            .chain(elves.range(..current))
            .cycle()
            .nth(1)
            .unwrap()
            .0;
    }

    *elves.keys().next().unwrap()
}
