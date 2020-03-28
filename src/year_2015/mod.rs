mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
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
        12 => println!("2015.{}: {}", day, day_12::answers()),
        13 => println!("2015.{}: {}", day, day_13::answers()),
        14 => println!("2015.{}: {}", day, day_14::answers()),
        15 => println!("2015.{}: {}", day, day_15::answers()),
        16 => println!("2015.{}: {}", day, day_16::answers()),
        17 => println!("2015.{}: {}", day, day_17::answers()),
        18 => println!("2015.{}: {}", day, day_18::answers()),
        19 => println!("2015.{}: {}", day, day_19::answers()),
        _ => println!("2015.{}: No answers.", day),
    }
}
