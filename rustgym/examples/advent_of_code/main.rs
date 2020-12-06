use rustgym::advent_of_code_2015;
use rustgym::advent_of_code_2020;
use std::fmt::Write;
use std::io::*;
use std::time::Instant;

macro_rules! runner {
    ($file: expr, $year:expr, $day:expr, $writer: expr) => {
        paste::item! {
            let start = Instant::now();
            let mut reader = BufReader::new(include_str!($file).as_bytes());
            writeln!(&mut $writer, "\nday{}", $day).unwrap();
            [< advent_of_code_ $year >]::[< day $day >]::solve(&mut reader, &mut $writer);
            let duration = start.elapsed();
            writeln!($writer, "{:?}", duration).unwrap();
        }
    };
}
fn main() {
    let mut writer = "".to_string();

    writeln!(&mut writer, "\n[2015]").unwrap();

    runner!("2015/day1.txt", 2015, 1, writer);
    runner!("2015/day2.txt", 2015, 2, writer);
    runner!("2015/day3.txt", 2015, 3, writer);
    // runner!("2015/day4.txt", 2015, 4, writer);
    runner!("2015/day5.txt", 2015, 5, writer);
    runner!("2015/day6.txt", 2015, 6, writer);
    runner!("2015/day7.txt", 2015, 7, writer);

    writeln!(&mut writer, "\n[2020]").unwrap();

    runner!("2020/day1.txt", 2020, 1, writer);
    runner!("2020/day2.txt", 2020, 2, writer);
    runner!("2020/day3.txt", 2020, 3, writer);
    runner!("2020/day4.txt", 2020, 4, writer);
    runner!("2020/day5.txt", 2020, 5, writer);
    runner!("2020/day6.txt", 2020, 6, writer);

    println!("{}", writer);
}
