pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    distance(false)
}

fn answer_two() -> String {
    distance(true)
}

fn distance(stop_at_visited: bool) -> String {
    let input = "R3, R1, R4, L4, R3, R1, R1, L3, L5, L5, L3, R1, R4, L2, L1, R3, L3, R2, R1, R1, L5, L2, L1, R2, L4, R1, L2, L4, R2, R2, L2, L4, L3, R1, R4, R3, L1, R1, L5, R4, L2, R185, L2, R4, R49, L3, L4, R5, R1, R1, L1, L1, R2, L1, L4, R4, R5, R4, L3, L5, R1, R71, L1, R1, R186, L5, L2, R5, R4, R1, L5, L2, R3, R2, R5, R5, R4, R1, R4, R2, L1, R4, L1, L4, L5, L4, R4, R5, R1, L2, L4, L1, L5, L3, L5, R2, L5, R4, L4, R3, R3, R1, R4, L1, L2, R2, L1, R4, R2, R2, R5, R2, R5, L1, R1, L4, R5, R4, R2, R4, L5, R3, R2, R5, R3, L3, L5, L4, L3, L2, L2, R3, R2, L1, L1, L5, R1, L3, R3, R4, R5, L3, L5, R1, L3, L5, L5, L2, R1, L3, L1, L3, R4, L1, R3, L2, L2, R3, R3, R4, R4, R1, L4, R1, L5";
    let instructions = input.split(", ").collect::<Vec<_>>();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut direction = 0;
    let mut position = (0i32, 0i32);
    let mut visited_positions = vec![position];

    for instruction in instructions {
        let mut chars = instruction.chars();
        let turn = chars.next().unwrap();
        let distance = chars.as_str().parse::<i32>().unwrap();

        if turn == 'R' {
            direction = (direction + 1) % 4;
        } else {
            direction = (direction + 3) % 4;
        }

        for _ in 0..distance {
            position.0 += directions[direction].0;
            position.1 += directions[direction].1;

            if stop_at_visited && visited_positions.contains(&position) {
                return (position.0.abs() + position.1.abs()).to_string();
            } else {
                visited_positions.push(position);
            }
        }
    }

        (position.0.abs() + position.1.abs()).to_string()
}
