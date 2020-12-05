aoc_day!(2020, 5);

fn answer_one() -> String {
    input()
        .lines()
        .map(row_col)
        .map(seat_id)
        .max()
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let seats: Vec<_> = input().lines().map(row_col).collect();
    (1..127)
        .flat_map(|row| (0..8).map(move |col| (row, col)))
        .filter(|seat| !seats.contains(seat))
        .find(|seat| seats.contains(&(seat.0 + 1, seat.1)) && seats.contains(&(seat.0 - 1, seat.1)))
        .map(seat_id)
        .unwrap()
        .to_string()
}

fn seat_id((row, col): (u32, u32)) -> u32 {
    row * 8 + col
}

fn row_col(seat: &str) -> (u32, u32) {
    let row = binary(0, 127, &seat[0..7]);
    let col = binary(0, 7, &seat[7..10]);
    (row, col)
}

fn binary(min: u32, max: u32, operations: &str) -> u32 {
    if min == max {
        return min;
    }

    let (current, rest) = operations.split_at(1);

    match current {
        "F" | "L" => binary(min, (min + max) / 2, rest),
        "B" | "R" => binary((min + max + 1) / 2, max, rest),
        _ => unreachable!(),
    }
}
