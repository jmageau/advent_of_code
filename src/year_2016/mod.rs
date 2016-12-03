mod day_1;
mod day_2;
mod day_3;

pub fn print_all_answers() {
    println!("2016");
    println!("1. {}", day_1::answers());
    println!("2. {}", day_2::answers());
    println!("3. {}", day_3::answers());
    println!("");
}