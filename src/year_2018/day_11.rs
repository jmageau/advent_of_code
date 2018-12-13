pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = 9221;

    let cells = get_cells(input);
    format!("{:?}", get_largest_square(&cells, 3).0)
}

fn answer_two() -> String {
    let input = 9221;

    let cells = get_cells(input);
    format!("{:?}", get_largest_square_size(&cells))
}

fn get_cells(input: isize) -> Vec<isize> {
    let mut cells = vec![];

    for y in 1..=300 {
        for x in 1..=300 {
            let rack_id = x + 10;
            let power = (rack_id * y + input) * rack_id % 1000 / 100 - 5;
            cells.push(power);
        }
    }

    cells
}

fn get_largest_square(cells: &Vec<isize>, square_size: usize) -> ((usize, usize), isize) {
    let mut max_coordinate = (1, 1);
    let mut max = cells[(max_coordinate.1 - 1) * 300 + (max_coordinate.0 - 1)];

    for x in 1..=300 - square_size + 1 {
        for y in 1..=300 - square_size + 1 {
            let mut sum = 0;
            for d_x in 0..square_size {
                for d_y in 0..square_size {
                    sum += cells[(y + d_y - 1) * 300 + (x + d_x - 1)];
                }
            }
            if sum > max {
                max_coordinate = (x, y);
                max = sum;
            }
        }
    }
    (max_coordinate, max)
}

fn get_largest_square_size(cells: &Vec<isize>) -> (usize, usize, usize) {
    let mut max_coordinate = (1, 1);
    let mut max = cells[(max_coordinate.1 - 1) * 300 + (max_coordinate.0 - 1)];
    let mut max_size = 1;

    for square_size in 1..=300 {
        let (coordinate, sum) = get_largest_square(&cells, square_size);
        if sum > max {
            max_coordinate = coordinate;
            max = sum;
            max_size = square_size;
        }
    }

    (max_coordinate.0, max_coordinate.1, max_size)
}
