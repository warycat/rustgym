use rustgym::advent_of_code_2020;
use std::fmt::Write;
use std::io::*;
fn main() {
    let mut writer = "".to_string();

    let mut reader = BufReader::new(include_str!("2020/day1.txt").as_bytes());
    writeln!(&mut writer, "\n[day1]").unwrap();
    advent_of_code_2020::day1::solve(&mut reader, &mut writer);

    let mut reader = BufReader::new(include_str!("2020/day2.txt").as_bytes());
    writeln!(&mut writer, "\n[day2]").unwrap();
    advent_of_code_2020::day2::solve(&mut reader, &mut writer);

    let mut reader = BufReader::new(include_str!("2020/day3.txt").as_bytes());
    writeln!(&mut writer, "\n[day3]").unwrap();
    advent_of_code_2020::day3::solve(&mut reader, &mut writer);

    let mut reader = BufReader::new(include_str!("2020/day4.txt").as_bytes());
    writeln!(&mut writer, "\n[day4]").unwrap();
    advent_of_code_2020::day4::solve(&mut reader, &mut writer);

    println!("{}", writer);
}
