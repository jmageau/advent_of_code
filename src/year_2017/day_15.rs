pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let mut generator_a_value = 883u64;
    let mut generator_b_value = 879u64;

    let mut match_count = 0;

    for _ in 0..40_000_000 {
        generator_a_value = generator_a_value * 16807 % 2147483647;
        generator_b_value = generator_b_value * 48271 % 2147483647;

        if generator_a_value & 65535 == generator_b_value & 65535 {
            match_count += 1;
        }
    }

    match_count.to_string()
}

fn answer_two() -> String {
    let mut generator_a_value = 883u64;
    let mut generator_b_value = 879u64;

    let mut match_count = 0;

    for _ in 0..5_000_000 {
        loop {
            generator_a_value = generator_a_value * 16807 % 2147483647;
            if generator_a_value % 4 == 0 {
                break;
            }
        }
        loop {
            generator_b_value = generator_b_value * 48271 % 2147483647;
            if generator_b_value % 8 == 0 {
                break;
            }
        }

        if generator_a_value & 65535 == generator_b_value & 65535 {
            match_count += 1;
        }
    }

    match_count.to_string()
}
