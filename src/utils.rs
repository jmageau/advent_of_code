#[macro_export]
macro_rules! aoc_year {
    ( $year:expr ) => {
        aoc_year!($year, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
    };
    ( $year:expr, $( $day:expr ),* ) => {
        paste::item! {
            mod [<year_ $year>] {
                $(
                    mod [<day_ $day>];
                )*

                pub fn print_day(day: u8) {
                    match day {
                        $( $day => println!("{}.{}: {}", $year, $day, [<day_ $day>]::answers()), )*
                        _ => eprintln!("{}.{}: No answers.", $year, day),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! aoc_day {
    ( $year:expr, $day:expr ) => {
        pub fn answers() -> String {
            format!("{}, {}", answer_one(), answer_two())
        }

        fn input() -> String {
            std::fs::read_to_string(format!("src/year_{}/input/input_day_{}", $year, $day)).unwrap()
        }
    };
}
