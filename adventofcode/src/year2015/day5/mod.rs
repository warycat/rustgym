use rustgym_util::*;
use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut res1 = 0;
    let mut res2 = 0;
    for line in it {
        if nice1(&line) {
            res1 += 1;
        }
        if nice2(&line) {
            res2 += 1;
        }
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn nice1(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let vowels = s
        .iter()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();
    let double = s.windows(2).any(|w| w[0] == w[1]);
    let blacklist = s
        .windows(2)
        .any(|w| matches!(w, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y']));

    vowels >= 3 && double && !blacklist
}

fn nice2(s: &str) -> bool {
    let mut hm: HashMap<String, Vec<usize>> = HashMap::new();
    let n = s.len();
    for i in 1..n {
        hm.entry(s[i - 1..=i].to_string()).or_default().push(i);
    }
    let s: Vec<char> = s.chars().collect();
    let rule1 = hm
        .values()
        .any(|v| v.len() >= 2 && v[0] + 1 < v[v.len() - 1]);
    let rule2 = s.windows(3).any(|w| w[0] == w[2]);
    rule1 && rule2
}

adventofcode!(test, "input.txt", "output.txt");
