use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap()).peekable();
    let mut res1 = 0;
    let mut res2 = 0;
    while it.peek().is_some() {
        let mut set: HashSet<char> = HashSet::new();
        let mut count: HashMap<char, usize> = HashMap::new();
        let mut k: usize = 0;
        while let Some(line) = it.next() {
            if line.is_empty() {
                break;
            } else {
                k += 1;
                for c in line.chars() {
                    set.insert(c);
                    *count.entry(c).or_default() += 1;
                }
            }
        }
        res1 += set.len();
        res2 += count.values().filter(|&&v| v == k).count();
    }
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
