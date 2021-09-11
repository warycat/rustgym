use rustgym_util::*;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let mut active1: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut active2: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for (i, line) in it.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active1.insert((i as i32, j as i32, 0));
                active2.insert((i as i32, j as i32, 0, 0));
            }
        }
    }
    let res1 = cycle1(&active1, 6);
    let res2 = cycle2(&active2, 6);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn cycle1(active: &HashSet<(i32, i32, i32)>, round: usize) -> usize {
    let mut cur = active.clone();
    for _ in 0..round {
        cur = conway1(cur);
    }
    cur.len()
}

fn cycle2(active: &HashSet<(i32, i32, i32, i32)>, round: usize) -> usize {
    let mut cur = active.clone();
    for _ in 0..round {
        cur = conway2(cur);
    }
    cur.len()
}

fn conway1(active: HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut region = HashSet::new();
    let mut res: HashSet<(i32, i32, i32)> = HashSet::new();
    for a in &active {
        for b in adj1(a) {
            region.insert(b);
        }
    }
    for b in region {
        let mut count = 0;
        for c in adj1(&b) {
            if active.contains(&c) {
                count += 1;
            }
        }
        if active.contains(&b) {
            if count == 3 || count == 4 {
                res.insert(b);
            }
        } else {
            if count == 3 {
                res.insert(b);
            }
        }
    }
    res
}

fn conway2(active: HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut region = HashSet::new();
    let mut res: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for a in &active {
        for b in adj2(a) {
            region.insert(b);
        }
    }
    for b in region {
        let mut count = 0;
        for c in adj2(&b) {
            if active.contains(&c) {
                count += 1;
            }
        }
        if active.contains(&b) {
            if count == 3 || count == 4 {
                res.insert(b);
            }
        } else {
            if count == 3 {
                res.insert(b);
            }
        }
    }
    res
}

fn adj1(p: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut res = vec![];
    for &dx in &[-1, 0, 1] {
        for &dy in &[-1, 0, 1] {
            for &dz in &[-1, 0, 1] {
                res.push((p.0 + dx, p.1 + dy, p.2 + dz));
            }
        }
    }
    res
}

fn adj2(p: &(i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut res = vec![];
    for &dx in &[-1, 0, 1] {
        for &dy in &[-1, 0, 1] {
            for &dz in &[-1, 0, 1] {
                for &dw in &[-1, 0, 1] {
                    res.push((p.0 + dx, p.1 + dy, p.2 + dz, p.3 + dw));
                }
            }
        }
    }
    res
}

adventofcode_ignore!(test, "input.txt", "output.txt");
