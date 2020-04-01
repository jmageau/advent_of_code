pub fn answers() -> String {
    answer_one()
}

fn answer_one() -> String {
    let (row, col) = parse(&input());
    let n = row_col_to_n(row, col);
    codes(20151125).nth(n - 1).unwrap().to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_25").unwrap()
}

fn parse(input: &str) -> (usize, usize) {
    let mut words = input.split(' ');
    let mut row = words.nth(16).unwrap().to_owned();
    row.pop();
    let mut col = words.nth(1).unwrap().trim().to_owned();
    col.pop();
    (row.parse().unwrap(), col.parse().unwrap())
}

fn row_col_to_n(row: usize, col: usize) -> usize {
    let mut result = 1;
    for c in 2..=col {
        result += c;
    }
    for r in 2..=row {
        result += col + r - 2;
    }
    result
}

fn codes(first: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(first), |prev| Some((prev * 252533) % 33554393))
}
