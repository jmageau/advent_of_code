mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub fn print_answer(day: u8) {
    match day {
        1 => println!("2015.{}: {}", day, day_1::answers()),
        2 => println!("2015.{}: {}", day, day_2::answers()),
        3 => println!("2015.{}: {}", day, day_3::answers()),
        4 => println!("2015.{}: {}", day, day_4::answers()),
        5 => println!("2015.{}: {}", day, day_5::answers()),
        6 => println!("2015.{}: {}", day, day_6::answers()),
        7 => println!("2015.{}: {}", day, day_7::answers()),
        8 => println!("2015.{}: {}", day, day_8::answers()),
        9 => println!("2015.{}: {}", day, day_9::answers()),
        10 => println!("2015.{}: {}", day, day_10::answers()),
        11 => println!("2015.{}: {}", day, day_11::answers()),
        _ => println!("2015.{}: No answers.", day),
    }
}
