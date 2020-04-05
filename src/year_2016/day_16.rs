aoc_day!(2016, 16);

fn answer_one() -> String {
    let initial = input().trim().to_owned();
    let disk_size = 272;

    let mut data = transform(initial).find(|t| t.len() >= disk_size).unwrap();
    data = data.chars().take(disk_size).collect();

    checksum(&data).to_string()
}

fn answer_two() -> String {
    let initial = input().trim().to_owned();
    let disk_size = 35651584;

    let mut data = transform(initial).find(|t| t.len() >= disk_size).unwrap();
    data = data.chars().take(disk_size).collect();

    checksum(&data).to_string()
}

fn transform(data: String) -> impl Iterator<Item = String> {
    std::iter::successors(Some(data), |a| {
        let b: String = a
            .chars()
            .rev()
            .map(|c| if c == '0' { '1' } else { '0' })
            .collect();
        Some(format!("{}0{}", a, b))
    })
}

fn checksum(data: &str) -> String {
    let chars: Vec<_> = data.chars().collect();
    let result: String = chars
        .chunks_exact(2)
        .map(|c| if c[0] == c[1] { '1' } else { '0' })
        .collect();
    if result.len() % 2 == 0 {
        checksum(&result)
    } else {
        result
    }
}
