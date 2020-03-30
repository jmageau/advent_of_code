pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let boss_stats = parse(&input());
    let mut min = usize::max_value();
    Spell::iter().for_each(|s| mana_cost(s, 50, 500, 0, &mut min, (0, 0, 0), boss_stats, false));
    min.to_string()
}

fn answer_two() -> String {
    let boss_stats = parse(&input());
    let mut min = usize::max_value();
    Spell::iter().for_each(|s| mana_cost(s, 50, 500, 0, &mut min, (0, 0, 0), boss_stats, true));
    min.to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_22").unwrap()
}

fn parse(input: &str) -> (usize, usize) {
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

    (hp, damage)
}

#[derive(Clone, Copy, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn iter() -> impl Iterator<Item = Spell> {
        vec![
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ]
        .into_iter()
    }

    fn cost(&self) -> usize {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

fn mana_cost(
    spell: Spell,
    mut hp: usize,
    mut current_mana: usize,
    mut spent_mana: usize,
    mut current_min_cost: &mut usize,
    (mut shield, mut poison, mut recharge): (usize, usize, usize),
    (mut boss_hp, boss_damage): (usize, usize),
    lose_health: bool,
) {
    if lose_health {
        hp -= 1;
        if hp == 0 {
            return;
        }
    }
    if shield > 0 {
        shield -= 1;
    }
    if poison > 0 {
        poison -= 1;
        boss_hp = boss_hp.saturating_sub(3);
        if boss_hp == 0 {
            *current_min_cost = spent_mana;
            return;
        }
    }
    if recharge > 0 {
        recharge -= 1;
        current_mana += 101;
    }

    if spent_mana + spell.cost() >= *current_min_cost {
        return;
    }

    if spell.cost() > current_mana {
        return;
    }

    current_mana -= spell.cost();
    spent_mana += spell.cost();

    match spell {
        Spell::MagicMissile => {
            boss_hp = boss_hp.saturating_sub(4);
        }
        Spell::Drain => {
            boss_hp = boss_hp.saturating_sub(2);
            hp += 2;
        }
        Spell::Shield => {
            shield = 6;
        }
        Spell::Poison => {
            poison = 6;
        }
        Spell::Recharge => {
            recharge = 5;
        }
    };

    if boss_hp == 0 {
        *current_min_cost = spent_mana;
        return;
    }

    let armor = if shield > 0 {
        shield -= 1;
        7
    } else {
        0
    };
    if poison > 0 {
        poison -= 1;
        boss_hp = boss_hp.saturating_sub(3);
        if boss_hp == 0 {
            *current_min_cost = spent_mana;
            return;
        }
    }
    if recharge > 0 {
        recharge -= 1;
        current_mana += 101;
    }

    let damage_to_player = std::cmp::max(1, boss_damage.saturating_sub(armor));
    hp = hp.saturating_sub(damage_to_player);
    if hp == 0 {
        return;
    }

    let is_valid = |s: &Spell| match s {
        Spell::MagicMissile => true,
        Spell::Drain => true,
        Spell::Shield => shield <= 1,
        Spell::Poison => poison <= 1,
        Spell::Recharge => recharge <= 1,
    };
    Spell::iter().filter(is_valid).for_each(|s| {
        mana_cost(
            s,
            hp,
            current_mana,
            spent_mana,
            &mut current_min_cost,
            (shield, poison, recharge),
            (boss_hp, boss_damage),
            lose_health,
        )
    });
}
