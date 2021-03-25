use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

fn solve(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = reader.collect_string();
    let n: usize = reader.parse_line();
    let res = repeated_string(s, n);
    writeln!(writer, "{}", res).unwrap();
}

fn repeated_string(s: String, n: usize) -> usize {
    let m = s.len();
    let all = s.chars().filter(|&c| c == 'a').count();
    let k = n % m;
    let part = s.chars().take(k).filter(|&c| c == 'a').count();
    all * (n / m) + part
}

test_gen!(test00, "input00.txt", "output00.txt");
test_gen!(test01, "input01.txt", "output01.txt");
