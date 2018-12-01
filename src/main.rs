extern crate regex;

mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;

fn main() {
    year_2015::print_all_answers();
    year_2016::print_all_answers();
    year_2017::print_all_answers();
    year_2018::print_all_answers();
}
