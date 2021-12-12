use parse_display::{Display, FromStr};

aoc_day!(2021, 2);

fn answer_one() -> String {
    let commands: Vec<Command> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command::Forward(x) => position += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }

    (position * depth).to_string()
}

fn answer_two() -> String {
    let commands: Vec<Command> = input().lines().map(|l| l.parse().unwrap()).collect();

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Forward(x) => {
                position += x;
                depth += aim * x;
            }
            Command::Down(x) => {
                aim += x;
            }
            Command::Up(x) => aim -= x,
        }
    }

    (position * depth).to_string()
}

#[derive(Display, FromStr, Debug)]
enum Command {
    #[display("forward {0}")]
    Forward(i32),
    #[display("down {0}")]
    Down(i32),
    #[display("up {0}")]
    Up(i32),
}
