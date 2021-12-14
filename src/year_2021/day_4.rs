use std::str::FromStr;

aoc_day!(2021, 4);

fn answer_one() -> String {
    let (numbers, mut boards) = parse_boards(&input());

    for number in numbers {
        for board in &mut boards {
            board.mark_number(number);
            if board.is_winner() {
                return board.score(number).to_string();
            }
        }
    }

    unreachable!()
}

fn answer_two() -> String {
    let (numbers, mut boards) = parse_boards(&input());

    for number in numbers {
        for board in &mut boards {
            board.mark_number(number);
        }
        if boards.len() == 1 && boards[0].is_winner() {
            return boards[0].score(number).to_string();
        }
        boards.retain(|b| !b.is_winner());
    }

    unreachable!()
}

fn parse_boards(input: &str) -> (Vec<i32>, Vec<Board>) {
    let mut parts = input.split("\n\n");

    let numbers = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = parts.map(|p| p.parse().unwrap()).collect();

    (numbers, boards)
}

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<(i32, bool)>>,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| (n.parse().unwrap(), false))
                    .collect()
            })
            .collect();

        Ok(Board { numbers })
    }
}

impl Board {
    fn mark_number(&mut self, number: i32) {
        for row in self.numbers.iter_mut() {
            for (n, m) in row.iter_mut() {
                if *n == number {
                    *m = true;
                }
            }
        }
    }

    fn score(&self, last_number: i32) -> i32 {
        self.numbers
            .iter()
            .flat_map(|l| l.iter().filter(|(_, m)| !m).map(|(n, _)| n))
            .sum::<i32>()
            * last_number
    }

    fn is_winner(&self) -> bool {
        let row_wins = (0..5).any(|row| self.numbers[row].iter().all(|&(_, m)| m));
        let col_wins = (0..5).any(|col| (0..5).all(|row| self.numbers[row][col].1));

        row_wins || col_wins
    }
}
