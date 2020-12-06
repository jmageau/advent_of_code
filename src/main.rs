#[macro_use]
mod utils;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// The year of the event to run.
    year: Option<u32>,

    /// The day to run.
    day: Option<u8>,
}

aoc_year!(2015);
aoc_year!(2016);
aoc_year!(2017, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);
aoc_year!(2018, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
aoc_year!(2019, 1, 2, 3, 4, 5, 6);
aoc_year!(2020, 1, 2, 3, 4, 5, 6);

fn main() {
    let opt = Opt::from_args();

    let print_day = |year, day| match year {
        2015 => year_2015::print_day(day),
        2016 => year_2016::print_day(day),
        2017 => year_2017::print_day(day),
        2018 => year_2018::print_day(day),
        2019 => year_2019::print_day(day),
        2020 => year_2020::print_day(day),
        _ => eprintln!("Invalid year: {}.", year),
    };

    let print_year = |year| {
        (1..=25).for_each(|day| print_day(year, day));
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
        print_year(2019);
        print_year(2020);
    }
}
