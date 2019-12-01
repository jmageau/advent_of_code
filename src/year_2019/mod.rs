mod day_1;

pub fn print_answer(day: u8) {
    match day {
        1 => println!("2019.{}: {}", day, day_1::answers()),
        _ => println!("2019.{}: No answers.", day),
    }
}
