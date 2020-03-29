pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let target = input().parse::<usize>().unwrap();

    (1..)
        .find(|&h| divisors(h).sum::<usize>() * 10 >= target)
        .unwrap()
        .to_string()
}

fn answer_two() -> String {
    let target = input().parse::<usize>().unwrap();

    (1..)
        .find(|&h| divisors(h).filter(|f| h <= f * 50).sum::<usize>() * 11 >= target)
        .unwrap()
        .to_string()
}

fn input() -> String {
    std::fs::read_to_string("src/year_2015/input/input_day_20").unwrap()
}

fn divisors(num: usize) -> impl Iterator<Item = usize> {
    (1..)
        .take_while(move |n| n * n <= num)
        .filter(move |f| num % f == 0)
        .flat_map(move |f| {
            if f * f == num {
                vec![f]
            } else {
                vec![f, num / f]
            }
        })
}
