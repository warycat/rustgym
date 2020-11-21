use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    writeln!(writer, "Hello, World.").unwrap();
    let line = reader.lines().next().unwrap().unwrap();
    write!(writer, "{}", line).unwrap();
}

hackerrank!(test00, "input00.txt", "output00.txt");
hackerrank!(test01, "input01.txt", "output01.txt");
