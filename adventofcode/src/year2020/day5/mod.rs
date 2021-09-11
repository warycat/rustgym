use rustgym_util::*;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut res1 = 0;
    let mut hs: HashSet<i32> = HashSet::new();
    for line in it {
        let mut id = 0;
        for c in line.chars() {
            let x = match c {
                'F' => 0,
                'B' => 1,
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            };
            id <<= 1;
            id |= x;
            hs.insert(id);
        }
        res1 = res1.max(id);
    }
    let res2 = middle(&hs);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn middle(hs: &HashSet<i32>) -> i32 {
    let max = 1 << 10;
    for i in 1..max {
        if !hs.contains(&i) && hs.contains(&(i + 1)) && hs.contains(&(i - 1)) {
            return i;
        }
    }
    0
}

adventofcode!(test, "input.txt", "output.txt");
