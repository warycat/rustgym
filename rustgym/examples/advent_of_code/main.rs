use rustgym::advent_of_code_2015;
use rustgym::advent_of_code_2020;
use std::fmt::Write;
use std::io::*;
use std::time::Instant;

macro_rules! run {
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

macro_rules! skip {
    ($file: expr, $year:expr, $day:expr, $writer: expr) => {
        paste::item! {
            let start = Instant::now();
            writeln!(&mut $writer, "\nday{}", $day).unwrap();
            writeln!(&mut $writer, "SKIPPED").unwrap();
            let duration = start.elapsed();
            writeln!($writer, "{:?}", duration).unwrap();
        }
    };
}

fn main() {
    let mut writer = "".to_string();

    writeln!(&mut writer, "\n[2015]").unwrap();

    run!("2015/day1.txt", 2015, 1, writer);
    skip!("2015/day2.txt", 2015, 2, writer);
    skip!("2015/day3.txt", 2015, 3, writer);
    skip!("2015/day4.txt", 2015, 4, writer);
    skip!("2015/day5.txt", 2015, 5, writer);
    skip!("2015/day6.txt", 2015, 6, writer);
    skip!("2015/day7.txt", 2015, 7, writer);
    skip!("2015/day8.txt", 2015, 8, writer);
    skip!("2015/day9.txt", 2015, 9, writer);
    skip!("2015/day10.txt", 2015, 10, writer);
    skip!("2015/day11.txt", 2015, 11, writer);
    run!("2015/day12.txt", 2015, 12, writer);

    writeln!(&mut writer, "\n[2020]").unwrap();

    run!("2020/day1.txt", 2020, 1, writer);
    run!("2020/day2.txt", 2020, 2, writer);
    run!("2020/day3.txt", 2020, 3, writer);
    run!("2020/day4.txt", 2020, 4, writer);
    run!("2020/day5.txt", 2020, 5, writer);
    run!("2020/day6.txt", 2020, 6, writer);
    run!("2020/day7.txt", 2020, 7, writer);

    println!("{}", writer);
}
