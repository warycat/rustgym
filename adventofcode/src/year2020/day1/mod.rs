use rustgym_util::*;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let it = reader.lines().map(|l| l.unwrap());
    let entries: Vec<i32> = it.map(|s| s.parse::<i32>().unwrap()).collect();
    let res1 = product_of_two_sum(&entries, 2020);
    let res2 = product_of_three_sum(entries, 2020);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn product_of_two_sum(entries: &[i32], sum: i32) -> i32 {
    let mut hs: HashSet<i32> = HashSet::new();
    for &x in entries {
        if hs.contains(&(sum - x)) {
            return x * (sum - x);
        }
        hs.insert(x);
    }
    0
}

fn product_of_three_sum(mut entries: Vec<i32>, sum: i32) -> i32 {
    let n = entries.len();
    entries.sort_unstable();
    for i in (0..n).rev() {
        let product = product_of_two_sum(&entries[0..i], sum - entries[i]);
        if product != 0 {
            return product * entries[i];
        }
    }
    0
}

adventofcode!(test, "input.txt", "output.txt");
