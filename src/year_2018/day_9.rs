use std::collections::HashMap;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    run_game(432, 71019).to_string()
}

fn answer_two() -> String {
    run_game(432, 7101900).to_string()
}

fn run_game(player_count: usize, last_marble_value: usize) -> usize {
    let mut current_marble = 0;
    let mut marbles = HashMap::new();
    marbles.insert(0, (0, 0));
    let mut player_scores = HashMap::new();

    for (current_player, marble_value) in (1..=player_count).cycle().zip(1..=last_marble_value) {
        if marble_value % 23 == 0 {
            let new_current_marble = (0..7).fold(current_marble, |acc, _| marbles[&acc].0);
            let score = player_scores.entry(current_player).or_insert(0);
            *score += marble_value + new_current_marble;
            current_marble = marbles[&new_current_marble].1;
            remove_marble(new_current_marble, &mut marbles);
        } else {
            let left_marble = marbles[&current_marble].1;
            let right_marble = marbles[&marbles[&current_marble].1].1;
            insert_marble_between(marble_value, left_marble, right_marble, &mut marbles);
            current_marble = marble_value;
        }
    }

    *player_scores.values().max().unwrap()
}

fn insert_marble_between(
    value: usize,
    left: usize,
    right: usize,
    marbles: &mut HashMap<usize, (usize, usize)>,
) {
    marbles.get_mut(&left).unwrap().1 = value;
    marbles.get_mut(&right).unwrap().0 = value;
    marbles.insert(value, (left, right));
}

fn remove_marble(value: usize, marbles: &mut HashMap<usize, (usize, usize)>) {
    let (left_marble, right_marble) = marbles[&value];

    marbles.get_mut(&left_marble).unwrap().1 = right_marble;
    marbles.get_mut(&right_marble).unwrap().0 = left_marble;
    marbles.remove(&value);
}
