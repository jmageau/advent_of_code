mod day_one;
mod day_two;

pub fn print_all_answers() {
    println!("2015");
    println!("1. {}", day_one::answers());
    println!("2. {}", day_two::answers());
    println!("");
}