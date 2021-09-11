use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    let res1: String = next_pass(&line);
    let res2: String = next_pass(&res1);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn next_pass(password: &str) -> String {
    let val = decode(password);
    let mut x = val;
    loop {
        x += 1;
        let s = encode(x);
        if rule1(&s) && rule2(&s) && rule3(&s) {
            break;
        }
    }
    encode(x).into_iter().map(|b| b as char).collect()
}

fn decode(s: &str) -> i64 {
    let mut res = 0;
    for b in s.bytes() {
        res *= 26;
        res += (b - b'a') as i64;
    }
    res
}

fn encode(mut x: i64) -> Vec<u8> {
    let mut s: Vec<u8> = vec![];
    for _ in 0..8 {
        let c = b'a' + (x % 26) as u8;
        s.push(c);
        x /= 26;
    }
    s.into_iter().rev().collect()
}

fn rule1(s: &[u8]) -> bool {
    s.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn rule2(s: &[u8]) -> bool {
    !s.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn rule3(s: &[u8]) -> bool {
    let mut pairs: Vec<usize> = vec![];
    for i in 1..8 {
        if s[i - 1] == s[i] {
            pairs.push(i);
        }
    }
    pairs.len() > 1 && pairs.last().unwrap() - pairs.first().unwrap() > 1
}

adventofcode_ignore!(test, "input.txt", "output.txt");
