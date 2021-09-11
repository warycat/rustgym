use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    let res1: i32 = line.chars().map(|c| if c == '(' { 1 } else { -1 }).sum();
    let mut floor = 0;
    let mut res2 = 0;
    for (i, c) in line.char_indices() {
        res2 = i + 1;
        floor += if c == '(' { 1 } else { -1 };
        if floor == -1 {
            break;
        }
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
