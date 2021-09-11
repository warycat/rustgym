use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let mut s = it.next().unwrap();
    let mut res1 = 0;
    for i in 0..50 {
        s = look_and_say(s);
        if i == 39 {
            res1 = s.len();
        }
    }
    let res2 = s.len();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn look_and_say(s: String) -> String {
    let mut it = s.chars().peekable();
    let mut res = "".to_string();
    while let Some(c) = it.next() {
        let mut count = 1;
        while let Some(&next_c) = it.peek() {
            if next_c == c {
                it.next();
                count += 1;
            } else {
                break;
            }
        }
        res += &format!("{}{}", count, c);
    }
    res
}

adventofcode_ignore!(test, "input.txt", "output.txt");
