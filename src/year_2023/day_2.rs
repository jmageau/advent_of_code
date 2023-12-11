use parse_display::FromStr;

aoc_day!(2023, 2);

#[derive(FromStr, Debug)]
#[display("Game {id}: {sets}")]
struct Game {
    id: usize,
    sets: CubeSets,
}

#[derive(Debug)]
struct CubeSets(Vec<Vec<CubeSet>>);

impl std::str::FromStr for CubeSets {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sets = s
            .split(';')
            .map(|s| s.split(',').map(|c| c.trim().parse().unwrap()).collect())
            .collect();

        Ok(CubeSets(sets))
    }
}

#[derive(FromStr, Debug)]
#[display("{count} {color}")]
struct CubeSet {
    color: String,
    count: usize,
}

impl Game {
    fn min_cube(&self, color: &str) -> usize {
        self.sets
            .0
            .iter()
            .map(|s| {
                s.iter()
                    .filter_map(|s| Some(s.count).filter(|_| s.color == color))
            })
            .flatten()
            .max()
            .unwrap()
    }
}

fn answer_one() -> String {
    input()
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| {
            g.min_cube("red") <= 12 && g.min_cube("green") <= 13 && g.min_cube("blue") <= 14
        })
        .map(|g| g.id)
        .sum::<usize>()
        .to_string()
}

fn answer_two() -> String {
    input()
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .map(|g| g.min_cube("red") * g.min_cube("green") * g.min_cube("blue"))
        .sum::<usize>()
        .to_string()
}
