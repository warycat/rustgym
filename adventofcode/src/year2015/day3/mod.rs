use rustgym_util::*;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut hs1: HashSet<(i32, i32)> = HashSet::new();
    let mut hs2: HashSet<(i32, i32)> = HashSet::new();
    let mut xs: [i32; 2] = [0, 0];
    let mut ys: [i32; 2] = [0, 0];
    hs1.insert((0, 0));
    hs2.insert((0, 0));
    for (i, c) in line.char_indices() {
        let j = i % 2;
        match c {
            '>' => {
                x += 1;
                xs[j] += 1;
            }
            '<' => {
                x -= 1;
                xs[j] -= 1;
            }
            '^' => {
                y += 1;
                ys[j] += 1;
            }
            'v' => {
                y -= 1;
                ys[j] -= 1;
            }
            _ => {}
        }
        hs1.insert((x, y));
        hs2.insert((xs[j], ys[j]));
    }
    let res1 = hs1.len();
    let res2 = hs2.len();
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

adventofcode!(test, "input.txt", "output.txt");
