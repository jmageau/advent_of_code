mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// The year of the event to run.
    year: Option<u32>,

    /// The day to run.
    day: Option<u8>,
}

fn main() {
    let opt = Opt::from_args();

    let print_day = |year, day| match year {
        2015 => year_2015::print_answer(day),
        2016 => year_2016::print_answer(day),
        2017 => year_2017::print_answer(day),
        2018 => year_2018::print_answer(day),
        _ => println!("Invalid year: {}.", year),
    };

    let print_year = |year: u32| {
        (1..=31).for_each(|day| print_day(year, day));
    };

    if let Some(year) = opt.year {
        if let Some(day) = opt.day {
            print_day(year, day);
        } else {
            print_year(year);
        }
    } else {
        print_year(2015);
        print_year(2016);
        print_year(2017);
        print_year(2018);
    }
}
