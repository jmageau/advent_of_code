aoc_day!(2020, 11);

fn answer_one() -> String {
    let mut seats: Vec<Vec<_>> = input().lines().map(|l| l.chars().collect()).collect();

    while simulate(&mut seats) {}

    seats
        .iter()
        .map(|row| row.iter().filter(|&&s| s == '#').count())
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    let mut seats: Vec<Vec<_>> = input().lines().map(|l| l.chars().collect()).collect();

    while simulate2(&mut seats) {}

    seats
        .iter()
        .map(|row| row.iter().filter(|&&s| s == '#').count())
        .sum::<usize>()
        .to_string()
}

fn simulate(seats: &mut Vec<Vec<char>>) -> bool {
    let mut updated = false;

    let old_seats = seats.clone();

    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            let adjacent_indexes = get_adjacent_indexes(row, col, seats.len(), seats[0].len());
            if old_seats[row][col] == 'L'
                && adjacent_indexes
                    .iter()
                    .all(|&(r, c)| old_seats[r][c] == 'L' || old_seats[r][c] == '.')
            {
                seats[row][col] = '#';
                updated = true;
            } else if old_seats[row][col] == '#'
                && adjacent_indexes
                    .iter()
                    .filter(|&&(r, c)| old_seats[r][c] == '#')
                    .count()
                    >= 4
            {
                seats[row][col] = 'L';
                updated = true;
            }
        }
    }

    updated
}

fn get_adjacent_indexes(
    row: usize,
    col: usize,
    height: usize,
    width: usize,
) -> Vec<(usize, usize)> {
    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    deltas
        .iter()
        .map(|(x, y)| (row as isize + x, col as isize + y))
        .filter(|&(r, c)| r >= 0 && r < height as isize && c >= 0 && c < width as isize)
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn simulate2(seats: &mut Vec<Vec<char>>) -> bool {
    let mut updated = false;

    let old_seats = seats.clone();

    for row in 0..seats.len() {
        for col in 0..seats[0].len() {
            let adjacent_seats = get_adjacent_seats(row, col, seats);
            if old_seats[row][col] == 'L'
                && adjacent_seats
                    .iter()
                    .all(|&(r, c)| old_seats[r][c] == 'L' || old_seats[r][c] == '.')
            {
                seats[row][col] = '#';
                updated = true;
            } else if old_seats[row][col] == '#'
                && adjacent_seats
                    .iter()
                    .filter(|&&(r, c)| old_seats[r][c] == '#')
                    .count()
                    >= 5
            {
                seats[row][col] = 'L';
                updated = true;
            }
        }
    }

    updated
}

fn get_adjacent_seats(row: usize, col: usize, seats: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let row = row as isize;
    let col = col as isize;
    let height = seats.len() as isize;
    let width = seats[0].len() as isize;
    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    deltas
        .iter()
        .map(|(x, y)| {
            (1..)
                .map(|i| (row + x * i, col + y * i))
                .find(|&(r, c)| {
                    r < 0
                        || r >= height
                        || c < 0
                        || c >= width
                        || seats[r as usize][c as usize] != '.'
                })
                .unwrap()
        })
        .filter(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width)
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}
