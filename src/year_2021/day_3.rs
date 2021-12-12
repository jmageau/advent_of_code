use itertools::Itertools;

aoc_day!(2021, 3);

fn answer_one() -> String {
    let numbers: Vec<Vec<u32>> = input()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let (gamma, epsilon) = gamma_epsilon(&numbers);

    let gamma = digits_to_num(&gamma);
    let epsilon = digits_to_num(&epsilon);

    (gamma * epsilon).to_string()
}

fn answer_two() -> String {
    let numbers: Vec<Vec<u32>> = input()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut oxygen = numbers.clone();
    let mut co2 = numbers;

    for i in 0.. {
        let (gamma, _) = gamma_epsilon(&oxygen);
        let (_, epsilon) = gamma_epsilon(&co2);

        if oxygen.len() > 1 {
            oxygen.retain(|o| o[i] == gamma[i]);
        }
        if co2.len() > 1 {
            co2.retain(|c| c[i] == epsilon[i]);
        }

        if oxygen.len() == 1 && co2.len() == 1 {
            break;
        }
    }

    let oxygen = digits_to_num(&oxygen[0]);
    let co2 = digits_to_num(&co2[0]);

    (oxygen * co2).to_string()
}

fn gamma_epsilon(numbers: &[Vec<u32>]) -> (Vec<u32>, Vec<u32>) {
    let sums = numbers.iter().skip(1).fold(numbers[0].clone(), |acc, n| {
        acc.into_iter().zip(n).map(|(a, b)| a + b).collect()
    });

    let gamma: Vec<u32> = sums
        .iter()
        .map(|&d| d as usize * 2 >= numbers.len())
        .map(|d| if d { 1 } else { 0 })
        .collect();
    let epsilon = gamma.iter().map(|d| 1 - d).collect();

    (gamma, epsilon)
}

fn digits_to_num(digits: &[u32]) -> u32 {
    u32::from_str_radix(&digits.iter().join(""), 2).unwrap()
}
