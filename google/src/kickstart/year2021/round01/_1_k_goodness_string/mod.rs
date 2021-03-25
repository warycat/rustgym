use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(case_no: usize, reader: &mut impl BufRead, writer: &mut impl Write) {
    let args: Vec<usize> = reader.parse_vec();
    let n = args[0];
    let k = args[1] as i32;
    let s: Vec<char> = reader.collect_string().chars().collect();
    let mut g = 0;
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            g += 1;
        }
    }
    let res = (g - k).abs();
    writeln!(writer, "Case #{}: {}", case_no, res).unwrap();
}

google_test_gen!(test, "input.txt", "output.txt");
