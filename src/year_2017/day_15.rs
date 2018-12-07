const DIV: usize = 2_147_483_647;
const FACTOR_A: usize = 16807;
const FACTOR_B: usize = 48271;
const BIN_16: usize = 65535;

pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let mut generator_a_value = 883usize;
    let mut generator_b_value = 879usize;

    let mut match_count = 0;

    for _ in 0..40_000_000 {
        generator_a_value = generator_a_value * FACTOR_A % DIV;
        generator_b_value = generator_b_value * FACTOR_B % DIV;

        if generator_a_value & BIN_16 == generator_b_value & BIN_16 {
            match_count += 1;
        }
    }

    match_count.to_string()
}

fn answer_two() -> String {
    let mut generator_a_value = 883usize;
    let mut generator_b_value = 879usize;

    let mut match_count = 0;

    for _ in 0..5_000_000 {
        loop {
            generator_a_value = generator_a_value * FACTOR_A % DIV;
            if generator_a_value % 4 == 0 {
                break;
            }
        }
        loop {
            generator_b_value = generator_b_value * FACTOR_B % DIV;
            if generator_b_value % 8 == 0 {
                break;
            }
        }

        if generator_a_value & BIN_16 == generator_b_value & BIN_16 {
            match_count += 1;
        }
    }

    match_count.to_string()
}
