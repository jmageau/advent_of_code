use crypto::digest::Digest;
use crypto::md5::Md5;
use std::collections::HashMap;

aoc_day!(2016, 14);

fn answer_one() -> String {
    let salt = input().trim().to_owned();
    keys(salt, 1).nth(63).unwrap().0.to_string()
}

fn answer_two() -> String {
    let salt = input().trim().to_owned();
    keys(salt, 2017).nth(63).unwrap().0.to_string()
}

fn hash(index: usize, salt: String, times: usize) -> String {
    let mut md5 = Md5::new();
    let mut value = format!("{}{}", salt, index);
    for _ in 0..times {
        md5.reset();
        md5.input_str(&value);
        value = md5.result_str().to_owned();
    }
    value
}

fn keys(salt: String, times: usize) -> impl Iterator<Item = (usize, String)> {
    let mut hashes = HashMap::new();
    let mut get_hash = move |i| {
        hashes
            .entry(i)
            .or_insert_with(|| hash(i, salt.clone(), times.clone()))
            .clone()
    };

    let mut main_index = 0;
    std::iter::from_fn(move || {
        for index in main_index.. {
            let h = get_hash(index);
            if let Some(c) = triple(&h) {
                if (index + 1..index + 1001).any(|i| get_hash(i).contains(&c.repeat(5))) {
                    main_index = index + 1;
                    return Some((index, h));
                }
            }
        }
        None
    })
}

fn triple(hash: &str) -> Option<String> {
    let chars: Vec<_> = hash.chars().collect();
    chars
        .windows(3)
        .find(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| w[0].to_string())
}
