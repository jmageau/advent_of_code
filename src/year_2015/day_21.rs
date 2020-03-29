use itertools::Itertools;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let boss_stats = parse(&input());
    let (weapons, armors, rings) = get_items();

    let mut min_cost = None;
    for w in weapons.iter() {
        for armor_count in 0..=1 {
            for armors in armors.iter().combinations(armor_count) {
                for ring_count in 0..=2 {
                    for rings in rings.iter().combinations(ring_count) {
                        let cost = w.cost
                            + armors.iter().map(|a| a.cost).sum::<usize>()
                            + rings.iter().map(|r| r.cost).sum::<usize>();
                        let damage = w.damage
                            + armors.iter().map(|a| a.damage).sum::<usize>()
                            + rings.iter().map(|r| r.damage).sum::<usize>();
                        let armor = w.armor
                            + armors.iter().map(|a| a.armor).sum::<usize>()
                            + rings.iter().map(|r| r.armor).sum::<usize>();

                        if wins_fight(damage, armor, boss_stats) {
                            min_cost =
                                Some(std::cmp::min(min_cost.unwrap_or(usize::max_value()), cost));
                        }
                    }
                }
            }
        }
    }

    min_cost.unwrap().to_string()
}

fn answer_two() -> String {
    let boss_stats = parse(&input());
    let (weapons, armors, rings) = get_items();

    let mut max_cost = None;
    for w in weapons.iter() {
        for armor_count in 0..=1 {
            for armors in armors.iter().combinations(armor_count) {
                for ring_count in 0..=2 {
                    for rings in rings.iter().combinations(ring_count) {
                        let cost = w.cost
                            + armors.iter().map(|a| a.cost).sum::<usize>()
                            + rings.iter().map(|r| r.cost).sum::<usize>();
                        let damage = w.damage
                            + armors.iter().map(|a| a.damage).sum::<usize>()
                            + rings.iter().map(|r| r.damage).sum::<usize>();
                        let armor = w.armor
                            + armors.iter().map(|a| a.armor).sum::<usize>()
                            + rings.iter().map(|r| r.armor).sum::<usize>();

                        if !wins_fight(damage, armor, boss_stats) {
                            max_cost =
                                Some(std::cmp::max(max_cost.unwrap_or(usize::min_value()), cost));
                        }
                    }
                }
            }
        }
    }

    max_cost.unwrap().to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_21").unwrap()
}

fn parse(input: &str) -> (usize, usize, usize) {
    let mut lines = input.lines();
    let hp = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let damage = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let armor = lines
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    (hp, damage, armor)
}

#[derive(Debug)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

fn get_items() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let weapons = vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];
    let armors = vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];
    let rings = vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    (weapons, armors, rings)
}

fn wins_fight(
    damage: usize,
    armor: usize,
    (mut boss_hp, boss_damage, boss_armor): (usize, usize, usize),
) -> bool {
    let mut hp = 100usize;

    loop {
        let damage_to_boss = std::cmp::max(1, damage.saturating_sub(boss_armor));
        boss_hp = boss_hp.saturating_sub(damage_to_boss);
        if boss_hp == 0 {
            return true;
        }

        let damage_to_player = std::cmp::max(1, boss_damage.saturating_sub(armor));
        hp = hp.saturating_sub(damage_to_player);
        if hp == 0 {
            return false;
        }
    }
}
