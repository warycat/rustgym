use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = reader.parse_line();
    let mut max_line = "".to_string();
    let mut res = 0;
    for _ in 0..n {
        let line = reader.collect_string();
        if line > max_line {
            max_line = line;
        } else {
            res += 1;
        }
    }
    writeln!(writer, "Case #{}: {}", case_no, res).unwrap();
}

google_test_gen!(test, "input.txt", "output.txt");
