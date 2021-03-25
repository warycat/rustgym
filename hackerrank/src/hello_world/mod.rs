use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut impl RustGymRead, writer: &mut impl Write) {
    writeln!(writer, "Hello, World.").unwrap();
    let line = reader.collect_string();
    write!(writer, "{}", line).unwrap();
}

test_gen!(test00, "input00.txt", "output00.txt");
test_gen!(test01, "input01.txt", "output01.txt");
