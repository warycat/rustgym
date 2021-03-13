use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) -> RustGymResult {
    let mut it = reader.lines().map(|l| l.unwrap());
    let t = it.next().unwrap().parse::<usize>()?;
    for i in 1..=t {
        let n = it.next().unwrap().parse::<usize>()?;
        let mut max_line = "".to_string();
        let mut res = 0;
        for _ in 0..n {
            let line = it.next().unwrap();
            if line > max_line {
                max_line = line;
            } else {
                res += 1;
            }
        }
        writeln!(writer, "Case #{}: {}", i, res)?;
    }
    Ok(())
}

test_gen!(test, "input.txt", "output.txt");
