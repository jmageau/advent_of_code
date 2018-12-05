pub fn answers() -> String {
    format!("{}, {}", answer_one(), answer_two())
}

fn answer_one() -> String {
    let input = 371;
    let buffer = create_buffer(input, 2017);

    buffer[buffer.iter().position(|&v| v == 2017).unwrap() + 1].to_string()
}

fn answer_two() -> String {
    let input = 371;
    buffer_value_at_index(1, input, 50_000_000).to_string()
}

fn create_buffer(step: usize, size: usize) -> Vec<usize> {
    let mut buffer = vec![0];
    let mut position = 0;

    for i in 1..=size {
        position = (position + step) % buffer.len() + 1;
        buffer.insert(position, i);
    }

    buffer
}

fn buffer_value_at_index(index: usize, step: usize, size: usize) -> usize {
    let mut current_buffer_size = 1;
    let mut value_at_index = 0;
    let mut position = 0;

    for i in 1..=size {
        position = (position + step) % current_buffer_size + 1;
        current_buffer_size += 1;
        if position == index {
            value_at_index = i;
        }
    }

    value_at_index
}
