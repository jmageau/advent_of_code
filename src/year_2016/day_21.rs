use itertools::Itertools;
use std::str::FromStr;

aoc_day!(2016, 21);

fn answer_one() -> String {
    let operations = parse(&input());
    scramble("abcdefgh", &operations)
}

fn answer_two() -> String {
    let operations = parse(&input());
    let target = "fbgdceah";
    let chars: Vec<_> = target.chars().collect();

    chars
        .into_iter()
        .permutations(8)
        .map(|p| p.iter().collect::<String>())
        .find(|p| scramble(p, &operations) == target)
        .unwrap()
}

#[derive(Debug)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    Rotate(bool, usize),
    RotatePosition(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split(' ').collect();

        Ok(match words[0] {
            "swap" => match words[1] {
                "position" => {
                    Operation::SwapPosition(words[2].parse().unwrap(), words[5].parse().unwrap())
                }
                _ => Operation::SwapLetter(
                    words[2].chars().next().unwrap(),
                    words[5].chars().next().unwrap(),
                ),
            },
            "rotate" => match words[1] {
                "based" => Operation::RotatePosition(words[6].chars().next().unwrap()),
                _ => Operation::Rotate(words[1] == "left", words[2].parse().unwrap()),
            },
            "reverse" => Operation::Reverse(words[2].parse().unwrap(), words[4].parse().unwrap()),
            "move" => Operation::Move(words[2].parse().unwrap(), words[5].parse().unwrap()),
            _ => unreachable!(),
        })
    }
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn scramble(password: &str, operations: &[Operation]) -> String {
    let mut chars: Vec<_> = password.chars().collect();
    for o in operations {
        match o {
            Operation::SwapPosition(p1, p2) => {
                chars.swap(*p1, *p2);
            }
            Operation::SwapLetter(l1, l2) => {
                let index1 = chars.iter().position(|c| c == l1).unwrap();
                let index2 = chars.iter().position(|c| c == l2).unwrap();
                chars.swap(index1, index2);
            }
            Operation::Rotate(left, steps) => {
                if *left {
                    chars.rotate_left(*steps);
                } else {
                    chars.rotate_right(*steps);
                }
            }
            Operation::RotatePosition(ch) => {
                let index = chars.iter().position(|c| c == ch).unwrap();
                let count = 1 + index + if index >= 4 { 1 } else { 0 };
                let length = chars.len();
                chars.rotate_right(count % length);
            }
            Operation::Reverse(from, to) => {
                let sub = &mut chars[*from..=*to];
                sub.reverse();
            }
            Operation::Move(p1, p2) => {
                let c = chars.remove(*p1);
                chars.insert(*p2, c)
            }
        }
    }

    chars.iter().collect()
}
