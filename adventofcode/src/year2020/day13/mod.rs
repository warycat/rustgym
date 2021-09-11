use rustgym_util::*;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let start = it.next().unwrap().parse::<i32>().unwrap();
    let words: Vec<String> = it
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let ids1: Vec<i32> = words.iter().filter_map(|w| w.parse::<i32>().ok()).collect();
    let ids2: Vec<(i32, i32)> = words
        .iter()
        .enumerate()
        .filter_map(|(i, w)| {
            if let Ok(x) = w.parse::<i32>() {
                Some((i as i32, x))
            } else {
                None
            }
        })
        .collect();
    let res1 = earliest_bus(start, &ids1);
    let res2 = earliest_time(&ids2);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn earliest_bus(start: i32, ids: &[i32]) -> i32 {
    let mut t = 0;
    loop {
        for &id in ids {
            if (start + t) % id == 0 {
                return id * t;
            }
        }
        t += 1;
    }
}

fn earliest_time(ids: &[(i32, i32)]) -> i64 {
    let mut t: i64 = 0;
    loop {
        if ids.iter().all(|&(i, id)| (t + i as i64) % id as i64 == 0) {
            return t;
        }
        let product: i64 = ids
            .iter()
            .map(|&(i, id)| {
                if (t + i as i64) % id as i64 == 0 {
                    id as i64
                } else {
                    1
                }
            })
            .product();
        t += product;
    }
}

adventofcode!(test, "input.txt", "output.txt");
