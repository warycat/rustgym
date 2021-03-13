use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    writeln!(writer, "Hello, World.")?;
    let line = reader.lines().next().unwrap()?;
    write!(writer, "{}", line)?;
    Ok(())
}

test_gen!(test00, "input00.txt", "output00.txt");
test_gen!(test01, "input01.txt", "output01.txt");
