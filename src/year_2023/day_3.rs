aoc_day!(2023, 3);

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn adjacent(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Debug)]
struct Number {
    value: u32,
    coordinates: Vec<Point>,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coordinate: Point,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl std::str::FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: Vec<Number> = Vec::new();
        let mut symbols = Vec::new();

        for (r_n, row) in s.lines().enumerate() {
            let mut prev_digit = false;
            for (c_n, c) in row.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    if prev_digit {
                        let last_number = numbers.last_mut().unwrap();
                        last_number.value = last_number.value * 10 + d;
                        last_number
                            .coordinates
                            .push(Point::new(c_n as i32, r_n as i32));
                    } else {
                        numbers.push(Number {
                            value: d,
                            coordinates: vec![Point::new(c_n as i32, r_n as i32)],
                        });
                    }
                } else if c != '.' {
                    symbols.push(Symbol {
                        value: c,
                        coordinate: Point::new(c_n as i32, r_n as i32),
                    });
                }
                prev_digit = c.is_digit(10);
            }
        }

        Ok(Schematic { numbers, symbols })
    }
}

impl Schematic {
    fn part_numbers(&self) -> impl Iterator<Item = &u32> {
        self.numbers
            .iter()
            .filter(|n| {
                self.symbols
                    .iter()
                    .any(|s| n.coordinates.iter().any(|c| s.coordinate.adjacent(c)))
            })
            .map(|n| &n.value)
    }

    fn gears_ratios(&self) -> impl Iterator<Item = u32> + '_ {
        self.symbols
            .iter()
            .filter(|s| s.value == '*')
            .filter_map(|g| {
                let numbers: Vec<_> = self
                    .numbers
                    .iter()
                    .filter(|n| n.coordinates.iter().any(|c| g.coordinate.adjacent(c)))
                    .map(|n| n.value)
                    .collect();
                if numbers.len() == 2 {
                    Some(numbers.iter().product())
                } else {
                    None
                }
            })
    }
}

fn answer_one() -> String {
    let schematic = input().parse::<Schematic>().unwrap();

    schematic.part_numbers().sum::<u32>().to_string()
}

fn answer_two() -> String {
    let schematic = input().parse::<Schematic>().unwrap();

    schematic.gears_ratios().sum::<u32>().to_string()
}
