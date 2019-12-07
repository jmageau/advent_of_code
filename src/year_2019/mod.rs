mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub fn print_answer(day: u8) {
    match day {
        1 => println!("2019.{}: {}", day, day_1::answers()),
        2 => println!("2019.{}: {}", day, day_2::answers()),
        3 => println!("2019.{}: {}", day, day_3::answers()),
        4 => println!("2019.{}: {}", day, day_4::answers()),
        5 => println!("2019.{}: {}", day, day_5::answers()),
        6 => println!("2019.{}: {}", day, day_6::answers()),
        _ => println!("2019.{}: No answers.", day),
    }
}
